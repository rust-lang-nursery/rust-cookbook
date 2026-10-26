// Enhances the book's search box with crate and topic matches from
// window.COOKBOOK_INDEX (see `cargo xtask search-index`), from the first
// character typed. mdBook's own full-text results are left untouched below it.
(function () {
  var index = window.COOKBOOK_INDEX;
  var input = document.getElementById("mdbook-searchbar");
  var anchor = document.getElementById("mdbook-searchresults-outer");
  if (!index || !input || !anchor) return;

  var chips = window.CookbookChips;
  if (!chips) return;

  var root = typeof path_to_root === "string" ? path_to_root : "";
  var LIMITS = { c: 4, s: 3, r: 6 };
  var RECIPES_SHOWN = 3;

  var panel = document.createElement("div");
  panel.id = "cookbook-quicksearch";
  panel.setAttribute("role", "region");
  panel.setAttribute("aria-label", "Crate and topic matches");
  panel.hidden = true;
  anchor.parentNode.insertBefore(panel, anchor);

  var crateRecipes = null;
  function recipesByCrate() {
    if (!crateRecipes) {
      crateRecipes = index.crates.map(function () { return []; });
      index.recipes.forEach(function (r, i) {
        r[3].forEach(function (c) { crateRecipes[c].push(i); });
      });
    }
    return crateRecipes;
  }

  function el(tag, className, text) {
    var node = document.createElement(tag);
    if (className) node.className = className;
    if (text != null) node.textContent = text;
    return node;
  }

  function link(href, className, text) {
    var a = el("a", className, text);
    a.href = href;
    return a;
  }

  function plural(n) {
    return n + (n === 1 ? " recipe" : " recipes");
  }

  function recipeItem(i) {
    var r = index.recipes[i];
    var li = el("li");
    li.appendChild(link(root + r[1], "qs-recipe", r[0].replace(/`/g, "")));
    li.appendChild(el("span", "qs-muted", " " + index.sections[r[2]][0]));
    return li;
  }

  function crateEntry(c) {
    var name = index.crates[c];
    var recipes = recipesByCrate()[c];
    var box = el("div", "qs-crate");
    var head = el("div", "qs-head");
    var chip = chips.crate(name);
    head.appendChild(chip);
    head.appendChild(el("span", "qs-muted", plural(recipes.length)));
    box.appendChild(head);

    // A "show 1 more" button costs as much space as the recipe itself, so just show it.
    var shown = recipes.length - RECIPES_SHOWN === 1 ? recipes.length : RECIPES_SHOWN;
    var list = el("ul");
    recipes.forEach(function (r, n) {
      var li = recipeItem(r);
      if (n >= shown) li.hidden = true;
      list.appendChild(li);
    });
    box.appendChild(list);
    if (recipes.length > shown) {
      var more = el("button", "qs-more", "Show " + (recipes.length - shown) + " more");
      more.type = "button";
      more.addEventListener("click", function () {
        Array.prototype.forEach.call(list.children, function (li) { li.hidden = false; });
        more.remove();
      });
      box.appendChild(more);
    }
    return box;
  }

  function topicEntry(s) {
    var section = index.sections[s];
    var count = index.recipes.filter(function (r) { return r[2] === s; }).length;
    var div = el("div", "qs-topic");
    div.appendChild(chips.category(section[0], root + section[1]));
    div.appendChild(el("span", "qs-muted", plural(count)));
    return div;
  }

  function group(title, nodes) {
    if (!nodes.length) return null;
    var wrap = el("div", "qs-group");
    wrap.appendChild(el("div", "qs-title", title));
    nodes.forEach(function (n) { wrap.appendChild(n); });
    return wrap;
  }

  // entries: ["c12", "s3", "r45", ...] in rank order
  function render(entries) {
    var crates = [], topics = [], recipes = [];
    // Recipes already listed under a matching crate are not repeated below.
    var covered = {};
    entries.forEach(function (e) {
      if (e[0] === "c" && crates.length < LIMITS.c) {
        crates.push(crateEntry(+e.slice(1)));
        recipesByCrate()[+e.slice(1)].forEach(function (r) { covered[r] = true; });
      }
    });
    entries.forEach(function (e) {
      var id = +e.slice(1);
      if (e[0] === "s" && topics.length < LIMITS.s) topics.push(topicEntry(id));
      else if (e[0] === "r" && recipes.length < LIMITS.r && !covered[id]) {
        var ul = el("ul");
        ul.appendChild(recipeItem(id));
        recipes.push(ul);
      }
    });
    panel.textContent = "";
    [group("Crates", crates), group("Topics", topics), group("Recipes", recipes)].forEach(
      function (g) { if (g) panel.appendChild(g); }
    );
    panel.hidden = !panel.firstChild;
  }

  // 3 = exact, 2 = starts with the query, 1 = contains it (for one or two
  // characters: a word in it starts with the query), 0 = no match.
  function matcher(q) {
    var short = q.length < 3;
    var wordStart = short
      ? new RegExp("(^|[^a-z0-9])" + q.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"))
      : null;
    function contains(t) {
      return short ? wordStart.test(t) : t.indexOf(q) !== -1;
    }
    return {
      score: function (text) {
        var t = text.toLowerCase();
        if (t === q) return 3;
        if (t.indexOf(q) === 0) return 2;
        return contains(t) ? 1 : 0;
      },
      contains: contains,
    };
  }

  // The whole index is a few hundred entries, so scan all of it on every keystroke.
  function search(q) {
    var tokens = q.split(/\s+/);
    var m = matcher(q);
    var tokenMatchers = tokens.map(matcher);
    var hits = [];
    index.crates.forEach(function (name, c) {
      var s = m.score(name);
      if (s) hits.push([0, -s, -recipesByCrate()[c].length, "c" + c]);
    });
    index.sections.forEach(function (section, i) {
      var s = Math.max.apply(null, [m.score(section[0])].concat(section[2].map(m.score)));
      if (s) hits.push([1, -s, i, "s" + i]);
    });
    index.recipes.forEach(function (r, i) {
      var hay = (r[0] + " " + r[3].map(function (c) { return index.crates[c]; }).join(" ")).toLowerCase();
      var ok = tokenMatchers.every(function (t) { return t.contains(hay); });
      if (ok) hits.push([2, -m.score(r[0]), i, "r" + i]);
    });
    hits.sort(function (a, b) { return a[0] - b[0] || a[1] - b[1] || a[2] - b[2]; });
    return hits.map(function (h) { return h[3]; });
  }

  function onInput() {
    var q = input.value.trim().toLowerCase();
    if (!q) {
      panel.hidden = true;
      return;
    }
    render(search(q));
  }

  input.addEventListener("input", onInput);
  if (input.value) onInput();
})();
