fn main() {
    println!("logical cores:  {}", num_cpus::get());
    println!("physical cores: {}", num_cpus::get_physical());
}
