// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Floor, UnsignedAbs};
use malachite_base::num::conversion::traits::{ExactFrom, FactoradicDigits};
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::num::number_theory::traits::Factorial;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_vec_pair_gen;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen_var_7;
use malachite_q::test_util::rational::conversion::digits::factoradic_digits::*;
use std::str::FromStr;

#[test]
fn test_factoradic_digits() {
    let test = |x, before_out, after_out| {
        let x = Rational::from_str(x).unwrap();
        let (before_point, after_point) = x.factoradic_digits();
        assert_eq!(before_point.to_debug_string(), before_out);
        assert_eq!(after_point.collect::<Vec<_>>().to_debug_string(), after_out);
    };
    test("0", "[]", "[]");
    test("7/2", "[1, 1]", "[1]");
    test("1/3", "[]", "[0, 2]");
    test("3/8", "[]", "[0, 2, 1]");
    test("25/12", "[0, 1]", "[0, 0, 2]");
    test("1/7", "[]", "[0, 0, 3, 2, 0, 6]");
    // - the absolute value is expanded
    test("-22/7", "[1, 1]", "[0, 0, 3, 2, 0, 6]");
    test("100/7", "[0, 1, 2]", "[0, 1, 2, 4, 1, 5]");
}

#[test]
fn test_from_factoradic_digits() {
    let test = |before_point: &[u64], after_point: &[u64], out| {
        let x = Rational::from_factoradic_digits(before_point, after_point);
        assert_eq!(x.to_debug_string(), out);
    };
    test(&[], &[], "Some(0)");
    test(&[1, 1], &[1], "Some(7/2)");
    test(&[], &[0, 2, 1, 0, 0], "Some(3/8)");
    test(&[0, 1, 2], &[0, 1, 2, 4, 1, 5], "Some(100/7)");
    // - a digit after the point too large
    test(&[], &[2], "None");
    test(&[], &[0, 3], "None");
    // - a digit before the point too large
    test(&[2], &[], "None");
}

#[test]
fn factoradic_digits_properties() {
    rational_gen_var_7().test_properties(|x| {
        let (before_point, after_point) = x.factoradic_digits();
        let after_point: Vec<u64> = after_point.collect();
        assert_eq!(
            factoradic_digits_naive(&x),
            (before_point.clone(), after_point.clone())
        );
        assert_ne!(after_point.last(), Some(&0));
        for (j, &d) in after_point.iter().enumerate() {
            assert!(d <= u64::exact_from(j) + 1);
        }
        assert_eq!(
            Rational::from_factoradic_digits(&before_point, &after_point),
            Some((&x).abs())
        );
        // The expansion ends with the coefficient of 1/k! for the least k with d | k!
        let denominator = x.denominator_ref();
        let k = u64::exact_from(after_point.len()) + 1;
        assert!(Natural::factorial(k).divisible_by(denominator));
        if !after_point.is_empty() {
            assert!(!Natural::factorial(k - 1).divisible_by(denominator));
        }
        assert_eq!(
            before_point,
            (&x).abs().floor().unsigned_abs().to_factoradic_digits_asc()
        );
    });

    unsigned_vec_pair_gen::<u8>().test_properties(|(xs, ys)| {
        let xs: Vec<u64> = xs.into_iter().map(u64::from).collect();
        let ys: Vec<u64> = ys.into_iter().map(u64::from).collect();
        assert_eq!(
            Rational::from_factoradic_digits(&xs, &ys),
            from_factoradic_digits_naive(&xs, &ys)
        );
    });
}
