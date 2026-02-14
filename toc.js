// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="intro.html">Table of Contents</a></li><li class="chapter-item expanded affix "><a href="about.html">About</a></li><li class="chapter-item expanded "><a href="algorithms.html"><strong aria-hidden="true">1.</strong> Algorithms</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="algorithms/randomness.html"><strong aria-hidden="true">1.1.</strong> Generate Random Values</a></li><li class="chapter-item expanded "><a href="algorithms/sorting.html"><strong aria-hidden="true">1.2.</strong> Sort a Vector</a></li></ol></li><li class="chapter-item expanded "><a href="cli.html"><strong aria-hidden="true">2.</strong> Command Line</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="cli/arguments.html"><strong aria-hidden="true">2.1.</strong> Argument Parsing</a></li><li class="chapter-item expanded "><a href="cli/ansi_terminal.html"><strong aria-hidden="true">2.2.</strong> ANSI Terminal</a></li></ol></li><li class="chapter-item expanded "><a href="compression.html"><strong aria-hidden="true">3.</strong> Compression</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="compression/tar.html"><strong aria-hidden="true">3.1.</strong> Working with Tarballs</a></li></ol></li><li class="chapter-item expanded "><a href="concurrency.html"><strong aria-hidden="true">4.</strong> Concurrency</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="concurrency/threads.html"><strong aria-hidden="true">4.1.</strong> Explicit Threads</a></li><li class="chapter-item expanded "><a href="concurrency/parallel.html"><strong aria-hidden="true">4.2.</strong> Data Parallelism</a></li></ol></li><li class="chapter-item expanded "><a href="cryptography.html"><strong aria-hidden="true">5.</strong> Cryptography</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="cryptography/hashing.html"><strong aria-hidden="true">5.1.</strong> Hashing</a></li><li class="chapter-item expanded "><a href="cryptography/encryption.html"><strong aria-hidden="true">5.2.</strong> Encryption</a></li></ol></li><li class="chapter-item expanded "><a href="data_structures.html"><strong aria-hidden="true">6.</strong> Data Structures</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="data_structures/bitfield.html"><strong aria-hidden="true">6.1.</strong> Bitfield</a></li></ol></li><li class="chapter-item expanded "><a href="database.html"><strong aria-hidden="true">7.</strong> Database</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="database/sqlite.html"><strong aria-hidden="true">7.1.</strong> SQLite</a></li><li class="chapter-item expanded "><a href="database/postgres.html"><strong aria-hidden="true">7.2.</strong> Postgres</a></li></ol></li><li class="chapter-item expanded "><a href="datetime.html"><strong aria-hidden="true">8.</strong> Date and Time</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="datetime/duration.html"><strong aria-hidden="true">8.1.</strong> Duration and Calculation</a></li><li class="chapter-item expanded "><a href="datetime/parse.html"><strong aria-hidden="true">8.2.</strong> Parsing and Displaying</a></li></ol></li><li class="chapter-item expanded "><a href="development_tools.html"><strong aria-hidden="true">9.</strong> Development Tools</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development_tools/debugging.html"><strong aria-hidden="true">9.1.</strong> Debugging</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="development_tools/debugging/log.html"><strong aria-hidden="true">9.1.1.</strong> Log Messages</a></li><li class="chapter-item expanded "><a href="development_tools/debugging/config_log.html"><strong aria-hidden="true">9.1.2.</strong> Configure Logging</a></li><li class="chapter-item expanded "><a href="development_tools/debugging/tracing.html"><strong aria-hidden="true">9.1.3.</strong> Tracing Messages</a></li></ol></li><li class="chapter-item expanded "><a href="development_tools/versioning.html"><strong aria-hidden="true">9.2.</strong> Versioning</a></li><li class="chapter-item expanded "><a href="development_tools/build_tools.html"><strong aria-hidden="true">9.3.</strong> Build Time Tooling</a></li></ol></li><li class="chapter-item expanded "><a href="encoding.html"><strong aria-hidden="true">10.</strong> Encoding</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="encoding/strings.html"><strong aria-hidden="true">10.1.</strong> Character Sets</a></li><li class="chapter-item expanded "><a href="encoding/csv.html"><strong aria-hidden="true">10.2.</strong> CSV processing</a></li><li class="chapter-item expanded "><a href="encoding/complex.html"><strong aria-hidden="true">10.3.</strong> Structured Data</a></li></ol></li><li class="chapter-item expanded "><a href="errors.html"><strong aria-hidden="true">11.</strong> Error Handling</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="errors/handle.html"><strong aria-hidden="true">11.1.</strong> Handle Error Variants</a></li></ol></li><li class="chapter-item expanded "><a href="file.html"><strong aria-hidden="true">12.</strong> File System</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="file/read-write.html"><strong aria-hidden="true">12.1.</strong> Read &amp; Write</a></li><li class="chapter-item expanded "><a href="file/dir.html"><strong aria-hidden="true">12.2.</strong> Directory Traversal</a></li></ol></li><li class="chapter-item expanded "><a href="hardware.html"><strong aria-hidden="true">13.</strong> Hardware Support</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="hardware/processor.html"><strong aria-hidden="true">13.1.</strong> Processor</a></li></ol></li><li class="chapter-item expanded "><a href="mem.html"><strong aria-hidden="true">14.</strong> Memory Management</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="mem/global_static.html"><strong aria-hidden="true">14.1.</strong> Global Static</a></li></ol></li><li class="chapter-item expanded "><a href="net.html"><strong aria-hidden="true">15.</strong> Network</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="net/server.html"><strong aria-hidden="true">15.1.</strong> Server</a></li></ol></li><li class="chapter-item expanded "><a href="os.html"><strong aria-hidden="true">16.</strong> Operating System</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="os/external.html"><strong aria-hidden="true">16.1.</strong> External Command</a></li></ol></li><li class="chapter-item expanded "><a href="science.html"><strong aria-hidden="true">17.</strong> Science</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="science/mathematics.html"><strong aria-hidden="true">17.1.</strong> Mathematics</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="science/mathematics/linear_algebra.html"><strong aria-hidden="true">17.1.1.</strong> Linear Algebra</a></li><li class="chapter-item expanded "><a href="science/mathematics/trigonometry.html"><strong aria-hidden="true">17.1.2.</strong> Trigonometry</a></li><li class="chapter-item expanded "><a href="science/mathematics/complex_numbers.html"><strong aria-hidden="true">17.1.3.</strong> Complex Numbers</a></li><li class="chapter-item expanded "><a href="science/mathematics/statistics.html"><strong aria-hidden="true">17.1.4.</strong> Statistics</a></li><li class="chapter-item expanded "><a href="science/mathematics/miscellaneous.html"><strong aria-hidden="true">17.1.5.</strong> Miscellaneous</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="text.html"><strong aria-hidden="true">18.</strong> Text Processing</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="text/regex.html"><strong aria-hidden="true">18.1.</strong> Regular Expressions</a></li><li class="chapter-item expanded "><a href="text/string_parsing.html"><strong aria-hidden="true">18.2.</strong> String Parsing</a></li></ol></li><li class="chapter-item expanded "><a href="web.html"><strong aria-hidden="true">19.</strong> Web Programming</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="web/scraping.html"><strong aria-hidden="true">19.1.</strong> Extracting Links</a></li><li class="chapter-item expanded "><a href="web/url.html"><strong aria-hidden="true">19.2.</strong> URL</a></li><li class="chapter-item expanded "><a href="web/mime.html"><strong aria-hidden="true">19.3.</strong> Media Types</a></li><li class="chapter-item expanded "><a href="web/clients.html"><strong aria-hidden="true">19.4.</strong> Clients</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="web/clients/requests.html"><strong aria-hidden="true">19.4.1.</strong> Making Requests</a></li><li class="chapter-item expanded "><a href="web/clients/apis.html"><strong aria-hidden="true">19.4.2.</strong> Calling a Web API</a></li><li class="chapter-item expanded "><a href="web/clients/download.html"><strong aria-hidden="true">19.4.3.</strong> Downloads</a></li><li class="chapter-item expanded "><a href="web/clients/authentication.html"><strong aria-hidden="true">19.4.4.</strong> Web Authentication</a></li></ol></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
