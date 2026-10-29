// The tab on each recipe card and each topic card sits further along the card's top edge the further
// down the list the card is, so the stack of cards reads as a progress indicator. Without JavaScript
// every tab stays at the left (see --tab-progress in custom.css).
(function () {
  function track(cards) {
    for (var i = 0; i < cards.length; i++) {
      cards[i].style.setProperty("--tab-progress", cards.length > 1 ? i / (cards.length - 1) : 0);
    }
  }

  function init() {
    track(document.querySelectorAll(".section-grid li > a"));
    var tables = document.querySelectorAll(".content table");
    for (var t = 0; t < tables.length; t++) {
      if (!tables[t].querySelector(".crate-chip")) continue;
      track(tables[t].querySelectorAll("tbody tr"));
    }
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init);
  else init();
})();
