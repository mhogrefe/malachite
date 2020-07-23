use malachite_base_test_util::generators::{signed_gen, unsigned_gen};

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;

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

fn significant_bits_helper_unsigned<T: PrimitiveUnsigned>(max: u128) {
    significant_bits_helper_common::<T>();

    let test = |n, out: u64| {
        assert_eq!(T::exact_from(n).significant_bits(), out);
    };

    test(max, T::WIDTH);
}

fn significant_bits_helper_signed<T: PrimitiveSigned>(max: i128, min: i128) {
    significant_bits_helper_common::<T>();

    let test = |n, out: u64| {
        assert_eq!(T::exact_from(n).significant_bits(), out);
    };

    test(max, T::WIDTH - 1);
    test(min, T::WIDTH);
}

#[test]
fn test_significant_bits() {
    significant_bits_helper_unsigned::<u8>(u8::MAX.into());
    significant_bits_helper_unsigned::<u16>(u16::MAX.into());
    significant_bits_helper_unsigned::<u32>(u32::MAX.into());
    significant_bits_helper_unsigned::<u64>(u64::MAX.into());
    significant_bits_helper_unsigned::<u128>(u128::MAX);
    significant_bits_helper_unsigned::<usize>(u128::wrapping_from(usize::MAX));
    significant_bits_helper_signed::<i8>(i8::MAX.into(), i8::MIN.into());
    significant_bits_helper_signed::<i16>(i16::MAX.into(), i16::MIN.into());
    significant_bits_helper_signed::<i32>(i32::MAX.into(), i32::MIN.into());
    significant_bits_helper_signed::<i64>(i64::MAX.into(), i64::MIN.into());
    significant_bits_helper_signed::<i128>(i128::MAX, i128::MIN);
    significant_bits_helper_signed::<isize>(
        i128::wrapping_from(isize::MAX),
        i128::wrapping_from(isize::MIN),
    );
}

fn significant_bits_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        let significant_bits = u.significant_bits();
        assert!(significant_bits <= T::WIDTH);
        assert_eq!(significant_bits == 0, u == T::ZERO);
        if u != T::ZERO {
            assert_eq!(significant_bits, u.floor_log_two() + 1)
        }
    });
}

fn significant_bits_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        let significant_bits = i.significant_bits();
        assert!(significant_bits <= T::WIDTH);
        assert_eq!(significant_bits == 0, i == T::ZERO);
        assert_eq!(significant_bits == T::WIDTH, i == T::MIN);
        assert_eq!(i.wrapping_neg().significant_bits(), significant_bits);
    });
}

#[test]
fn significant_bits_properties() {
    apply_fn_to_unsigneds!(significant_bits_properties_helper_unsigned);
    apply_fn_to_signeds!(significant_bits_properties_helper_signed);
}
