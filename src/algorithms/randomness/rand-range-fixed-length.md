```rust
use rand::Rng;
extern crate rand;
use std::convert::TryInto;
// Creates a random number which is a fixed length. Specifically, the length of the upper bound -1
pub fn create_fixed_length_random_number(_lower_bound: i64, _upper_bound: i64)  {
        let length: u64 = (_upper_bound.to_string().len() - 1).try_into().unwrap();
        let mut rng = rand::thread_rng();
        let mut my_rand: i64 = rng.gen_range(_lower_bound, _upper_bound).abs();
        while my_rand.to_string().len() < length.try_into().unwrap() {
            my_rand = my_rand + rng.gen_range(_lower_bound, _upper_bound - my_rand);
        }
        println!("Generated {:?} digit random number between {:?} and {:?}: {:?}", (_upper_bound.to_string().len() - 1), _lower_bound, _upper_bound - 1 , my_rand);
    }

fn main() {
    create_fixed_length_random_number(-10, -1);
 // Returns Generated 1 digit random number between -10 and -2: 7
    create_fixed_length_random_number(1, 99);   
 // Returns Generated 1 digit random number between 1 and 98: 24
    create_fixed_length_random_number(1, 100);
 // Returns Generated 2 digit random number between 1 and 99: 87
    create_fixed_length_random_number(1, 1000);
 // Returns Generated 3 digit random number between 1 and 999: 118
}
```
