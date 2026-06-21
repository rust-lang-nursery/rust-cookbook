# Option and Result Combinators

{{#include combinators/transform-option.md}}

{{#include combinators/option-fallback.md}}

{{#include combinators/option-to-result.md}}

{{#include combinators/chain-result.md}}

{{#include combinators/collect-results.md}}

{{#include ../links.md}}

[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Option::and_then`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
[`Option::filter`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.filter
[`Option::map`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.map
[`Option::ok_or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
[`Option::ok_or_else`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else
[`Option::unwrap_or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
[`Option::unwrap_or_default`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default
[`Option::unwrap_or_else`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
[`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
[`Result::and_then`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then
[`Result::map`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map
[`Result::map_err`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err
[`Iterator::collect`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[`Iterator::filter_map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
[`OsStr::to_str`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html#method.to_str
[`Path::extension`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.extension
