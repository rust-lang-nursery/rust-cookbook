## Find the latest version satisfying given range

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Given a list of version &strs, finds the latest [`semver::Version`].
[`semver::VersionReq`] filters the list with [`VersionReq::matches`].
Also demonstrates `semver` pre-release preferences.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate semver;

use semver::{Version, VersionReq};
#
# error_chain! {
#     foreign_links {
#         SemVer(semver::SemVerError);
#         SemVerReq(semver::ReqParseError);
#     }
# }

fn find_max_matching_version<'a, I>(version_req_str: &str, iterable: I) -> Result<Option<Version>>
where
    I: IntoIterator<Item = &'a str>,
{
    let vreq = VersionReq::parse(version_req_str)?;

    Ok(
        iterable
            .into_iter()
            .filter_map(|s| Version::parse(s).ok())
            .filter(|s| vreq.matches(s))
            .max(),
    )
}

fn run() -> Result<()> {
    assert_eq!(
        find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,
        Some(Version::parse("1.0.0")?)
    );

    assert_eq!(
        find_max_matching_version(
            ">1.2.3-alpha.3",
            vec![
                "1.2.3-alpha.3",
                "1.2.3-alpha.4",
                "1.2.3-alpha.10",
                "1.2.3-beta.4",
                "3.4.5-alpha.9",
            ]
        )?,
        Some(Version::parse("1.2.3-beta.4")?)
    );

    Ok(())
}
#
# quick_main!(run);
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`semver::VersionReq`]: https://docs.rs/semver/*/semver/struct.VersionReq.html
[`VersionReq::matches`]: https://docs.rs/semver/*/semver/struct.VersionReq.html#method.matches
