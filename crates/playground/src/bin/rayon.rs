use rayon::prelude::*;

fn main() {
    let squares: u64 = (1..=1_000u64).into_par_iter().map(|n| n * n).sum();
    println!("sum of squares: {squares}");

    let mut v = vec![5, 3, 8, 1, 9, 2];
    v.par_sort();
    println!("{v:?}");
}
