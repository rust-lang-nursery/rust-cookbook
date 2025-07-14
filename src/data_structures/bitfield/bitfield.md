## Define and operate on a type represented as a bitfield

[![bitflags-badge]][bitflags] [![cat-no-std-badge]][cat-no-std]

Creates type safe bitfield type `MyFlags` with elementary `clear` operation as well as [`Display`] trait for it.
Subsequently, shows basic bitwise operations and formatting.

```rust,edition2018
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MyFlags(u32);

impl MyFlags {
    const FLAG_A: MyFlags = MyFlags(0b00000001);
    const FLAG_B: MyFlags = MyFlags(0b00000010);
    const FLAG_C: MyFlags = MyFlags(0b00000100);
    const FLAG_ABC: MyFlags = MyFlags(Self::FLAG_A.0 | Self::FLAG_B.0 | Self::FLAG_C.0);

    fn empty() -> Self {
        MyFlags(0)
    }

    fn bits(&self) -> u32 {
        self.0
    }

    pub fn clear(&mut self) -> &mut MyFlags {
        *self = MyFlags::empty();
        self
    }
}

impl std::ops::BitOr for MyFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        MyFlags(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for MyFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        MyFlags(self.0 & rhs.0)
    }
}

impl std::ops::Sub for MyFlags {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        MyFlags(self.0 & !rhs.0)
    }
}

impl std::ops::Not for MyFlags {
    type Output = Self;
    fn not(self) -> Self {
        MyFlags(!self.0 & 0b00000111) // Only consider defined flags
    }
}

impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}

fn main() {
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), MyFlags::FLAG_ABC);   
    assert_eq!((e1 & e2), MyFlags::FLAG_C);    
    assert_eq!((e1 - e2), MyFlags::FLAG_A);    
    assert_eq!(!e2, MyFlags::FLAG_A);           

    let mut flags = MyFlags::FLAG_ABC;
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(format!("{}", flags.clear()), "00000000000000000000000000000000");
    assert_eq!(format!("{:?}", MyFlags::FLAG_B), "MyFlags(2)");
    assert_eq!(format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B), "MyFlags(3)");
}
