## Distance between two points on the Earth

[![std-badge]][std]

By default, Rust provides mathematical [float methods] such as
trigonometric functions, square root, conversion functions between
radians and degrees, and so forth.

The following example computes the distance in kilometers between two
points on the Earth with the [Haversine formula]. Points are expressed
as pairs of latitude and longitude in degrees. Then, [`to_radians`]
converts them in radian. [`sin`], [`cos`], [`powi`] and [`sqrt`]
compute the central angle. Finally, it's possible to calculate the
distance.

```rust
fn main() {
    let earth_radius_kilometer = 6371.0_f64;
    let (paris_latitude_degrees, paris_longitude_degrees) = (48.85341_f64, -2.34880_f64);
    let (london_latitude_degrees, london_longitude_degrees) = (51.50853_f64, -0.12574_f64);

    let paris_latitude = paris_latitude_degrees.to_radians();
    let london_latitude = london_latitude_degrees.to_radians();

    let delta_latitude = (paris_latitude_degrees - london_latitude_degrees).to_radians();
    let delta_longitude = (paris_longitude_degrees - london_longitude_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + paris_latitude.cos() * london_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between Paris and London on the surface of Earth is {:.1} kilometers",
        distance
    );
}
```

[float methods]: https://doc.rust-lang.org/std/primitive.f64.html#methods
[`to_radians`]: https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians
[`sin`]: https://doc.rust-lang.org/std/primitive.f64.html#method.sin
[`cos`]: https://doc.rust-lang.org/std/primitive.f64.html#method.cos
[`powi`]: https://doc.rust-lang.org/std/primitive.f64.html#method.powi
[`sqrt`]: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
[Haversine formula]: https://en.wikipedia.org/wiki/Haversine_formula
