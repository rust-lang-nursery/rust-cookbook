// Builds the crate index page (src/crates.md), the alphabetical recipe index
// (src/recipe-index.md) and the recipe counts on the home page topic grid, all
// from window.COOKBOOK_INDEX.
(function () {
  var index;
  var SHOWN = 5;
  var root = typeof path_to_root === "string" ? path_to_root : "";

  function el(tag, className, text) {
    var node = document.createElement(tag);
    if (className) node.className = className;
    if (text != null) node.textContent = text;
    return node;
  }

  function plural(n) {
    return n + (n === 1 ? " recipe" : " recipes");
  }

  function crateIndex(host) {
    var chips = window.CookbookChips;
    var byCrate = index.crates.map(function () { return []; });
    index.recipes.forEach(function (r, i) {
      r[3].forEach(function (c) { byCrate[c].push(i); });
    });

    var filter = el("input", "ci-filter");
    filter.type = "search";
    filter.placeholder = "Filter crates";
    filter.setAttribute("aria-label", "Filter crates");
    host.appendChild(filter);

    var grid = el("div", "ci-grid");
    host.appendChild(grid);

    var cards = index.crates
      .map(function (name, c) { return { name: name, c: c }; })
      .sort(function (a, b) {
        return byCrate[b.c].length - byCrate[a.c].length || a.name.localeCompare(b.name);
      })
      .map(function (crate) {
        var card = el("section", "ci-card");
        var head = el("div", "ci-head");
        head.appendChild(chips ? chips.crate(crate.name) : el("span", null, crate.name));
        head.appendChild(el("span", "qs-muted", plural(byCrate[crate.c].length)));
        card.appendChild(head);
        var recipes = byCrate[crate.c];
        // Showing all but one is silly; only collapse when at least two are hidden.
        var shown = recipes.length - SHOWN >= 2 ? SHOWN : recipes.length;
        var list = el("ul");
        recipes.forEach(function (i, n) {
          var r = index.recipes[i];
          var a = el("a", null, r[0].replace(/`/g, ""));
          a.href = root + r[1];
          var li = el("li");
          li.hidden = n >= shown;
          li.appendChild(a);
          list.appendChild(li);
        });
        card.appendChild(list);
        if (recipes.length > shown) {
          var more = el("button", "ci-more", "Show " + (recipes.length - shown) + " more");
          more.type = "button";
          more.addEventListener("click", function () {
            Array.prototype.forEach.call(list.children, function (li) { li.hidden = false; });
            more.remove();
          });
          card.appendChild(more);
        }
        grid.appendChild(card);
        return { name: crate.name, node: card };
      });

    filter.addEventListener("input", function () {
      var q = filter.value.trim().toLowerCase();
      cards.forEach(function (card) {
        card.node.hidden = q !== "" && card.name.toLowerCase().indexOf(q) === -1;
      });
    });
  }

  function recipeIndex(host) {
    var entries = index.recipes
      .map(function (r) {
        return { title: r[0].replace(/`/g, ""), url: r[1], section: index.sections[r[2]][0] };
      })
      .sort(function (a, b) {
        return a.title.localeCompare(b.title, undefined, { sensitivity: "base" });
      });

    var groups = {};
    var letters = [];
    entries.forEach(function (e) {
      var first = e.title.charAt(0).toUpperCase();
      var letter = /[A-Z]/.test(first) ? first : "#";
      if (!groups[letter]) {
        groups[letter] = [];
        letters.push(letter);
      }
      groups[letter].push(e);
    });

    var nav = el("nav", "ri-letters");
    nav.setAttribute("aria-label", "Jump to letter");
    host.appendChild(nav);

    letters.forEach(function (letter) {
      var a = el("a", null, letter);
      a.href = "#index-" + (letter === "#" ? "other" : letter);
      nav.appendChild(a);

      var heading = el("h2", "ri-letter", letter);
      heading.id = "index-" + (letter === "#" ? "other" : letter);
      host.appendChild(heading);
      var list = el("ul", "ri-list");
      groups[letter].forEach(function (e) {
        var li = el("li");
        var link = el("a", null, e.title);
        link.href = root + e.url;
        li.appendChild(link);
        li.appendChild(el("span", "ri-section", e.section));
        list.appendChild(li);
      });
      host.appendChild(list);
    });
  }

  function topicCounts(grid) {
    var counts = {};
    index.recipes.forEach(function (r) {
      var url = index.sections[r[2]][1];
      counts[url] = (counts[url] || 0) + 1;
    });
    Array.prototype.forEach.call(grid.querySelectorAll("a[href]"), function (a) {
      var n = counts[a.getAttribute("href").replace(/^\.\//, "")];
      if (n) a.appendChild(el("span", "sg-count", plural(n)));
    });
  }

  function init() {
    var host = document.getElementById("crate-index");
    var recipes = document.getElementById("recipe-index");
    var grid = document.querySelector(".section-grid");
    if (!host && !recipes && !grid) return;
    window.CookbookIndex.load().then(function (loaded) {
      index = loaded;
      if (host) crateIndex(host);
      if (recipes) recipeIndex(recipes);
      if (grid) topicCounts(grid);
    }, function () {});
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init);
  else init();
})();
