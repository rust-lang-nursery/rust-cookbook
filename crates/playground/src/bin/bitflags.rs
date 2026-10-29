use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Permissions: u32 {
        const READ = 0b001;
        const WRITE = 0b010;
        const EXECUTE = 0b100;
    }
}

fn main() {
    let mut p = Permissions::READ | Permissions::WRITE;
    println!("{p:?} can write: {}", p.contains(Permissions::WRITE));
    p.remove(Permissions::WRITE);
    p.insert(Permissions::EXECUTE);
    println!("{p:?} bits: {:#05b}", p.bits());
}
