use approx::assert_abs_diff_eq;
use ndarray::Array;

fn main() {
    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;

    assert_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

    println!("c = {}", c);
    c[0] = 10.;
    d[1] = 10.;

    assert_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));

    // compare with a tolerance instead of exact equality
    let e = Array::from(vec![0.1 + 0.2, 1.0]);
    assert_abs_diff_eq!(e.to_vec()[..], [0.3, 1.0][..], epsilon = 1e-9);
}
