use rand::RngExt;
use rayon::prelude::*;

fn main() {
    let mut vec = vec![0; 1_000_000];
    rand::rng().fill(&mut vec[..]);

    vec.par_sort_unstable();

    let first = vec.first().unwrap();
    let last = vec.last().unwrap();
    assert!(first <= last);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sort() {
        let mut vec = vec![0i32; 1_000];
        rand::rng().fill(&mut vec[..]);
        vec.par_sort_unstable();
        assert!(vec.windows(2).all(|w| w[0] <= w[1]));
    }
}
