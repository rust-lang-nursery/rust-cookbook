## 複雑なバージョン文字列のパース.

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

[`Version::parse`]でバージョン文字列をパースして[`semver::Version`]を作る。この文字列はpre-releaseやメタデータなどの[Semantic Versioning Specification]を含む。

仕様に従って、ビルドメタデータは解析されますが、バージョンの比較時には考慮されないことに注意してください。言い換えると、2つのバージョンは、ビルド文字列が異なっていても同じである場合があります。

```rust
extern crate semver;

use semver::{Identifier, Version, SemVerError};

fn main() -> Result<(), SemVerError> {
    let version_str = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(version_str)?;

    assert_eq!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: vec![Identifier::Numeric(125)],
            build: vec![],
        }
    );
    assert_eq!(
        parsed_version.build,
        vec![Identifier::AlphaNumeric(String::from("g72ee7853"))]
    );

    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);

    Ok(())
}
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse

[Semantic Versioning Specification]: http://semver.org/
