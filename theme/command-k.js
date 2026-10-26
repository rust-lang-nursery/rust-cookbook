// Turns mdBook's search into a Cmd/Ctrl+K overlay: the search wrapper becomes
// a centred modal over a dimmed backdrop. mdBook keeps doing the searching,
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
  var hint = mac ? "⌘K" : "Ctrl K";
  input.placeholder = "Search recipes, crates, topics… (" + hint + ")";
  toggle.title = "Search (" + hint + " or /)";

  function isOpen() {
    return !wrapper.classList.contains("hidden");
  }

  document.addEventListener("keydown", function (e) {
    if ((e.metaKey || e.ctrlKey) && !e.altKey && e.key.toLowerCase() === "k") {
      e.preventDefault();
      if (isOpen()) {
        input.focus();
        input.select();
      } else {
        toggle.click();
      }
    }
  });

  // Click on the backdrop closes the overlay, like Esc does.
  wrapper.addEventListener("mousedown", function (e) {
    if (e.target === wrapper && isOpen()) toggle.click();
  });
})();
