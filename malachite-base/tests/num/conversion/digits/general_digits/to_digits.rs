// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::digits::general_digits::unsigned_to_digits_asc_naive;
use malachite_base::num::conversion::traits::{Digits, ExactFrom, SaturatingFrom, WrappingFrom};
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_4, unsigned_pair_gen_var_6,
};
use std::panic::catch_unwind;

#[test]
pub fn test_to_digits_asc() {
    fn test<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(x: T, base: U, out: &[U]) {
        assert_eq!(x.to_digits_asc(&base), out);
    }
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

fn to_digits_asc_fail_helper<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(100).to_digits_asc(&U::ZERO));
    assert_panic!(T::exact_from(100).to_digits_asc(&U::ONE));
    if T::WIDTH < U::WIDTH {
        assert_panic!(T::exact_from(100).to_digits_asc(&U::power_of_2(T::WIDTH)));
    }
}

#[test]
fn to_digits_asc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_asc_fail_helper);
}

#[test]
pub fn test_to_digits_desc() {
    fn test<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>(x: T, base: U, out: &[U]) {
        assert_eq!(x.to_digits_desc(&base), out);
    }
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

fn to_digits_desc_fail_helper<T: Digits<U> + PrimitiveUnsigned, U: PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(100).to_digits_desc(&U::ZERO));
    assert_panic!(T::exact_from(100).to_digits_desc(&U::ONE));
    if T::WIDTH < U::WIDTH {
        assert_panic!(T::exact_from(100).to_digits_desc(&U::power_of_2(T::WIDTH)));
    }
}

#[test]
fn to_digits_desc_fail() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_desc_fail_helper);
}

fn to_digits_asc_helper<
    T: Digits<U> + ExactFrom<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T> + WrappingFrom<T>,
>() {
    unsigned_pair_gen_var_6::<T, U>().test_properties(|(u, base)| {
        let digits = u.to_digits_asc(&base);
        assert_eq!(unsigned_to_digits_asc_naive(&u, base), digits);
        assert_eq!(
            T::from_digits_asc(&base, digits.iter().copied()).unwrap(),
            u
        );
        if u != T::ZERO {
            assert_ne!(*digits.last().unwrap(), U::ZERO);
        }
        assert_eq!(
            digits.iter().copied().rev().collect_vec(),
            u.to_digits_desc(&base)
        );
        if u != T::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                u.floor_log_base(T::exact_from(base)) + 1
            );
        }
        assert!(digits.iter().all(|&digit| digit <= base));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_digits_asc(&U::TWO)
                .into_iter()
                .map(|digit| digit == U::ONE)
                .collect_vec(),
            u.to_bits_asc()
        );
    });

    unsigned_gen_var_4::<T, U>().test_properties(|base| {
        assert!(T::ZERO.to_digits_asc(&base).is_empty());
    });
}

#[test]
fn to_digits_asc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_asc_helper);
}

fn to_digits_desc_helper<
    T: Digits<U> + ExactFrom<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() {
    unsigned_pair_gen_var_6::<T, U>().test_properties(|(u, base)| {
        let digits = u.to_digits_desc(&base);
        assert_eq!(
            T::from_digits_desc(&base, digits.iter().copied()).unwrap(),
            u
        );
        if u != T::ZERO {
            assert_ne!(digits[0], U::ZERO);
        }
        assert_eq!(
            digits.iter().copied().rev().collect_vec(),
            u.to_digits_asc(&base)
        );
        if u != T::ZERO {
            assert_eq!(
                u64::exact_from(digits.len()),
                u.floor_log_base(T::exact_from(base)) + 1
            );
        }
        assert!(digits.iter().all(|&digit| digit <= base));
    });

    unsigned_gen::<T>().test_properties(|u| {
        assert_eq!(
            u.to_digits_desc(&U::TWO)
                .into_iter()
                .map(|digit| digit == U::ONE)
                .collect_vec(),
            u.to_bits_desc()
        );
    });

    unsigned_gen_var_4::<T, U>().test_properties(|base| {
        assert!(T::ZERO.to_digits_desc(&base).is_empty());
    });
}

#[test]
fn to_digits_desc_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(to_digits_desc_helper);
}
