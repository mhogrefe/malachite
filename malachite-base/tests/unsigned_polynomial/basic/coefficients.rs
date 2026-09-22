// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{
    unsigned_polynomial_gen, unsigned_polynomial_unsigned_pair_gen_var_1,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_coefficients_asc() {
    let test = |s, out| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        assert_eq!(p.coefficients_asc().to_debug_string(), out);
        assert_eq!(p.into_coefficients_asc().to_debug_string(), out);
    };
    test("0", "[]");
    test("1", "[1]");
    test("5", "[5]");
    test("x", "[0, 1]");
    test("x^2+3*x+2", "[2, 3, 1]");
    test("x^3", "[0, 0, 0, 1]");
}

#[test]
fn test_degree() {
    let test = |s, out| {
        assert_eq!(
            UnsignedPolynomial::<u64>::from_str(s).unwrap().degree(),
            out
        );
    };
    test("0", None);
    test("1", Some(0));
    test("5", Some(0));
    test("x", Some(1));
    test("x^2+3*x+2", Some(2));
    test("x^100", Some(100));
}

#[test]
fn test_coefficient() {
    let test = |s, i, out| {
        assert_eq!(
            UnsignedPolynomial::<u64>::from_str(s)
                .unwrap()
                .coefficient(i),
            out
        );
    };
    test("0", 0, 0);
    test("0", 100, 0);
    test("x^2+3*x+2", 0, 2);
    test("x^2+3*x+2", 1, 3);
    test("x^2+3*x+2", 2, 1);
    // A coefficient past the degree is zero, however far past.
    test("x^2+3*x+2", 3, 0);
    test("x^2+3*x+2", 1000000, 0);
    test("x^2+3*x+2", u64::MAX, 0);
}

#[test]
fn test_leading_coefficient() {
    let test = |s, out| {
        assert_eq!(
            UnsignedPolynomial::<u64>::from_str(s)
                .unwrap()
                .leading_coefficient(),
            out
        );
    };
    test("0", 0);
    test("5", 5);
    test("x", 1);
    test("7*x^2+3*x+2", 7);
}

#[test]
fn coefficients_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let cs = p.coefficients_asc().to_vec();
        // The coefficients are what the polynomial is: they build it back.
        let q = UnsignedPolynomial::<u64>::from_coefficients_asc(cs.clone());
        assert!(q.is_valid());
        assert_eq!(q, p);
        assert_eq!(p.clone().into_coefficients_asc(), cs);

        // There are as many coefficients as the degree implies, and no trailing zero among them.
        assert_eq!(
            p.degree(),
            u64::try_from(cs.len()).ok().and_then(|l| l.checked_sub(1))
        );
        assert_ne!(cs.last(), Some(&0));

        // The leading coefficient is the last one, and zero when there is none.
        assert_eq!(p.leading_coefficient(), cs.last().copied().unwrap_or(0));
        if let Some(d) = p.degree() {
            assert_eq!(p.leading_coefficient(), p.coefficient(d));
            assert_ne!(p.leading_coefficient(), 0);
            // Nothing lives past the degree.
            assert_eq!(p.coefficient(d + 1), 0);
            assert_eq!(p.coefficient(u64::MAX), 0);
        }

        // Indexing agrees with the slice, for every index the slice has.
        for (i, c) in cs.iter().enumerate() {
            assert_eq!(p.coefficient(u64::exact_from(i)), *c);
        }
    });

    unsigned_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, i)| {
        let c = p.coefficient(i);
        // An index is in range exactly when it is at most the degree.
        assert_eq!(
            p.degree().is_some_and(|d| i <= d),
            p.coefficients_asc().len() > usize::exact_from(i)
        );
        if p.degree().is_none_or(|d| i > d) {
            assert_eq!(c, 0);
        }
    });
}
