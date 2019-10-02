## Matrix eigenvalues 
[![nalgebra-badge]][nalgebra] 
[![nalgebra-lapack-badge]][nalgebra-lapack] [![cat-science-badge]][cat-science]

Creates a real 2x2 matrix with [`nalgebra::Matrix2`] and finds the eigenvalues, if possible. See [Matrix decompositions] for other matrix decomposition methods. 

```rust
extern crate nalgebra;
extern crate nalgebra_lapack; 

use nalgebra::Matrix2;
use nalgebra_lapack::Eigen; 

fn main() {
    let m1 = Matrix2::new(0.0, 1.0, -2.0, -3.0);
    println!("m1 = {}", m1);

    let find_eigenvectors = false; 
    let find_left_eigenvectors = false; 

    let eigensystem = Eigen::new(m1, find_eigenvectors, find_left_eigenvectors);
    match eigensystem {
        Some(soln) => {
            println!("The eigenvalues of m1 are: {}", soln.eigenvalues);
        }
        None => {
            println!("Couldn't solve the eigensystem!");
        }
    }
}
```

[`nalgebra::Matrix2`]: https://docs.rs/nalgebra/*/nalgebra/base/type.Matrix2.html
[Matrix decompositions]: https://nalgebra.org/decompositions_and_lapack/
