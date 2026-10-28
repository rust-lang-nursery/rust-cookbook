## Draw fractal dispatching work to a thread pool

[![rayon-badge]][rayon] [![num-badge]][num] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]

This recipe generates an image by drawing a fractal from the [Julia set]
with rayon's thread pool for distributed computation.

<a href="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png"><img src="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png" width="150" /></a>

Allocate memory for output image of given width and height with [`ImageBuffer::new`].
`wavelength_to_rgb` calculates RGB pixel values.
[`par_enumerate_pixels_mut`] hands rayon every pixel of the image along with
its coordinates, and rayon distributes them over its global thread pool, which
has one thread per available core by default. Each worker writes its pixels
directly into the buffer, so no channel is needed.
[`ImageBuffer::save`] writes the image to `output.png`.

```rust,no_run
{{#rustdoc_include ../../../crates/concurrency/parallel/src/bin/fractal.rs:1:20 }}
```

[`ImageBuffer::new`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.new
[`ImageBuffer::save`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.save
[`par_enumerate_pixels_mut`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.par_enumerate_pixels_mut

[Julia set]: https://en.wikipedia.org/wiki/Julia_set
