# Parallel Prime Calculations
[![crossbeam-badge]][crossbeam]

## Calculate primes upto a number

```rust

/**
 * Simple parallel programming example,
 * calculating all primes less than n
 * using sieve of erastonthes algorithm.
 */

extern crate crossbeam;

// Basic implementation of Sieve of Erastonthes
fn eratosthenes(mut primes: Vec<bool>, n: usize) {
    let limit = (n as f64).sqrt() as usize;

    for i in 2..limit {
        if primes[i] == true {
            let mut j = 2*i;
            while j < n {
                primes[j] = false;
                j += i;
            }
        }
    }
}

// Parallel implementation of Sieve of Erastonthes
fn parallel_eratosthenes(mut primes: Vec<bool>, n: usize) {
    let limit = (n as f64).sqrt() as usize;

    // For all numbers between i and sqrt(n),
    // Look for all 'true' and report multiples
    // as false.
    for i in 2..limit {
        if primes[i] == true {
            let mut j = 2*i;

            // Using crossbeam's scoped thread implementation
            crossbeam::scope(|scope| {
                scope.spawn(|| {
                    while j < n {
                        primes[j] = false;
                        j += i;
                    }
                });
            });
        }
    }
}

fn main() {

    // We shall calculate all primes up to 100
    let num = 100;

    // First, we create an array of size num,
    // with all values except the first set to true
    let mut primes = vec![true; num];
    primes[0] = false;
    primes[1] = false;

    let par_primes = primes.clone();

    eratosthenes(primes, num);

    parallel_eratosthenes(par_primes, num);

}
```

This code above uses the Sieve of Erastonthes algorithm to calculate all primes up to the value passed to it. This is done using two separate implementations of the same algorithm, one with a single-threaded function and another using the Scope struct from the crate crossbeam. The purpose of this is to show the strength of the Scope struct, as a way to allow for threads to spawn with data shared with their parents.

<!-- Links -->

[crossbeam-badge]: https://img.shields.io/crates/v/crossbeam.svg?label=crossbeam
[crossbeam]: https://docs.rs/crossbeam
