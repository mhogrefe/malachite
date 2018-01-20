use common::LARGE_LIMIT;
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_test::base::num::get_bit::{select_inputs_i, select_inputs_u};
use malachite_test::common::GenerationMode;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;

fn get_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n, index, out| {
        assert_eq!(T::from_u64(n).get_bit(index), out);
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

    let test = |n, index, out| {
        assert_eq!(T::from_i64(n).get_bit(index), out);
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

fn get_bit_properties_helper_unsigned<T: 'static + PrimitiveUnsigned>() {
    // if index >= T::WIDTH, !n.get_bit(index)
    // if index < T::WIDTH, n.get_bit(index) = !(!n).get_bit(index)
    let unsigned_and_u64 = |n: T, index: u64| {
        let bit = n.get_bit(index);
        if index >= T::WIDTH.into() {
            assert!(!bit);
        } else {
            assert_eq!(bit, !(!n).get_bit(index));
        }
    };

    // !n.get_bit(n.significant_bits())
    // if n != 0, n.get_bit(n.significant_bits() - 1)
    let one_unsigned = |n: T| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != T::ZERO {
            assert!(n.get_bit(significant_bits - 1));
        }
    };

    for (n, index) in select_inputs_u(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        unsigned_and_u64(n, index);
    }

    for (n, index) in select_inputs_u(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        unsigned_and_u64(n, index);
    }

    for n in exhaustive_u().take(LARGE_LIMIT) {
        one_unsigned(n);
    }

    for n in random_x(&EXAMPLE_SEED).take(LARGE_LIMIT) {
        one_unsigned(n);
    }
}

fn get_bit_properties_helper_signed<T: 'static + PrimitiveSigned>() {
    // if index >= T::WIDTH, n.get_bit(index) == (n < 0)
    // if index < T::WIDTH, n.get_bit(index) = !(!n).get_bit(index)
    let signed_and_u64 = |n: T, index: u64| {
        let bit = n.get_bit(index);
        if index >= T::WIDTH.into() {
            assert_eq!(bit, n < T::ZERO);
        } else {
            assert_eq!(bit, !(!n).get_bit(index));
        }
    };

    for (n, index) in select_inputs_i(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        signed_and_u64(n, index);
    }

    for (n, index) in select_inputs_i(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        signed_and_u64(n, index);
    }
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
