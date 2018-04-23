use common::test_properties;
use malachite_base::num::{CeilingLogTwo, FloorLogTwo, PrimitiveUnsigned, Zero};
use malachite_test::inputs::base::positive_unsigneds;
use std::{u16, u32, u64, u8};

fn floor_log_two_helper_unsigned<T: PrimitiveUnsigned>(max: u64) {
    let test = |n, out| {
        assert_eq!(T::checked_from(n).unwrap().floor_log_two(), out);
    };

    test(1, 0);
    test(2, 1);
    test(3, 1);
    test(4, 2);
    test(5, 2);
    test(100, 6);
    test(128, 7);
    test(max, u64::from(T::WIDTH) - 1);
}

#[test]
pub fn test_floor_log_two() {
    floor_log_two_helper_unsigned::<u8>(u8::MAX.into());
    floor_log_two_helper_unsigned::<u16>(u16::MAX.into());
    floor_log_two_helper_unsigned::<u32>(u32::MAX.into());
    floor_log_two_helper_unsigned::<u64>(u64::MAX);
}

macro_rules! floor_log_two_fail {
    ($t:ident, $floor_log_two_fail:ident) => {
        #[test]
        #[should_panic(expected = "Cannot take the base-2 logarithm of 0.")]
        fn $floor_log_two_fail() {
            $t::ZERO.floor_log_two();
        }
    };
}

floor_log_two_fail!(u8, floor_log_two_u8_fail);
floor_log_two_fail!(u16, floor_log_two_u16_fail);
floor_log_two_fail!(u32, floor_log_two_u32_fail);
floor_log_two_fail!(u64, floor_log_two_u64_fail);

fn ceiling_log_two_helper_unsigned<T: PrimitiveUnsigned>(max: u64) {
    let test = |n, out| {
        assert_eq!(T::checked_from(n).unwrap().ceiling_log_two(), out);
    };

    test(1, 0);
    test(2, 1);
    test(3, 2);
    test(4, 2);
    test(5, 3);
    test(100, 7);
    test(128, 7);
    test(max, T::WIDTH.into());
}

#[test]
pub fn test_ceiling_log_two() {
    ceiling_log_two_helper_unsigned::<u8>(u8::MAX.into());
    ceiling_log_two_helper_unsigned::<u16>(u16::MAX.into());
    ceiling_log_two_helper_unsigned::<u32>(u32::MAX.into());
    ceiling_log_two_helper_unsigned::<u64>(u64::MAX);
}

macro_rules! ceiling_log_two_fail {
    ($t:ident, $ceiling_log_two_fail:ident) => {
        #[test]
        #[should_panic(expected = "Cannot take the base-2 logarithm of 0.")]
        fn $ceiling_log_two_fail() {
            $t::ZERO.ceiling_log_two();
        }
    };
}

ceiling_log_two_fail!(u8, ceiling_log_two_u8_fail);
ceiling_log_two_fail!(u16, ceiling_log_two_u16_fail);
ceiling_log_two_fail!(u32, ceiling_log_two_u32_fail);
ceiling_log_two_fail!(u64, ceiling_log_two_u64_fail);

fn floor_log_two_properties_helper<T: 'static + PrimitiveUnsigned>() {
    test_properties(positive_unsigneds, |&n: &T| {
        let floor_log_two = n.floor_log_two();
        assert_eq!(floor_log_two, n.significant_bits() - 1);
        assert!(floor_log_two < u64::from(T::WIDTH));
        assert_eq!(floor_log_two == 0, n == T::ONE);
    });
}

#[test]
fn floor_log_two_properties() {
    floor_log_two_properties_helper::<u8>();
    floor_log_two_properties_helper::<u16>();
    floor_log_two_properties_helper::<u32>();
    floor_log_two_properties_helper::<u64>();
}

fn ceiling_log_two_properties_helper<T: 'static + PrimitiveUnsigned>() {
    test_properties(positive_unsigneds, |&n: &T| {
        let ceiling_log_two = n.ceiling_log_two();
        assert!(ceiling_log_two <= u64::from(T::WIDTH));
        assert_eq!(ceiling_log_two == 0, n == T::ONE);
    });
}

#[test]
fn ceiling_log_two_properties() {
    ceiling_log_two_properties_helper::<u8>();
    ceiling_log_two_properties_helper::<u16>();
    ceiling_log_two_properties_helper::<u32>();
    ceiling_log_two_properties_helper::<u64>();
}
