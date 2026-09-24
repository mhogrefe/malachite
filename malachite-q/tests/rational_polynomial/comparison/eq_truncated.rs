// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, OneHalf};
use malachite_base::polynomial::{EqTruncated, Polynomial};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_integer_polynomial_pair_gen,
    rational_polynomial_natural_polynomial_pair_gen, rational_polynomial_pair_gen,
    rational_polynomial_unsigned_pair_gen_var_1, rational_polynomial_unsigned_polynomial_pair_gen,
};

#[test]
fn test_eq_truncated() {
    let test = |s, t, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        assert_eq!(p.eq_truncated(&q, len), out);
        assert_eq!(q.eq_truncated(&p, len), out);
        assert_eq!(p.truncate(len) == q.truncate(len), out);
    };
    test("0", "0", 0, true);
    test("0", "0", 5, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 3, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 4, false);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", u64::MAX, false);
    test("x^3+2*x^2+3*x+4", "3*x+4", 2, true);
    test("x^3+2*x^2+3*x+4", "3*x+4", 3, false);
    test("x^2+1", "1", 1, true);
    test("x^2+1", "1", 2, true);
    test("x^2+1", "1", 3, false);
    test("x+1", "x+1", u64::MAX, true);
    test("x", "1", 0, true);
    test("x", "1", 1, false);
    test("x^5", "0", 5, true);
    test("x^5", "0", 6, false);
    // The example FLINT's equal_trunc is checked with in the porting notes.
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 4, true);
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 5, false);
    test("1/2*x+1", "1", 1, true);
    test("1/2*x+1", "1", 2, false);
    test("1/2*x", "0", 1, true);
    test("1/2*x", "0", 2, false);
    test("1/2*x^2+x+3", "x+3", 2, true);
    test("1/2*x^2+x+3", "x+3", 3, false);
    test("1/2", "0", 1, false);
    test("1/2*x+1", "1/3*x+1", 1, true);
    test("1/2*x+1", "1/3*x+1", 2, false);
    test("1/2*x+1/3", "1/5*x+1/3", 1, true);
    test("1/6*x+1/3", "1/4*x+1/3", 1, true);
    test("1/6*x+1/3", "1/4*x+1/3", 2, false);
    test("1/6*x+1/3", "1/6*x+1/3", u64::MAX, true);
}

#[test]
fn test_eq_truncated_integer_polynomial() {
    let test = |s, t, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        assert_eq!(p.eq_truncated(&q, len), out);
        assert_eq!(q.eq_truncated(&p, len), out);
        assert_eq!(p.truncate(len) == q.truncate(len), out);
    };
    test("0", "0", 0, true);
    test("0", "0", 5, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 3, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 4, false);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", u64::MAX, false);
    test("x^3+2*x^2+3*x+4", "3*x+4", 2, true);
    test("x^3+2*x^2+3*x+4", "3*x+4", 3, false);
    test("x^2+1", "1", 1, true);
    test("x^2+1", "1", 2, true);
    test("x^2+1", "1", 3, false);
    test("x+1", "x+1", u64::MAX, true);
    test("x", "1", 0, true);
    test("x", "1", 1, false);
    test("x^5", "0", 5, true);
    test("x^5", "0", 6, false);
    // The example FLINT's equal_trunc is checked with in the porting notes.
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 4, true);
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 5, false);
    test("1/2*x+1", "1", 1, true);
    test("1/2*x+1", "1", 2, false);
    test("1/2*x", "0", 1, true);
    test("1/2*x", "0", 2, false);
    test("1/2*x^2+x+3", "x+3", 2, true);
    test("1/2*x^2+x+3", "x+3", 3, false);
    test("1/2", "0", 1, false);
    test("-1/2*x-1", "-1", 1, true);
    test("-1/2*x-1", "-1", 2, false);
}

