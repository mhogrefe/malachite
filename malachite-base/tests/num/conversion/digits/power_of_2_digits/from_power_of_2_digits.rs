// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::repeat_n;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::PowerOf2Digits;
use malachite_base::slices::{slice_leading_zeros, slice_trailing_zeros};
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_5, unsigned_vec_unsigned_pair_gen_var_2,
    unsigned_vec_unsigned_pair_gen_var_3, unsigned_vec_unsigned_pair_gen_var_6,
};
use std::panic::catch_unwind;

#[test]
pub fn test_from_power_of_2_digits_asc() {
    fn test_ok<T: PowerOf2Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        log_base: u64,
        digits: &[U],
        out: T,
    ) {
        assert_eq!(
            T::from_power_of_2_digits_asc(log_base, digits.iter().copied()).unwrap(),
            out
        );
    }
    test_ok::<u8, u64>(6, &[], 0);
    test_ok::<u16, u64>(6, &[2], 2);
    test_ok::<u32, u16>(3, &[3, 7, 1], 123);
    test_ok::<u32, u8>(8, &[64, 66, 15], 1000000);
    test_ok::<u32, u64>(8, &[64, 66, 15], 1000000);
    test_ok::<u64, u32>(1, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1], 1000);

    fn test_err<T: PowerOf2Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        log_base: u64,
        digits: &[U],
    ) {
        assert_eq!(
            T::from_power_of_2_digits_asc(log_base, digits.iter().copied()),
            None
        );
    }
    test_err::<u8, u64>(4, &[1; 100]);
    test_err::<u8, u64>(1, &[2]);
}

fn from_power_of_2_digits_asc_fail_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_2_digits_asc(U::WIDTH + 1, digits.iter().copied());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_2_digits_asc(0, digits.iter().copied());
    });
}

#[test]
fn from_power_of_2_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_2_digits_asc_fail_helper);
}

#[test]
pub fn test_from_power_of_2_digits_desc() {
    fn test_ok<T: PowerOf2Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        log_base: u64,
        digits: &[U],
        out: T,
    ) {
        assert_eq!(
            T::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap(),
            out
        );
    }
    test_ok::<u8, u64>(6, &[], 0);
    test_ok::<u16, u64>(6, &[2], 2);
    test_ok::<u32, u16>(3, &[1, 7, 3], 123);
    test_ok::<u32, u8>(8, &[15, 66, 64], 1000000);
    test_ok::<u32, u64>(8, &[15, 66, 64], 1000000);
    test_ok::<u64, u32>(1, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0], 1000);

    fn test_err<T: PowerOf2Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        log_base: u64,
        digits: &[U],
    ) {
        assert_eq!(
            T::from_power_of_2_digits_desc(log_base, digits.iter().copied()),
            None
        );
    }
    test_err::<u8, u64>(4, &[1; 100]);
    test_err::<u8, u64>(1, &[2]);
}

fn from_power_of_2_digits_desc_fail_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_2_digits_desc(U::WIDTH + 1, digits.iter().copied());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_power_of_2_digits_desc(0, digits.iter().copied());
    });
}

#[test]
fn from_power_of_2_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_2_digits_desc_fail_helper);
}

fn from_power_of_2_digits_asc_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_vec_unsigned_pair_gen_var_6::<U>().test_properties(|(digits, log_base)| {
        T::from_power_of_2_digits_asc(log_base, digits.iter().copied());
    });

    unsigned_vec_unsigned_pair_gen_var_2::<T, U>().test_properties(|(digits, log_base)| {
        let n = T::from_power_of_2_digits_asc(log_base, digits.iter().copied()).unwrap();
        assert_eq!(
            T::from_power_of_2_digits_desc(log_base, digits.iter().rev().copied()).unwrap(),
            n
        );
        let trailing_zeros = slice_trailing_zeros(&digits);
        assert_eq!(
            PowerOf2Digits::<U>::to_power_of_2_digits_asc(&n, log_base),
            &digits[..digits.len() - trailing_zeros]
        );
    });

    unsigned_pair_gen_var_5::<usize, U>().test_properties(|(u, log_base)| {
        assert_eq!(
            T::from_power_of_2_digits_asc(log_base, repeat_n(U::ZERO, u)).unwrap(),
            T::ZERO
        );
    });
}

#[test]
fn from_power_of_2_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_2_digits_asc_helper);
}

fn from_power_of_2_digits_desc_helper<
    T: PowerOf2Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() {
    unsigned_vec_unsigned_pair_gen_var_6::<U>().test_properties(|(digits, log_base)| {
        T::from_power_of_2_digits_desc(log_base, digits.iter().copied());
    });

    unsigned_vec_unsigned_pair_gen_var_3::<T, U>().test_properties(|(digits, log_base)| {
        let n = T::from_power_of_2_digits_desc(log_base, digits.iter().copied()).unwrap();
        assert_eq!(
            T::from_power_of_2_digits_asc(log_base, digits.iter().rev().copied()).unwrap(),
            n
        );
        let leading_zeros = slice_leading_zeros(&digits);
        assert_eq!(
            PowerOf2Digits::<U>::to_power_of_2_digits_desc(&n, log_base),
            &digits[leading_zeros..]
        );
    });

    unsigned_pair_gen_var_5::<usize, U>().test_properties(|(u, log_base)| {
        assert_eq!(
            T::from_power_of_2_digits_desc(log_base, repeat_n(U::ZERO, u)).unwrap(),
            T::ZERO
        );
    });
}

#[test]
fn from_power_of_2_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_power_of_2_digits_desc_helper);
}
