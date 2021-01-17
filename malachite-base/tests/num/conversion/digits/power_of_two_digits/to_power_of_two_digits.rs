use itertools::Itertools;
use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOfTwoDigits};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::{
    unsigned_gen, unsigned_gen_var_3, unsigned_pair_gen_var_4,
};
use std::panic::catch_unwind;

#[test]
pub fn test_to_power_of_two_digits_asc() {
    fn test<T: PowerOfTwoDigits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        log_base: u64,
        out: &[U],
    ) {
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&x, log_base),
            out
        );
    };

    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[3, 7, 1]);
    test::<u32, u8>(1000000, 8, &[64, 66, 15]);
    test::<u32, u64>(1000000, 8, &[64, 66, 15]);
    test::<u64, u32>(1000, 1, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1]);
}

fn to_power_of_two_digits_asc_fail_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(
        &T::exact_from(100),
        U::WIDTH + 1
    ));
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(
        &T::exact_from(100),
        0
    ));
}

#[test]
fn to_power_of_two_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_two_digits_asc_fail_helper);
}

#[test]
pub fn test_to_power_of_two_digits_desc() {
    fn test<T: PowerOfTwoDigits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        log_base: u64,
        out: &[U],
    ) {
        assert_eq!(
            PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&x, log_base),
            out
        );
    };

    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[1, 7, 3]);
    test::<u32, u8>(1000000, 8, &[15, 66, 64]);
    test::<u32, u64>(1000000, 8, &[15, 66, 64]);
    test::<u64, u32>(1000, 1, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0]);
}

fn to_power_of_two_digits_desc_fail_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(
        &T::exact_from(100),
        U::WIDTH + 1
    ));
    assert_panic!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(
        &T::exact_from(100),
        0
    ));
}

#[test]
fn to_power_of_two_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_two_digits_desc_fail_helper);
}

fn to_power_of_two_digits_asc_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_pair_gen_var_4::<T, U>().test_properties(|(u, log_base)| {
        let digits = PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&u, log_base);
        assert_eq!(
            T::from_power_of_two_digits_asc(log_base, digits.iter().cloned()),
            u
        );
        if u != T::ZERO {
            assert_ne!(*digits.last().unwrap(), U::ZERO);
        }
        assert_eq!(
            digits.iter().cloned().rev().collect_vec(),
            u.to_power_of_two_digits_desc(log_base)
        );
        assert_eq!(
            digits.len(),
            usize::exact_from(
                u.significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling)
            )
        );
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_power_of_two_digits_asc(1)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect_vec(),
            u.to_bits_asc()
        );
    });

    unsigned_gen_var_3::<U>().test_properties(|log_base| {
        assert!(PowerOfTwoDigits::<U>::to_power_of_two_digits_asc(&T::ZERO, log_base).is_empty());
    });
}

#[test]
fn to_power_of_two_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_two_digits_asc_helper);
}

fn to_power_of_two_digits_desc_helper<
    T: PowerOfTwoDigits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_pair_gen_var_4::<T, U>().test_properties(|(u, log_base)| {
        let digits = PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&u, log_base);
        assert_eq!(
            T::from_power_of_two_digits_desc(log_base, digits.iter().cloned()),
            u
        );
        if u != T::ZERO {
            assert_ne!(digits[0], U::ZERO);
        }
        assert_eq!(
            digits.iter().cloned().rev().collect_vec(),
            u.to_power_of_two_digits_asc(log_base)
        );
        assert_eq!(
            digits.len(),
            usize::exact_from(
                u.significant_bits()
                    .div_round(log_base, RoundingMode::Ceiling)
            )
        );
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_power_of_two_digits_desc(1)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect_vec(),
            u.to_bits_desc()
        );
    });

    unsigned_gen_var_3::<U>().test_properties(|log_base| {
        assert!(PowerOfTwoDigits::<U>::to_power_of_two_digits_desc(&T::ZERO, log_base).is_empty());
    });
}

#[test]
fn to_power_of_two_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_two_digits_desc_helper);
}
