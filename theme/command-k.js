// Turns mdBook's search into a Cmd/Ctrl+K overlay: the search wrapper becomes
// a centred modal over a dimmed backdrop. Cmd/Ctrl+K, / and S all open it. mdBook keeps doing the searching,
// highlighting and keyboard handling (S, /, Esc); this only adds the shortcut,
// a click-outside to close, and the layout hooks used by custom.css.
(function () {
  var wrapper = document.getElementById("mdbook-search-wrapper");
  var toggle = document.getElementById("mdbook-search-toggle");
  var input = document.getElementById("mdbook-searchbar");
  if (!wrapper || !toggle || !input) return;

  // Wrap the form and the results in one box so the modal has a single surface.
  var box = document.createElement("div");
  box.className = "cmdk-box";
  while (wrapper.firstChild) box.appendChild(wrapper.firstChild);
  wrapper.appendChild(box);

  var mac = /Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent);
  var shortcuts = (mac ? "⌘K" : "Ctrl K") + ", / or S";
  input.placeholder = "Search recipes, crates, topics… (" + shortcuts + ")";
  input.setAttribute("aria-label", "Search the cookbook");
  toggle.title = "Search (" + shortcuts + ")";

  // The overlay is a modal dialog: announce it as one, keep Tab inside it and hand focus back
  // to wherever it came from when it closes.
  wrapper.setAttribute("role", "dialog");
  wrapper.setAttribute("aria-modal", "true");
  wrapper.setAttribute("aria-label", "Search");

  function isOpen() {
    return !wrapper.classList.contains("hidden");
  }

  var returnTo = null;
  document.addEventListener("focusin", function (e) {
    if (!wrapper.contains(e.target)) returnTo = e.target;
  });
  var wasOpen = isOpen();
  new MutationObserver(function () {
    var open = isOpen();
    if (wasOpen && !open && returnTo && document.contains(returnTo)) returnTo.focus();
    wasOpen = open;
  }).observe(wrapper, { attributes: true, attributeFilter: ["class"] });

  function focusable() {
    return Array.prototype.filter.call(
      wrapper.querySelectorAll("a[href], button, input, [tabindex]:not([tabindex='-1'])"),
      function (n) { return !n.disabled && n.getClientRects().length > 0; }
    );
  }

  function editable(node) {
    return node && (node.isContentEditable || /^(INPUT|TEXTAREA|SELECT)$/.test(node.tagName));
  }

  document.addEventListener("keydown", function (e) {
    if (e.key === "Tab" && isOpen()) {
      var nodes = focusable();
      if (!nodes.length) return;
      var first = nodes[0];
      var last = nodes[nodes.length - 1];
      if (!wrapper.contains(document.activeElement)) {
        e.preventDefault();
        first.focus();
      } else if (e.shiftKey && document.activeElement === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault();
        first.focus();
      }
      return;
    }
    if ((e.metaKey || e.ctrlKey) && !e.altKey && e.key.toLowerCase() === "k") {
      // Leave the shortcut alone while typing in another field or if the page already handled it.
      if (e.defaultPrevented || (editable(e.target) && e.target !== input)) return;
      e.preventDefault();
      if (isOpen()) {
        input.focus();
        input.select();
      } else {
        toggle.click();
      }
    }
  });

  // Click or tap on the backdrop closes the overlay, like Esc does.
  wrapper.addEventListener("click", function (e) {
    if (e.target === wrapper && isOpen()) toggle.click();
  });

  // A dimmed magnifier at the bottom of the page, centred between the previous
  // and next links. Narrow layouts use the nav row; wide ones, which have side
  // arrows instead, get a centred copy under the content (CSS picks which shows).
  var nav = document.querySelector(".nav-wrapper");
  if (nav) {
    var make = function (className) {
      var b = document.createElement("button");
      b.type = "button";
      b.className = "bottom-search " + className;
      b.title = "Search (" + shortcuts + ")";
      b.setAttribute("aria-label", "Search the cookbook");
      b.innerHTML =
        '<svg viewBox="0 0 24 24" width="22" height="22" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><circle cx="10.5" cy="10.5" r="6.5"/><path d="M15.5 15.5 21 21"/></svg>';
      b.addEventListener("click", function () {
        if (!isOpen()) toggle.click();
        input.focus();
      });
      return b;
    };
    nav.insertBefore(make("in-nav"), nav.lastElementChild);
    var wide = document.createElement("div");
    wide.className = "bottom-search-wide";
    wide.appendChild(make("standalone"));
    nav.parentNode.insertBefore(wide, nav.nextSibling);
  }
})();
