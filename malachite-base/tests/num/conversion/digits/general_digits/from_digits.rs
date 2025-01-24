// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::repeat_n;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Digits, SaturatingFrom};
use malachite_base::slices::{slice_leading_zeros, slice_trailing_zeros};
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_10, unsigned_vec_unsigned_pair_gen_var_7,
    unsigned_vec_unsigned_pair_gen_var_8, unsigned_vec_unsigned_pair_gen_var_9,
};
use std::panic::catch_unwind;

#[test]
pub fn test_from_digits_asc() {
    fn test_ok<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        base: U,
        digits: &[U],
        out: T,
    ) {
        assert_eq!(T::from_digits_asc(&base, digits.iter().copied()), Some(out));
    }
    test_ok::<u8, u64>(64, &[], 0);
    test_ok::<u8, u64>(64, &[0, 0, 0], 0);
    test_ok::<u16, u64>(64, &[2], 2);
    test_ok::<u32, u16>(8, &[3, 7, 1], 123);
    test_ok::<u32, u16>(256, &[64, 66, 15], 1000000);
    test_ok::<u32, u64>(256, &[64, 66, 15], 1000000);
    test_ok::<u64, u32>(2, &[0, 0, 0, 1, 0, 1, 1, 1, 1, 1], 1000);

    test_ok::<u64, u32>(3, &[], 0);
    test_ok::<u64, u32>(3, &[2], 2);
    test_ok::<u64, u32>(3, &[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2], 123456);
    test_ok::<u64, u32>(10, &[6, 5, 4, 3, 2, 1], 123456);
    test_ok::<u64, u32>(100, &[56, 34, 12], 123456);
    test_ok::<u64, u32>(123, &[87, 19, 8], 123456);
    test_ok::<u64, u32>(123, &[87, 19, 8, 0, 0, 0], 123456);

    fn test_err<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(base: U, digits: &[U]) {
        assert_eq!(T::from_digits_asc(&base, digits.iter().copied()), None);
    }
    test_err::<u8, u64>(64, &[1; 1000]);
    test_err::<u8, u64>(2, &[2]);
    test_err::<u8, u16>(1000, &[1, 2, 3]);
}

fn from_digits_asc_fail_helper<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_asc(&U::ZERO, digits.iter().copied());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_asc(&U::ONE, digits.iter().copied());
    });
}

#[test]
pub fn test_from_digits_desc() {
    fn test_ok<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
        base: U,
        digits: &[U],
        out: T,
    ) {
        assert_eq!(
            T::from_digits_desc(&base, digits.iter().copied()),
            Some(out)
        );
    }
    test_ok::<u8, u64>(64, &[], 0);
    test_ok::<u8, u64>(64, &[0, 0, 0], 0);
    test_ok::<u16, u64>(64, &[2], 2);
    test_ok::<u32, u16>(8, &[1, 7, 3], 123);
    test_ok::<u32, u16>(256, &[15, 66, 64], 1000000);
    test_ok::<u32, u64>(256, &[15, 66, 64], 1000000);
    test_ok::<u64, u32>(2, &[1, 1, 1, 1, 1, 0, 1, 0, 0, 0], 1000);

    test_ok::<u64, u32>(3, &[], 0);
    test_ok::<u64, u32>(3, &[2], 2);
    test_ok::<u64, u32>(3, &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0], 123456);
    test_ok::<u64, u32>(10, &[1, 2, 3, 4, 5, 6], 123456);
    test_ok::<u64, u32>(100, &[12, 34, 56], 123456);
    test_ok::<u64, u32>(123, &[8, 19, 87], 123456);
    test_ok::<u64, u32>(123, &[0, 0, 0, 8, 19, 87], 123456);

    fn test_err<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(base: U, digits: &[U]) {
        assert_eq!(T::from_digits_desc(&base, digits.iter().copied()), None);
    }
    test_err::<u8, u64>(64, &[1; 1000]);
    test_err::<u8, u64>(2, &[2]);
    test_err::<u8, u16>(1000, &[1, 2, 3]);
}

fn from_digits_desc_fail_helper<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_desc(&U::ZERO, digits.iter().copied());
    });
    assert_panic!({
        let digits: &[U] = &[U::ONE];
        T::from_digits_desc(&U::ONE, digits.iter().copied());
    });
}

#[test]
fn from_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_asc_fail_helper);
}

#[test]
fn from_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_desc_fail_helper);
}

fn from_digits_asc_helper<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() {
    unsigned_vec_unsigned_pair_gen_var_9::<U>().test_properties(|(digits, base)| {
        T::from_digits_asc(&base, digits.iter().copied());
    });

    unsigned_vec_unsigned_pair_gen_var_8::<U, T>().test_properties(|(digits, base)| {
        let n = T::from_digits_asc(&base, digits.iter().copied()).unwrap();
        assert_eq!(
            T::from_digits_desc(&base, digits.iter().rev().copied()).unwrap(),
            n
        );
        let trailing_zeros = slice_trailing_zeros(&digits);
        assert_eq!(
            Digits::<U>::to_digits_asc(&n, &base),
            &digits[..digits.len() - trailing_zeros]
        );
    });

    unsigned_pair_gen_var_10::<U, T, usize>().test_properties(|(base, u)| {
        assert_eq!(
            T::from_digits_asc(&base, repeat_n(U::ZERO, u)).unwrap(),
            T::ZERO
        );
    });
}

#[test]
fn from_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_asc_helper);
}

fn from_digits_desc_helper<
    T: Digits<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() {
    unsigned_vec_unsigned_pair_gen_var_9::<U>().test_properties(|(digits, base)| {
        T::from_digits_asc(&base, digits.iter().copied());
    });

    unsigned_vec_unsigned_pair_gen_var_7::<U, T>().test_properties(|(digits, base)| {
        let n = T::from_digits_desc(&base, digits.iter().copied()).unwrap();
        assert_eq!(
            T::from_digits_asc(&base, digits.iter().rev().copied()).unwrap(),
            n
        );
        let leading_zeros = slice_leading_zeros(&digits);
        assert_eq!(
            Digits::<U>::to_digits_desc(&n, &base),
            &digits[leading_zeros..]
        );
    });

    unsigned_pair_gen_var_10::<U, T, usize>().test_properties(|(base, u)| {
        assert_eq!(
            T::from_digits_desc(&base, repeat_n(U::ZERO, u)).unwrap(),
            T::ZERO
        );
    });
}

#[test]
fn from_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(from_digits_desc_helper);
}
