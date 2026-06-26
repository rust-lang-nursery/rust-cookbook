## Resize and re-encode an image

[![image-badge]][image] [![cat-multimedia-badge]][cat-multimedia]

Serving a small preview of a large upload means decoding the original, shrinking
it, and writing it back in a web-friendly format. [`image::open`] reads the file
and infers the format from its contents. [`DynamicImage::resize`] scales the
image to fit a bounding box while preserving its aspect ratio. Its last argument
is the resampling [`FilterType`]: [`Lanczos3`] gives the best quality, while
[`Nearest`], [`Triangle`], [`CatmullRom`], and [`Gaussian`] trade quality for
speed. [`DynamicImage::save`] selects the encoder from the output extension, so
writing to `thumbnail.jpg` produces a JPEG.

```rust,no_run
{{#include ../../crates/multimedia/image/src/bin/resize.rs::26}}
```

[`image::open`]: https://docs.rs/image/*/image/fn.open.html
[`DynamicImage::resize`]: https://docs.rs/image/*/image/enum.DynamicImage.html#method.resize
[`FilterType`]: https://docs.rs/image/*/image/imageops/enum.FilterType.html
[`Lanczos3`]: https://docs.rs/image/*/image/imageops/enum.FilterType.html#variant.Lanczos3
[`Nearest`]: https://docs.rs/image/*/image/imageops/enum.FilterType.html#variant.Nearest
[`Triangle`]: https://docs.rs/image/*/image/imageops/enum.FilterType.html#variant.Triangle
[`CatmullRom`]: https://docs.rs/image/*/image/imageops/enum.FilterType.html#variant.CatmullRom
[`Gaussian`]: https://docs.rs/image/*/image/imageops/enum.FilterType.html#variant.Gaussian
[`DynamicImage::save`]: https://docs.rs/image/*/image/enum.DynamicImage.html#method.save

## Detect an image format from its bytes

[![image-badge]][image] [![cat-multimedia-badge]][cat-multimedia]

An upload may arrive with a misleading extension or none at all. [`guess_format`]
reads the leading magic bytes and returns the matching [`ImageFormat`], which in
turn gives the MIME type through [`ImageFormat::to_mime_type`]. Only the header is
examined, so a short prefix of the file is enough.

```rust,no_run
{{#include ../../crates/multimedia/image/src/bin/guess_format.rs::17}}
```

[`guess_format`]: https://docs.rs/image/*/image/fn.guess_format.html
[`ImageFormat`]: https://docs.rs/image/*/image/enum.ImageFormat.html
[`ImageFormat::to_mime_type`]: https://docs.rs/image/*/image/enum.ImageFormat.html#method.to_mime_type

## Inspect image EXIF metadata

[![kamadak-exif-badge]][kamadak-exif] [![cat-multimedia-badge]][cat-multimedia]

A photograph carries EXIF metadata describing the camera and the capture. The
[`kamadak-exif`][kamadak-exif] crate, imported as `exif`, reads it without
decoding the pixels. [`Reader::read_from_container`] parses the EXIF block from a
buffered reader. EXIF splits its fields across directories: the primary image and
an optional embedded thumbnail. [`Exif::get_field`] looks up a [`Tag`] in one of
them, chosen with [`In`], here [`In::PRIMARY`] for the primary image.
[`Field::display_value`] renders the value.

```rust,no_run
{{#include ../../crates/multimedia/image/src/bin/exif.rs::23}}
```

[`Reader::read_from_container`]: https://docs.rs/kamadak-exif/*/exif/struct.Reader.html#method.read_from_container
[`Exif::get_field`]: https://docs.rs/kamadak-exif/*/exif/struct.Exif.html#method.get_field
[`Field::display_value`]: https://docs.rs/kamadak-exif/*/exif/struct.Field.html#method.display_value
[`Tag`]: https://docs.rs/kamadak-exif/*/exif/struct.Tag.html
[`In`]: https://docs.rs/kamadak-exif/*/exif/struct.In.html
[`In::PRIMARY`]: https://docs.rs/kamadak-exif/*/exif/struct.In.html#associatedconstant.PRIMARY

{{#include ../links.md}}
