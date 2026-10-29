// Crate pills are rendered into the pages at build time (`mdbook-chips`), including
// each crate's Cargo.toml line and repository link. This script only makes the copy buttons work,
// loads the search index on demand (window.CookbookIndex) and builds pills for the scripts that
// list crates dynamically (window.CookbookChips).
(function () {
  var COPY_ICON =
    '<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M4 2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V2zm2-.5a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5h6a.5.5 0 0 0 .5-.5V2a.5.5 0 0 0-.5-.5H6zM2 5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-1H8.5v1a.5.5 0 0 1-.5.5H2a.5.5 0 0 1-.5-.5V7A.5.5 0 0 1 2 6.5h1V5H2z"/></svg>';
  var CHECK_ICON =
    '<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M13.78 3.97a.75.75 0 0 1 0 1.06l-6.5 6.5a.75.75 0 0 1-1.06 0l-3-3a.75.75 0 1 1 1.06-1.06l2.47 2.47 5.97-5.97a.75.75 0 0 1 1.06 0z"/></svg>';
  var GITHUB_ICON =
    '<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82a7.6 7.6 0 0 1 4 0c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>';
  var PLAYGROUND_ICON =
    '<svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true"><path fill="currentColor" d="M4 2.2v11.6a.6.6 0 0 0 .92.5l9-5.8a.6.6 0 0 0 0-1l-9-5.8A.6.6 0 0 0 4 2.2z"/></svg>';
  var STD_REPO = "https://github.com/rust-lang/rust";

  var root = typeof path_to_root === "string" ? path_to_root : "";

  // The search index is only fetched when a page or the search box needs it.
  var loading = null;
  window.CookbookIndex = {
    load: function () {
      if (window.COOKBOOK_INDEX) return Promise.resolve(window.COOKBOOK_INDEX);
      if (!loading) {
        loading = new Promise(function (resolve, reject) {
          var script = document.createElement("script");
          script.src = root + "cookbook-index.js";
          script.onload = function () { resolve(window.COOKBOOK_INDEX); };
          script.onerror = function () { loading = null; reject(new Error("cookbook-index.js")); };
          document.head.appendChild(script);
        });
      }
      return loading;
    },
  };

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

  // One handler for every copy button, whether rendered at build time or created below.
  var timers = new WeakMap();
  document.addEventListener("click", function (event) {
    var button = event.target.closest && event.target.closest(".crate-chip-copy");
    if (!button) return;
    event.preventDefault();
    event.stopPropagation();
    var chip = button.closest(".crate-chip");
    var text = button.getAttribute("data-copy");
    copyText(text).then(function () {
      var timer = timers.get(button);
      if (timer) timer.forEach(clearTimeout);
      // "Copied" first, then the line that went to the clipboard.
      chip.classList.add("copied");
      button.innerHTML = CHECK_ICON + "<span>Copied</span>";
      button.setAttribute("aria-label", "Copied " + text);
      var showValue = setTimeout(function () {
        var code = document.createElement("code");
        code.textContent = text;
        button.innerHTML = CHECK_ICON;
        button.appendChild(code);
      }, 900);
      var reset = setTimeout(function () {
        chip.classList.remove("copied");
        button.innerHTML = COPY_ICON;
        button.setAttribute("aria-label", "Copy " + text);
      }, 3500);
      timers.set(button, [showValue, reset]);
    }, function () {});
  });

  function crateMeta(name) {
    var index = window.COOKBOOK_INDEX;
    var i = index && index.crates.indexOf(name);
    var meta = i >= 0 && index.meta ? index.meta[i] : null;
    return {
      snippet: (meta && meta[0]) || name,
      repo: name === "std" ? STD_REPO : meta && meta[1],
      playground: (meta && meta[2]) || null,
    };
  }

  // A pill for a crate: name (links to docs.rs), copy button, repository link. Needs the index.
  window.CookbookChips = {
    crate: function (name, href) {
      var meta = crateMeta(name);
      var chip = document.createElement("span");
      chip.className = "crate-chip";

      var link = document.createElement("a");
      link.className = "crate-chip-name";
      link.href = href || (name === "std" ? "https://doc.rust-lang.org/std" : "https://docs.rs/" + name);
      link.textContent = name;
      link.title = name === "std" ? "Open the std documentation" : "Open " + name + " on docs.rs";
      chip.appendChild(link);

      // std ships with Rust, so there is nothing to add to Cargo.toml.
      if (name !== "std") {
        var button = document.createElement("button");
        button.type = "button";
        button.className = "crate-chip-copy";
        button.setAttribute("data-copy", meta.snippet);
        button.title = "Copy " + meta.snippet;
        button.setAttribute("aria-label", "Copy " + meta.snippet);
        button.innerHTML = COPY_ICON;
        chip.appendChild(button);
      }
      if (meta.playground) {
        var play = document.createElement("a");
        play.className = "crate-chip-playground";
        play.href = meta.playground;
        play.title = "Try " + name + " in the Rust Playground";
        play.setAttribute("aria-label", "Try " + name + " in the Rust Playground");
        play.innerHTML = PLAYGROUND_ICON;
        chip.appendChild(play);
      }
      if (meta.repo) {
        var repo = document.createElement("a");
        repo.className = "crate-chip-repo";
        repo.href = meta.repo;
        repo.title = "Open " + name + " source repository";
        repo.setAttribute("aria-label", name + " source repository");
        repo.innerHTML = GITHUB_ICON;
        chip.appendChild(repo);
      }
      return chip;
    },

    // A pill for a topic, e.g. "Science".
    category: function (label, href) {
      var a = document.createElement("a");
      a.className = "crate-chip cat-chip";
      a.href = href;
      a.textContent = label;
      return a;
    },
  };
})();
