// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::mem::replace;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_integer_polynomial_natural_triple_gen_var_1,
    rational_polynomial_integer_polynomial_pair_gen, rational_polynomial_natural_pair_gen_var_1,
};
use std::str::FromStr;

#[test]
fn test_mutate_numerator() {
    let test = |s, numerator, out| {
        let mut p = RationalPolynomial::from_str(s).unwrap();
        let ret = p.mutate_numerator(|n| {
            *n = IntegerPolynomial::from_str(numerator).unwrap();
            true
        });
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert!(ret);
    };
    test("1/2*x+1/3", "3*x", "1/2*x");
    test("1/2*x+1/3", "3*x+2", "1/2*x+1/3");
    // - the numerator becomes zero, and the denominator becomes 1
    test("1/2*x+1/3", "0", "0");
    test("0", "x^2-1", "x^2-1");
    test("1/6", "-4*x+2", "-2/3*x+1/3");
}

#[test]
fn test_mutate_denominator() {
    let test = |s, denominator: u32, out| {
        let mut p = RationalPolynomial::from_str(s).unwrap();
        let ret = p.mutate_denominator(|d| {
            *d = Natural::from(denominator);
            true
        });
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert!(ret);
    };
    test("1/2*x+1/3", 3, "x+2/3");
    test("1/2*x+1/3", 1, "3*x+2");
    test("1/2*x+1/3", 4, "3/4*x+1/2");
    test("0", 5, "0");
}

#[test]
#[should_panic]
fn mutate_denominator_fail() {
    let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    p.mutate_denominator(|d| {
        *d = Natural::ZERO;
    });
}

#[test]
fn test_mutate_numerator_and_denominator() {
    let test = |s, numerator, denominator: u32, out| {
        let mut p = RationalPolynomial::from_str(s).unwrap();
        let ret = p.mutate_numerator_and_denominator(|n, d| {
            *n = IntegerPolynomial::from_str(numerator).unwrap();
            *d = Natural::from(denominator);
            true
        });
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert!(ret);
    };
    test("1/2*x+1/3", "6*x+4", 4, "3/2*x+1");
    test("1/2*x+1/3", "0", 7, "0");
    test("0", "x", 2, "1/2*x");
}

#[test]
#[should_panic]
fn mutate_numerator_and_denominator_fail() {
    let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    p.mutate_numerator_and_denominator(|n, d| {
        *n = IntegerPolynomial::from_str("1").unwrap();
        *d = Natural::ZERO;
    });
}

#[test]
fn mutate_numerator_properties() {
    rational_polynomial_integer_polynomial_pair_gen().test_properties(|(mut p, numerator)| {
        let old_numerator = p.numerator_ref().clone();
        let old_denominator = p.denominator_ref().clone();
        let ret = p.mutate_numerator(|n| replace(n, numerator.clone()));
        assert_eq!(ret, old_numerator);
        assert!(p.is_valid());
        assert_eq!(
            p,
            RationalPolynomial::from_numerator_and_denominator(numerator, old_denominator)
        );
    });
}

#[test]
fn mutate_denominator_properties() {
    rational_polynomial_natural_pair_gen_var_1().test_properties(|(mut p, denominator)| {
        let old_numerator = p.numerator_ref().clone();
        let old_denominator = p.denominator_ref().clone();
        let ret = p.mutate_denominator(|d| replace(d, denominator.clone()));
        assert_eq!(ret, old_denominator);
        assert!(p.is_valid());
        assert_eq!(
            p,
            RationalPolynomial::from_numerator_and_denominator(old_numerator, denominator)
        );
    });
}

#[test]
fn mutate_numerator_and_denominator_properties() {
    rational_polynomial_integer_polynomial_natural_triple_gen_var_1().test_properties(
        |(mut p, numerator, denominator)| {
            let old = p.clone();
            let ret = p.mutate_numerator_and_denominator(|n, d| {
                (
                    replace(n, numerator.clone()),
                    replace(d, denominator.clone()),
                )
            });
            assert_eq!(ret, old.into_numerator_and_denominator());
            assert!(p.is_valid());
            assert_eq!(
                p,
                RationalPolynomial::from_numerator_and_denominator(numerator, denominator)
            );
        },
    );

    rational_polynomial_gen().test_properties(|mut p| {
        // a closure that changes nothing leaves the polynomial as it was
        let old = p.clone();
        p.mutate_numerator(|_| {});
        assert_eq!(p, old);
        p.mutate_denominator(|_| {});
        assert_eq!(p, old);
        p.mutate_numerator_and_denominator(|_, _| {});
        assert_eq!(p, old);
    });
}
