// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{InverseTruncated, MulTruncated, Polynomial};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::inverse_truncated::*;
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_1, integer_polynomial_unsigned_pair_gen_var_7,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::inverse_truncated::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::generated_coefficients;

#[test]
fn test_inverse_truncated() {
    let test = |s, len, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let r = (&p).inverse_truncated(len);
        assert!(r.as_ref().is_none_or(IntegerPolynomial::is_valid));
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(p.clone().inverse_truncated(len), r);
        assert_eq!(inverse_truncated_naive(&p, len), r);
    };
    // - inverse_truncated_helper: xs.len() == 1
    test("1", 3, Some("1"));
    test("-1", 3, Some("-1"));
    // - inverse_truncated_helper: len == 0
    test("x+1", 0, Some("0"));
    test("0", 0, Some("0"));
    // - inverse_truncated_helper: xs.is_empty()
    test("0", 3, None);
    // - inverse_truncated_helper: x0.unsigned_abs_ref() != 1
    test("x+2", 3, None);
    // - inverse_truncated_basecase: x_len == 1
    // - inverse_truncated_dispatch: basecase
    // - inverse_truncated_helper: general case
    test("x+1", 1, Some("1"));
    // - inverse_truncated_basecase: binomial x_len == 2
    // - inverse_truncated_basecase: binomial neg
    test("2*x+1", 4, Some("-8*x^3+4*x^2-2*x+1"));
    // - small_dot: bits < W
    // - small_dot_bits: i >= n_small
    // - small_bits: n_small > 0
    // - inverse_truncated_basecase: general neg
    // - inverse_truncated_basecase: small_dot is used
    // - inverse_truncated_basecase: small_value(&out[i]) is Some
    // - inverse_truncated_basecase: dot is used
    test("x^2+x+1", 6, Some("-x^4+x^3-x+1"));
    test("x^3+x+1", 8, Some("-9*x^7+6*x^6-4*x^5+3*x^4-2*x^3+x^2-x+1"));
    // - inverse_truncated_basecase: general !neg
    test("-x^2+x-1", 5, Some("x^4+x^3-x-1"));
    // Binomials
    // - inverse_truncated_basecase: binomial x_len > 2
    // - inverse_truncated_basecase: binomial !neg
    test("-x^3-1", 7, Some("-x^6+x^3-1"));
    test("3*x^2+1", 5, Some("9*x^4-3*x^2+1"));
    test(
        "18446744073709551616*x+1",
        3,
        Some("340282366920938463463374607431768211456*x^2-18446744073709551616*x+1"),
    );
    // - small_dot: bits < 3W
    // - small_dot: 3W p < 0
    // - small_dot: 3W p >= 0
    test(
        "-494*x^5-3355550598108864573*x^4-247749477252196*x^3-32*x^2-12*x-1",
        10,
        Some(
            "15207218299458705737655703501644404230872536*x^9-\
            11199910770884274841630864922200011737*x^8-1660462140239744770491870090692936*x^7-\
            61379802245035392753703310768*x^6-80442042546983876618*x^5+3349604610654803933*x^4+\
            247749477253156*x^3-112*x^2+12*x-1",
        ),
    );
}

// The kernels, on power series built from coefficients of chosen sizes.
#[test]
fn test_inverse_truncated_algorithms() {
    let test = |x0: i32, len: usize, bits: u64, n: usize| {
        let mut xs = vec![Integer::from(x0)];
        xs.extend(generated_coefficients(len - 1, bits));
        let p = IntegerPolynomial::from_coefficients_asc(xs.clone());
        let expected = inverse_truncated_naive(&p, u64::exact_from(n)).unwrap();
        let check = |out: Vec<Integer>| {
            assert_eq!(out.len(), n);
            assert_eq!(IntegerPolynomial::from_coefficients_asc(out), expected);
        };
        check(inverse_truncated_basecase(&xs, n));
        check(inverse_truncated_newton(&xs, n));
        check(inverse_truncated_dispatch(&xs, n));
    };
    // - inverse_truncated_newton: x_len < CUTOFF
    test(1, 3, 3, 10);
    test(-1, 3, 3, 10);
    // - small_dot: bits < 2W
    // - inverse_truncated_basecase: small_value(&out[i]) is None
    test(1, 5, 20, 12);
    test(-1, 5, 40, 12);
    // - small_bits: small_value(&ys[n_small]) is None
    test(1, 5, 70, 12);
    // - inverse_truncated_newton: x_len >= CUTOFF
    // - inverse_truncated_newton: w_len == n
    // - inverse_truncated_dispatch: Newton
    test(1, 70, 3, 100);
    // - inverse_truncated_newton: w_len < n
    test(-1, 70, 2, 150);
    test(1, 200, 1, 300);
}

#[test]
fn inverse_truncated_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        let r = (&p).inverse_truncated(len);
        assert!(r.as_ref().is_none_or(IntegerPolynomial::is_valid));
        assert_eq!(p.clone().inverse_truncated(len), r);
        assert_eq!(inverse_truncated_naive(&p, len), r);
        // The inverse exists exactly when the constant coefficient is 1 or -1.
        assert_eq!(
            r.is_some(),
            len == 0
                || p.coefficients_asc()
                    .first()
                    .is_some_and(|c| *c == 1u32 || *c == -1i32)
        );
    });

    integer_polynomial_unsigned_pair_gen_var_7().test_properties(|(p, len)| {
        let r = (&p).inverse_truncated(len).unwrap();
        assert!(r.is_valid());
        assert_eq!(p.clone().inverse_truncated(len).unwrap(), r);
        assert_eq!(inverse_truncated_naive(&p, len).unwrap(), r);
        assert!(r.len() <= len);
        // The product is 1 modulo x^len.
        let one = if len == 0 {
            IntegerPolynomial::ZERO
        } else {
            IntegerPolynomial::from(Integer::ONE)
        };
        assert_eq!((&p).mul_truncated(&r, len), one);
        // The inverse of the inverse is the truncation of the original.
        assert_eq!(
            (&r).inverse_truncated(len).unwrap(),
            p.clone().truncate(len)
        );
        // Negating the series negates the inverse.
        assert_eq!((-&p).inverse_truncated(len).unwrap(), -r);
    });
}
