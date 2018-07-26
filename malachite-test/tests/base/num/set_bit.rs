use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{BitAccess, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_test::inputs::base::{
    pairs_of_signed_and_u64_width_range_var_1, pairs_of_unsigned_and_u64_width_range,
};

fn set_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::checked_from(n).unwrap();
        n.set_bit(index);
        assert_eq!(n, T::checked_from(out).unwrap());
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

    let test = |n: i64, index, out: i64| {
        let mut n = T::checked_from(n).unwrap();
        n.set_bit(index);
        assert_eq!(n, T::checked_from(out).unwrap());
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
    ($t:ident, $fail:ident, $err:expr) => {
        #[test]
        #[should_panic(expected = $err)]
        fn $fail() {
            let mut n = $t::checked_from(5).unwrap();
            n.set_bit(100);
        }
    };
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

fn set_bit_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    test_properties(pairs_of_unsigned_and_u64_width_range, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.set_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, true);
        assert_eq!(mut_n_2, mut_n);

        assert_ne!(mut_n, T::ZERO);
        assert!(mut_n >= n);
        if n.get_bit(index) {
            assert_eq!(mut_n, n);
        } else {
            assert_ne!(mut_n, n);
            mut_n.clear_bit(index);
            assert_eq!(mut_n, n);
        }
    });
}

fn set_bit_properties_helper_signed<T: PrimitiveSigned>() {
    test_properties(pairs_of_signed_and_u64_width_range_var_1, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.set_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, true);
        assert_eq!(mut_n_2, mut_n);

        assert_ne!(mut_n, T::ZERO);
        if n >= T::ZERO && index == u64::from(T::WIDTH) - 1 {
            assert!(mut_n < T::ZERO);
        } else {
            assert!(mut_n >= n);
        }
        if n.get_bit(index) {
            assert_eq!(mut_n, n);
        } else {
            assert_ne!(mut_n, n);
            mut_n.clear_bit(index);
            assert_eq!(mut_n, n);
        }

        let mut m = !n;
        m.clear_bit(index);
        m.not_assign();
        let mut mut_n = n;
        mut_n.set_bit(index);
        assert_eq!(m, mut_n);
    });
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
