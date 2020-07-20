use malachite_base_test_util::generators::unsigned_gen_var_1;

use malachite_base::num::arithmetic::traits::{CeilingLogTwo, FloorLogTwo};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;

fn floor_log_two_helper_unsigned<T: PrimitiveUnsigned>(max: u128) {
    let test = |n, out| {
        assert_eq!(T::exact_from(n).floor_log_two(), out);
    };

    test(1, 0);
    test(2, 1);
    test(3, 1);
    test(4, 2);
    test(5, 2);
    test(100, 6);
    test(128, 7);
    test(max, T::WIDTH - 1);
}

#[test]
fn test_floor_log_two() {
    floor_log_two_helper_unsigned::<u8>(u8::MAX.into());
    floor_log_two_helper_unsigned::<u16>(u16::MAX.into());
    floor_log_two_helper_unsigned::<u32>(u32::MAX.into());
    floor_log_two_helper_unsigned::<u64>(u64::MAX.into());
    floor_log_two_helper_unsigned::<u128>(u128::MAX);
    floor_log_two_helper_unsigned::<usize>(u128::wrapping_from(usize::MAX));
}

macro_rules! floor_log_two_fail {
    ($t:ident, $floor_log_two_fail:ident) => {
        #[test]
        #[should_panic]
        fn $floor_log_two_fail() {
            $t::ZERO.floor_log_two();
        }
    };
}
floor_log_two_fail!(u8, floor_log_two_u8_fail);
floor_log_two_fail!(u16, floor_log_two_u16_fail);
floor_log_two_fail!(u32, floor_log_two_u32_fail);
floor_log_two_fail!(u64, floor_log_two_u64_fail);
floor_log_two_fail!(u128, floor_log_two_u128_fail);
floor_log_two_fail!(usize, floor_log_two_usize_fail);

fn ceiling_log_two_helper_unsigned<T: PrimitiveUnsigned>(max: u128) {
    let test = |n, out| {
        assert_eq!(T::exact_from(n).ceiling_log_two(), out);
    };

    test(1, 0);
    test(2, 1);
    test(3, 2);
    test(4, 2);
    test(5, 3);
    test(100, 7);
    test(128, 7);
    test(max, T::WIDTH);
}

#[test]
fn test_ceiling_log_two() {
    ceiling_log_two_helper_unsigned::<u8>(u8::MAX.into());
    ceiling_log_two_helper_unsigned::<u16>(u16::MAX.into());
    ceiling_log_two_helper_unsigned::<u32>(u32::MAX.into());
    ceiling_log_two_helper_unsigned::<u64>(u64::MAX.into());
    ceiling_log_two_helper_unsigned::<u128>(u128::MAX);
    ceiling_log_two_helper_unsigned::<usize>(u128::wrapping_from(usize::MAX));
}

macro_rules! ceiling_log_two_fail {
    ($t:ident, $ceiling_log_two_fail:ident) => {
        #[test]
        #[should_panic]
        fn $ceiling_log_two_fail() {
            $t::ZERO.ceiling_log_two();
        }
    };
}
ceiling_log_two_fail!(u8, ceiling_log_two_u8_fail);
ceiling_log_two_fail!(u16, ceiling_log_two_u16_fail);
ceiling_log_two_fail!(u32, ceiling_log_two_u32_fail);
ceiling_log_two_fail!(u64, ceiling_log_two_u64_fail);
ceiling_log_two_fail!(u128, ceiling_log_two_u128_fail);
ceiling_log_two_fail!(usize, ceiling_log_two_usize_fail);

fn floor_log_two_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|n| {
        let floor_log_two = n.floor_log_two();
        assert_eq!(floor_log_two, n.significant_bits() - 1);
        assert!(floor_log_two < T::WIDTH);
        assert_eq!(floor_log_two == 0, n == T::ONE);
    });
}

#[test]
fn floor_log_two_properties() {
    apply_fn_to_unsigneds!(floor_log_two_properties_helper);
}

fn ceiling_log_two_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_1::<T>().test_properties(|n| {
        let ceiling_log_two = n.ceiling_log_two();
        assert!(ceiling_log_two <= T::WIDTH);
        assert_eq!(ceiling_log_two == 0, n == T::ONE);
    });
}

#[test]
fn ceiling_log_two_properties() {
    apply_fn_to_unsigneds!(ceiling_log_two_properties_helper);
}
