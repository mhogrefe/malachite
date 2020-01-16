use malachite_base::comparison::{Max, Min};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn significant_bits_helper_common<T: PrimitiveInteger>() {
    let test = |n, out| {
        assert_eq!(T::exact_from(n).significant_bits(), out);
    };
    test(0, 0);
    test(1, 1);
    test(2, 2);
    test(3, 2);
    test(4, 3);
    test(5, 3);
    test(100, 7);
    test(63, 6);
    test(64, 7);
}

fn significant_bits_helper_unsigned<T: PrimitiveUnsigned>(max: u64) {
    significant_bits_helper_common::<T>();

    let test = |n, out: u64| {
        assert_eq!(T::exact_from(n).significant_bits(), out);
    };

    test(max, T::WIDTH.into());
}

fn significant_bits_helper_signed<T: PrimitiveSigned>(max: i64, min: i64) {
    significant_bits_helper_common::<T>();

    let test = |n, out: u64| {
        assert_eq!(T::exact_from(n).significant_bits(), out);
    };

    let width = T::WIDTH.into();
    test(max, width - 1);
    test(min, width);
}

#[test]
fn test_significant_bits() {
    significant_bits_helper_unsigned::<u8>(u8::MAX.into());
    significant_bits_helper_unsigned::<u16>(u16::MAX.into());
    significant_bits_helper_unsigned::<u32>(u32::MAX.into());
    significant_bits_helper_unsigned::<u64>(u64::MAX);
    significant_bits_helper_signed::<i8>(i8::MAX.into(), i8::MIN.into());
    significant_bits_helper_signed::<i16>(i16::MAX.into(), i16::MIN.into());
    significant_bits_helper_signed::<i32>(i32::MAX.into(), i32::MIN.into());
    significant_bits_helper_signed::<i64>(i64::MAX, i64::MIN);
}
