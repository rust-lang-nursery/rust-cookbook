# Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Test in parallel if any or all elements of a collection match a given predicate][ex-rayon-any-all] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Search items using given predicate in parallel][ex-rayon-parallel-search] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Sort a vector in parallel][ex-rayon-parallel-sort] | [![rayon-badge]][rayon] [![rand-badge]][rand] | [![cat-concurrency-badge]][cat-concurrency] |
| [Map-reduce in parallel][ex-rayon-map-reduce] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Generate jpg thumbnails in parallel][ex-rayon-thumbnails] | [![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |
| [Spawn a short-lived thread][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Draw fractal dispatching work to a thread pool][ex-threadpool-fractal] | [![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering] |
| [Calculate SHA1 sum of *.iso files concurrently][ex-threadpool-walk]  | [![threadpool-badge]][threadpool] [![walkdir-badge]][walkdir] [![num_cpus-badge]][num_cpus] [![ring-badge]][ring] | [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem] |

[ex-rayon-iter-mut]: #ex-rayon-iter-mut
<a name="ex-rayon-iter-mut"></a>
## Mutate the elements of an array in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

The example uses the `rayon` crate, which is a data parallelism library for Rust.
`rayon` provides the [`par_iter_mut`] method for any parallel iterable data type.
It lets us write iterator-like chains that execute in parallel.

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];

    arr.par_iter_mut().for_each(|p| *p -= 1);

    println!("{:?}", arr);
}
```

[ex-rayon-any-all]: #ex-rayon-any-all
<a name="ex-rayon-any-all"></a>
## Test in parallel if any or all elements of a collection match a given predicate

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example demonstrates using the [`rayon::any`] and [`rayon::all`] methods, which are parallelized counterparts to [`std::any`] and [`std::all`]. [`rayon::any`] checks in parallel whether any element of the iterator matches the predicate, and returns as soon as one is found. [`rayon::all`] checks in parallel whether all elements of the iterator match the predicate, and returns as soon as a non-matching element is found.

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0)); // None are odd.
    assert!(vec.par_iter().all(|n| (*n % 2) == 0)); // All are even.
    assert!(!vec.par_iter().any(|n| *n > 8 )); // None are greater than 8.
    assert!(vec.par_iter().all(|n| *n <= 8 )); // All are less than or equal to 8.

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0)); // At least 1 is odd.
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0)); // Not all are even.
    assert!(vec.par_iter().any(|n| *n > 8 )); // At least 1 is greater than 8.
    assert!(!vec.par_iter().all(|n| *n <= 8 )); // Not all are less than or equal to 8.
}
```

[ex-rayon-parallel-sort]: #ex-rayon-parallel-sort
<a name="ex-rayon-parallel-sort"></a>
## Sort a vector in parallel

[![rayon-badge]][rayon] [![rand-badge]][rand] [![cat-concurrency-badge]][cat-concurrency]

This example will sort in parallel a vector of Strings.

[1] We start by preallocating a vector of empty Strings, so we can mutate the information in parallel later,
to populate the vector with random Strings.

[2] `par_iter_mut().for_each` takes a closure and applies it in parallel on all the elements of the vector.<br/>
[3] Inside the passed closure we modify the element in the vector with a 5 character-long String, random generated.

[4] We have [`multiple options`] to sort an Iterable data type, we chose here to use [`par_sort_unstable`]
because it is usually faster than [`stable sorting`] algorithms which `rayon` also supports.

```rust
extern crate rand;
extern crate rayon;

use rand::Rng;
use rayon::prelude::*;

fn main() {
    // [1]
    let mut vec = vec![String::new(); 100_000];

    // [2]
    vec.par_iter_mut().for_each(|p| {
        // [3]
        *p = rand::weak_rng().gen_ascii_chars().take(5).collect()
    });

    // [4]
    vec.par_sort_unstable();
}
```

[ex-rayon-map-reduce]: #ex-rayon-map-reduce
<a name="ex-rayon-map-reduce"></a>
## Map-reduce in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::filter`], [`rayon::map`], and [`rayon::reduce`]
to calculate the conditional average age of a vector of `Person` objects.

[`rayon::filter`] allows us to (in parallel) conditionally include elements from
a collection that satisfy the given predicate.  Similarly, [`rayon::map`] and
[`rayon::reduce`] allow us to transform the filtered elements via a unary
operation and reduce them to a single value via a given binary operation,
respectively. We also show use of [`rayon::sum`], which has the same result as
the reduce operation in this example.

```rust
extern crate rayon;

use rayon::prelude::*;

struct Person {
    age: u32,
}

fn main() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32/ num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}
```

[ex-rayon-parallel-search]: #ex-rayon-parallel-search
<a name="ex-rayon-parallel-search"></a>
## Search items using given predicate in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

This example uses [`rayon::find_any`] and [`par_iter`] to search a vector in
parallel for an element satisfying the predicate in the given closure.

If there are multiple elements satisfying the predicate defined in the closure
argument of [`rayon::find_any`], we are only guaranteed that one of them will be
found, but not necessarily that the first will be found.

Also note that the argument to the closure is a reference to a reference
(`&&x`). Please see the discussion on [`std::find`] for additional details.

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}
```

[ex-rayon-thumbnails]: #ex-rayon-thumbnails
<a name="ex-rayon-thumbnails"></a>
## Generate jpg thumbnails in parallel

[![rayon-badge]][rayon] [![glob-badge]][glob] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency] [![cat-filesystem-badge]][cat-filesystem]

This example generates thumbnails for all .jpg in the current directory and saves them in a new folder called `thumbnails`.

Files are found using [`glob::glob_with`] to match case insensitively on both `.jpg` and `.JPG`. `rayon` is then used to resize images in parallel using [`par_iter`] along with the `make_thumbnail()` helper function which internally uses [`DynamicImage::resize`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate glob;
extern crate image;
extern crate rayon;

use std::path::Path;
use std::fs::create_dir_all;

# use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::{FilterType, ImageError};
use rayon::prelude::*;

# error_chain! {
#     foreign_links {
#         Image(ImageError);
#         Io(std::io::Error);
#         Glob(glob::PatternError);
#     }
# }

fn run() -> Result<()> {
    // find all files in current directory that have a .jpg extension
    // use the default MatchOptions so the search is case insensitive
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", &options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        bail!("No .jpg files found in current directory");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|e| e.chain_err(|| path.display().to_string()))
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x.display_chain()));

    println!("{} thumbnails saved successfully", files.len() - image_failures.len());
    Ok(())
}

/// Resize `original` to have a maximum dimension of `longest_edge` and save the
/// resized image to the `thumb_dir` folder
fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
          .save(file_path)?)
}
#
# quick_main!(run);
```

