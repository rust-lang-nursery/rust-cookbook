// Runs axe-core (WCAG 2.0/2.1/2.2 A and AA, plus best practices) over the built
// book (`book/`) in every mdBook theme, and fails on any violation.
//
// The only violations tolerated are known mdBook template issues, listed in
// KNOWN below by rule and *exact* selector. The list is deliberately narrow so
// that new problems fail the run. It also fails when a listed issue stops
// reproducing, so a fix upstream breaks this script and the entry gets removed.
//
//   make a11y                    (builds the book first)
//   node run.js                  (uses the existing book/)
//   CHROME_PATH=/path/to/chrome node run.js
//   A11Y_ONLY=coal node run.js   (a single theme)

const fs = require("fs");
const http = require("http");
const path = require("path");
const puppeteer = require("puppeteer-core");

const BOOK = path.resolve(__dirname, "..", "..", "book");
const AXE = fs.readFileSync(require.resolve("axe-core/axe.min.js"), "utf8");

const THEMES = ["light", "rust", "coal", "navy", "ayu"].filter(
  (t) => !process.env.A11Y_ONLY || t === process.env.A11Y_ONLY
);

const PAGES = [
  ["home", "/index.html"],
  ["crates", "/crates.html"],
  ["recipe index", "/recipe-index.html"],
  ["recipe table", "/algorithms.html"],
  ["recipe page", "/compression/gzip.html"],
  ["code and links", "/errors/handle.html"],
  ["about", "/about.html"],
  ["search overlay", "/index.html?search=rand"],
];

// Known mdBook 0.5.x template issues: [rule, exact selector (string or RegExp)].
const KNOWN = [
  ["region", "#mdbook-sidebar-toggle"],
  ["region", ".menu-title"],
  ["region", ".right-buttons"],
  ["aria-allowed-attr", "#mdbook-sidebar-toggle"],
  ["aria-prohibited-attr", "#mdbook-sidebar-toggle"],
  ["aria-prohibited-attr", /^#mdbook-teaser_\d+$/],
];

const seen = KNOWN.map(() => false);

function known(rule, target) {
  const i = KNOWN.findIndex(
    ([r, sel]) => r === rule && (sel instanceof RegExp ? sel.test(target) : sel === target)
  );
  if (i >= 0) seen[i] = true;
  return i >= 0;
}

function chromePath() {
  const candidates = [
    process.env.CHROME_PATH,
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/usr/bin/google-chrome",
    "/usr/bin/google-chrome-stable",
    "/usr/bin/chromium",
    "/usr/bin/chromium-browser",
  ];
  const found = candidates.find((p) => p && fs.existsSync(p));
  if (!found) throw new Error("Chrome not found; set CHROME_PATH");
  return found;
}

function serve() {
  const types = { ".html": "text/html", ".css": "text/css", ".js": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png", ".json": "application/json", ".woff2": "font/woff2" };
  const server = http.createServer((req, res) => {
    const url = new URL(req.url, "http://x");
    const file = path.join(BOOK, path.normalize(decodeURIComponent(url.pathname)));
    if (!file.startsWith(BOOK) || !fs.existsSync(file) || fs.statSync(file).isDirectory()) {
      res.writeHead(404);
      res.end();
      return;
    }
    res.writeHead(200, { "content-type": types[path.extname(file)] || "application/octet-stream" });
    fs.createReadStream(file).pipe(res);
  });
  return new Promise((resolve) => server.listen(0, "127.0.0.1", () => resolve(server)));
}

(async () => {
  if (!fs.existsSync(path.join(BOOK, "index.html"))) {
    console.error("book/ is missing; run `make build` first");
    process.exit(2);
  }
  const server = await serve();
  const base = `http://127.0.0.1:${server.address().port}`;
  const browser = await puppeteer.launch({ executablePath: chromePath(), headless: "new" });

  const failures = [];
  let checked = 0;
  for (const theme of THEMES) {
    for (const [name, url] of PAGES) {
      const page = await browser.newPage();
      await page.setViewport({ width: 1200, height: 900 });
      await page.evaluateOnNewDocument((t) => localStorage.setItem("mdbook-theme", t), theme);
      await page.goto(base + url, { waitUntil: "networkidle2", timeout: 30000 });
      await new Promise((r) => setTimeout(r, 1200)); // scripts and the lazily loaded index settle
      // The theme scripts bail out silently when mdBook's markup changes, so assert the
      // elements they hook into (and what they build) are really there.
      const missing = await page.evaluate(
        (name) => {
          const need = [
            "#mdbook-search-wrapper",
            "#mdbook-search-toggle",
            "#mdbook-searchbar",
            "#mdbook-searchresults-outer",
            "mdbook-sidebar-scrollbox",
            ".nav-wrapper",
            "#mdbook-search-wrapper[role=dialog][aria-modal=true]",
          ];
          if (name === "recipe page") need.push("a.nav-chapters", "a.mobile-nav-chapters");
          if (name === "recipe table") need.push("table .crate-chip .crate-chip-copy[data-copy]");
          if (name === "crates") need.push(".ci-card .crate-chip");
          const gone = need.filter((sel) => !document.querySelector(sel));
          if (typeof path_to_root !== "string") gone.push("path_to_root");
          if (document.querySelector("img[src*='img.shields.io']")) gone.push("no shields.io images");
          return gone;
        },
        name
      );
      for (const sel of missing) {
        failures.push({ theme, name, rule: "theme-hooks", target: sel, why: "expected by theme/*.js" });
      }
      await page.evaluate(AXE);
      const violations = await page.evaluate(async () => {
        const res = await axe.run(document, {
          runOnly: { type: "tag", values: ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa", "best-practice"] },
        });
        return res.violations.map((v) => ({
          id: v.id,
          help: v.help,
          nodes: v.nodes.map((n) => ({ target: n.target.join(" "), why: n.failureSummary.replace(/\s+/g, " ").slice(0, 200) })),
        }));
      });
      for (const v of violations) {
        for (const n of v.nodes) {
          if (!known(v.id, n.target)) failures.push({ theme, name, rule: v.id, target: n.target, why: n.why });
        }
      }
      checked++;
      await page.close();
    }
  }
  await browser.close();
  server.close();

  let bad = false;
  if (failures.length) {
    bad = true;
    console.error(`\n${failures.length} accessibility violation(s):`);
    for (const f of failures) {
      console.error(`  [${f.theme}] ${f.name}: ${f.rule}\n    ${f.target}\n    ${f.why}`);
    }
  }
  // Only meaningful when every theme ran.
  if (THEMES.length === 5) {
    KNOWN.forEach(([rule, sel], i) => {
      if (!seen[i]) {
        bad = true;
        console.error(`\nstale exception: ${rule} ${sel} no longer reproduces; remove it from KNOWN in run.js`);
      }
    });
  }
  console.log(`\nchecked ${checked} page/theme combinations`);
  if (bad) process.exit(1);
  console.log("accessibility checks passed");
})().catch((e) => {
  console.error(e);
  process.exit(2);
});
