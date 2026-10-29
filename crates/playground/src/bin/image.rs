use image::{ImageBuffer, Rgb};

fn main() {
    // A 4x4 gradient built in memory; use `image::open(path)` to load a file.
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(4, 4, |x, y| Rgb([(x * 64) as u8, (y * 64) as u8, 128]));

    println!("{}x{} image", img.width(), img.height());
    println!("pixel (3, 3) = {:?}", img.get_pixel(3, 3));
}
