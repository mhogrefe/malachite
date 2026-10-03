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
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_unsigned_pair_gen_var_5,
    integer_polynomial_unsigned_unsigned_triple_gen_var_2,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::generated_coefficients;
use malachite_nz::test_util::integer_polynomial::arithmetic::pow_truncated::pow_truncated_naive;

#[test]
fn test_pow_truncated() {
    let test = |s, e: u64, len: u64, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
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
    test("x+1", 3, 0, "0");
    // - e == 0
    test("x+1", 0, 5, "1");
    test("0", 0, 3, "1");
    // - the polynomial is zero
    test("0", 3, 2, "0");
    // - the polynomial is zero modulo x^n
    test("x^2+x", 3, 1, "0");
    // - the polynomial is a constant
    test("-2", 3, 2, "-8");
    // - only the constant term of q^e is kept
    test("3*x^2-x+2", 3, 1, "8");
    // - e == 1
    test("x^2+x+1", 1, 2, "x+1");
    // - e == 2
    test("x^2+x+1", 2, 3, "3*x^2+2*x+1");
    // - e >= 3, low == 0
    test("x+1", 5, 3, "10*x^2+5*x+1");
    test("x^2+x+1", 5, 4, "30*x^3+15*x^2+5*x+1");
    test("2*x-3", 6, 2, "-2916*x+729");
    // - low != 0, e * low < n
    test("x^2+x", 4, 5, "x^4");
    test("x^3+x^2", 3, 8, "3*x^7+x^6");
    // - e * low >= n
    test("x^2+x", 4, 4, "0");
    test(
        "4*x^3+3*x^2+2*x+1",
        7,
        6,
        "8106*x^5+2345*x^4+560*x^3+105*x^2+14*x+1",
    );
    test(
        "3*x^2-7*x+5",
        9,
        10,
        "-5956716997*x^9+6960155895*x^8-6646399200*x^7+5148795750*x^6-3192288750*x^5+\
        1549518750*x^4-568312500*x^3+148359375*x^2-24609375*x+1953125",
    );
    // - len beyond the untruncated length
    test("x+1", 3, u64::MAX, "x^3+3*x^2+3*x+1");
    // Generated coefficients of `bits` bits, compared with the naive truncated power.
    let test_generated = |len: usize, bits: u64, e: u64, n: u64| {
        let p = IntegerPolynomial::from_coefficients_asc(generated_coefficients(len, bits));
        assert_eq!((&p).pow_truncated(e, n), pow_truncated_naive(&p, e, n));
    };
    test_generated(3, 64 * 60, 5, 7);
    test_generated(200, 10, 5, 300);
}

#[test]
fn pow_truncated_properties() {
    integer_polynomial_unsigned_unsigned_triple_gen_var_2().test_properties(|(p, e, len)| {
        let power = p.clone().pow_truncated(e, len);
        assert!(power.is_valid());
        assert_eq!((&p).pow_truncated(e, len), power);
        let mut q = p.clone();
        q.pow_truncated_assign(e, len);
        assert_eq!(q, power);
        assert_eq!(pow_truncated_naive(&p, e, len), power);
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

    integer_polynomial_unsigned_pair_gen_var_5().test_properties(|(p, e)| {
        // with no truncation, the full power
        let power = (&p).pow(e);
        assert_eq!((&p).pow_truncated(e, power.len()), power);
        assert_eq!((&p).pow_truncated(e, u64::MAX), power);
    });
}
