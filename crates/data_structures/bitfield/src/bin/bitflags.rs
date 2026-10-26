use bitflags::{Flags, bitflags};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct MyFlags: u32 {
        const FLAG_A = 0b00000001;
        const FLAG_B = 0b00000010;
        const FLAG_C = 0b00000100;
        const FLAG_ABC = Self::FLAG_A.bits() | Self::FLAG_B.bits() | Self::FLAG_C.bits();
    }
}

fn main() {
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), MyFlags::FLAG_ABC);
    assert_eq!((e1 & e2), MyFlags::FLAG_C);
    assert_eq!((e1 - e2), MyFlags::FLAG_A);
    assert_eq!(!e2, MyFlags::FLAG_A);

    assert!(e1.contains(MyFlags::FLAG_A));
    assert!(!e1.contains(MyFlags::FLAG_B));

    let mut flags = MyFlags::FLAG_ABC;
    assert_eq!(
        format!("{:032b}", flags.bits()),
        "00000000000000000000000000000111"
    );
    flags.clear();
    assert_eq!(
        format!("{:032b}", flags.bits()),
        "00000000000000000000000000000000"
    );

    assert_eq!(
        format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B),
        "MyFlags(FLAG_A | FLAG_B)"
    );

    // Bits that do not match a defined flag are dropped by `from_bits_truncate`.
    assert_eq!(
        MyFlags::from_bits_truncate(0b1000_0011),
        MyFlags::FLAG_A | MyFlags::FLAG_B
    );
    assert_eq!(MyFlags::from_bits(0b1000_0011), None);
}
