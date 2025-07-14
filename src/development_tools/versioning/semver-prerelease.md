## Check if given version is pre-release.

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Given two versions, [`is_prerelease`] asserts that one is pre-release and the other is not.

```rust,edition2018
use semver::{Version, Error};

fn main() -> Result<(), Error> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert!(!version_1.pre.is_empty());
    assert!(version_2.pre.is_empty());

    Ok(())
}
```

[`