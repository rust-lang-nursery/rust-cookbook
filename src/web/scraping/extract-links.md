## Extract all links from a webpage HTML

[![reqwest-badge]][reqwest] [![select-badge]][select] [![cat-net-badge]][cat-net]

Use [`reqwest::get`] to perform a HTTP GET request and then use
[`Document::from_read`] to parse the response into a HTML document.
[`find`] with the criteria of [`Name`] is "a" retrieves all links.
Call [`filter_map`] on the [`Selection`] retrieves URLs
from links that have the "href" [`attr`] (attribute).

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;
#
# error_chain! {
#    foreign_links {
#        ReqError(reqwest::Error);
#        IoError(std::io::Error);
#    }
# }

fn run() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")?;

    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
#
# quick_main!(run);
```

[`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[`Document::from_read`]: https://docs.rs/select/*/select/document/struct.Document.html#method.from_read
[`filter_map`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[`find`]: https://docs.rs/select/*/select/document/struct.Document.html#method.find
[`Name`]: https://docs.rs/select/*/select/predicate/struct.Name.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`Selection`]: https://docs.rs/select/*/select/selection/struct.Selection.html
