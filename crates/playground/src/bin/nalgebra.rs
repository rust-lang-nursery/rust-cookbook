use nalgebra::Matrix3;

fn main() {
    let m = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    println!("m = {m}");
    match m.try_inverse() {
        Some(inverse) => println!("inverse = {inverse}"),
        None => println!("not invertible"),
    }
}
