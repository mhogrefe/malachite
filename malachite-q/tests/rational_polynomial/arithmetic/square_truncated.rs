// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    MulTruncated, Polynomial, SquareTruncated, SquareTruncatedAssign,
};
use malachite_nz::test_util::generators::integer_polynomial_unsigned_pair_gen_var_1;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_unsigned_pair_gen_var_1,
};
use malachite_q::test_util::rational_polynomial::arithmetic::square_truncated::*;

#[test]
fn test_square_truncated() {
    let test = |s, len, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let r = (&p).square_truncated(len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().square_truncated(len), r);
        let mut s = p.clone();
        s.square_truncated_assign(len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(square_truncated_naive(&p, len), r);
    };
    test("0", 3, "0");
    test("x+1", 0, "0");
    test("x-3", 2, "-6*x+9");
    test("1/2*x+1/3", 1, "1/9");
    test("1/2*x+1/3", 2, "1/3*x+1/9");
    // A length past the end of the square keeps all of it.
    test("1/2*x+1/3", 5, "1/4*x^2+1/3*x+1/9");
    // Cutting the square leaves its numerator sharing a factor with the denominator.
    test("1/2*x^2+x+1", 3, "2*x^2+2*x+1");
    test("1/2*x^2+x+1", 2, "2*x+1");
    test("1/2*x^2+1", 2, "1");
}

#[test]
fn square_truncated_properties() {
    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        let r = (&p).square_truncated(len);
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().square_truncated(len), r);
        let mut s = p.clone();
        s.square_truncated_assign(len);
        assert!(s.is_valid());
        assert_eq!(s, r);

        // It is the truncation of the whole square, the truncated product with itself, and only the
        // first `len` coefficients matter.
        assert_eq!(square_truncated_naive(&p, len), r);
        assert_eq!((&p).square().truncate(len), r);
        assert_eq!((&p).mul_truncated(p.clone(), len), r);
        assert_eq!(p.truncate(len).square_truncated(len), r);
        assert!(r.len() <= len);
    });

    rational_polynomial_gen().test_properties(|p| {
        // A zero length keeps nothing, and a length past the square keeps everything.
        assert_eq!((&p).square_truncated(0), RationalPolynomial::ZERO);
        assert_eq!((&p).square_truncated(p.len() << 1), (&p).square());
        assert_eq!((&p).square_truncated(u64::MAX), (&p).square());
    });

    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, len)| {
        // On polynomials with integer coefficients, this is the `IntegerPolynomial` operation.
        assert_eq!(
            RationalPolynomial::from(p.clone()).square_truncated(len),
            RationalPolynomial::from(p.square_truncated(len))
        );
    });
}
