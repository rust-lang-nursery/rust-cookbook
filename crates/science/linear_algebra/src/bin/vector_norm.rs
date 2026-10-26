use ndarray::{Array1, ArrayView1, array};

fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e / norm);
    x
}

fn main() {
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn norms() {
        let x = array![3., -4.];
        assert_abs_diff_eq!(l1_norm(x.view()), 7.0);
        assert_abs_diff_eq!(l2_norm(x.view()), 5.0);
    }

    #[test]
    fn normalized_vector_has_unit_length() {
        let x = normalize(array![1., 2., 3., 4., 5.]);
        assert_abs_diff_eq!(l2_norm(x.view()), 1.0, epsilon = 1e-12);
    }
}
