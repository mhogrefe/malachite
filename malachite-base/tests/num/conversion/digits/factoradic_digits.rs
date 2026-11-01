// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, FactoradicDigits};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_vec_gen};
use malachite_base::test_util::num::conversion::digits::factoradic_digits::*;

#[test]
fn test_to_factoradic_digits() {
    fn test<T: FactoradicDigits + PrimitiveUnsigned>(x: T, out: &[u64]) {
        assert_eq!(x.to_factoradic_digits_asc(), out);
        let mut out_desc = out.to_vec();
        out_desc.reverse();
        assert_eq!(x.to_factoradic_digits_desc(), out_desc);
    }
    test(0u8, &[]);
    test(1u8, &[1]);
    test(5u8, &[1, 2]);
    test(23u8, &[1, 2, 3]);
    test(255u8, &[1, 1, 2, 0, 2]);
    // - largest number with 5 digits, and the smallest with 6
    test(719u16, &[1, 2, 3, 4, 5]);
    test(720u16, &[0, 0, 0, 0, 0, 1]);
    test(463u32, &[1, 0, 1, 4, 3]);
    test(
        u64::MAX,
        &[1, 1, 2, 0, 0, 0, 5, 3, 8, 0, 5, 3, 5, 3, 15, 3, 4, 12, 11, 7],
    );
    test(
        u128::MAX,
        &[
            1, 1, 2, 0, 2, 0, 0, 8, 8, 4, 11, 10, 13, 6, 8, 13, 4, 18, 8, 15, 1, 16, 11, 16, 11, 5,
            23, 11, 9, 18, 6, 6, 5, 1,
        ],
    );
}

#[test]
fn test_from_factoradic_digits() {
    fn test<T: FactoradicDigits + PrimitiveUnsigned>(xs: &[u64], out: Option<T>) {
        assert_eq!(T::from_factoradic_digits_asc(xs.iter().copied()), out);
        assert_eq!(
            T::from_factoradic_digits_desc(xs.iter().rev().copied()),
            out
        );
    }
    test::<u8>(&[], Some(0));
    test::<u8>(&[0, 0, 0], Some(0));
    test::<u8>(&[1, 2, 3], Some(23));
    test::<u8>(&[1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0], Some(23));
    test::<u8>(&[1, 1, 2, 0, 2], Some(255));
    // - too large for the type
    test::<u8>(&[0, 0, 0, 0, 0, 1], None);
    test::<u8>(&[1, 2, 3, 0, 2], None);
    test::<u16>(&[0, 0, 0, 0, 0, 1], Some(720));
    // - a digit greater than its place
    test::<u8>(&[2], None);
    test::<u8>(&[1, 3], None);
    test::<u64>(&[0, 0, 0, 0, 0, 0, 8], None);
    test::<u64>(
        &[1, 1, 2, 0, 0, 0, 5, 3, 8, 0, 5, 3, 5, 3, 15, 3, 4, 12, 11, 7],
        Some(u64::MAX),
    );
    test::<u64>(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        None,
    );
}

fn factoradic_digits_properties_helper<T: FactoradicDigits + PrimitiveUnsigned>()
where
    u128: ExactFrom<T>,
{
    unsigned_gen::<T>().test_properties(|x| {
        let digits = x.to_factoradic_digits_asc();
        assert_eq!(digits, to_factoradic_digits_asc_naive(x));
        assert_eq!(digits.is_empty(), x == T::ZERO);
        assert_ne!(digits.last(), Some(&0));
        for (i, &d) in digits.iter().enumerate() {
            assert!(d <= u64::exact_from(i) + 1);
        }
        let mut digits_desc = digits.clone();
        digits_desc.reverse();
        assert_eq!(x.to_factoradic_digits_desc(), digits_desc);
        assert_eq!(
            T::from_factoradic_digits_asc(digits.iter().copied()),
            Some(x)
        );
        assert_eq!(
            T::from_factoradic_digits_desc(digits_desc.iter().copied()),
            Some(x)
        );
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let xs: Vec<u64> = xs.into_iter().map(u64::from).collect();
        let x = T::from_factoradic_digits_asc(xs.iter().copied());
        assert_eq!(x, from_factoradic_digits_asc_naive(&xs));
        assert_eq!(T::from_factoradic_digits_desc(xs.iter().rev().copied()), x);
    });
}

#[test]
fn factoradic_digits_properties() {
    apply_fn_to_unsigneds!(factoradic_digits_properties_helper);
}
