use malachite_base::named::Named;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};

macro_rules! test_unsigned_constants {
    ($t: ident) => {
        assert_eq!($t::ZERO, 0);
        assert_eq!($t::ONE, 1);
        assert_eq!($t::TWO, 2);
    };
}

macro_rules! test_signed_constants {
    ($t: ident) => {
        test_unsigned_constants!($t);
        assert_eq!($t::NEGATIVE_ONE, -1);
    };
}

#[test]
fn test_constants() {
    apply_to_unsigneds!(test_unsigned_constants);
    apply_to_signeds!(test_signed_constants);
}

#[test]
fn test_width_constants() {
    assert_eq!(u8::WIDTH, 8);
    assert_eq!(u8::LOG_WIDTH, 3);
    assert_eq!(u8::WIDTH_MASK, 0x7);

    assert_eq!(u16::WIDTH, 16);
    assert_eq!(u16::LOG_WIDTH, 4);
    assert_eq!(u16::WIDTH_MASK, 0xf);

    assert_eq!(u32::WIDTH, 32);
    assert_eq!(u32::LOG_WIDTH, 5);
    assert_eq!(u32::WIDTH_MASK, 0x1f);

    assert_eq!(u64::WIDTH, 64);
    assert_eq!(u64::LOG_WIDTH, 6);
    assert_eq!(u64::WIDTH_MASK, 0x3f);

    assert_eq!(u128::WIDTH, 128);
    assert_eq!(u128::LOG_WIDTH, 7);
    assert_eq!(u128::WIDTH_MASK, 0x7f);

    assert_eq!(i8::WIDTH, 8);
    assert_eq!(i8::LOG_WIDTH, 3);
    assert_eq!(i8::WIDTH_MASK, 0x7);

    assert_eq!(i16::WIDTH, 16);
    assert_eq!(i16::LOG_WIDTH, 4);
    assert_eq!(i16::WIDTH_MASK, 0xf);

    assert_eq!(i32::WIDTH, 32);
    assert_eq!(i32::LOG_WIDTH, 5);
    assert_eq!(i32::WIDTH_MASK, 0x1f);

    assert_eq!(i64::WIDTH, 64);
    assert_eq!(i64::LOG_WIDTH, 6);
    assert_eq!(i64::WIDTH_MASK, 0x3f);

    assert_eq!(i128::WIDTH, 128);
    assert_eq!(i128::LOG_WIDTH, 7);
    assert_eq!(i128::WIDTH_MASK, 0x7f);
}

#[test]
pub fn test_named() {
    fn test<T: Named>(out: &str) {
        assert_eq!(T::NAME, out);
    }
    test::<u8>("u8");
    test::<u16>("u16");
    test::<u32>("u32");
    test::<u64>("u64");
    test::<u128>("u128");
    test::<usize>("usize");
    test::<i8>("i8");
    test::<i16>("i16");
    test::<i32>("i32");
    test::<i64>("i64");
    test::<i128>("i128");
    test::<isize>("isize");
}
