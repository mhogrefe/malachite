use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeOne;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitAccess;
use rand::Rand;

use common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_u64_width_range_var_2, pairs_of_unsigned_and_small_unsigned,
};

fn clear_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::checked_from(n).unwrap();
        n.clear_bit(index);
        assert_eq!(n, T::checked_from(out).unwrap());
    };

    test(0, 10, 0);
    test(0, 100, 0);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_001_024, 10, 1_000_000_000_000);
        test(1_000_000_001_024, 100, 1_000_000_001_024);
    }
}

fn clear_bit_helper_signed<T: PrimitiveSigned>() {
    clear_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::checked_from(n).unwrap();
        n.clear_bit(index);
        assert_eq!(n, T::checked_from(out).unwrap());
    };

    test(-1, 5, -33);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-999_999_998_976, 10, -1_000_000_000_000);
    }
}

#[test]
pub fn test_clear_bit() {
    clear_bit_helper_unsigned::<u8>();
    clear_bit_helper_unsigned::<u16>();
    clear_bit_helper_unsigned::<u32>();
    clear_bit_helper_unsigned::<u64>();
    clear_bit_helper_signed::<i8>();
    clear_bit_helper_signed::<i16>();
    clear_bit_helper_signed::<i32>();
    clear_bit_helper_signed::<i64>();
}

macro_rules! clear_bit_fail_helper {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            let mut n = $t::NEGATIVE_ONE;
            n.clear_bit(100);
        }
    };
}

clear_bit_fail_helper!(i8, clear_bit_i8_fail_helper);
clear_bit_fail_helper!(i16, clear_bit_i16_fail_helper);
clear_bit_fail_helper!(i32, clear_bit_i32_fail_helper);
clear_bit_fail_helper!(i64, clear_bit_i64_fail_helper);

fn clear_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigned_and_small_unsigned, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.clear_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, false);
        assert_eq!(mut_n_2, mut_n);

        assert!(mut_n <= n);
        if n.get_bit(index) {
            assert_ne!(mut_n, n);
            mut_n.set_bit(index);
            assert_eq!(mut_n, n);
        } else {
            assert_eq!(mut_n, n);
        }
    });
}

fn clear_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_u64_width_range_var_2, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.clear_bit(index);

        let mut mut_n_2 = n;
        mut_n_2.assign_bit(index, false);
        assert_eq!(mut_n_2, mut_n);

        if n < T::ZERO && index == u64::from(T::WIDTH) - 1 {
            assert!(mut_n >= T::ZERO);
        } else {
            assert!(mut_n <= n);
        }
        if n.get_bit(index) {
            assert_ne!(mut_n, n);
            mut_n.set_bit(index);
            assert_eq!(mut_n, n);
        } else {
            assert_eq!(mut_n, n);
        }

        let mut m = !n;
        m.set_bit(index);
        m.not_assign();
        let mut mut_n = n;
        mut_n.clear_bit(index);
        assert_eq!(m, mut_n);
    });
}

#[test]
fn clear_bit_properties() {
    clear_bit_properties_helper_unsigned::<u8>();
    clear_bit_properties_helper_unsigned::<u16>();
    clear_bit_properties_helper_unsigned::<u32>();
    clear_bit_properties_helper_unsigned::<u64>();
    clear_bit_properties_helper_signed::<i8>();
    clear_bit_properties_helper_signed::<i16>();
    clear_bit_properties_helper_signed::<i32>();
    clear_bit_properties_helper_signed::<i64>();
}
