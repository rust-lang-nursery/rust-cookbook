## Check external command version for compatibility

[![semver-badge]][semver] [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os]

Runs `git --version` using [`Command`], then parses the version number into a
[`semver::Version`] using [`Version::parse`]. [`VersionReq::matches`] compares
[`semver::VersionReq`] to the parsed version.  The command output resembles
"git version x.y.z".

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate semver;

use std::process::Command;
use semver::{Version, VersionReq};
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Utf8(std::string::FromUtf8Error);
#         SemVer(semver::SemVerError);
#         SemVerReq(semver::ReqParseError);
#     }
# }

fn run() -> Result<()> {
    let version_constraint = "> 1.12.0";
    let version_test = VersionReq::parse(version_constraint)?;
    let output = Command::new("git").arg("--version").output()?;

    if !output.status.success() {
        bail!("Command executed with failing error code");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let version = stdout.split(" ").last().ok_or_else(|| {
        "Invalid command output"
    })?;
    let parsed_version = Version::parse(version)?;

    if !version_test.matches(&parsed_version) {
        bail!("Command version lower than minimum supported version (found {}, need {})",
            parsed_version, version_constraint);
    }

    Ok(())
}
#
# quick_main!(run);
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`semver::VersionReq`]: https://docs.rs/semver/*/semver/struct.VersionReq.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse
[`VersionReq::matches`]: https://docs.rs/semver/*/semver/struct.VersionReq.html#method.matches
