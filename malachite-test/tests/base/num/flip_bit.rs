use common::LARGE_LIMIT;
use malachite_base::num::{BitAccess, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_base::num::NegativeOne;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::{pairs_of_signed_and_u64_width_range,
                                   pairs_of_unsigned_and_u64_width_range};

fn flip_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n, index, out| {
        let mut n = T::from_u64(n);
        n.flip_bit(index);
        assert_eq!(n, T::from_u64(out));
    };

    test(100, 0, 101);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, 1_000_000_001_024);
        test(1_000_000_001_024, 10, 1_000_000_000_000);
    }
}

fn flip_bit_helper_signed<T: PrimitiveSigned>() {
    flip_bit_helper_unsigned::<T>();

    let test = |n, index, out| {
        let mut n = T::from_i64(n);
        n.flip_bit(index);
        assert_eq!(n, T::from_i64(out));
    };

    test(-1, 5, -33);
    test(-33, 5, -1);
    test(-32, 0, -31);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, -999_999_998_976);
        test(-999_999_998_976, 10, -1_000_000_000_000);
    }
}

#[test]
pub fn test_flip_bit() {
    flip_bit_helper_unsigned::<u8>();
    flip_bit_helper_unsigned::<u16>();
    flip_bit_helper_unsigned::<u32>();
    flip_bit_helper_unsigned::<u64>();
    flip_bit_helper_signed::<i8>();
    flip_bit_helper_signed::<i16>();
    flip_bit_helper_signed::<i32>();
    flip_bit_helper_signed::<i64>();
}

macro_rules! flip_bit_fail_helper_unsigned {
    ($t: ident, $fail: ident) => {
        #[test]
        #[should_panic(expected = "")]
        fn $fail() {
            let mut n = $t::from_u64(5);
            n.flip_bit(100);
        }
    }
}

macro_rules! flip_bit_fail_helper_signed {
    ($t: ident, $fail_1: ident, $fail_2: ident) => {
        flip_bit_fail_helper_unsigned!($t, $fail_1);

        #[test]
        #[should_panic(expected = "")]
        fn $fail_2() {
            let mut n = $t::NEGATIVE_ONE;
            n.flip_bit(100);
        }
    }
}

flip_bit_fail_helper_unsigned!(u8, flip_bit_u8_fail_helper);
flip_bit_fail_helper_unsigned!(u16, flip_bit_u16_fail_helper);
flip_bit_fail_helper_unsigned!(u32, flip_bit_u32_fail_helper);
flip_bit_fail_helper_unsigned!(u64, flip_bit_u64_fail_helper);
flip_bit_fail_helper_signed!(i8, flip_bit_i8_fail_1_helper, flip_bit_i8_fail_2_helper);
flip_bit_fail_helper_signed!(i16, flip_bit_i16_fail_1_helper, flip_bit_i16_fail_2_helper);
flip_bit_fail_helper_signed!(i32, flip_bit_i32_fail_1_helper, flip_bit_i32_fail_2_helper);
flip_bit_fail_helper_signed!(i64, flip_bit_i64_fail_1_helper, flip_bit_i64_fail_2_helper);

fn flip_bit_properties_helper_unsigned<T: 'static + PrimitiveUnsigned>() {
    // Flipping a bit once always changes a number.
    // Flipping the same bit twice leaves a number unchanged.
    let unsigned_and_u64 = |mut n: T, index: u64| {
        let old_n = n;
        n.flip_bit(index);
        assert_ne!(n, old_n);

        n.flip_bit(index);
        assert_eq!(n, old_n);
    };

    for (n, index) in
        pairs_of_unsigned_and_u64_width_range(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        unsigned_and_u64(n, index);
    }

    for (n, index) in
        pairs_of_unsigned_and_u64_width_range(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        unsigned_and_u64(n, index);
    }
}

fn flip_bit_properties_helper_signed<T: 'static + PrimitiveSigned>() {
    // Flipping a bit once always changes a number.
    // Flipping the same bit twice leaves a number unchanged.
    let signed_and_u64 = |mut n: T, index: u64| {
        let old_n = n;
        n.flip_bit(index);
        assert_ne!(n, old_n);

        n.flip_bit(index);
        assert_eq!(n, old_n);
    };

    for (n, index) in
        pairs_of_signed_and_u64_width_range(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        signed_and_u64(n, index);
    }

    for (n, index) in
        pairs_of_signed_and_u64_width_range(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        signed_and_u64(n, index);
    }
}

#[test]
fn flip_bit_properties() {
    flip_bit_properties_helper_unsigned::<u8>();
    flip_bit_properties_helper_unsigned::<u16>();
    flip_bit_properties_helper_unsigned::<u32>();
    flip_bit_properties_helper_unsigned::<u64>();
    flip_bit_properties_helper_signed::<i8>();
    flip_bit_properties_helper_signed::<i16>();
    flip_bit_properties_helper_signed::<i32>();
    flip_bit_properties_helper_signed::<i64>();
}
