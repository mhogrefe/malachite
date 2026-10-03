// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::polynomial::{
    MulTruncated, Polynomial, PowTruncated, PowTruncatedAssign, SquareTruncated,
};
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_unsigned_pair_gen_var_1,
    rational_polynomial_unsigned_unsigned_triple_gen_var_1,
};
use malachite_q::test_util::rational_polynomial::arithmetic::pow_truncated::pow_truncated_naive;

#[test]
fn test_pow_truncated() {
    let test = |s, e: u64, len: u64, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let power = p.clone().pow_truncated(e, len);
        assert!(power.is_valid());
        assert_eq!(power.to_string(), out);
        assert_eq!((&p).pow_truncated(e, len).to_string(), out);
        let mut q = p.clone();
        q.pow_truncated_assign(e, len);
        assert_eq!(q.to_string(), out);
        assert_eq!(pow_truncated_naive(&p, e, len).to_string(), out);
    };
    // - len == 0
    test("1/2*x+1", 3, 0, "0");
    // - e == 0
    test("1/2*x+1", 0, 5, "1");
    test("0", 0, 3, "1");
    // - the polynomial is zero
    test("0", 3, 2, "0");
    // - the polynomial is zero modulo x^n
    test("1/2*x^2+x", 3, 1, "0");
    // - the polynomial is a constant
    test("-2/3", 3, 2, "-8/27");
    // - e == 1
    test("1/2*x^2+x+1", 1, 2, "x+1");
    // - e == 2
    test("1/2*x^2+x+1", 2, 3, "2*x^2+2*x+1");
    // - e >= 3
    test("1/2*x+1/3", 3, 2, "1/6*x+1/27");
    test("1/3*x^2+1/2*x+1", 5, 4, "55/12*x^3+25/6*x^2+5/2*x+1");
    // - truncation leaves no denominator
    test("1/2*x^2+x+1", 3, 2, "3*x+1");
    // - a factor of x
    test("1/2*x^2+1/3*x", 3, 4, "1/27*x^3");
    test("1/2*x^2+1/3*x", 3, 3, "0");
    // - len beyond the untruncated length
    test(
        "1/3*x+1/2",
        5,
        u64::MAX,
        "1/243*x^5+5/162*x^4+5/54*x^3+5/36*x^2+5/48*x+1/32",
    );
}

#[test]
fn pow_truncated_properties() {
    rational_polynomial_unsigned_unsigned_triple_gen_var_1().test_properties(|(p, e, len)| {
        let power = p.clone().pow_truncated(e, len);
        assert!(power.is_valid());
        assert_eq!((&p).pow_truncated(e, len), power);
        let mut q = p.clone();
        q.pow_truncated_assign(e, len);
        assert_eq!(q, power);
        // The naive power is slow for large exponents; the integer kernels it would check are
        // cross-checked against it in malachite-nz.
        if e <= 6 {
            assert_eq!(pow_truncated_naive(&p, e, len), power);
        }
        assert_eq!((&p).pow(e).truncate(len), power);
        assert_eq!(p.truncate(len).pow_truncated(e, len), power);
        assert!(power.len() <= len);
        if e != 0 {
            assert_eq!((&p).pow_truncated(e - 1, len).mul_truncated(&p, len), power);
        }
        if e == 2 {
            assert_eq!((&p).square_truncated(len), power);
        }
    });

    rational_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, e)| {
        let power = (&p).pow(e);
        assert_eq!((&p).pow_truncated(e, power.len()), power);
        assert_eq!((&p).pow_truncated(e, u64::MAX), power);
    });
}
