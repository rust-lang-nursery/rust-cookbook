## Draw fractal dispatching work to a thread pool

[![rayon-badge]][rayon] [![num-badge]][num] [![std-badge]][std] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]

This recipe generates an image by drawing a fractal from the [Julia set]
with rayon's thread pool for distributed computation.

<a href="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png"><img src="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png" width="150" /></a>

Allocate memory for output image of given width and height with [`ImageBuffer::new`].
`wavelength_to_rgb` calculates RGB pixel values.
The image's raw bytes are split into one chunk per row with [`par_chunks_mut`],
and rayon distributes the rows over its global thread pool, which has one
thread per available core by default. Each worker writes the pixels of its row
directly into the buffer, so no channel is needed.
[`ImageBuffer::save`] writes the image to `output.png`.

```rust,edition2021,no_run
use anyhow::Result;
use image::{ImageBuffer, Rgb};
use num::complex::Complex;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSliceMut};
#
# // Function converting intensity values to RGB
# // Based on http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
# fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
#     let wave = wavelength as f32;
#
#     let (r, g, b) = match wavelength {
#         380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
#         440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
#         490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
#         510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
#         580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
#         645..=780 => (1.0, 0.0, 0.0),
#         _ => (0.0, 0.0, 0.0),
#     };
#
#     let factor = match wavelength {
#         380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
#         701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
#         _ => 1.0,
#     };
#
#     let (r, g, b) = (normalize(r, factor), normalize(g, factor), normalize(b, factor));
#     Rgb([r, g, b])
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

fn main() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let iterations = 300;

    let c = Complex::new(-0.8, 0.156);

    img.par_chunks_mut(width as usize * 3)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, bytes) in row.chunks_exact_mut(3).enumerate() {
                let i = julia(c, x as u32, y as u32, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                bytes.copy_from_slice(&pixel.0);
            }
        });

    let _ = img.save("output.png")?;
    Ok(())
}
```

[`ImageBuffer::new`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.new
[`ImageBuffer::save`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.save
[`par_chunks_mut`]: https://docs.rs/rayon/*/rayon/slice/trait.ParallelSliceMut.html#method.par_chunks_mut

[Julia set]: https://en.wikipedia.org/wiki/Julia_set
