// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedDiv, DivRem};
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::{CheckedDivRem, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::div::{
    div_basecase_wrapper, div_divide_and_conquer,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_4,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::generated_coefficients;

#[test]
fn test_div() {
    let test = |p, q, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        let s = (&p).checked_div(&q);
        assert_eq!(
            s.as_ref().map(ToString::to_string),
            out.map(ToString::to_string)
        );
        if let Some(s) = s {
            assert!(s.is_valid());
            assert_eq!(&p / &q, s);
            let mut t = p.clone();
            t /= q;
            assert_eq!(t, s);
        }
    };
    // - div_divide_and_conquer_balanced: q2 > 0
    // - div_divide_and_conquer: shift < y_len
    // - div_divide_and_conquer: final x_len >= y_len
    // - checked_div_helper: general case
    test("x^3+2*x+1", "x+1", Some("x^2-x+3"));
    test("6*x^2+3*x+4", "3*x", Some("2*x+1"));
    test("x^3+2*x+1", "2*x+1", None);
    // - checked_div_helper: q == 0
    test("x+1", "0", None);
    // - checked_div_helper: p.coefficients.len() < q.coefficients.len()
    test("x+1", "x^2+1", Some("0"));
}

#[test]
#[should_panic]
fn div_fail() {
    let _ = IntegerPolynomial::from_str("x+1").unwrap() / IntegerPolynomial::from_str("0").unwrap();
}

#[test]
fn div_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let s = (&p).checked_div(&q);
        assert_eq!(s, (&p).checked_div_rem(&q).map(|x| x.0));
        assert_eq!(p.clone().checked_div(q.clone()), s);
    });

    integer_polynomial_pair_gen_var_4().test_properties(|(p, q)| {
        let s = &p / &q;
        assert_eq!(p.clone() / q.clone(), s);
        assert_eq!(s, (&p).div_rem(&q).0);
    });
}

// The quotient kernels, on products of polynomials built from coefficients of chosen sizes, with
// divisors whose leading coefficient is `lead`.
#[test]
fn test_div_algorithms() {
    let test = |lead: i32, x_len: usize, y_len: usize, bits: u64| {
        let mut ys = generated_coefficients(y_len - 1, bits);
        ys.push(Integer::from(lead));
        let q = IntegerPolynomial::from_coefficients_asc(ys.clone());
        let r = IntegerPolynomial::from_coefficients_asc(generated_coefficients(x_len, bits));
        let mut xs = (&r * &q).into_coefficients_asc();
        let expected = r.into_coefficients_asc();
        assert_eq!(div_basecase_wrapper(&xs, &ys).unwrap(), expected);
        assert_eq!(div_divide_and_conquer(&xs, &ys).unwrap(), expected);
        // Perturbing the dividend's leading coefficient makes the quotient non-integral, unless the
        // divisor's leading coefficient is a unit.
        if lead != 1 && lead != -1 {
            *xs.last_mut().unwrap() += Integer::ONE;
            assert!(div_basecase_wrapper(&xs, &ys).is_none());
            assert!(div_divide_and_conquer(&xs, &ys).is_none());
        }
    };
    // - basecase
    test(1, 3, 3, 3);
    test(-3, 5, 4, 20);
    // - x_len < 2 * y_len - 1: the division is made balanced
    test(2, 10, 30, 10);
    // - x_len == 2 * y_len - 1, with the divisor split recursively
    test(3, 40, 41, 10);
    test(-1, 100, 101, 5);
    // - x_len > 2 * y_len - 1: the dividend is divided a window at a time
    // - div_divide_and_conquer: shift >= y_len
    // - div_divide_and_conquer: final x_len < y_len
    test(2, 100, 20, 10);
    test(5, 333, 40, 4);
}
