## Parse and increment a version string.

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Constructs a [`semver::Version`] from a string literal using [`Version::parse`],
then increments it by patch, minor, and major version number one by one.

Note that in accordance with the [Semantic Versioning Specification],
incrementing the minor version number resets the patch version number to 0 and
incrementing the major version number resets both the minor and patch version
numbers to 0.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate semver;

use semver::Version;
#
# error_chain! {
#     foreign_links {
#         SemVer(semver::SemVerError);
#     }
# }

fn run() -> Result<()> {
    let mut parsed_version = Version::parse("0.2.6")?;

    assert_eq!(
        parsed_version,
        Version {
            major: 0,
            minor: 2,
            patch: 6,
            pre: vec![],
            build: vec![],
        }
    );

    parsed_version.increment_patch();
    assert_eq!(parsed_version.to_string(), "0.2.7");
    println!("New patch release: v{}", parsed_version);

    parsed_version.increment_minor();
    assert_eq!(parsed_version.to_string(), "0.3.0");
    println!("New minor release: v{}", parsed_version);

    parsed_version.increment_major();
    assert_eq!(parsed_version.to_string(), "1.0.0");
    println!("New major release: v{}", parsed_version);

    Ok(())
}
#
# quick_main!(run);
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse

[Semantic Versioning Specification]: http://semver.org/
