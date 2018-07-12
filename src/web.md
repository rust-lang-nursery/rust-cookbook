# Web Programming

## Scraping Web Pages

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Extract all links from a webpage HTML][ex-extract-links-webpage] | [![reqwest-badge]][reqwest] [![select-badge]][select] | [![cat-net-badge]][cat-net] |
| [Check webpage for broken links][ex-check-broken-links] | [![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract all unique links from a MediaWiki markup][ex-extract-mediawiki-links] | [![reqwest-badge]][reqwest] [![regex-badge]][regex] | [![cat-net-badge]][cat-net] |

## Uniform Resource Locations (URL)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse a URL from a string to a `Url` type][ex-url-parse] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create a base URL by removing path segments][ex-url-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create new URLs from a base URL][ex-url-new-from-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract the URL origin (scheme / host / port)][ex-url-origin] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Remove fragment identifiers and query pairs from a URL][ex-url-rm-frag] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |

## Media Types (MIME)

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Get MIME type from string][ex-mime-from-string] | [![mime-badge]][mime] | [![cat-encoding-badge]][cat-encoding] |
| [Get MIME type from filename][ex-mime-from-filename] | [![mime-badge]][mime] | [![cat-encoding-badge]][cat-encoding] |
| [Parse the MIME type of a HTTP response][ex-http-response-mime-type] | [![mime-badge]][mime] [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |


{{#include web/clients.md}}

[ex-extract-links-webpage]: web/scraping.html#extract-all-links-from-a-webpage-html
[ex-check-broken-links]: web/scraping.html#check-a-webpage-for-broken-links
[ex-extract-mediawiki-links]: web/scraping.html#extract-all-unique-links-from-a-mediawiki-markup

[ex-url-parse]: web/url.html#parse-a-url-from-a-string-to-a-url-type
[ex-url-base]: web/url.html#create-a-base-url-by-removing-path-segments
[ex-url-new-from-base]: web/url.html#create-new-urls-from-a-base-url
[ex-url-origin]: web/url.html#extract-the-url-origin-scheme--host--port
[ex-url-rm-frag]: web/url.html#remove-fragment-identifiers-and-query-pairs-from-a-url

[ex-mime-from-string]: web/mime.html#get-mime-type-from-string
[ex-mime-from-filename]: web/mime.html#get-mime-type-from-filename
[ex-http-response-mime-type]: web/mime.html#parse-the-mime-type-of-a-http-response

{{#include links.md}}
