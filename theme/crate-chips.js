// Replaces crates.io version badge images with text chips whose copy button
// copies the crate name. Falls back to the original images without JS.
(function () {
  var BADGE = /img\.shields\.io\/crates\/v\/([A-Za-z0-9_-]+)/;
  // `std` has no crates.io page; its badge is a static shields.io image.
  var STD_BADGE = /img\.shields\.io\/badge\/std-/;
  var STD_REPO = "https://github.com/rust-lang/rust";
  var COPY_ICON =
    '<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M4 2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V2zm2-.5a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5h6a.5.5 0 0 0 .5-.5V2a.5.5 0 0 0-.5-.5H6zM2 5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-1H8.5v1a.5.5 0 0 1-.5.5H2a.5.5 0 0 1-.5-.5V7A.5.5 0 0 1 2 6.5h1V5H2z"/></svg>';
  var CHECK_ICON =
    '<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M13.78 3.97a.75.75 0 0 1 0 1.06l-6.5 6.5a.75.75 0 0 1-1.06 0l-3-3a.75.75 0 1 1 1.06-1.06l2.47 2.47 5.97-5.97a.75.75 0 0 1 1.06 0z"/></svg>';

  function copyText(text) {
    if (navigator.clipboard && window.isSecureContext) {
      return navigator.clipboard.writeText(text);
    }
    return new Promise(function (resolve, reject) {
      var ta = document.createElement("textarea");
      ta.value = text;
      ta.setAttribute("readonly", "");
      ta.style.position = "fixed";
      ta.style.opacity = "0";
      document.body.appendChild(ta);
      ta.select();
      try {
        document.execCommand("copy") ? resolve() : reject();
      } catch (e) {
        reject(e);
      } finally {
        document.body.removeChild(ta);
      }
    });
  }

  var GITHUB_ICON =
    '<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82a7.6 7.6 0 0 1 4 0c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>';
  var CACHE_PREFIX = "crate-info:";
  var CACHE_MS = 7 * 24 * 60 * 60 * 1000;

  // "1.10.3" -> "1.10", "0.12.1" -> "0.12", "0.0.4" -> "0.0.4": a Cargo.toml requirement.
  function requirement(version) {
    var parts = String(version).split("-")[0].split(".");
    if (parts.length < 3 || !/^\d+$/.test(parts[0])) return null;
    if (parts[0] !== "0") return parts[0] + "." + parts[1];
    return parts[1] !== "0" ? "0." + parts[1] : parts.slice(0, 3).join(".");
  }

  // Resolves {repo, version} for a crate from crates.io, cached in localStorage.
  function crateInfo(name) {
    if (name === "std") return Promise.resolve({ repo: STD_REPO, version: null });
    try {
      var hit = JSON.parse(localStorage.getItem(CACHE_PREFIX + name));
      if (hit && Date.now() - hit.at < CACHE_MS) return Promise.resolve(hit.info);
    } catch (e) {}
    return fetch("https://crates.io/api/v1/crates/" + encodeURIComponent(name))
      .then(function (r) { return r.ok ? r.json() : null; })
      .then(function (data) {
        var crate = data && data.crate;
        var repo = crate && crate.repository;
        var latest = crate && (crate.max_stable_version || crate.newest_version);
        var info = {
          repo: typeof repo === "string" && /^https:\/\//.test(repo) ? repo : null,
          version: latest ? requirement(latest) : null,
        };
        try {
          localStorage.setItem(CACHE_PREFIX + name, JSON.stringify({ at: Date.now(), info: info }));
        } catch (e) {}
        return info;
      })
      .catch(function () { return { repo: null, version: null }; });
  }

  function addRepoLink(chip, name, url) {
    var a = document.createElement("a");
    a.className = "crate-chip-repo";
    a.href = url;
    a.title = "Open " + name + " source repository";
    a.setAttribute("aria-label", name + " source repository");
    a.innerHTML = GITHUB_ICON;
    chip.appendChild(a);
  }

  function makeChip(img) {
    var src = img.getAttribute("src") || "";
    var match = BADGE.exec(src);
    var name = match ? match[1] : STD_BADGE.test(src) ? "std" : null;
    if (!name) return;
    var link = img.closest("a");
    var chip = document.createElement("span");
    chip.className = "crate-chip";

    if (link) {
      link.classList.add("crate-chip-name");
      link.textContent = name;
      link.title = name === "std" ? "Open the std documentation" : "Open " + name + " on docs.rs";
      link.parentNode.insertBefore(chip, link);
      chip.appendChild(link);
    } else {
      var label = document.createElement("span");
      label.className = "crate-chip-name";
      label.textContent = name;
      img.parentNode.insertBefore(chip, img);
      chip.appendChild(label);
      img.remove();
    }

    var button = document.createElement("button");
    button.type = "button";
    button.className = "crate-chip-copy";
    // Ready to paste under [dependencies]; just the name until the version is known.
    var snippet = name;
    function labelFor(prefix) { return prefix + " " + snippet; }
    button.title = labelFor("Copy");
    button.setAttribute("aria-label", labelFor("Copy"));
    button.innerHTML = COPY_ICON;
    var timer;
    button.addEventListener("click", function (event) {
      event.preventDefault();
      event.stopPropagation();
      copyText(snippet).then(function () {
        chip.classList.add("copied");
        button.innerHTML = CHECK_ICON + "<span>Copied</span>";
        button.setAttribute("aria-label", labelFor("Copied"));
        clearTimeout(timer);
        timer = setTimeout(function () {
          chip.classList.remove("copied");
          button.innerHTML = COPY_ICON;
          button.setAttribute("aria-label", labelFor("Copy"));
        }, 1500);
      }, function () {});
    });
    // std ships with Rust, so there is nothing to add to Cargo.toml.
    if (name !== "std") chip.appendChild(button);
    crateInfo(name).then(function (info) {
      if (info.version) {
        snippet = name + ' = "' + info.version + '"';
        button.title = labelFor("Copy");
        button.setAttribute("aria-label", labelFor("Copy"));
      }
      if (info.repo) addRepoLink(chip, name, info.repo);
    });
  }

  function init() {
    var imgs = document.querySelectorAll(
      ".content img[src*='img.shields.io/crates/v/'], .content img[src*='img.shields.io/badge/std-']"
    );
    for (var i = 0; i < imgs.length; i++) makeChip(imgs[i]);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
