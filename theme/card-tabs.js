// The tab on each recipe card sits further along the card's top edge the further down the list the
// card is, so the stack of cards reads as a progress indicator. Without JavaScript every tab stays
// at the left (see --tab-progress in custom.css).
(function () {
  function init() {
    var tables = document.querySelectorAll(".content table");
    for (var t = 0; t < tables.length; t++) {
      if (!tables[t].querySelector(".crate-chip")) continue;
      var rows = tables[t].querySelectorAll("tbody tr");
      for (var i = 0; i < rows.length; i++) {
        rows[i].style.setProperty("--tab-progress", rows.length > 1 ? i / (rows.length - 1) : 0);
      }
    }
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init);
  else init();
})();
