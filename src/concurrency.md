# Concurrency

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Mutate the elements of an array in parallel][ex-rayon-iter-mut] | [![rayon-badge]][rayon] | [![cat-concurrency-badge]][cat-concurrency] |
| [Spawn a short-lived thread][ex-crossbeam-spawn] | [![crossbeam-badge]][crossbeam] | [![cat-concurrency-badge]][cat-concurrency] |
| [Draw fractal dispatching work to a thread pool][ex-threadpool-fractal] | [![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] | [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering] |

[ex-rayon-iter-mut]: #ex-rayon-iter-mut
<a name="ex-rayon-iter-mut"></a>
## Mutate the elements of an array in parallel

[![rayon-badge]][rayon] [![cat-concurrency-badge]][cat-concurrency]

```rust
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];

    arr.par_iter_mut().for_each(|p| *p -= 1);

    println!("{:?}", arr);
}
```

The example uses the Rayon crate, which is a data parallelism library for Rust.
Rayon provides the `par_iter_mut()` method for any parallel iterable data type.
It lets us write iterator-like chains that execute in parallel.

[ex-crossbeam-spawn]: #ex-crossbeam-spawn
<a name="ex-crossbeam-spawn"></a>
## Spawn a short-lived thread

[![crossbeam-badge]][crossbeam] [![cat-concurrency-badge]][cat-concurrency]

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

The example uses `crossbeam` crate, which provides data structures and functions
for concurrent and parallel programming. [`Scope::spawn`] spawns a new scoped thread that is guaranteed
to terminate before returning from the closure that passed into [`crossbeam::scope`] function, meaning that
you can reference data from the calling function.

[ex-threadpool-fractal]: #ex-threadpool-fractal
<a name="ex-threadpool-fractal"></a>
## Draw fractal dispatching work to a thread pool

[![threadpool-badge]][threadpool] [![num-badge]][num] [![num_cpus-badge]][num_cpus] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]

Draws a fractal from [Julia set] to an image utilizing a thread pool for computation.

<a href="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png"><img src="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png" width="150" /></a>

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

Firstly, the example allocates memory for output image of given width and height with [`ImageBuffer::new`]
and pre-calculates all possible RGB pixel values using [`Rgb::from_channels`].
Secondly, creates a new [`ThreadPool`] with thread count equal to number of
logical cores in CPU obtained with [`num_cpus::get`].
Subsequently, dispatches calculation to thread pool [`ThreadPool::execute`].

Lastly, collects calculation results via [`mpsc::channel`] with [`Receiver::recv`], draws them with [`ImageBuffer::put_pixel`] and encodes the final image into `output.png` using [`ImageBuffer::save`].

<!-- Categories -->

[cat-concurrency-badge]: https://badge-cache.kominick.com/badge/concurrency--x.svg?style=social
[cat-concurrency]: https://crates.io/categories/concurrency
[cat-rendering-badge]: https://badge-cache.kominick.com/badge/rendering--x.svg?style=social
[cat-rendering]: https://crates.io/categories/rendering
[cat-science-badge]: https://badge-cache.kominick.com/badge/science--x.svg?style=social
[cat-science]: https://crates.io/categories/science

<!-- Crates -->

[crossbeam-badge]: https://badge-cache.kominick.com/crates/v/crossbeam.svg?label=crossbeam
[crossbeam]: https://docs.rs/crossbeam/
[image-badge]: https://badge-cache.kominick.com/crates/v/image.svg?label=image
[image]: https://docs.rs/image/
[num-badge]: https://badge-cache.kominick.com/crates/v/num.svg?label=num
[num]: https://docs.rs/num/
[num_cpus-badge]: https://badge-cache.kominick.com/crates/v/num_cpus.svg?label=num_cpus
[num_cpus]: https://docs.rs/num_cpus/
[rayon-badge]: https://badge-cache.kominick.com/crates/v/rayon.svg?label=rayon
[rayon]: https://docs.rs/rayon/
[threadpool-badge]: https://badge-cache.kominick.com/crates/v/threadpool.svg?label=threadpool
[threadpool]: https://docs.rs/threadpool/

<!-- Reference -->

[Julia set]: https://en.wikipedia.org/wiki/Julia_set
[`ImageBuffer::new`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.new
[`ImageBuffer::put_pixel`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.put_pixel
[`ImageBuffer::save`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.save
[`Receiver::recv`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv
[`Rgb::from_channels`]: https://docs.rs/image/*/image/struct.Rgb.html#method.from_channels
[`Scope::spawn`]: https://docs.rs/crossbeam/0.*/crossbeam/struct.Scope.html#method.spawn
[`ThreadPool::execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute
[`ThreadPool`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html
[`crossbeam::scope`]: https://docs.rs/crossbeam/0.*/crossbeam/fn.scope.html
[`mpsc::channel`]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
[`num_cpus::get`]: https://docs.rs/num_cpus/*/num_cpus/fn.get.html