#[test]
fn test_eq_truncated_natural_polynomial() {
    let test = |s, t, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        assert_eq!(p.eq_truncated(&q, len), out);
        assert_eq!(q.eq_truncated(&p, len), out);
        assert_eq!(p.truncate(len) == q.truncate(len), out);
    };
    test("0", "0", 0, true);
    test("0", "0", 5, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 3, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 4, false);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", u64::MAX, false);
    test("x^3+2*x^2+3*x+4", "3*x+4", 2, true);
    test("x^3+2*x^2+3*x+4", "3*x+4", 3, false);
    test("x^2+1", "1", 1, true);
    test("x^2+1", "1", 2, true);
    test("x^2+1", "1", 3, false);
    test("x+1", "x+1", u64::MAX, true);
    test("x", "1", 0, true);
    test("x", "1", 1, false);
    test("x^5", "0", 5, true);
    test("x^5", "0", 6, false);
    // The example FLINT's equal_trunc is checked with in the porting notes.
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 4, true);
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 5, false);
    test("1/2*x+1", "1", 1, true);
    test("1/2*x+1", "1", 2, false);
    test("1/2*x", "0", 1, true);
    test("1/2*x", "0", 2, false);
    test("1/2*x^2+x+3", "x+3", 2, true);
    test("1/2*x^2+x+3", "x+3", 3, false);
    test("1/2", "0", 1, false);
}

#[test]
fn test_eq_truncated_unsigned_polynomial() {
    let test = |s, t, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = UnsignedPolynomial::<u64>::from_str(t).unwrap();
        assert_eq!(p.eq_truncated(&q, len), out);
        assert_eq!(q.eq_truncated(&p, len), out);
        assert_eq!(p.truncate(len) == q.truncate(len), out);
    };
    test("0", "0", 0, true);
    test("0", "0", 5, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 3, true);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", 4, false);
    test("x^3+2*x^2+3*x+4", "5*x^3+2*x^2+3*x+4", u64::MAX, false);
    test("x^3+2*x^2+3*x+4", "3*x+4", 2, true);
    test("x^3+2*x^2+3*x+4", "3*x+4", 3, false);
    test("x^2+1", "1", 1, true);
    test("x^2+1", "1", 2, true);
    test("x^2+1", "1", 3, false);
    test("x+1", "x+1", u64::MAX, true);
    test("x", "1", 0, true);
    test("x", "1", 1, false);
    test("x^5", "0", 5, true);
    test("x^5", "0", 6, false);
    // The example FLINT's equal_trunc is checked with in the porting notes.
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 4, true);
    test("3*x^5+2*x+1", "6*x^4+2*x+1", 5, false);
    test("1/2*x+1", "1", 1, true);
    test("1/2*x+1", "1", 2, false);
    test("1/2*x", "0", 1, true);
    test("1/2*x", "0", 2, false);
    test("1/2*x^2+x+3", "x+3", 2, true);
    test("1/2*x^2+x+3", "x+3", 3, false);
    test("1/2", "0", 1, false);
}

#[test]
fn eq_truncated_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        let (p_len, q_len) = (p.len(), q.len());
        for len in [0, 1, 2, p_len, q_len, p_len.max(q_len), u64::MAX] {
            let eq = p.eq_truncated(&q, len);
            assert_eq!(q.eq_truncated(&p, len), eq);
            // It agrees with comparing the two truncations.
            assert_eq!(p.truncate(len) == q.truncate(len), eq);
            // Agreement below x^len implies agreement below every lower power.
            if eq && len != 0 {
                assert!(p.eq_truncated(&q, len - 1));
            }
        }
        assert!(p.eq_truncated(&q, 0));
        assert_eq!(p.eq_truncated(&q, u64::MAX), p == q);
    });
}

#[test]
fn eq_truncated_agreement_properties() {
    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, i)| {
        assert!(p.eq_truncated(&p, i));
        // Changing the coefficient of x^i leaves agreement exactly below it.
        let mut q = p.clone();
        q.mutate_coefficient(i, |c| *c += Rational::ONE_HALF);
        for len in [0, i, i + 1, u64::MAX] {
            assert_eq!(p.eq_truncated(&q, len), len <= i);
        }
    });
}

