//! Runs the recipes against a real GTK application. Each test starts its own virtual display
//! (Xvfb), D-Bus session and accessibility bus, so nothing touches the desktop of the machine
//! running the tests.
//!
//! Needs `Xvfb`, `dbus-daemon`, `at-spi2-core`, `zenity` and `xdotool`. Without them the tests
//! skip, unless `ACCESSIBILITY_LIVE_TESTS=required` is set (as in CI), which makes them fail.

#![cfg(target_os = "linux")]

use std::env;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const BUS_LAUNCHER: &str = "/usr/libexec/at-spi-bus-launcher";
const TOOLS: [&str; 4] = ["Xvfb", "dbus-daemon", "zenity", "xdotool"];

/// A private display, session bus and accessibility bus, stopped when dropped.
struct Session {
    env: Vec<(&'static str, String)>,
    children: Vec<Child>,
}

impl Session {
    /// Starts a session, or returns `None` (after saying why) if the tools are missing.
    fn start() -> Option<Session> {
        let missing: Vec<&str> = TOOLS
            .into_iter()
            .filter(|tool| !on_path(tool))
            .chain((!Path::new(BUS_LAUNCHER).exists()).then_some("at-spi2-core"))
            .collect();
        if !missing.is_empty() {
            let reason = format!("missing {}", missing.join(", "));
            if env::var("ACCESSIBILITY_LIVE_TESTS").as_deref() == Ok("required") {
                panic!("{reason}");
            }
            eprintln!("skipping the live accessibility tests: {reason}");
            return None;
        }

        let mut session = Session {
            env: Vec::new(),
            children: Vec::new(),
        };
        // Xvfb picks a free display number and writes it to the given file descriptor.
        let display = session.spawn_and_read_line(
            "Xvfb",
            &[
                "-displayfd",
                "1",
                "-nolisten",
                "tcp",
                "-screen",
                "0",
                "1024x768x24",
            ],
        );
        session.env.push(("DISPLAY", format!(":{display}")));
        let bus = session.spawn_and_read_line(
            "dbus-daemon",
            &["--session", "--nofork", "--print-address=1"],
        );
        session.env.push(("DBUS_SESSION_BUS_ADDRESS", bus));
        session.spawn(BUS_LAUNCHER, &["--launch-immediately"]);
        Some(session)
    }

    fn command(&self, program: &str) -> Command {
        let mut command = Command::new(program);
        command
            .env_remove("AT_SPI_BUS_ADDRESS")
            .env_remove("NO_AT_BRIDGE");
        command.envs(self.env.iter().map(|(key, value)| (key, value)));
        command
    }

    fn spawn(&mut self, program: &str, args: &[&str]) {
        let child = self
            .command(program)
            .args(args)
            .process_group(0)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        self.children.push(child);
    }

    fn spawn_and_read_line(&mut self, program: &str, args: &[&str]) -> String {
        let mut child = self
            .command(program)
            .args(args)
            .process_group(0)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let mut line = String::new();
        BufReader::new(child.stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        self.children.push(child);
        line.trim().to_string()
    }

    /// Opens a zenity text-entry dialog and waits until it is on the accessibility bus.
    fn open_dialog(&mut self, title: &str, label: &str) {
        self.spawn("zenity", &["--entry", "--title", title, "--text", label]);
        let deadline = Instant::now() + Duration::from_secs(20);
        while Instant::now() < deadline {
            if self.run("list_applications", &[]).0.contains("zenity") {
                return;
            }
            thread::sleep(Duration::from_millis(250));
        }
        panic!("zenity did not appear on the accessibility bus");
    }

    /// Runs one of the recipes and returns its standard output and whether it succeeded.
    fn run(&self, recipe: &str, args: &[&str]) -> (String, bool) {
        let mut child = self
            .command(&recipe_path(recipe))
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(30);
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if Instant::now() > deadline {
                child.kill().ok();
                panic!("{recipe} did not finish within 30 seconds");
            }
            thread::sleep(Duration::from_millis(50));
        };
        let mut stdout = String::new();
        child
            .stdout
            .take()
            .unwrap()
            .read_to_string(&mut stdout)
            .unwrap();
        (stdout, status.success())
    }

