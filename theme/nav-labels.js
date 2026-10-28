// Adds the destination chapter's title to the previous/next page links, using
// the sidebar table of contents as the source of titles.
(function () {
  function titles() {
    var map = {};
    var links = document.querySelectorAll("mdbook-sidebar-scrollbox a[href]");
    for (var i = 0; i < links.length; i++) {
      var copy = links[i].cloneNode(true);
      var number = copy.querySelector("strong");
      if (number) number.remove();
      var text = copy.textContent.trim();
      if (text && !(links[i].pathname in map)) map[links[i].pathname] = text;
    }
    return map;
  }

  function label(link, title) {
    var span = document.createElement("span");
    span.className = "nav-label";
    var kicker = document.createElement("span");
    kicker.className = "nav-kicker";
    kicker.textContent = link.classList.contains("next") ? "Next" : "Previous";
    var name = document.createElement("span");
    name.className = "nav-title";
    name.textContent = title;
    span.appendChild(kicker);
    span.appendChild(document.createTextNode(" "));
    span.appendChild(name);
    link.appendChild(span);
    // The accessible name must contain the visible text (WCAG 2.5.3).
    link.setAttribute("aria-label", kicker.textContent + " " + title);
  }

  function init(retry) {
    var map = titles();
    if (!Object.keys(map).length && retry) {
      setTimeout(function () { init(false); }, 300);
      return;
    }
    var links = document.querySelectorAll("a.nav-chapters, a.mobile-nav-chapters");
    for (var i = 0; i < links.length; i++) {
      var title = map[links[i].pathname];
      if (title) label(links[i], title);
    }
  }

  if (document.readyState === "complete") init(true);
  else window.addEventListener("load", function () { init(true); });
})();
