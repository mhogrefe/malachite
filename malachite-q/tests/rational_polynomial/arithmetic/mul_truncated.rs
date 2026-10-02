// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign, Polynomial};
use malachite_nz::test_util::generators::*;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1,
};
use malachite_q::test_util::rational_polynomial::arithmetic::mul_truncated::mul_truncated_naive;

#[test]
fn test_mul_truncated() {
    let test = |s, t, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).mul_truncated(&q, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mul_truncated(q.clone(), len), r);
        assert_eq!(p.clone().mul_truncated(&q, len), r);
        assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
        let mut s = p.clone();
        s.mul_truncated_assign(&q, len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mul_truncated_assign(q.clone(), len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_truncated_naive(&p, &q, len), r);
    };
    // - self == Self::ZERO || other == Self::ZERO || len == 0
    test("0", "x", 3, "0");
    test("x+1", "0", 3, "0");
    test("x+1", "x-1", 0, "0");
    // - denominator == 1u32
    test("x+1", "x-1", 2, "-1");
    // - denominator != 1u32
    // - g == 1u32
    test("1/2*x+1/3", "x-1/2", 1, "-1/6");
    test("1/2*x+1/3", "x-1/2", 2, "1/12*x-1/6");
    // A length past the end of the product keeps all of it.
    test("1/2*x+1/3", "x-1/2", 10, "1/2*x^2+1/12*x-1/6");
    // - g != 1u32: cutting the product leaves its numerator sharing a factor with the denominator.
    test("1/2*x^2+x+1", "x+2", 2, "3*x+2");
    test("1/2*x^2+x+1", "x+2", 1, "2");
    test("1/2*x^2+1", "2*x^2+2", 2, "2");
}

#[test]
fn test_mul_truncated_self() {
    // A polynomial multiplied by itself, through the same reference.
    let test = |s, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = (&p).mul_truncated(&p, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(mul_truncated_naive(&p, &p, len), r);
    };
    // - ptr::eq(self, other)
    test("0", 2, "0");
    test("1/2*x+1/3", 2, "1/3*x+1/9");
    test("1/2*x^2+x+1", 3, "2*x^2+2*x+1");
}

#[test]
fn mul_truncated_properties() {
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            let r = (&p).mul_truncated(&q, len);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mul_truncated(q.clone(), len), r);
            assert_eq!(p.clone().mul_truncated(&q, len), r);
            assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.mul_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mul_truncated_assign(q.clone(), len);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // It is the truncation of the whole product, and only the first `len` coefficients of
            // each factor matter.
            assert_eq!(mul_truncated_naive(&p, &q, len), r);
            assert_eq!((&p * &q).truncate(len), r);
            assert_eq!(p.truncate(len).mul_truncated(q.truncate(len), len), r);
            // Multiplication is commutative.
            assert_eq!((&q).mul_truncated(&p, len), r);
            assert!(r.len() <= len);
        },
    );

    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        // A zero length keeps nothing, and a length past the product keeps everything.
        assert_eq!((&p).mul_truncated(&q, 0), RationalPolynomial::ZERO);
        let len = p.len() + q.len();
        assert_eq!((&p).mul_truncated(&q, len), &p * &q);
        assert_eq!((&p).mul_truncated(&q, u64::MAX), &p * &q);
    });

    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            // On polynomials with integer coefficients, this is the `IntegerPolynomial` operation.
            assert_eq!(
                RationalPolynomial::from(p.clone())
                    .mul_truncated(RationalPolynomial::from(q.clone()), len),
                RationalPolynomial::from(p.mul_truncated(q, len))
            );
        },
    );
}