    /// Gives the window keyboard focus (there is no window manager to do it) and presses keys.
    fn press_keys(&self, window_title: &str, keys: &[&str]) {
        let search = self
            .command("xdotool")
            .args(["search", "--onlyvisible", "--name", window_title])
            .output()
            .unwrap();
        let window = String::from_utf8(search.stdout).unwrap();
        let window = window
            .lines()
            .next()
            .expect("the dialog window is not on the display");
        let focus = self
            .command("xdotool")
            .args(["windowfocus", "--sync", window])
            .status();
        assert!(focus.unwrap().success());
        for key in keys {
            thread::sleep(Duration::from_millis(400));
            assert!(
                self.command("xdotool")
                    .args(["key", key])
                    .status()
                    .unwrap()
                    .success()
            );
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Each helper leads its own process group, so this also stops what it started, such as
        // the bus launcher's own `dbus-daemon`.
        for child in self.children.iter_mut().rev() {
            let group = format!("-{}", child.id());
            Command::new("kill")
                .args(["-KILL", "--", &group])
                .status()
                .ok();
            child.wait().ok();
        }
    }
}

fn on_path(tool: &str) -> bool {
    env::var_os("PATH")
        .is_some_and(|path| env::split_paths(&path).any(|dir| dir.join(tool).is_file()))
}

fn recipe_path(recipe: &str) -> String {
    match recipe {
        "list_applications" => env!("CARGO_BIN_EXE_list_applications"),
        "dump_tree" => env!("CARGO_BIN_EXE_dump_tree"),
        "audit_labels" => env!("CARGO_BIN_EXE_audit_labels"),
        "track_focus" => env!("CARGO_BIN_EXE_track_focus"),
        _ => panic!("no recipe named {recipe}"),
    }
    .to_string()
}

#[test]
fn recipes_read_a_labeled_dialog() {
    let Some(mut session) = Session::start() else {
        return;
    };
    session.open_dialog("Cookbook", "Your name:");

    let (apps, ok) = session.run("list_applications", &[]);
    assert!(
        ok && apps.contains("zenity, 1 window(s)"),
        "list_applications printed:\n{apps}"
    );

    let (tree, ok) = session.run("dump_tree", &["zenity"]);
    assert!(ok, "dump_tree failed:\n{tree}");
    for expected in [
        r#"dialog "Cookbook""#,
        r#"text "Your name:""#,
        r#"button "OK""#,
        r#"button "Cancel""#,
    ] {
        assert!(
            tree.contains(expected),
            "dump_tree has no {expected}:\n{tree}"
        );
    }

    let (audit, ok) = session.run("audit_labels", &["zenity"]);
    assert!(
        ok,
        "audit_labels failed on a dialog where everything is labeled:\n{audit}"
    );
}

#[test]
fn audit_reports_an_unlabeled_field() {
    let Some(mut session) = Session::start() else {
        return;
    };
    session.open_dialog("Unlabeled", "");

    let (audit, ok) = session.run("audit_labels", &["zenity"]);
    assert!(!ok, "audit_labels passed a field with no label:\n{audit}");
    assert!(
        audit.contains("unlabeled text"),
        "audit_labels printed:\n{audit}"
    );
}

#[test]
fn focus_tracker_announces_each_control() {
    let Some(mut session) = Session::start() else {
        return;
    };
    session.open_dialog("Focus", "Your name:");

    let mut tracker = session
        .command(&recipe_path("track_focus"))
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let (lines, announcements) = mpsc::channel();
    let stdout = tracker.stdout.take().unwrap();
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if lines.send(line).is_err() {
                break;
            }
        }
    });
    let ready = announcements.recv_timeout(Duration::from_secs(10));
    assert!(ready.is_ok_and(|line| line.starts_with("Move the keyboard focus")));

    session.press_keys("Focus", &["Tab", "Tab", "Tab"]);
    let mut heard = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(10);
    while !heard.iter().any(|line| line == "OK, button") {
        let left = deadline.saturating_duration_since(Instant::now());
        match announcements.recv_timeout(left) {
            Ok(line) => heard.push(line),
            Err(_) => break,
        }
    }
    tracker.kill().ok();
    tracker.wait().ok();
    assert!(
        heard.iter().any(|line| line == "OK, button"),
        "track_focus announced {heard:?}"
    );
}
