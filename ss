[1mdiff --git a/src/about.md b/src/about.md[m
[1mindex ebfe3ed..2126d46 100644[m
[1m--- a/src/about.md[m
[1m+++ b/src/about.md[m
[36m@@ -2,11 +2,13 @@[m
 [m
 ## Table of contents[m
 [m
[31m-- [Who this book is for](#who-this-book-is-for)[m
[31m-- [How to read this book](#how-to-read-this-book)[m
[31m-- [How to use the recipes](#how-to-use-the-recipes)[m
[31m-- [A note about error handling](#a-note-about-error-handling)[m
[31m-- [A note about crate representation](#a-note-about-crate-representation)[m
[32m+[m[32m- [About "Cookin' with Rust"](#about-%22cookin-with-rust%22)[m
[32m+[m[32m    - [Table of contents](#table-of-contents)[m
[32m+[m[32m    - [Who this book is for](#who-this-book-is-for)[m
[32m+[m[32m    - [How to read this book](#how-to-read-this-book)[m
[32m+[m[32m    - [How to use the recipes](#how-to-use-the-recipes)[m
[32m+[m[32m    - [A note about error handling](#a-note-about-error-handling)[m
[32m+[m[32m    - [A note about crate representation](#a-note-about-crate-representation)[m
 [m
 ## Who this book is for[m
 [m
[36m@@ -107,9 +109,7 @@[m [mSince these recipes are intended to be reused as-is and encourage best[m
 practices, they set up error handling correctly when there are[m
 `Result` types involved.[m
 [m
[31m-The basic pattern we use is to have a `fn run() -> Result` that acts[m
[31m-like the "real" main function. We use the [error-chain] crate to make[m
[31m-`?` work within `run`.[m
[32m+[m[32mThe basic pattern we use is to have a `fn main() -> Result` .[m
 [m
 The structure generally looks like:[m
 [m
[36m@@ -127,7 +127,7 @@[m [merror_chain! {[m
     }[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let bytes = b"2001:db8::1";[m
 [m
     // Bytes to string.[m
[36m@@ -140,14 +140,13 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 [m
[31m-quick_main!(run);[m
 ```[m
 [m
 This is using the `error_chain!` macro to define a custom `Error` and[m
 `Result` type, along with automatic conversions from two standard[m
 library error types. The automatic conversions make the `?` operator[m
[31m-work. The `quick_main!` macro generates the actual `main` function and[m
[31m-prints out the error if one occurred.[m
[32m+[m[32mwork. Since rust 1.26, the main function can return a value, no need[m[41m [m
[32m+[m[32mto use a custom `run` function like before.[m
 [m
 For the sake of readability error handling boilerplate is hidden by[m
 default like below.  In order to read full contents click on the[m
[36m@@ -167,14 +166,14 @@[m [muse url::{Url, Position};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let parsed = Url::parse("https://httpbin.org/cookies/set?k2=v2&k1=v1")?;[m
     let cleaned: &str = &parsed[..Position::AfterPath];[m
     println!("cleaned: {}", cleaned);[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 For more background on error handling in Rust, read [this page of the[m
[1mdiff --git a/src/compression/tar/tar-compress.md b/src/compression/tar/tar-compress.md[m
[1mindex 0e3d9c3..c9a8d36 100644[m
[1m--- a/src/compression/tar/tar-compress.md[m
[1m+++ b/src/compression/tar/tar-compress.md[m
[36m@@ -26,7 +26,7 @@[m [muse std::fs::File;[m
 use flate2::Compression;[m
 use flate2::write::GzEncoder;[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let tar_gz = File::create("archive.tar.gz")?;[m
     let enc = GzEncoder::new(tar_gz, Compression::default());[m
     let mut tar = tar::Builder::new(enc);[m
[36m@@ -34,7 +34,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Builder::append_dir_all`]: https://docs.rs/tar/*/tar/struct.Builder.html#method.append_dir_all[m
[1mdiff --git a/src/compression/tar/tar-decompress.md b/src/compression/tar/tar-decompress.md[m
[1mindex 55a245e..33567ec 100644[m
[1m--- a/src/compression/tar/tar-decompress.md[m
[1m+++ b/src/compression/tar/tar-decompress.md[m
[36m@@ -23,7 +23,7 @@[m [muse tar::Archive;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let path = "archive.tar.gz";[m
 [m
     let tar_gz = File::open(path)?;[m
[36m@@ -34,7 +34,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Archive::unpack`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.unpack[m
[1mdiff --git a/src/compression/tar/tar-strip-prefix.md b/src/compression/tar/tar-strip-prefix.md[m
[1mindex 79c1029..38e2efa 100644[m
[1m--- a/src/compression/tar/tar-strip-prefix.md[m
[1m+++ b/src/compression/tar/tar-strip-prefix.md[m
[36m@@ -24,7 +24,7 @@[m [muse std::path::PathBuf;[m
 use flate2::read::GzDecoder;[m
 use tar::Archive;[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let file = File::open("archive.tar.gz")?;[m
     let mut archive = Archive::new(GzDecoder::new(file));[m
     let prefix = "bundle/logs";[m
[36m@@ -44,7 +44,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Archive::entries`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.entries[m
[1mdiff --git a/src/concurrency/parallel/rayon-thumbnails.md b/src/concurrency/parallel/rayon-thumbnails.md[m
[1mindex d7151bf..72b3879 100644[m
[1m--- a/src/concurrency/parallel/rayon-thumbnails.md[m
[1m+++ b/src/concurrency/parallel/rayon-thumbnails.md[m
[36m@@ -31,7 +31,7 @@[m [muse rayon::prelude::*;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let options: MatchOptions = Default::default();[m
     let files: Vec<_> = glob_with("*.jpg", &options)?[m
         .filter_map(|x| x.ok())[m
[36m@@ -73,7 +73,7 @@[m [mwhere[m
         .save(file_path)?)[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`glob::glob_with`]: https://docs.rs/glob/*/glob/fn.glob_with.html[m
[1mdiff --git a/src/concurrency/thread/global-mut-state.md b/src/concurrency/thread/global-mut-state.md[m
[1mindex c2c8d42..72143a6 100644[m
[1m--- a/src/concurrency/thread/global-mut-state.md[m
[1m+++ b/src/concurrency/thread/global-mut-state.md[m
[36m@@ -29,7 +29,7 @@[m [mfn insert(fruit: &str) -> Result<()> {[m
     Ok(())[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     insert("apple")?;[m
     insert("orange")?;[m
     insert("peach")?;[m
[36m@@ -42,7 +42,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html[m
[1mdiff --git a/src/concurrency/thread/threadpool-fractal.md b/src/concurrency/thread/threadpool-fractal.md[m
[1mindex 98e65a8..23d497c 100644[m
[1m--- a/src/concurrency/thread/threadpool-fractal.md[m
[1m+++ b/src/concurrency/thread/threadpool-fractal.md[m
[36m@@ -88,7 +88,7 @@[m [muse image::{ImageBuffer, Pixel, Rgb};[m
 #     ((color * factor).powf(0.8) * 255.) as u8[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let (width, height) = (1920, 1080);[m
     let mut img = ImageBuffer::new(width, height);[m
     let iterations = 300;[m
[36m@@ -115,7 +115,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`ImageBuffer::new`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.new[m
[1mdiff --git a/src/concurrency/thread/threadpool-walk.md b/src/concurrency/thread/threadpool-walk.md[m
[1mindex 463f220..e0b1369 100644[m
[1m--- a/src/concurrency/thread/threadpool-walk.md[m
[1m+++ b/src/concurrency/thread/threadpool-walk.md[m
[36m@@ -54,7 +54,7 @@[m [mfn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P)> {[m
     Ok((context.finish(), filepath))[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let pool = ThreadPool::new(num_cpus::get());[m
 [m
     let (tx, rx) = channel();[m
[36m@@ -80,7 +80,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute[m
[1mdiff --git a/src/cryptography/encryption/pbkdf2.md b/src/cryptography/encryption/pbkdf2.md[m
[1mindex 39530ae..ed88635 100644[m
[1m--- a/src/cryptography/encryption/pbkdf2.md[m
[1m+++ b/src/cryptography/encryption/pbkdf2.md[m
[36m@@ -25,7 +25,7 @@[m [muse data_encoding::HEXUPPER;[m
 use ring::{digest, pbkdf2, rand};[m
 use ring::rand::SecureRandom;[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
   const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;[m
   const N_ITER: u32 = 100_000;[m
   let rng = rand::SystemRandom::new();[m
[36m@@ -67,7 +67,7 @@[m [mfn run() -> Result<()> {[m
   Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`pbkdf2::derive`]: https://briansmith.org/rustdoc/ring/pbkdf2/fn.derive.html[m
[1mdiff --git a/src/cryptography/hashing/hmac.md b/src/cryptography/hashing/hmac.md[m
[1mindex fbfe116..8e29b11 100644[m
[1m--- a/src/cryptography/hashing/hmac.md[m
[1m+++ b/src/cryptography/hashing/hmac.md[m
[36m@@ -18,7 +18,7 @@[m [mextern crate ring;[m
 use ring::{digest, hmac, rand};[m
 use ring::rand::SecureRandom;[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let mut key_value = [0u8; 48];[m
     let rng = rand::SystemRandom::new();[m
     rng.fill(&mut key_value)?;[m
[36m@@ -31,7 +31,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`hmac::Signature`]: https://briansmith.org/rustdoc/ring/hmac/struct.Signature.html[m
[1mdiff --git a/src/cryptography/hashing/sha-digest.md b/src/cryptography/hashing/sha-digest.md[m
[1mindex d4988d1..0583091 100644[m
[1m--- a/src/cryptography/hashing/sha-digest.md[m
[1m+++ b/src/cryptography/hashing/sha-digest.md[m
[36m@@ -38,7 +38,7 @@[m [mfn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {[m
     Ok(context.finish())[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let path = "file.txt";[m
 [m
     let mut output = File::create(path)?;[m
[36m@@ -53,7 +53,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`digest::Context`]: https://briansmith.org/rustdoc/ring/digest/struct.Context.html[m
[1mdiff --git a/src/datetime/parse/string.md b/src/datetime/parse/string.md[m
[1mindex 4066f0a..c9ce84d 100644[m
[1m--- a/src/datetime/parse/string.md[m
[1m+++ b/src/datetime/parse/string.md[m
[36m@@ -26,7 +26,7 @@[m [muse chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;[m
     println!("{}", rfc2822);[m
 [m
[36m@@ -48,7 +48,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`chrono::format::strftime`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html[m
[1mdiff --git a/src/development_tools/build_tools/cc-bundled-static.md b/src/development_tools/build_tools/cc-bundled-static.md[m
[1mindex 3e24c13..14537fe 100644[m
[1m--- a/src/development_tools/build_tools/cc-bundled-static.md[m
[1m+++ b/src/development_tools/build_tools/cc-bundled-static.md[m
[36m@@ -83,7 +83,7 @@[m [mextern {[m
     fn greet(name: *const c_char);[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     unsafe { hello() }[m
     let name = prompt("What's your name? ")?;[m
     let c_name = CString::new(name)?;[m
[36m@@ -91,7 +91,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`cc::Build::define`]: https://docs.rs/cc/*/cc/struct.Build.html#method.define[m
[1mdiff --git a/src/development_tools/debugging/config_log/log-custom.md b/src/development_tools/debugging/config_log/log-custom.md[m
[1mindex fa68233..54746e7 100644[m
[1m--- a/src/development_tools/debugging/config_log/log-custom.md[m
[1m+++ b/src/development_tools/debugging/config_log/log-custom.md[m
[36m@@ -31,7 +31,7 @@[m [muse log4rs::config::{Appender, Config, Root};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let logfile = FileAppender::builder()[m
         .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))[m
         .build("log/output.log")?;[m
[36m@@ -49,7 +49,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`log4rs::append::file::FileAppender`]: https://docs.rs/log4rs/*/log4rs/append/file/struct.FileAppender.html[m
[1mdiff --git a/src/development_tools/debugging/log/log-custom-logger.md b/src/development_tools/debugging/log/log-custom-logger.md[m
[1mindex 5844bc7..2b43de6 100644[m
[1m--- a/src/development_tools/debugging/log/log-custom-logger.md[m
[1m+++ b/src/development_tools/debugging/log/log-custom-logger.md[m
[36m@@ -38,7 +38,7 @@[m [mimpl log::Log for ConsoleLogger {[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     log::set_logger(&CONSOLE_LOGGER)?;[m
     log::set_max_level(LevelFilter::Info);[m
 [m
[36m@@ -48,7 +48,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`log::Log`]: https://docs.rs/log/*/log/trait.Log.html[m
[1mdiff --git a/src/development_tools/debugging/log/log-syslog.md b/src/development_tools/debugging/log/log-syslog.md[m
[1mindex 8bf7a88..c21f23d 100644[m
[1m--- a/src/development_tools/debugging/log/log-syslog.md[m
[1m+++ b/src/development_tools/debugging/log/log-syslog.md[m
[36m@@ -27,7 +27,7 @@[m [muse syslog::Facility;[m
 # }[m
 [m
 # #[cfg(target_os = "linux")][m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     syslog::init(Facility::LOG_USER,[m
                  log::LevelFilter::Debug,[m
                  Some("My app name"))?;[m
[36m@@ -39,11 +39,11 @@[m [mfn run() -> Result<()> {[m
 # #[cfg(not(target_os = "linux"))][m
 # error_chain! {}[m
 # #[cfg(not(target_os = "linux"))][m
[31m-# fn run() -> Result<()> {[m
[32m+[m[32m# fn main() -> Result<()> {[m
 #     Ok(())[m
 # }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`log::LevelFilter`]: https://docs.rs/log/*/log/enum.LevelFilter.html[m
[1mdiff --git a/src/development_tools/versioning/semver-command.md b/src/development_tools/versioning/semver-command.md[m
[1mindex 8849acd..6996283 100644[m
[1m--- a/src/development_tools/versioning/semver-command.md[m
[1m+++ b/src/development_tools/versioning/semver-command.md[m
[36m@@ -24,7 +24,7 @@[m [muse semver::{Version, VersionReq};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let version_constraint = "> 1.12.0";[m
     let version_test = VersionReq::parse(version_constraint)?;[m
     let output = Command::new("git").arg("--version").output()?;[m
[36m@@ -47,7 +47,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html[m
[1mdiff --git a/src/development_tools/versioning/semver-complex.md b/src/development_tools/versioning/semver-complex.md[m
[1mindex 2e7df61..bcf67ce 100644[m
[1m--- a/src/development_tools/versioning/semver-complex.md[m
[1m+++ b/src/development_tools/versioning/semver-complex.md[m
[36m@@ -21,7 +21,7 @@[m [muse semver::{Identifier, Version};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let version_str = "1.0.49-125+g72ee7853";[m
     let parsed_version = Version::parse(version_str)?;[m
 [m
[36m@@ -46,7 +46,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html[m
[1mdiff --git a/src/development_tools/versioning/semver-increment.md b/src/development_tools/versioning/semver-increment.md[m
[1mindex 39714cd..cfd3213 100644[m
[1m--- a/src/development_tools/versioning/semver-increment.md[m
[1m+++ b/src/development_tools/versioning/semver-increment.md[m
[36m@@ -23,7 +23,7 @@[m [muse semver::Version;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let mut parsed_version = Version::parse("0.2.6")?;[m
 [m
     assert_eq!([m
[36m@@ -52,7 +52,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html[m
[1mdiff --git a/src/development_tools/versioning/semver-latest.md b/src/development_tools/versioning/semver-latest.md[m
[1mindex 03bd046..ea05f84 100644[m
[1m--- a/src/development_tools/versioning/semver-latest.md[m
[1m+++ b/src/development_tools/versioning/semver-latest.md[m
[36m@@ -35,7 +35,7 @@[m [mwhere[m
     )[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     assert_eq!([m
         find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,[m
         Some(Version::parse("1.0.0")?)[m
[36m@@ -58,7 +58,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html[m
[1mdiff --git a/src/development_tools/versioning/semver-prerelease.md b/src/development_tools/versioning/semver-prerelease.md[m
[1mindex f0e6416..ab9819e 100644[m
[1m--- a/src/development_tools/versioning/semver-prerelease.md[m
[1m+++ b/src/development_tools/versioning/semver-prerelease.md[m
[36m@@ -17,7 +17,7 @@[m [muse semver::Version;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let version_1 = Version::parse("1.0.0-alpha")?;[m
     let version_2 = Version::parse("1.0.0")?;[m
 [m
[36m@@ -27,7 +27,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`is_prerelease`]: https://docs.rs/semver/*/semver/struct.Version.html#method.is_prerelease[m
[1mdiff --git a/src/encoding/complex/endian-byte.md b/src/encoding/complex/endian-byte.md[m
[1mindex 17a637e..d1d7c13 100644[m
[1m--- a/src/encoding/complex/endian-byte.md[m
[1m+++ b/src/encoding/complex/endian-byte.md[m
[36m@@ -25,7 +25,7 @@[m [mstruct Payload {[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let original_payload = Payload::default();[m
     let encoded_bytes = encode(&original_payload)?;[m
     let decoded_payload = decode(&encoded_bytes)?;[m
[36m@@ -48,5 +48,5 @@[m [mfn decode(mut bytes: &[u8]) -> Result<Payload> {[m
     Ok(payload)[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
[1mdiff --git a/src/encoding/complex/json.md b/src/encoding/complex/json.md[m
[1mindex 8eb1c79..128dcb8 100644[m
[1m--- a/src/encoding/complex/json.md[m
[1m+++ b/src/encoding/complex/json.md[m
[36m@@ -24,7 +24,7 @@[m [muse serde_json::Value;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let j = r#"{[m
                  "userid": 103609,[m
                  "verified": true,[m
[36m@@ -50,7 +50,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`from_str`]: https://docs.serde.rs/serde_json/fn.from_str.html[m
[1mdiff --git a/src/encoding/complex/toml.md b/src/encoding/complex/toml.md[m
[1mindex 1d5114f..5d1db93 100644[m
[1m--- a/src/encoding/complex/toml.md[m
[1m+++ b/src/encoding/complex/toml.md[m
[36m@@ -18,7 +18,7 @@[m [muse toml::Value;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let toml_content = r#"[m
           [package][m
           name = "your_package"[m
[36m@@ -38,7 +38,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 Parse TOML into your own structs using [Serde].[m
[36m@@ -71,7 +71,7 @@[m [mstruct Package {[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let toml_content = r#"[m
           [package][m
           name = "your_package"[m
[36m@@ -92,5 +92,5 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
[1mdiff --git a/src/encoding/csv/delimiter.md b/src/encoding/csv/delimiter.md[m
[1mindex 15887bb..4035a1b 100644[m
[1m--- a/src/encoding/csv/delimiter.md[m
[1m+++ b/src/encoding/csv/delimiter.md[m
[36m@@ -27,7 +27,7 @@[m [muse csv::ReaderBuilder;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let data = "name\tplace\tid[m
 Mark\tMelbourne\t46[m
 Ashley\tZurich\t92";[m
[36m@@ -40,7 +40,7 @@[m [mAshley\tZurich\t92";[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`delimiter`]: https://docs.rs/csv/1.0.0-beta.3/csv/struct.ReaderBuilder.html#method.delimiter[m
[1mdiff --git a/src/encoding/csv/filter.md b/src/encoding/csv/filter.md[m
[1mindex a097d6b..c6e70fc 100644[m
[1m--- a/src/encoding/csv/filter.md[m
[1m+++ b/src/encoding/csv/filter.md[m
[36m@@ -18,7 +18,7 @@[m [muse std::io;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let query = "CA";[m
     let data = "\[m
 City,State,Population,Latitude,Longitude[m
[36m@@ -43,7 +43,7 @@[m [mWest Hollywood,CA,37031,34.0900000,-118.3608333";[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 _Disclaimer: this example has been adapted from [the csv crate tutorial](https://docs.rs/csv/*/csv/tutorial/index.html#filter-by-search)_.[m
[1mdiff --git a/src/encoding/csv/invalid.md b/src/encoding/csv/invalid.md[m
[1mindex 96a34f5..dc86325 100644[m
[1m--- a/src/encoding/csv/invalid.md[m
[1m+++ b/src/encoding/csv/invalid.md[m
[36m@@ -27,7 +27,7 @@[m [mstruct Record {[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let data = "name,place,id[m
 mark,sydney,46.5[m
 ashley,zurich,92[m
[36m@@ -43,7 +43,7 @@[m [malisha,colombo,xyz";[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`csv::invalid_option`]: https://docs.rs/csv/*/csv/fn.invalid_option.html[m
[1mdiff --git a/src/encoding/csv/read.md b/src/encoding/csv/read.md[m
[1mindex 196d2ce..0509589 100644[m
[1m--- a/src/encoding/csv/read.md[m
[1m+++ b/src/encoding/csv/read.md[m
[36m@@ -17,7 +17,7 @@[m [mextern crate csv;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let csv = "year,make,model,description[m
 1948,Porsche,356,Luxury sports car[m
 1967,Ford,Mustang fastback 1967,American car";[m
[36m@@ -37,7 +37,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 Serde deserializes data into strongly type structures. See the[m
[36m@@ -64,7 +64,7 @@[m [mstruct Record {[m
     description: String,[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let csv = "year,make,model,description[m
 1948,Porsche,356,Luxury sports car[m
 1967,Ford,Mustang fastback 1967,American car";[m
[36m@@ -85,7 +85,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`csv::ByteRecord`]: https://docs.rs/csv/*/csv/struct.ByteRecord.html[m
[1mdiff --git a/src/encoding/csv/serde-serialize.md b/src/encoding/csv/serde-serialize.md[m
[1mindex 6142fa9..03db271 100644[m
[1m--- a/src/encoding/csv/serde-serialize.md[m
[1m+++ b/src/encoding/csv/serde-serialize.md[m
[36m@@ -28,7 +28,7 @@[m [mstruct Record<'a> {[m
     id: u64,[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let mut wtr = csv::Writer::from_writer(io::stdout());[m
 [m
     let rec1 = Record { name: "Mark", place: "Melbourne", id: 56};[m
[36m@@ -44,5 +44,5 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
[1mdiff --git a/src/encoding/csv/serialize.md b/src/encoding/csv/serialize.md[m
[1mindex fdf4983..b38a1dc 100644[m
[1m--- a/src/encoding/csv/serialize.md[m
[1m+++ b/src/encoding/csv/serialize.md[m
[36m@@ -22,7 +22,7 @@[m [muse std::io;[m
 #    }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let mut wtr = csv::Writer::from_writer(io::stdout());[m
 [m
     wtr.write_record(&["Name", "Place", "ID"])?;[m
[36m@@ -35,7 +35,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`csv::Writer`]: https://docs.rs/csv/*/csv/struct.Writer.html[m
[1mdiff --git a/src/encoding/csv/transform.md b/src/encoding/csv/transform.md[m
[1mindex 16d3945..eebe0cf 100644[m
[1m--- a/src/encoding/csv/transform.md[m
[1m+++ b/src/encoding/csv/transform.md[m
[36m@@ -70,7 +70,7 @@[m [mimpl<'de> Deserialize<'de> for HexColor {[m
     }[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let data = "color_name,color[m
 red,#ff0000[m
 green,#00ff00[m
[36m@@ -95,7 +95,7 @@[m [mmagenta,#ff00ff"[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`csv::Reader::deserialize`]: https://docs.rs/csv/\*/csv/struct.Reader.html#method.deserialize[m
[1mdiff --git a/src/encoding/string/base64.md b/src/encoding/string/base64.md[m
[1mindex 4aca527..df8475f 100644[m
[1m--- a/src/encoding/string/base64.md[m
[1m+++ b/src/encoding/string/base64.md[m
[36m@@ -20,7 +20,7 @@[m [muse base64::{encode, decode};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let hello = b"hello rustaceans";[m
     let encoded = encode(hello);[m
     let decoded = decode(&encoded)?;[m
[36m@@ -32,7 +32,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`decode`]: https://docs.rs/base64/*/base64/fn.decode.html[m
[1mdiff --git a/src/encoding/string/hex.md b/src/encoding/string/hex.md[m
[1mindex 084b99c..6a316fc 100644[m
[1m--- a/src/encoding/string/hex.md[m
[1m+++ b/src/encoding/string/hex.md[m
[36m@@ -25,7 +25,7 @@[m [muse data_encoding::{HEXUPPER, DecodeError};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let original = b"The quick brown fox jumps over the lazy dog.";[m
     let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\[m
         657220746865206C617A7920646F672E";[m
[36m@@ -39,7 +39,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`data_encoding`]: https://docs.rs/data-encoding/*/data_encoding/[m
[1mdiff --git a/src/encoding/string/percent-encode.md b/src/encoding/string/percent-encode.md[m
[1mindex 1e101c7..6ee6dbc 100644[m
[1m--- a/src/encoding/string/percent-encode.md[m
[1m+++ b/src/encoding/string/percent-encode.md[m
[36m@@ -19,7 +19,7 @@[m [muse url::percent_encoding::{utf8_percent_encode, percent_decode, DEFAULT_ENCODE_[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let input = "confident, productive systems programming";[m
 [m
     let iter = utf8_percent_encode(input, DEFAULT_ENCODE_SET);[m
[36m@@ -33,7 +33,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 The encode set defines which bytes (in addition to non-ASCII and controls) need[m
[1mdiff --git a/src/file/dir/find-file.md b/src/file/dir/find-file.md[m
[1mindex 0a0f0d3..131addd 100644[m
[1m--- a/src/file/dir/find-file.md[m
[1m+++ b/src/file/dir/find-file.md[m
[36m@@ -21,7 +21,7 @@[m [muse walkdir::WalkDir;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     for entry in WalkDir::new(".")[m
             .follow_links(true)[m
             .into_iter()[m
[36m@@ -37,7 +37,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`follow_links`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.follow_links[m
[1mdiff --git a/src/file/dir/ignore-case.md b/src/file/dir/ignore-case.md[m
[1mindex 8e99f6b..c49c620 100644[m
[1m--- a/src/file/dir/ignore-case.md[m
[1m+++ b/src/file/dir/ignore-case.md[m
[36m@@ -20,7 +20,7 @@[m [muse glob::{glob_with, MatchOptions};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let options = MatchOptions {[m
         case_sensitive: false,[m
         ..Default::default()[m
[36m@@ -33,7 +33,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html[m
[1mdiff --git a/src/file/dir/modified.md b/src/file/dir/modified.md[m
[1mindex 7a0a5ed..6f6987d 100644[m
[1m--- a/src/file/dir/modified.md[m
[1m+++ b/src/file/dir/modified.md[m
[36m@@ -23,7 +23,7 @@[m [muse std::{env, fs};[m
 #     }[m
 # }[m
 #[m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let current_dir = env::current_dir()?;[m
     println!([m
         "Entries modified in the last 24 hours in {:?}:",[m
[36m@@ -51,7 +51,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`DirEntry::path`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.path[m
[1mdiff --git a/src/file/dir/png.md b/src/file/dir/png.md[m
[1mindex 264632b..317534f 100644[m
[1m--- a/src/file/dir/png.md[m
[1m+++ b/src/file/dir/png.md[m
[36m@@ -22,7 +22,7 @@[m [muse glob::glob;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     for entry in glob("**/*.png")? {[m
         println!("{}", entry?.display());[m
     }[m
[36m@@ -30,5 +30,5 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
[1mdiff --git a/src/file/read-write/memmap.md b/src/file/read-write/memmap.md[m
[1mindex 261ce0b..d40ea54 100644[m
[1m--- a/src/file/read-write/memmap.md[m
[1m+++ b/src/file/read-write/memmap.md[m
[36m@@ -25,7 +25,7 @@[m [muse memmap::Mmap;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
 #     write!(File::create("content.txt")?, "My hovercraft is full of eels!")?;[m
 #[m
     let file = File::open("content.txt")?;[m
[36m@@ -40,7 +40,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Mmap::map`]: https://docs.rs/memmap/*/memmap/struct.Mmap.html#method.map[m
[1mdiff --git a/src/file/read-write/read-file.md b/src/file/read-write/read-file.md[m
[1mindex 3948373..ef1057c 100644[m
[1m--- a/src/file/read-write/read-file.md[m
[1m+++ b/src/file/read-write/read-file.md[m
[36m@@ -21,7 +21,7 @@[m [muse std::io::{Write, BufReader, BufRead};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let path = "lines.txt";[m
 [m
     let mut output = File::create(path)?;[m
[36m@@ -37,7 +37,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`BufRead::lines`]: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines[m
[1mdiff --git a/src/file/read-write/same-file.md b/src/file/read-write/same-file.md[m
[1mindex 1689a2d..8068b22 100644[m
[1m--- a/src/file/read-write/same-file.md[m
[1m+++ b/src/file/read-write/same-file.md[m
[36m@@ -22,7 +22,7 @@[m [muse std::io::{BufRead, BufReader};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let path_to_read = Path::new("new.txt");[m
 [m
     let stdout_handle = Handle::stdout()?;[m
[36m@@ -41,7 +41,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 ```bash[m
[1mdiff --git a/src/net/server/listen-unused.md b/src/net/server/listen-unused.md[m
[1mindex efde4d9..f6994f4 100644[m
[1m--- a/src/net/server/listen-unused.md[m
[1m+++ b/src/net/server/listen-unused.md[m
[36m@@ -19,7 +19,7 @@[m [muse std::io::Read;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let loopback = Ipv4Addr::new(127, 0, 0, 1);[m
     let socket = SocketAddrV4::new(loopback, 0);[m
     let listener = TcpListener::bind(socket)?;[m
[36m@@ -33,5 +33,5 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
[1mdiff --git a/src/os/external/continuous.md b/src/os/external/continuous.md[m
[1mindex 24f39ec..6a5bc5e 100644[m
[1m--- a/src/os/external/continuous.md[m
[1m+++ b/src/os/external/continuous.md[m
[36m@@ -23,7 +23,7 @@[m [muse std::io::{BufRead, BufReader};[m
 #     }[m
 # }[m
 #[m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let stdout = Command::new("journalctl")[m
         .stdout(Stdio::piped())[m
         .spawn()?[m
[36m@@ -41,7 +41,7 @@[m [mfn run() -> Result<()> {[m
      Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html[m
[1mdiff --git a/src/os/external/error-file.md b/src/os/external/error-file.md[m
[1mindex 62c4393..444bb2b 100644[m
[1m--- a/src/os/external/error-file.md[m
[1m+++ b/src/os/external/error-file.md[m
[36m@@ -25,7 +25,7 @@[m [muse std::process::{Command, Stdio};[m
 #     }[m
 # }[m
 #[m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let outputs = File::create("out.txt")?;[m
     let errors = outputs.try_clone()?;[m
 [m
[36m@@ -39,7 +39,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`File::try_clone`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone[m
[1mdiff --git a/src/os/external/piped.md b/src/os/external/piped.md[m
[1mindex 42942cb..1ae7b0c 100644[m
[1m--- a/src/os/external/piped.md[m
[1m+++ b/src/os/external/piped.md[m
[36m@@ -22,7 +22,7 @@[m [muse std::process::{Command, Stdio};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let directory = std::env::current_dir()?;[m
     let mut du_output_child = Command::new("du")[m
         .arg("-ah")[m
[36m@@ -61,7 +61,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html[m
[1mdiff --git a/src/os/external/process-output.md b/src/os/external/process-output.md[m
[1mindex bd140a6..44c74fc 100644[m
[1m--- a/src/os/external/process-output.md[m
[1m+++ b/src/os/external/process-output.md[m
[36m@@ -27,7 +27,7 @@[m [mstruct Commit {[m
     message: String,[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let output = Command::new("git").arg("log").arg("--oneline").output()?;[m
 [m
     if !output.status.success() {[m
[36m@@ -53,7 +53,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html[m
[1mdiff --git a/src/os/external/send-input.md b/src/os/external/send-input.md[m
[1mindex ebcda5d..8edf940 100644[m
[1m--- a/src/os/external/send-input.md[m
[1m+++ b/src/os/external/send-input.md[m
[36m@@ -21,7 +21,7 @@[m [muse std::process::{Command, Stdio};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m
     let mut child = Command::new("python").stdin(Stdio::piped())[m
         .stderr(Stdio::piped())[m
         .stdout(Stdio::piped())[m
[36m@@ -48,7 +48,7 @@[m [mfn run() -> Result<()> {[m
     }[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m
 ```[m
 [m
 [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html[m
[1mdiff --git a/src/text/regex/filter-log.md b/src/text/regex/filter-log.md[m
[1mindex c27e181..e92be98 100644[m
[1m--- a/src/text/regex/filter-log.md[m
[1m+++ b/src/text/regex/filter-log.md[m
[36m@@ -26,7 +26,7 @@[m [muse regex::RegexSetBuilder;[m
 #     }[m
 # }[m
 #[m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let log_path = "application.log";[m
     let buffered = BufReader::new(File::open(log_path)?);[m
 [m
[36m@@ -46,7 +46,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`regex::RegexSet`]: https://docs.rs/regex/*/regex/struct.RegexSet.html[m
[1mdiff --git a/src/text/regex/phone.md b/src/text/regex/phone.md[m
[1mindex 5911c83..78497c5 100644[m
[1m--- a/src/text/regex/phone.md[m
[1m+++ b/src/text/regex/phone.md[m
[36m@@ -32,7 +32,7 @@[m [mimpl<'a> fmt::Display for PhoneNumber<'a> {[m
     }[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let phone_text = "[m
     +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)[m
     (202) 991 9534[m
[36m@@ -79,7 +79,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`Regex::captures_iter`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.captures_iter[m
[1mdiff --git a/src/web/clients/api/paginated.md b/src/web/clients/api/paginated.md[m
[1mindex a5f41a4..7c01552 100644[m
[1m--- a/src/web/clients/api/paginated.md[m
[1m+++ b/src/web/clients/api/paginated.md[m
[36m@@ -90,12 +90,12 @@[m [mimpl Iterator for ReverseDependencies {[m
     }[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     for dep in ReverseDependencies::of("serde")? {[m
         println!("reverse dependency: {}", dep?.crate_id);[m
     }[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
[1mdiff --git a/src/web/clients/api/rate-limited.md b/src/web/clients/api/rate-limited.md[m
[1mindex 07d5b54..d267759 100644[m
[1m--- a/src/web/clients/api/rate-limited.md[m
[1m+++ b/src/web/clients/api/rate-limited.md[m
[36m@@ -30,7 +30,7 @@[m [mheader! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }[m
 header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize] }[m
 header! { (XRateLimitReset, "X-RateLimit-Reset") => [u64] }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let url = "https://api.github.com/users/rust-lang-nursery ";[m
     let client = reqwest::Client::new();[m
     let response = client.get(url).send()?;[m
[36m@@ -66,7 +66,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`hyper::header!`]: https://doc.servo.org/hyper/header/index.html#defining-custom-headers[m
[1mdiff --git a/src/web/clients/api/rest-get.md b/src/web/clients/api/rest-get.md[m
[1mindex 0f63cee..53f4497 100644[m
[1m--- a/src/web/clients/api/rest-get.md[m
[1m+++ b/src/web/clients/api/rest-get.md[m
[36m@@ -24,7 +24,7 @@[m [mstruct User {[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",[m
                               owner = "rust-lang-nursery",[m
                               repo = "rust-cookbook");[m
[36m@@ -36,7 +36,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html[m
[1mdiff --git a/src/web/clients/api/rest-head.md b/src/web/clients/api/rest-head.md[m
[1mindex 8c4ac64..0895fa7 100644[m
[1m--- a/src/web/clients/api/rest-head.md[m
[1m+++ b/src/web/clients/api/rest-head.md[m
[36m@@ -22,7 +22,7 @@[m [muse reqwest::ClientBuilder;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let user = "ferris-the-crab";[m
     let request_url = format!("https://api.github.com/users/{}", user);[m
     println!("{}", request_url);[m
[36m@@ -40,7 +40,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head[m
[1mdiff --git a/src/web/clients/api/rest-post.md b/src/web/clients/api/rest-post.md[m
[1mindex 460d72b..a3c7260 100644[m
[1m--- a/src/web/clients/api/rest-post.md[m
[1m+++ b/src/web/clients/api/rest-post.md[m
[36m@@ -36,7 +36,7 @@[m [mstruct Gist {[m
     html_url: String,[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let gh_user = env::var("GH_USER")?;[m
     let gh_pass = env::var("GH_PASS")?;[m
 [m
[36m@@ -69,7 +69,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 The example uses [HTTP Basic Auth] in order to authorize access to [GitHub API].[m
[1mdiff --git a/src/web/clients/download/basic.md b/src/web/clients/download/basic.md[m
[1mindex abb63ff..c668409 100644[m
[1m--- a/src/web/clients/download/basic.md[m
[1m+++ b/src/web/clients/download/basic.md[m
[36m@@ -26,7 +26,7 @@[m [muse tempdir::TempDir;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let tmp_dir = TempDir::new("example")?;[m
     let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";[m
     let mut response = reqwest::get(target)?;[m
[36m@@ -48,7 +48,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`File`]: https://doc.rust-lang.org/std/fs/struct.File.html[m
[1mdiff --git a/src/web/clients/download/partial.md b/src/web/clients/download/partial.md[m
[1mindex bcfeab7..ff9644c 100644[m
[1m--- a/src/web/clients/download/partial.md[m
[1m+++ b/src/web/clients/download/partial.md[m
[36m@@ -64,7 +64,7 @@[m [muse reqwest::StatusCode;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let url = "https://httpbin.org/range/102400?duration=2";[m
     const CHUNK_SIZE: u32 = 10240;[m
 [m
[36m@@ -94,7 +94,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`reqwest::Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head[m
[1mdiff --git a/src/web/clients/download/post-file.md b/src/web/clients/download/post-file.md[m
[1mindex d75755d..5f03294 100644[m
[1m--- a/src/web/clients/download/post-file.md[m
[1m+++ b/src/web/clients/download/post-file.md[m
[36m@@ -26,7 +26,7 @@[m [muse reqwest::Client;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let paste_api = "https://paste.rs";[m
     let file = File::open("message")?;[m
 [m
[36m@@ -37,7 +37,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`Client::post`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post[m
[1mdiff --git a/src/web/clients/requests/get.md b/src/web/clients/requests/get.md[m
[1mindex 2c6a3af..9195fef 100644[m
[1m--- a/src/web/clients/requests/get.md[m
[1m+++ b/src/web/clients/requests/get.md[m
[36m@@ -21,7 +21,7 @@[m [muse std::io::Read;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let mut res = reqwest::get("http://httpbin.org/get")?;[m
     let mut body = String::new();[m
     res.read_to_string(&mut body)?;[m
[36m@@ -33,7 +33,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string[m
[1mdiff --git a/src/web/clients/requests/header.md b/src/web/clients/requests/header.md[m
[1mindex fc27e25..68a41fe 100644[m
[1m--- a/src/web/clients/requests/header.md[m
[1m+++ b/src/web/clients/requests/header.md[m
[36m@@ -43,7 +43,7 @@[m [mpub struct HeadersEcho {[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let url = Url::parse_with_params("http://httpbin.org/headers",[m
                                      &[("lang", "rust"), ("browser", "servo")])?;[m
 [m
[36m@@ -66,7 +66,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`header::Authorization`]: https://doc.servo.org/hyper/header/struct.Authorization.html[m
[1mdiff --git a/src/web/mime/request.md b/src/web/mime/request.md[m
[1mindex 46ba256..8d3276f 100644[m
[1m--- a/src/web/mime/request.md[m
[1m+++ b/src/web/mime/request.md[m
[36m@@ -26,7 +26,7 @@[m [muse reqwest::header::ContentType;[m
 #    }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png")?;[m
     let headers = response.headers();[m
 [m
[36m@@ -50,7 +50,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`mime::Mime`]: https://docs.rs/mime/*/mime/struct.Mime.html[m
[1mdiff --git a/src/web/scraping/broken.md b/src/web/scraping/broken.md[m
[1mindex 2037fe7..07460f2 100644[m
[1m--- a/src/web/scraping/broken.md[m
[1m+++ b/src/web/scraping/broken.md[m
[36m@@ -49,7 +49,7 @@[m [mfn check_link(url: &Url) -> Result<bool> {[m
     Ok(res.status() != StatusCode::NotFound)[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let url = Url::parse("https://www.rust-lang.org/en-US/")?;[m
 [m
     let res = reqwest::get(url.as_ref())?;[m
[36m@@ -73,7 +73,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr[m
[1mdiff --git a/src/web/scraping/extract-links.md b/src/web/scraping/extract-links.md[m
[1mindex 0ba3176..58d7d8a 100644[m
[1m--- a/src/web/scraping/extract-links.md[m
[1m+++ b/src/web/scraping/extract-links.md[m
[36m@@ -24,7 +24,7 @@[m [muse select::predicate::Name;[m
 #    }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let res = reqwest::get("https://www.rust-lang.org/en-US/")?;[m
 [m
     Document::from_read(res)?[m
[36m@@ -35,7 +35,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr[m
[1mdiff --git a/src/web/scraping/unique.md b/src/web/scraping/unique.md[m
[1mindex 3a15b44..5d81f98 100644[m
[1m--- a/src/web/scraping/unique.md[m
[1m+++ b/src/web/scraping/unique.md[m
[36m@@ -51,7 +51,7 @@[m [mfn extract_links(content: &str) -> Result<HashSet<Cow<str>>> {[m
     Ok(links)[m
 }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let mut content = String::new();[m
     reqwest::get([m
         "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",[m
[36m@@ -63,7 +63,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html[m
[1mdiff --git a/src/web/url/base.md b/src/web/url/base.md[m
[1mindex e6ff4d9..2813cc8 100644[m
[1m--- a/src/web/url/base.md[m
[1m+++ b/src/web/url/base.md[m
[36m@@ -23,7 +23,7 @@[m [muse url::Url;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let full = "https://github.com/rust-lang/cargo?asdf";[m
 [m
     let url = Url::parse(full)?;[m
[36m@@ -50,7 +50,7 @@[m [mfn base_url(mut url: Url) -> Result<Url> {[m
     Ok(url)[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`PathSegmentsMut::clear`]: https://docs.rs/url/*/url/struct.PathSegmentsMut.html#method.clear[m
[1mdiff --git a/src/web/url/fragment.md b/src/web/url/fragment.md[m
[1mindex 457f9c4..d876ffc 100644[m
[1m--- a/src/web/url/fragment.md[m
[1m+++ b/src/web/url/fragment.md[m
[36m@@ -17,14 +17,14 @@[m [muse url::{Url, Position};[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;[m
     let cleaned: &str = &parsed[..Position::AfterPath];[m
     println!("cleaned: {}", cleaned);[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`url::Position`]: https://docs.rs/url/*/url/enum.Position.html[m
[1mdiff --git a/src/web/url/new.md b/src/web/url/new.md[m
[1mindex dfac5a9..49727bc 100644[m
[1m--- a/src/web/url/new.md[m
[1m+++ b/src/web/url/new.md[m
[36m@@ -17,7 +17,7 @@[m [muse url::Url;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let path = "/rust-lang/cargo";[m
 [m
     let gh = build_github_url(path)?;[m
[36m@@ -37,7 +37,7 @@[m [mfn build_github_url(path: &str) -> Result<Url> {[m
     Ok(joined)[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`join`]: https://docs.rs/url/*/url/struct.Url.html#method.join[m
[1mdiff --git a/src/web/url/origin.md b/src/web/url/origin.md[m
[1mindex 6d7d91f..6142fe9 100644[m
[1m--- a/src/web/url/origin.md[m
[1m+++ b/src/web/url/origin.md[m
[36m@@ -18,7 +18,7 @@[m [muse url::{Url, Host};[m
 #     }[m
 # }[m
 #[m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let s = "ftp://rust-lang.org/examples";[m
 [m
     let url = Url::parse(s)?;[m
[36m@@ -31,7 +31,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`origin`] produces the same result.[m
[36m@@ -49,7 +49,7 @@[m [muse url::{Url, Origin, Host};[m
 #     }[m
 # }[m
 #[m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let s = "ftp://rust-lang.org/examples";[m
 [m
     let url = Url::parse(s)?;[m
[36m@@ -66,7 +66,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`origin`]: https://docs.rs/url/*/url/struct.Url.html#method.origin[m
[1mdiff --git a/src/web/url/parse.md b/src/web/url/parse.md[m
[1mindex 3a4631c..ab50f48 100644[m
[1m--- a/src/web/url/parse.md[m
[1m+++ b/src/web/url/parse.md[m
[36m@@ -22,7 +22,7 @@[m [muse url::Url;[m
 #     }[m
 # }[m
 [m
[31m-fn run() -> Result<()> {[m
[32m+[m[32mfn main() -> Result<()> {[m[41m[m
     let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";[m
 [m
     let parsed = Url::parse(s)?;[m
[36m@@ -31,7 +31,7 @@[m [mfn run() -> Result<()> {[m
     Ok(())[m
 }[m
 #[m
[31m-# quick_main!(run);[m
[32m+[m[41m[m
 ```[m
 [m
 [`parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse[m