[ex-crossbeam-spawn]: #ex-crossbeam-spawn
<a name="ex-crossbeam-spawn"></a>
## Spawn a short-lived thread

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

The example uses the `crossbeam` crate, which provides data structures and functions
for concurrent and parallel programming. [`Scope::spawn`] spawns a new scoped thread that is guaranteed
to terminate before returning from the closure that passed into [`crossbeam::scope`] function, meaning that
you can reference data from the calling function.

```rust
extern crate crossbeam;

use std::cmp;

fn main() {
    let arr = &[-4, 1, 10, 25];
    let max = find_max(arr, 0, arr.len());
    assert_eq!(25, max);
}

fn find_max(arr: &[i32], start: usize, end: usize) -> i32 {
    // Perform sequential computation if there are only a few elements.
    const THRESHOLD: usize = 2;
    if end - start <= THRESHOLD {
        return *arr.iter().max().unwrap();
    }

    let mid = start + (end - start) / 2;
    crossbeam::scope(|scope| {
        let left = scope.spawn(|| find_max(arr, start, mid));
        let right = scope.spawn(|| find_max(arr, mid, end));

        cmp::max(left.join(), right.join())
    })
}
```

[ex-threadpool-fractal]: #ex-threadpool-fractal
<a name="ex-threadpool-fractal"></a>
## Draw fractal dispatching work to a thread pool

