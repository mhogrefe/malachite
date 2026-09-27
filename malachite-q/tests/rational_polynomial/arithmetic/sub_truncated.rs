// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    AddTruncated, EqTruncated, Polynomial, SubTruncated, SubTruncatedAssign,
};
use malachite_nz::test_util::generators::*;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::*;
use malachite_q::test_util::rational_polynomial::arithmetic::sub_truncated::*;

#[test]
fn test_sub_truncated() {
    let test = |s, t, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).sub_truncated(&q, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).sub_truncated(q.clone(), len), r);
        assert_eq!(p.clone().sub_truncated(&q, len), r);
        assert_eq!(p.clone().sub_truncated(q.clone(), len), r);
        let mut s = p.clone();
        s.sub_truncated_assign(&q, len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.sub_truncated_assign(q.clone(), len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(sub_truncated_naive(&p, &q, len), r);
    };
    // A zero length keeps nothing.
    test("0", "0", 0, "0");
    test("0", "0", 5, "0");
    test("1/2*x^2+1/3", "0", 0, "0");
    // Cutting the x^2 term leaves 2/6, which reduces to 1/3.
    test("1/2*x^2+1/3", "0", 2, "1/3");
    test("0", "1/2*x^2+1/3", 2, "-1/3");
    // Equal denominators.
    test("1/2*x^2+1/3*x+1/4", "1/2*x^2+2/3*x-1/4", 3, "-1/3*x+1/2");
    test("1/2*x^2+1/3*x+1/4", "1/2*x^2+2/3*x-1/4", 1, "1/2");
    // A length past both degrees is the whole difference.
    test("1/2*x^2+1/3*x+1/4", "1/2*x^2+2/3*x-1/4", 100, "-1/3*x+1/2");
    // Denominators 6 and 4: after cutting, 8/12 reduces by 4, more than their GCD of 2.
    test("1/6*x^2+1/6", "-1/4*x^2-1/2", 1, "2/3");
    test("1/6*x^2+1/6", "-1/4*x^2-1/2", 3, "5/12*x^2+2/3");
    // With different denominators, the kept parts can cancel completely.
    test("x^2+1/2", "-1/3*x^2+1/2", 1, "0");
    test("1/1000000000000000000000*x+1", "-x+1", 1, "0");
}

#[test]
fn test_sub_truncated_self() {
    // A polynomial combined with itself, through the same reference.
    let test = |s, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = (&p).sub_truncated(&p, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(r, sub_truncated_naive(&p, &p, len));
    };
    test("0", 3, "0");
    test("x-3", 5, "0");
    test("1/4*x+1/2", 1, "0");
    test("1/3*x+2/3", 1, "0");
}

#[test]
fn sub_truncated_properties() {
    rational_polynomial_rational_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            let r = (&p).sub_truncated(&q, len);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).sub_truncated(q.clone(), len), r);
            assert_eq!(p.clone().sub_truncated(&q, len), r);
            assert_eq!(p.clone().sub_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.sub_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.sub_truncated_assign(q.clone(), len);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // This is the truncation of the whole result, and the result of the truncations.
            assert_eq!(sub_truncated_naive(&p, &q, len), r);
            assert_eq!(p.truncate(len) - q.truncate(len), r);
            // It agrees with the whole result below the cut, and has nothing above it.
            assert!(r.eq_truncated(&(&p - &q), len));
            assert!(r.len() <= len);
            // Through one reference, it is the same as through two.
            let s = (&p).sub_truncated(&p, len);
            assert!(s.is_valid());
            assert_eq!(s, (&p).sub_truncated(p.clone(), len));
            // Swapping the operands negates the difference.
            assert_eq!((&q).sub_truncated(&p, len), -&r);
            // A polynomial minus itself is zero.
            assert_eq!((&p).sub_truncated(p.clone(), len), RationalPolynomial::ZERO);
            // This is adding the negation.
            assert_eq!((&p).add_truncated(-&q, len), r);
        },
    );

    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        // A zero length keeps nothing, and a length past both degrees keeps everything.
        assert_eq!((&p).sub_truncated(&q, 0), RationalPolynomial::ZERO);
        let len = p.len().max(q.len());
        assert_eq!((&p).sub_truncated(&q, len), &p - &q);
        assert_eq!((&p).sub_truncated(&q, u64::MAX), &p - &q);
    });

    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            // On polynomials with integer coefficients, this is the IntegerPolynomial operation.
            assert_eq!(
                RationalPolynomial::from(p.clone())
                    .sub_truncated(RationalPolynomial::from(q.clone()), len),
                RationalPolynomial::from(p.sub_truncated(q, len))
            );
        },
    );
}