#[test]
fn eq_truncated_integer_polynomial_properties() {
    rational_polynomial_integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let (p_len, q_len) = (p.len(), q.len());
        for len in [0, 1, 2, p_len, q_len, p_len.max(q_len), u64::MAX] {
            let eq = p.eq_truncated(&q, len);
            assert_eq!(q.eq_truncated(&p, len), eq);
            // It agrees with comparing the two truncations.
            assert_eq!(p.truncate(len) == q.truncate(len), eq);
            // Agreement below x^len implies agreement below every lower power.
            if eq && len != 0 {
                assert!(p.eq_truncated(&q, len - 1));
            }
        }
        assert!(p.eq_truncated(&q, 0));
        assert_eq!(p.eq_truncated(&q, u64::MAX), p == q);

        // A polynomial agrees everywhere with its conversion to a wider type, and changing one
        // coefficient of the conversion leaves agreement exactly below it.
        let mut r = RationalPolynomial::from(q.clone());
        assert!(r.eq_truncated(&q, u64::MAX));
        let k = q.len() >> 1;
        r.mutate_coefficient(k, |c| *c += Rational::ONE_HALF);
        for len in [0, k, k + 1, u64::MAX] {
            assert_eq!(r.eq_truncated(&q, len), len <= k);
            assert_eq!(q.eq_truncated(&r, len), len <= k);
        }
    });
}

#[test]
fn eq_truncated_natural_polynomial_properties() {
    rational_polynomial_natural_polynomial_pair_gen().test_properties(|(p, q)| {
        let (p_len, q_len) = (p.len(), q.len());
        for len in [0, 1, 2, p_len, q_len, p_len.max(q_len), u64::MAX] {
            let eq = p.eq_truncated(&q, len);
            assert_eq!(q.eq_truncated(&p, len), eq);
            // It agrees with comparing the two truncations.
            assert_eq!(p.truncate(len) == q.truncate(len), eq);
            // Agreement below x^len implies agreement below every lower power.
            if eq && len != 0 {
                assert!(p.eq_truncated(&q, len - 1));
            }
        }
        assert!(p.eq_truncated(&q, 0));
        assert_eq!(p.eq_truncated(&q, u64::MAX), p == q);

        // A polynomial agrees everywhere with its conversion to a wider type, and changing one
        // coefficient of the conversion leaves agreement exactly below it.
        let mut r = RationalPolynomial::from(q.clone());
        assert!(r.eq_truncated(&q, u64::MAX));
        let k = q.len() >> 1;
        r.mutate_coefficient(k, |c| *c += Rational::ONE_HALF);
        for len in [0, k, k + 1, u64::MAX] {
            assert_eq!(r.eq_truncated(&q, len), len <= k);
            assert_eq!(q.eq_truncated(&r, len), len <= k);
        }
    });
}

#[test]
fn eq_truncated_unsigned_polynomial_properties() {
    rational_polynomial_unsigned_polynomial_pair_gen::<u64>().test_properties(|(p, q)| {
        let (p_len, q_len) = (p.len(), q.len());
        for len in [0, 1, 2, p_len, q_len, p_len.max(q_len), u64::MAX] {
            let eq = p.eq_truncated(&q, len);
            assert_eq!(q.eq_truncated(&p, len), eq);
            // It agrees with comparing the two truncations.
            assert_eq!(p.truncate(len) == q.truncate(len), eq);
            // Agreement below x^len implies agreement below every lower power.
            if eq && len != 0 {
                assert!(p.eq_truncated(&q, len - 1));
            }
        }
        assert!(p.eq_truncated(&q, 0));
        assert_eq!(p.eq_truncated(&q, u64::MAX), p == q);

        // A polynomial agrees everywhere with its conversion to a wider type, and changing one
        // coefficient of the conversion leaves agreement exactly below it.
        let mut r = RationalPolynomial::from(q.clone());
        assert!(r.eq_truncated(&q, u64::MAX));
        let k = q.len() >> 1;
        r.mutate_coefficient(k, |c| *c += Rational::ONE);
        for len in [0, k, k + 1, u64::MAX] {
            assert_eq!(r.eq_truncated(&q, len), len <= k);
            assert_eq!(q.eq_truncated(&r, len), len <= k);
        }
    });
}