[![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]

This example draws a fractal from [Julia set] to an image utilizing a thread pool for computation.

<a href="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png"><img src="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png" width="150" /></a>

Firstly, the example allocates memory for output image of given width and height with [`ImageBuffer::new`]
and pre-calculates all possible RGB pixel values using [`Rgb::from_channels`].
Secondly, creates a new [`ThreadPool`] with thread count equal to number of
logical cores in CPU obtained with [`num_cpus::get`].
Subsequently, dispatches calculation to thread pool [`ThreadPool::execute`].

Lastly, collects calculation results via [`mpsc::channel`] with [`Receiver::recv`], draws them with [`ImageBuffer::put_pixel`] and encodes the final image into `output.png` using [`ImageBuffer::save`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate threadpool;
extern crate num;
extern crate num_cpus;
extern crate image;

use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;
use num::complex::Complex;
use image::{ImageBuffer, Pixel, Rgb};
#
# error_chain! {
#     foreign_links {
#         MpscRecv(RecvError);
#         Io(std::io::Error);
#     }
# }
#
# // Function converting intensity values to RGB
# // Based on http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
# fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
#     let wave = wavelength as f32;
#
#     let (r, g, b) = match wavelength {
#         380...439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
#         440...489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
#         490...509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
#         510...579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
#         580...644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
#         645...780 => (1.0, 0.0, 0.0),
#         _ => (0.0, 0.0, 0.0),
#     };
#
#     let factor = match wavelength {
#         380...419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
#         701...780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
#         _ => 1.0,
#     };
#
#     let (r, g, b) = (normalize(r, factor), normalize(g, factor), normalize(b, factor));
#     Rgb::from_channels(r, g, b, 0)
# }
#
# // Maps Julia set distance estimation to intensity values
# fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
#     let width = width as f32;
#     let height = height as f32;
#
#     let mut z = Complex {
#         // scale and translate the point to image coordinates
#         re: 3.0 * (x as f32 - 0.5 * width) / width,
#         im: 2.0 * (y as f32 - 0.5 * height) / height,
#     };
#
#     let mut i = 0;
#     for t in 0..max_iter {
#         if z.norm() >= 2.0 {
#             break;
#         }
#         z = z * z + c;
#         i = t;
#     }
#     i
# }
#
# // Normalizes color intensity values within RGB range
# fn normalize(color: f32, factor: f32) -> u8 {
#     ((color * factor).powf(0.8) * 255.) as u8
# }

fn run() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iterations = 300;

    // constants to tweak for appearance
    let c = Complex::new(-0.8, 0.156);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || for x in 0..width {
                         let i = julia(c, x, y, width, height, iterations);
                         let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                         tx.send((x, y, pixel)).expect("Could not send data!");
                     });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png")?;
    Ok(())
}
#
# quick_main!(run);
```

[ex-threadpool-walk]: #ex-threadpool-walk
<a name="ex-threadpool-walk"></a>

## Calculate SHA1 sum of *.iso files concurrently

[![threadpool-badge]][threadpool] [![num_cpus-badge]][num_cpus] [![walkdir-badge]][walkdir] [![ring-badge]][ring] [![cat-concurrency-badge]][cat-concurrency][![cat-filesystem-badge]][cat-filesystem]

This example calculates the SHA1 for every file present in the current directory. A threadpool is created using the number of cpus present in the system with [`num_cpus::get`]. Then every returned by [`Walkdir::new`] is passed into this pool to perform the operations of reading and computing SHA1. At the end the program waits for all jobs to finish. To get better results, compile this program in release mode.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate walkdir;
extern crate ring;
extern crate num_cpus;
extern crate threadpool;

# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#     }
# }
#
use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA1};

# // Verify the iso extension
# fn is_iso(entry: &Path) -> bool {
#     match entry.extension() {
#         Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
#         _ => false,
#     }
# }
#
fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P)> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA1);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

fn run() -> Result<()> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    // Look in the current directory.
    for entry in WalkDir::new("/home/user/Downloads")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path())) {
            let path = entry.path().to_owned();
            let tx = tx.clone();
            pool.execute(move || {
                let digest = compute_digest(path);
                tx.send(digest).expect("Could not send data!");
            });
        }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
#
# quick_main!(run);
```

{{#include links.md}}

<!-- API Reference -->

[`crossbeam::scope`]: https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
[`DynamicImage::resize`]: https://docs.rs/image/*/image/enum.DynamicImage.html#method.resize
[`glob::glob_with`]: https://docs.rs/glob/*/glob/fn.glob_with.html
[`ImageBuffer::new`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.new
[`ImageBuffer::put_pixel`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.put_pixel
[`ImageBuffer::save`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.save
[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[`mpsc::channel`]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
[`multiple options`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html
[`num_cpus::get`]: https://docs.rs/num_cpus/*/num_cpus/fn.get.html
[`par_iter_mut`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefMutIterator.html#tymethod.par_iter_mut
[`par_iter`]: https://docs.rs/rayon/*/rayon/iter/trait.IntoParallelRefIterator.html#tymethod.par_iter
[`par_sort_unstable`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort_unstable
[`ParallelIterator`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html
[`rayon::all`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.all
[`rayon::any`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.any
[`rayon::filter`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.filter
[`rayon::find_any`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.find_any
[`rayon::map`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.map
[`rayon::reduce`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.reduce
[`rayon::sum`]: https://docs.rs/rayon/*/rayon/iter/trait.ParallelIterator.html#method.sum
[`Receiver::recv`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv
[`Rgb::from_channels`]: https://docs.rs/image/*/image/struct.Rgb.html#method.from_channels
[`Scope::spawn`]: https://docs.rs/crossbeam/*/crossbeam/struct.Scope.html#method.spawn
[`stable sorting`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_sort
[`std::all`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
[`std::any`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
[`std::find`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
[`ThreadPool::execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute
[`ThreadPool`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html
[`Walkdir::new`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.new

<!-- Other Reference -->

[Julia set]: https://en.wikipedia.org/wiki/Julia_set
