// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ExactFrom, FactoradicDigits};
use malachite_base::num::number_theory::traits::Factorial;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_vec_gen};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::natural::conversion::digits::factoradic_digits::*;
use std::str::FromStr;

#[test]
fn test_to_factoradic_digits() {
    let test = |x, out: &[u64]| {
        let x = Natural::from_str(x).unwrap();
        assert_eq!(x.to_factoradic_digits_asc(), out);
        let mut out_desc = out.to_vec();
        out_desc.reverse();
        assert_eq!(x.to_factoradic_digits_desc(), out_desc);
    };
    test("0", &[]);
    test("1", &[1]);
    test("23", &[1, 2, 3]);
    test("463", &[1, 0, 1, 4, 3]);
    test(
        "100000000000000000000",
        &[0, 2, 2, 1, 5, 1, 1, 8, 8, 1, 8, 10, 10, 4, 5, 12, 3, 1, 2, 20, 1],
    );
    // - 21!, the smallest number with 21 digits
    test(
        "51090942171709440000",
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    );
}

#[test]
fn test_from_factoradic_digits() {
    let test = |xs: &[u64], out: Option<&str>| {
        let x = Natural::from_factoradic_digits_asc(xs.iter().copied());
        assert_eq!(x.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(
            Natural::from_factoradic_digits_desc(xs.iter().rev().copied()),
            x
        );
    };
    test(&[], Some("0"));
    test(&[0, 0, 0], Some("0"));
    test(&[1, 2, 3, 0, 0], Some("23"));
    test(
        &[0, 2, 2, 1, 5, 1, 1, 8, 8, 1, 8, 10, 10, 4, 5, 12, 3, 1, 2, 20, 1],
        Some("100000000000000000000"),
    );
    // - a digit greater than its place
    test(&[2], None);
    test(&[1, 3], None);
    test(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22],
        None,
    );
}

#[test]
fn factoradic_digits_properties() {
    natural_gen().test_properties(|x| {
        let digits = x.to_factoradic_digits_asc();
        assert_eq!(digits, to_factoradic_digits_asc_naive(&x));
        assert_eq!(digits.is_empty(), x == 0u32);
        assert_ne!(digits.last(), Some(&0));
        for (i, &d) in digits.iter().enumerate() {
            assert!(d <= u64::exact_from(i) + 1);
        }
        let mut digits_desc = digits.clone();
        digits_desc.reverse();
        assert_eq!(x.to_factoradic_digits_desc(), digits_desc);
        assert_eq!(
            Natural::from_factoradic_digits_asc(digits.iter().copied()).as_ref(),
            Some(&x)
        );
        assert_eq!(
            Natural::from_factoradic_digits_desc(digits_desc.iter().copied()).as_ref(),
            Some(&x)
        );
        // the number of digits is the largest k with k! <= x
        if let Some(&last) = digits.last() {
            let k = u64::exact_from(digits.len());
            assert!(Natural::factorial(k) * Natural::from(last) <= x);
            assert!(Natural::factorial(k + 1) > x);
        }
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let xs: Vec<u64> = xs.into_iter().map(u64::from).collect();
        let x = Natural::from_factoradic_digits_asc(xs.iter().copied());
        assert_eq!(x, from_factoradic_digits_asc_naive(&xs));
        assert_eq!(
            Natural::from_factoradic_digits_desc(xs.iter().rev().copied()),
            x
        );
    });

    unsigned_gen::<u64>().test_properties(|x| {
        assert_eq!(
            Natural::from(x).to_factoradic_digits_asc(),
            x.to_factoradic_digits_asc()
        );
    });
}
