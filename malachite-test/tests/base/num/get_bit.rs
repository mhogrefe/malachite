use std::u32;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned, unsigneds,
};

fn get_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out| {
        assert_eq!(T::checked_from(n).unwrap().get_bit(index), out);
    };

    test(0, 0, false);
    test(0, 100, false);
    test(123, 2, false);
    test(123, 3, true);
    test(123, 100, false);
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 12, true);
        test(1_000_000_000_000, 100, false);
    }
}

fn get_bit_helper_signed<T: PrimitiveSigned>() {
    get_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out| {
        assert_eq!(T::checked_from(n).unwrap().get_bit(index), out);
    };

    test(-123, 0, true);
    test(-123, 1, false);
    test(-123, 100, true);
    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 12, true);
        test(-1_000_000_000_000, 100, true);
        test(-i64::from(u32::MAX), 0, true);
        test(-i64::from(u32::MAX), 1, false);
        test(-i64::from(u32::MAX), 31, false);
        test(-i64::from(u32::MAX), 32, true);
        test(-i64::from(u32::MAX), 33, true);
        test(-i64::from(u32::MAX) - 1, 0, false);
        test(-i64::from(u32::MAX) - 1, 31, false);
        test(-i64::from(u32::MAX) - 1, 32, true);
        test(-i64::from(u32::MAX) - 1, 33, true);
    }
}

#[test]
pub fn test_get_bit() {
    get_bit_helper_unsigned::<u8>();
    get_bit_helper_unsigned::<u16>();
    get_bit_helper_unsigned::<u32>();
    get_bit_helper_unsigned::<u64>();
    get_bit_helper_signed::<i8>();
    get_bit_helper_signed::<i16>();
    get_bit_helper_signed::<i32>();
    get_bit_helper_signed::<i64>();
}

fn get_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, index)| {
            let bit = n.get_bit(index);
            if index >= u64::from(T::WIDTH) {
                assert!(!bit);
            } else {
                assert_eq!(bit, !(!n).get_bit(index));
            }
        },
    );

    test_properties(unsigneds, |&n: &T| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != T::ZERO {
            assert!(n.get_bit(significant_bits - 1));
        }
    });
}

fn get_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        pairs_of_signed_and_small_unsigned,
        |&(n, index): &(T, u64)| {
            let bit = n.get_bit(index);
            if index >= u64::from(T::WIDTH) {
                assert_eq!(bit, n < T::ZERO);
            } else {
                assert_eq!(bit, !(!n).get_bit(index));
            }
        },
    );
}

#[test]
fn get_bit_properties() {
    get_bit_properties_helper_unsigned::<u8>();
    get_bit_properties_helper_unsigned::<u16>();
    get_bit_properties_helper_unsigned::<u32>();
    get_bit_properties_helper_unsigned::<u64>();
    get_bit_properties_helper_signed::<i8>();
    get_bit_properties_helper_signed::<i16>();
    get_bit_properties_helper_signed::<i32>();
    get_bit_properties_helper_signed::<i64>();
}
