## Parse a complex version string.

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Constructs a [`semver::Version`] from a complex version string using [`Version::parse`]. The string
contains pre-release and build metadata as defined in the [Semantic Versioning Specification].

Note that, in accordance with the Specification, build metadata is parsed but not considered when
comparing versions. In other words, two versions may be equal even if their build strings differ.

```rust,edition2018
use semver::{Version, Prerelease, BuildMetadata, Error};

fn main() -> Result<(), Error> {
    let version_str = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(version_str)?;

    assert_eq!(parsed_version.major, 1);
    assert_eq!(parsed_version.minor, 0);
    assert_eq!(parsed_version.patch, 49);
    assert_eq!(parsed_version.pre, Prerelease::new("125")?);
    assert_eq!(parsed_version.build, BuildMetadata::new("g72ee7853")?);

    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);

    Ok(())
}
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse

[Semantic Versioning Specification]: http://semver.org/
