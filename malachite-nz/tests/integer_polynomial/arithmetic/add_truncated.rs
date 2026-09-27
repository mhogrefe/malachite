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
    AddTruncated, AddTruncatedAssign, EqTruncated, Polynomial, SubTruncated,
};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::integer_polynomial::arithmetic::add_truncated::*;

#[test]
fn test_add_truncated() {
    let test = |s, t, len, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&p).add_truncated(&q, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).add_truncated(q.clone(), len), r);
        assert_eq!(p.clone().add_truncated(&q, len), r);
        assert_eq!(p.clone().add_truncated(q.clone(), len), r);
        let mut s = p.clone();
        s.add_truncated_assign(&q, len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.add_truncated_assign(q.clone(), len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(add_truncated_naive(&p, &q, len), r);
    };
    // A zero length keeps nothing.
    test("0", "0", 0, "0");
    test("0", "0", 5, "0");
    test("x^2+1", "0", 0, "0");
    // Truncation drops the high coefficients of either operand.
    test("x^2+1", "0", 2, "1");
    test("x^2+1", "0", 3, "x^2+1");
    test("0", "x^2+1", 2, "1");
    // Cancellation below the cut lowers the degree further.
    test("x^3+2*x^2-x+5", "4*x^2+x-2", 3, "6*x^2+3");
    test("x^3+2*x^2-x+5", "4*x^2+x-2", 2, "3");
    test("x^3+2*x^2-x+5", "4*x^2+x-2", 1, "3");
    // A length past both degrees is the whole result.
    test("x^3+2*x^2-x+5", "4*x^2+x-2", 4, "x^3+6*x^2+3");
    test("x^3+2*x^2-x+5", "4*x^2+x-2", 100, "x^3+6*x^2+3");
    // Everything cancels.
    test("-x^3+x", "x^3-x", 4, "0");
    // The longer operand is cut below its leading coefficient.
    test("x^5+x", "x^4+x^3", 4, "x^3+x");
    // Coefficients of two limbs.
    test(
        "1000000000000000000000*x^2-1",
        "-1000000000000000000000*x^2+3*x",
        3,
        "3*x-1",
    );
}

#[test]
fn add_truncated_properties() {
    integer_polynomial_integer_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, len)| {
            let r = (&p).add_truncated(&q, len);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).add_truncated(q.clone(), len), r);
            assert_eq!(p.clone().add_truncated(&q, len), r);
            assert_eq!(p.clone().add_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.add_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.add_truncated_assign(q.clone(), len);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // This is the truncation of the whole sum, and the sum of the truncations.
            assert_eq!(add_truncated_naive(&p, &q, len), r);
            assert_eq!(p.truncate(len) + q.truncate(len), r);
            // It agrees with the whole sum below the cut, and has nothing above it.
            assert!(r.eq_truncated(&(&p + &q), len));
            assert!(r.len() <= len);
            // Addition is commutative.
            assert_eq!((&q).add_truncated(&p, len), r);
            // Adding the negation gives zero.
            assert_eq!((&p).add_truncated(-&p, len), IntegerPolynomial::ZERO);
            // This is subtracting the negation.
            assert_eq!((&p).sub_truncated(-&q, len), r);
        },
    );

    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        // A zero length keeps nothing, and a length past both degrees keeps everything.
        assert_eq!((&p).add_truncated(&q, 0), IntegerPolynomial::ZERO);
        let len = p.len().max(q.len());
        assert_eq!((&p).add_truncated(&q, len), &p + &q);
        assert_eq!((&p).add_truncated(&q, u64::MAX), &p + &q);
    });
}
