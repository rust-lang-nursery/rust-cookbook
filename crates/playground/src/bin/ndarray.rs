use ndarray::arr2;

fn main() {
    let a = arr2(&[[1, 2], [3, 4]]);
    let b = arr2(&[[5, 6], [7, 8]]);
    println!("sum:\n{}", &a + &b);
    println!("product:\n{}", a.dot(&b));
}
