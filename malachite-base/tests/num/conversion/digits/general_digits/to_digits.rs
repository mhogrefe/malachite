use itertools::Itertools;
use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::digits::general_digits::_unsigned_to_digits_asc_naive;
use malachite_base::num::conversion::traits::{Digits, ExactFrom, SaturatingFrom, WrappingFrom};
use malachite_base_test_util::generators::{
    unsigned_gen, unsigned_gen_var_4, unsigned_pair_gen_var_6,
};
use std::panic::catch_unwind;

#[test]
pub fn test_to_digits_asc() {
    fn test<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        base: u64,
        out: &[U],
    ) {
        assert_eq!(Digits::<U, u64>::to_digits_asc(&x, base), out);
    };

    test::<u8, u64>(0, 64, &[]);
    test::<u16, u64>(2, 64, &[2]);
    test::<u32, u16>(123, 8, &[3, 7, 1]);
    test::<u32, u16>(1000000, 256, &[64, 66, 15]);
    test::<u32, u64>(1000000, 256, &[64, 66, 15]);
    test::<u64, u32>(1000, 2, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1]);

    test::<u64, u32>(0, 3, &[]);
    test::<u64, u32>(2, 3, &[2]);
    test::<u64, u32>(123456, 3, &[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2]);
    test::<u64, u32>(123456, 10, &[6, 5, 4, 3, 2, 1]);
    test::<u64, u32>(123456, 100, &[56, 34, 12]);
    test::<u64, u32>(123456, 123, &[87, 19, 8]);
}

fn to_digits_asc_fail_helper<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!(Digits::<U, u64>::to_digits_asc(&T::exact_from(100), 0));
    assert_panic!(Digits::<U, u64>::to_digits_asc(&T::exact_from(100), 1));
    if T::WIDTH < u64::WIDTH {
        assert_panic!(Digits::<U, u64>::to_digits_asc(
            &T::exact_from(100),
            u64::power_of_two(T::WIDTH)
        ));
    }
    if U::WIDTH < u64::WIDTH {
        assert_panic!(Digits::<U, u64>::to_digits_asc(
            &T::exact_from(100),
            u64::power_of_two(U::WIDTH)
        ));
    }
}

#[test]
fn to_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_asc_fail_helper);
}

#[test]
pub fn test_to_digits_desc() {
    fn test<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        base: u64,
        out: &[U],
    ) {
        assert_eq!(Digits::<U, u64>::to_digits_desc(&x, base), out);
    };

    test::<u8, u64>(0, 64, &[]);
    test::<u16, u64>(2, 64, &[2]);
    test::<u32, u16>(123, 8, &[1, 7, 3]);
    test::<u32, u16>(1000000, 256, &[15, 66, 64]);
    test::<u32, u64>(1000000, 256, &[15, 66, 64]);
    test::<u64, u32>(1000, 2, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0]);

    test::<u64, u32>(0, 3, &[]);
    test::<u64, u32>(2, 3, &[2]);
    test::<u64, u32>(123456, 3, &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]);
    test::<u64, u32>(123456, 10, &[1, 2, 3, 4, 5, 6]);
    test::<u64, u32>(123456, 100, &[12, 34, 56]);
    test::<u64, u32>(123456, 123, &[8, 19, 87]);
}

fn to_digits_desc_fail_helper<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!(Digits::<U, u64>::to_digits_desc(&T::exact_from(100), 0));
    assert_panic!(Digits::<U, u64>::to_digits_desc(&T::exact_from(100), 1));
    if T::WIDTH < u64::WIDTH {
        assert_panic!(Digits::<U, u64>::to_digits_desc(
            &T::exact_from(100),
            u64::power_of_two(T::WIDTH)
        ));
    }
    if U::WIDTH < u64::WIDTH {
        assert_panic!(Digits::<U, u64>::to_digits_desc(
            &T::exact_from(100),
            u64::power_of_two(U::WIDTH)
        ));
    }
}

#[test]
fn to_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_desc_fail_helper);
}

fn to_digits_asc_helper<
    T: Digits<U, u64> + PrimitiveUnsigned,
    U: ExactFrom<u64> + PrimitiveUnsigned + WrappingFrom<T>,
>()
where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    unsigned_pair_gen_var_6::<T, U>().test_properties(|(u, base)| {
        let digits = Digits::<U, u64>::to_digits_asc(&u, base);
        assert_eq!(_unsigned_to_digits_asc_naive::<T, U>(&u, base), digits);
        //TODO from_digits
        if u != T::ZERO {
            assert_ne!(*digits.last().unwrap(), U::ZERO);
        }
        assert_eq!(
            digits.iter().cloned().rev().collect_vec(),
            u.to_digits_desc(base)
        );
        assert!(digits.iter().all(|&digit| digit <= U::exact_from(base)));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_digits_asc(2)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect_vec(),
            u.to_bits_asc()
        );
    });

    unsigned_gen_var_4::<T, U>().test_properties(|base| {
        assert!(Digits::<U, u64>::to_digits_asc(&T::ZERO, base).is_empty());
    });
}

#[test]
fn to_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_asc_helper);
}

fn to_digits_desc_helper<
    T: Digits<U, u64> + PrimitiveUnsigned,
    U: ExactFrom<u64> + PrimitiveUnsigned + WrappingFrom<T>,
>()
where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    unsigned_pair_gen_var_6::<T, U>().test_properties(|(u, base)| {
        let digits = Digits::<U, u64>::to_digits_desc(&u, base);
        //TODO from_digits
        if u != T::ZERO {
            assert_ne!(digits[0], U::ZERO);
        }
        assert_eq!(
            digits.iter().cloned().rev().collect_vec(),
            u.to_digits_asc(base)
        );
        assert!(digits.iter().all(|&digit| digit <= U::exact_from(base)));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_digits_desc(2)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect_vec(),
            u.to_bits_desc()
        );
    });

    unsigned_gen_var_4::<T, U>().test_properties(|base| {
        assert!(Digits::<U, u64>::to_digits_desc(&T::ZERO, base).is_empty());
    });
}

#[test]
fn to_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_desc_helper);
}
