use common::LARGE_LIMIT;
use malachite_base::num::{BitAccess, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::{pairs_of_signed_and_u64_width_range_var_1,
                                   pairs_of_unsigned_and_u64_width_range};

fn set_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n, index, out| {
        let mut n = T::from_u64(n);
        n.set_bit(index);
        assert_eq!(n, T::from_u64(out));
    };

    test(100, 0, 101);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, 1_000_000_001_024);
    }
}

fn set_bit_helper_signed<T: PrimitiveSigned>() {
    set_bit_helper_unsigned::<T>();

    let test = |n, index, out| {
        let mut n = T::from_i64(n);
        n.set_bit(index);
        assert_eq!(n, T::from_i64(out));
    };

    test(-1, 5, -1);
    test(-1, 100, -1);
    test(-33, 5, -1);
    test(-32, 0, -31);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, -999_999_998_976);
        test(-1_000_000_000_000, 100, -1_000_000_000_000);
    }
}

#[test]
pub fn test_set_bit() {
    set_bit_helper_unsigned::<u8>();
    set_bit_helper_unsigned::<u16>();
    set_bit_helper_unsigned::<u32>();
    set_bit_helper_unsigned::<u64>();
    set_bit_helper_signed::<i8>();
    set_bit_helper_signed::<i16>();
    set_bit_helper_signed::<i32>();
    set_bit_helper_signed::<i64>();
}

macro_rules! set_bit_fail_helper {
    ($t: ident, $fail: ident, $err: expr) => {
        #[test]
        #[should_panic(expected = $err)]
        fn $fail() {
            let mut n = $t::from_u64(5);
            n.set_bit(100);
        }
    }
}

set_bit_fail_helper!(
    u8,
    set_bit_u8_fail_helper,
    "Cannot set bit 100 in non-negative value of width 8"
);
set_bit_fail_helper!(
    u16,
    set_bit_u16_fail_helper,
    "Cannot set bit 100 in non-negative value of width 16"
);
set_bit_fail_helper!(
    u32,
    set_bit_u32_fail_helper,
    "Cannot set bit 100 in non-negative value of width 32"
);
set_bit_fail_helper!(
    u64,
    set_bit_u64_fail_helper,
    "Cannot set bit 100 in non-negative value of width 64"
);
set_bit_fail_helper!(
    i8,
    set_bit_i8_fail_helper,
    "Cannot set bit 100 in non-negative value of width 8"
);
set_bit_fail_helper!(
    i16,
    set_bit_i16_fail_helper,
    "Cannot set bit 100 in non-negative value of width 16"
);
set_bit_fail_helper!(
    i32,
    set_bit_i32_fail_helper,
    "Cannot set bit 100 in non-negative value of width 32"
);
set_bit_fail_helper!(
    i64,
    set_bit_i64_fail_helper,
    "Cannot set bit 100 in non-negative value of width 64"
);

fn set_bit_properties_helper_unsigned<T: 'static + PrimitiveUnsigned>() {
    // n.set_bit(index) is equivalent to n.assign_bit(index, true).
    // n.set_bit(index); n != 0
    // Setting a bit does not decrease n.
    // If n.get_bit(index), setting at index won't do anything.
    // If !n.get_bit(index), setting and then clearing at index won't do anything.
    let unsigned_and_u64 = |mut n: T, index: u64| {
        let old_n = n;
        n.set_bit(index);

        let mut n2 = old_n;
        n2.assign_bit(index, true);
        assert_eq!(n2, n);

        assert_ne!(n, T::ZERO);
        assert!(n >= old_n);
        if old_n.get_bit(index) {
            assert_eq!(n, old_n);
        } else {
            assert_ne!(n, old_n);
            n.clear_bit(index);
            assert_eq!(n, old_n);
        }
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

fn set_bit_properties_helper_signed<T: 'static + PrimitiveSigned>() {
    // n.set_bit(index) is equivalent to n.assign_bit(index, true).
    // n.set_bit(index); n != 0
    // Setting a bit does not decrease n, unless n >= 0 and index = T::WIDTH - 1, in which case the
    //      sign bit flips and n becomes negative.
    // If n.get_bit(index), setting at index won't do anything.
    // If !n.get_bit(index), setting and then clearing at index won't do anything.
    // { n.set_bit(index) } is equivalent to { n := !n; n.clear_bit(index); n := !n }
    let signed_and_u64 = |mut n: T, index: u64| {
        let old_n = n;
        n.set_bit(index);

        let mut n2 = old_n;
        n2.assign_bit(index, true);
        assert_eq!(n2, n);

        assert_ne!(n, T::ZERO);
        if old_n >= T::ZERO && index == u64::from(T::WIDTH) - 1 {
            assert!(n < T::ZERO);
        } else {
            assert!(n >= old_n);
        }
        if old_n.get_bit(index) {
            assert_eq!(n, old_n);
        } else {
            assert_ne!(n, old_n);
            n.clear_bit(index);
            assert_eq!(n, old_n);
        }

        let mut m = !old_n;
        m.clear_bit(index);
        m = !m; //TODO use not_assign
        let mut n = old_n;
        n.set_bit(index);
        assert_eq!(m, n);
    };

    for (n, index) in
        pairs_of_signed_and_u64_width_range_var_1(GenerationMode::Exhaustive).take(LARGE_LIMIT)
    {
        signed_and_u64(n, index);
    }

    for (n, index) in
        pairs_of_signed_and_u64_width_range_var_1(GenerationMode::Random(32)).take(LARGE_LIMIT)
    {
        signed_and_u64(n, index);
    }
}

#[test]
fn set_bit_properties() {
    set_bit_properties_helper_unsigned::<u8>();
    set_bit_properties_helper_unsigned::<u16>();
    set_bit_properties_helper_unsigned::<u32>();
    set_bit_properties_helper_unsigned::<u64>();
    set_bit_properties_helper_signed::<i8>();
    set_bit_properties_helper_signed::<i16>();
    set_bit_properties_helper_signed::<i32>();
    set_bit_properties_helper_signed::<i64>();
}
