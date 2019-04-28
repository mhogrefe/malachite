use common::test_properties;
use malachite_base::num::integers::PrimitiveInteger;
use malachite_base::num::signeds::PrimitiveSigned;
use malachite_base::num::traits::{BitAccess, NegativeOne};
use malachite_base::num::unsigneds::PrimitiveUnsigned;
use malachite_test::inputs::base::{
    pairs_of_signed_and_u64_width_range, pairs_of_unsigned_and_u64_width_range,
};
use rand::Rand;

fn flip_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::checked_from(n).unwrap();
        n.flip_bit(index);
        assert_eq!(n, T::checked_from(out).unwrap());
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

    let test = |n: i64, index, out: i64| {
        let mut n = T::checked_from(n).unwrap();
        n.flip_bit(index);
        assert_eq!(n, T::checked_from(out).unwrap());
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
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            let mut n = $t::from(5u8);
            n.flip_bit(100);
        }
    };
}

macro_rules! flip_bit_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $fail_1() {
            let mut n = $t::from(5i8);
            n.flip_bit(100);
        }

        #[test]
        #[should_panic]
        fn $fail_2() {
            let mut n = $t::NEGATIVE_ONE;
            n.flip_bit(100);
        }
    };
}

flip_bit_fail_helper_unsigned!(u8, flip_bit_u8_fail_helper);
flip_bit_fail_helper_unsigned!(u16, flip_bit_u16_fail_helper);
flip_bit_fail_helper_unsigned!(u32, flip_bit_limb_fail_helper);
flip_bit_fail_helper_unsigned!(u64, flip_bit_u64_fail_helper);
flip_bit_fail_helper_signed!(i8, flip_bit_i8_fail_1_helper, flip_bit_i8_fail_2_helper);
flip_bit_fail_helper_signed!(i16, flip_bit_i16_fail_1_helper, flip_bit_i16_fail_2_helper);
flip_bit_fail_helper_signed!(
    i32,
    flip_bit_signed_limb_fail_1_helper,
    flip_bit_signed_limb_fail_2_helper
);
flip_bit_fail_helper_signed!(i64, flip_bit_i64_fail_1_helper, flip_bit_i64_fail_2_helper);

fn flip_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigned_and_u64_width_range, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.flip_bit(index);
        assert_ne!(mut_n, n);

        mut_n.flip_bit(index);
        assert_eq!(mut_n, n);
    });
}

fn flip_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
{
    test_properties(pairs_of_signed_and_u64_width_range, |&(n, index)| {
        let mut mut_n: T = n;
        mut_n.flip_bit(index);
        assert_ne!(mut_n, n);

        mut_n.flip_bit(index);
        assert_eq!(mut_n, n);
    });
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
