use semver::{Version, VersionReq};

fn main() -> Result<(), semver::Error> {
    let version = Version::parse("1.4.2-beta.1")?;
    let req = VersionReq::parse(">=1.2, <2.0")?;

    println!(
        "{version}: major {}, pre-release {:?}",
        version.major, version.pre
    );
    println!("matches {req}: {}", req.matches(&Version::parse("1.9.0")?));
    Ok(())
}
