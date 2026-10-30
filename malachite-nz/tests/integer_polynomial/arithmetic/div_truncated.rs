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
use malachite_base::polynomial::{
    CheckedDivTruncated, DivTruncated, DivTruncatedAssign, InverseTruncated, MulTruncated,
    Polynomial,
};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::div_truncated::*;
use malachite_nz::test_util::generators::{
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1,
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4,
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_5,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::div_truncated::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::generated_coefficients;

#[test]
fn test_div_truncated() {
    let test = |s, t, len, out: Option<&str>| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        let r = (&p).checked_div_truncated(&q, len);
        assert!(r.as_ref().is_none_or(IntegerPolynomial::is_valid));
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!((&p).checked_div_truncated(q.clone(), len), r);
        assert_eq!(p.clone().checked_div_truncated(&q, len), r);
        assert_eq!(p.clone().checked_div_truncated(q.clone(), len), r);
        assert_eq!(div_truncated_naive(&p, &q, len), r);
        if let Some(r) = r {
            assert_eq!((&p).div_truncated(&q, len), r);
            assert_eq!((&p).div_truncated(q.clone(), len), r);
            assert_eq!(p.clone().div_truncated(&q, len), r);
            assert_eq!(p.clone().div_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.div_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p;
            s.div_truncated_assign(q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
        }
    };
    // - checked_div_truncated_helper: xs.is_empty()
    test("0", "x+3", 3, Some("0"));
    // - checked_div_truncated_helper: len == 0
    test("x", "0", 0, Some("0"));
    test("x", "x", 0, Some("0"));
    // - checked_div_truncated_helper: ys.is_empty()
    test("x", "0", 3, None);
    // - checked_div_truncated_helper: ys[0] == 0
    test("x", "x", 3, None);
    // Constant divisors
    // - div_truncated_basecase: y_len == 1 && y0 == 1
    // - div_truncated_dispatch: n < cutoff
    // - checked_div_truncated_helper: general case
    test("x+1", "1", 3, Some("x+1"));
    // - div_truncated_basecase: y_len == 1 && y0 == -1
    test("x+1", "-1", 3, Some("-x-1"));
    // - div_exact_checked: divisible
    // - div_truncated_basecase: y_len == 1 && !unit
    test("4*x+6", "2", 2, Some("2*x+3"));
    // - div_exact_checked: not divisible
    test("4*x+5", "2", 2, None);
    test("6", "3", 1, Some("2"));
    test("5", "3", 1, None);
    // Constant dividends
    // - div_truncated_basecase: x_len == 1 && x0 == 1
    test("1", "x+1", 4, Some("-x^3+x^2-x+1"));
    // - div_truncated_basecase: x_len == 1 && x0 != 1
    test("3", "x+1", 3, Some("3*x^2-3*x+3"));
    test("-2", "-x+1", 4, Some("-2*x^3-2*x^2-2*x-2"));
    // Exact quotients
    // - small_bits: n_small == 0
    // - div_truncated_basecase: general y0 == 1
    // - div_truncated_basecase: small_dot is used
    // - div_truncated_basecase: dot is used
    // - div_truncated_basecase: i < x_len && y0 == 1
    // - div_truncated_basecase: i >= x_len && y0 == 1
    // - div_truncated_basecase: small_value(&out[i]) is Some
    test("x^2+3*x+2", "x+1", 5, Some("x+2"));
    // - div_truncated_basecase: general !unit
    // - div_truncated_basecase: i < x_len && !unit
    test("x^2+3*x+2", "x+2", 2, Some("x+1"));
    test("6*x^2+7*x+2", "3*x+2", 3, Some("2*x+1"));
    test("2*x", "x+2", 2, Some("x"));
    test("x^2+1", "x+1", 4, Some("-2*x^3+2*x^2-x+1"));
    test("x^2+1", "-x+1", 4, Some("2*x^3+2*x^2+x+1"));
    test("4*x^2+2", "2*x+2", 3, Some("3*x^2-x+1"));
    // Quotients that are not integral
    test("x+1", "x+2", 2, None);
    test("x", "x+2", 3, None);
    // - small_dot_bits: q_bits > SMALL
    test(
        "x+36893488147419103232",
        "x+1",
        3,
        Some("36893488147419103231*x^2-36893488147419103231*x+36893488147419103232"),
    );
}

#[test]
#[should_panic]
fn div_truncated_fail_1() {
    let _ = IntegerPolynomial::from_str("x")
        .unwrap()
        .div_truncated(IntegerPolynomial::ZERO, 3);
}

#[test]
#[should_panic]
fn div_truncated_fail_2() {
    let _ = IntegerPolynomial::from_str("x")
        .unwrap()
        .div_truncated(IntegerPolynomial::from_str("x").unwrap(), 3);
}

#[test]
#[should_panic]
fn div_truncated_fail_3() {
    let _ = IntegerPolynomial::from_str("x+1")
        .unwrap()
        .div_truncated(IntegerPolynomial::from_str("x+2").unwrap(), 2);
}

#[test]
#[should_panic]
fn div_truncated_ref_ref_fail() {
    let _ = (&IntegerPolynomial::from_str("x+1").unwrap())
        .div_truncated(&IntegerPolynomial::from_str("x+2").unwrap(), 2);
}

#[test]
#[should_panic]
fn div_truncated_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x+1").unwrap();
    p.div_truncated_assign(IntegerPolynomial::from_str("x+2").unwrap(), 2);
}

// The kernels, on products of power series built from coefficients of chosen sizes, with divisors
// whose constant coefficient is `y0`.
#[test]
fn test_div_truncated_algorithms() {
    let test = |y0: i32, x_len: usize, y_len: usize, bits: u64, n: usize| {
        let mut ys = vec![Integer::from(y0)];
        ys.extend(generated_coefficients(y_len - 1, bits));
        let q = IntegerPolynomial::from_coefficients_asc(ys.clone());
        let r = IntegerPolynomial::from_coefficients_asc(generated_coefficients(x_len, bits));
        let p = (&r).mul_truncated(&q, u64::exact_from(n));
        let xs = p.coefficients_asc().to_vec();
        let expected = r.clone().truncate(u64::exact_from(n));
        let check = |out: Option<Vec<Integer>>| {
            let out = out.unwrap();
            assert_eq!(out.len(), n);
            assert_eq!(IntegerPolynomial::from_coefficients_asc(out), expected);
        };
        check(div_truncated_basecase(&xs, &ys, n));
        check(div_truncated_divide_and_conquer(&xs, &ys, n));
        check(div_truncated_dispatch(&xs, &ys, n));
        // Perturbing the dividend's constant coefficient makes the quotient non-integral, unless
        // the divisor's constant coefficient is a unit.
        if y0 != 1 && y0 != -1 {
            let mut xs = xs.clone();
            xs[0] += Integer::ONE;
            assert!(div_truncated_basecase(&xs, &ys, n).is_none());
            assert!(div_truncated_divide_and_conquer(&xs, &ys, n).is_none());
            assert!(div_truncated_dispatch(&xs, &ys, n).is_none());
        }
    };
    // - div_basecase: *top != 0
    // - div_basecase: *top == 0
    // - div_basecase: y_count > i
    // - div_divide_and_conquer_recursive: n <= CUTOFF
    // - div_divide_and_conquer_balanced: q2 == 0
    // - div_divide_and_conquer: x_len <= n
    // - div_truncated_divide_and_conquer
    test(1, 3, 3, 3, 10);
    // - div_truncated_basecase: general y0 == -1
    // - div_truncated_basecase: i < x_len && y0 == -1
    // - div_truncated_basecase: i >= x_len && y0 == -1
    test(-1, 3, 3, 3, 10);
    // - div_basecase: !top.divisible_by(lead)
    // - div_truncated_basecase: i >= x_len && !unit
    test(2, 3, 3, 3, 10);
    test(-3, 5, 5, 20, 12);
    // - div_truncated_basecase: small_value(&out[i]) is None
    test(5, 5, 5, 70, 12);
    // - div_rem_low_divide_and_conquer: n <= CUTOFF
    // - div_rem_low_divide_and_conquer: n > CUTOFF
    // - div_rem_low_divide_and_conquer: k + n1 >= n2 + 1
    // - div_rem_low_divide_and_conquer: k + n1 < n2 + 1
    // - div_divide_and_conquer_recursive: n > CUTOFF
    // - div_divide_and_conquer_recursive: j >= 2 * n2
    // - div_divide_and_conquer_recursive: j < 2 * n2
    test(1, 80, 80, 3, 100);
    test(-1, 80, 80, 3, 100);
    test(2, 80, 80, 3, 100);
    test(3, 150, 100, 2, 200);
    // - div_truncated_dispatch: ys.len() < cutoff
    test(1, 80, 10, 300, 200);
    // - div_truncated_dispatch: divide-and-conquer is used
    test(2, 200, 150, 300, 200);
    test(1, 200, 150, 300, 200);
    // Constant dividends, with divisors whose constant coefficient is `y0`.
    let test_constant = |x0: i32, y0: i32, y_len: usize, bits: u64, n: usize| {
        let xs = vec![Integer::from(x0)];
        let mut ys = vec![Integer::from(y0)];
        ys.extend(generated_coefficients(y_len - 1, bits));
        let p = IntegerPolynomial::from_coefficients_asc(xs.clone());
        let q = IntegerPolynomial::from_coefficients_asc(ys.clone());
        let expected = div_truncated_naive(&p, &q, u64::exact_from(n));
        let to_polynomial =
            |out: Option<Vec<Integer>>| out.map(IntegerPolynomial::from_coefficients_asc);
        assert_eq!(to_polynomial(div_truncated_basecase(&xs, &ys, n)), expected);
        assert_eq!(to_polynomial(div_truncated_dispatch(&xs, &ys, n)), expected);
    };
    // - div_truncated_dispatch: xs.len() == 1
    test_constant(3, 1, 200, 300, 200);
    test_constant(3, 2, 200, 300, 200);
}

#[test]
fn div_truncated_properties() {
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            let r = (&p).checked_div_truncated(&q, len);
            assert!(r.as_ref().is_none_or(IntegerPolynomial::is_valid));
            assert_eq!((&p).checked_div_truncated(q.clone(), len), r);
            assert_eq!(p.clone().checked_div_truncated(&q, len), r);
            assert_eq!(p.clone().checked_div_truncated(q.clone(), len), r);
            assert_eq!(div_truncated_naive(&p, &q, len), r);
            if let Some(r) = r {
                assert_eq!((&q).mul_truncated(&r, len), p.truncate(len));
            }
        },
    );

    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_4().test_properties(
        |(p, q, len)| {
            let r = (&p).div_truncated(&q, len);
            assert!(r.is_valid());
            assert_eq!((&p).div_truncated(q.clone(), len), r);
            assert_eq!(p.clone().div_truncated(&q, len), r);
            assert_eq!(p.clone().div_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.div_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.div_truncated_assign(q.clone(), len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            assert_eq!((&p).checked_div_truncated(&q, len), Some(r.clone()));
            assert_eq!(div_truncated_naive(&p, &q, len), Some(r.clone()));
            assert!(r.len() <= len);
            assert_eq!((&q).mul_truncated(&r, len), p.clone().truncate(len));
            // Negating the dividend or the divisor negates the quotient.
            assert_eq!((-&p).div_truncated(&q, len), -&r);
            assert_eq!((&p).div_truncated(-&q, len), -&r);
        },
    );

    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, len)| {
            let r = (&p).div_truncated(&q, len);
            assert!(r.is_valid());
            assert_eq!(div_truncated_naive(&p, &q, len), Some(r.clone()));
            // Dividing by a series whose constant coefficient is 1 or -1 is multiplying by its
            // inverse.
            assert_eq!(
                (&p).mul_truncated(q.inverse_truncated(len).unwrap(), len),
                r
            );
        },
    );
}
