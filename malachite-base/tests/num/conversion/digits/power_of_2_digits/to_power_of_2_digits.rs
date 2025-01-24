// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::DivRound;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_3, unsigned_pair_gen_var_4,
};
use std::panic::catch_unwind;

#[test]
pub fn test_to_power_of_2_digits_asc() {
    fn test<T: PowerOf2Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        log_base: u64,
        out: &[U],
    ) {
        assert_eq!(
            PowerOf2Digits::<U>::to_power_of_2_digits_asc(&x, log_base),
            out
        );
    }
    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[3, 7, 1]);
    test::<u32, u8>(1000000, 8, &[64, 66, 15]);
    test::<u32, u64>(1000000, 8, &[64, 66, 15]);
    test::<u64, u32>(1000, 1, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1]);
}

fn to_power_of_2_digits_asc_fail_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOf2Digits::<U>::to_power_of_2_digits_asc(
        &T::exact_from(100),
        U::WIDTH + 1
    ));
    assert_panic!(PowerOf2Digits::<U>::to_power_of_2_digits_asc(
        &T::exact_from(100),
        0
    ));
}

#[test]
fn to_power_of_2_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_2_digits_asc_fail_helper);
}

#[test]
pub fn test_to_power_of_2_digits_desc() {
    fn test<T: PowerOf2Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        x: T,
        log_base: u64,
        out: &[U],
    ) {
        assert_eq!(
            PowerOf2Digits::<U>::to_power_of_2_digits_desc(&x, log_base),
            out
        );
    }
    test::<u8, u64>(0, 6, &[]);
    test::<u16, u64>(2, 6, &[2]);
    test::<u32, u16>(123, 3, &[1, 7, 3]);
    test::<u32, u8>(1000000, 8, &[15, 66, 64]);
    test::<u32, u64>(1000000, 8, &[15, 66, 64]);
    test::<u64, u32>(1000, 1, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0]);
}

fn to_power_of_2_digits_desc_fail_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!(PowerOf2Digits::<U>::to_power_of_2_digits_desc(
        &T::exact_from(100),
        U::WIDTH + 1
    ));
    assert_panic!(PowerOf2Digits::<U>::to_power_of_2_digits_desc(
        &T::exact_from(100),
        0
    ));
}

#[test]
fn to_power_of_2_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_2_digits_desc_fail_helper);
}

fn to_power_of_2_digits_asc_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_pair_gen_var_4::<T, U>().test_properties(|(u, log_base)| {
        let digits = PowerOf2Digits::<U>::to_power_of_2_digits_asc(&u, log_base);
        assert_eq!(
            T::from_power_of_2_digits_asc(log_base, digits.iter().copied()).unwrap(),
            u
        );
        if u != T::ZERO {
            assert_ne!(*digits.last().unwrap(), U::ZERO);
        }
        assert_eq!(
            digits.iter().copied().rev().collect_vec(),
            u.to_power_of_2_digits_desc(log_base)
        );
        if u != T::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                u.floor_log_base_power_of_2(log_base) + 1
            );
        }
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_power_of_2_digits_asc(1)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect_vec(),
            u.to_bits_asc()
        );
    });

    unsigned_gen_var_3::<U>().test_properties(|log_base| {
        assert!(PowerOf2Digits::<U>::to_power_of_2_digits_asc(&T::ZERO, log_base).is_empty());
    });
}

#[test]
fn to_power_of_2_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_2_digits_asc_helper);
}

fn to_power_of_2_digits_desc_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_pair_gen_var_4::<T, U>().test_properties(|(u, log_base)| {
        let digits = PowerOf2Digits::<U>::to_power_of_2_digits_desc(&u, log_base);
        assert_eq!(
            T::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap(),
            u
        );
        if u != T::ZERO {
            assert_ne!(digits[0], U::ZERO);
        }
        if u != T::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                u.floor_log_base_power_of_2(log_base) + 1
            );
        }
        assert_eq!(
            digits.len(),
            usize::exact_from(u.significant_bits().div_round(log_base, Ceiling).0)
        );
        assert!(digits
            .iter()
            .all(|digit| digit.significant_bits() <= log_base));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_power_of_2_digits_desc(1)
                .into_iter()
                .map(|digit: U| digit == U::ONE)
                .collect_vec(),
            u.to_bits_desc()
        );
    });

    unsigned_gen_var_3::<U>().test_properties(|log_base| {
        assert!(PowerOf2Digits::<U>::to_power_of_2_digits_desc(&T::ZERO, log_base).is_empty());
    });
}

#[test]
fn to_power_of_2_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_power_of_2_digits_desc_helper);
}
