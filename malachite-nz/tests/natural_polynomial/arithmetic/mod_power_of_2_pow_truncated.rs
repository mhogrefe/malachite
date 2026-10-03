// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Pow};
use malachite_base::polynomial::{
    ModPowerOf2MulTruncated, ModPowerOf2PowTruncated, ModPowerOf2PowTruncatedAssign,
    ModPowerOf2SquareTruncated, Polynomial, PowTruncated,
};
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_pow_truncated::*;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_pow_truncated() {
    let test = |s, e: u64, len: u64, pow: u64, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let power = p.clone().mod_power_of_2_pow_truncated(e, len, pow);
        assert!(power.is_valid());
        assert_eq!(power.to_string(), out);
        assert_eq!(
            (&p).mod_power_of_2_pow_truncated(e, len, pow).to_string(),
            out
        );
        let mut q = p.clone();
        q.mod_power_of_2_pow_truncated_assign(e, len, pow);
        assert_eq!(q.to_string(), out);
        assert_eq!(
            mod_power_of_2_pow_truncated_naive(&p, e, len, pow).to_string(),
            out
        );
    };
    // - len == 0
    test("x+1", 3, 0, 4, "0");
    // - pow == 0
    test("0", 0, 4, 0, "0");
    // - e == 0
    test("x+1", 0, 5, 4, "1");
    test("0", 0, 3, 4, "1");
    // - the polynomial is zero
    test("0", 3, 2, 4, "0");
    // - the polynomial is zero modulo x^n
    test("x^2+x", 3, 1, 4, "0");
    // - the polynomial is a constant
    test("2", 3, 2, 3, "0");
    // - only the constant term of q^e is kept
    test("3*x^2+x+2", 3, 1, 4, "8");
    // - e == 1
    test("x^2+x+1", 1, 2, 3, "x+1");
    // - e == 2
    test("x^2+x+1", 2, 3, 3, "3*x^2+2*x+1");
    // - the power needs no reduction
    test("x+1", 3, 1000, 10, "x^3+3*x^2+3*x+1");
    // - binary exponentiation modulo 2^k
    test("x+1", 5, 3, 3, "2*x^2+5*x+1");
    test(
        "3*x^2+7*x+5",
        9,
        10,
        4,
        "5*x^9+7*x^8+6*x^6+14*x^5+14*x^4+4*x^3+15*x^2+15*x+5",
    );
    test("2*x^2+x+1", 7, 6, 3, "x^5+x^4+7*x^3+3*x^2+7*x+1");
    test(
        "6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        11,
        20,
        5,
        "27*x^19+24*x^18+12*x^17+13*x^16+15*x^15+7*x^14+2*x^13+30*x^12+29*x^11+3*x^10+29*x^9\
        +16*x^8+6*x^7+10*x^6+18*x^5+19*x^4+16*x^3+17*x^2+3*x+27",
    );
    // - the power vanishes after a square
    test("2*x+2", 3, 5, 2, "0");
    // - the power vanishes after a multiplication
    test("2*x+2", 3, 5, 3, "0");
    // - low != 0, e * low < n
    test("x^2+x", 4, 5, 3, "x^4");
    test("x^3+x^2", 3, 8, 2, "3*x^7+x^6");
    // - e * low >= n
    test("x^2+x", 4, 4, 3, "0");
    // Generated coefficients, compared with the naive truncated power.
    let test_generated = |len: usize, pow: u64, e: u64, n: u64| {
        let p = NaturalPolynomial::from_coefficients_asc(
            natural_mod_power_of_2_generated_coefficients(len, pow),
        );
        assert_eq!(
            (&p).mod_power_of_2_pow_truncated(e, n, pow),
            mod_power_of_2_pow_truncated_naive(&p, e, n, pow)
        );
    };
    test_generated(8, 100, 13, 50);
    test_generated(30, 64, 7, 100);
    test_generated(3, 1000, 5, 7);
    test_generated(40, 10, 9, 20);
}

#[test]
fn mod_power_of_2_pow_truncated_fail() {
    let p = NaturalPolynomial::from_str("x+4").unwrap();
    assert_panic!(p.clone().mod_power_of_2_pow_truncated(3, 2, 2));
    assert_panic!((&p).mod_power_of_2_pow_truncated(3, 2, 2));
    assert_panic!({
        let mut q = p.clone();
        q.mod_power_of_2_pow_truncated_assign(3, 2, 2);
    });
}

#[test]
fn mod_power_of_2_pow_truncated_properties() {
    natural_polynomial_unsigned_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(p, e, len, pow)| {
            let power = p.clone().mod_power_of_2_pow_truncated(e, len, pow);
            assert!(power.is_valid());
            assert!(power.mod_power_of_2_is_reduced(pow));
            assert!(power.len() <= len);
            assert_eq!((&p).mod_power_of_2_pow_truncated(e, len, pow), power);
            let mut q = p.clone();
            q.mod_power_of_2_pow_truncated_assign(e, len, pow);
            assert_eq!(q, power);
            assert_eq!(mod_power_of_2_pow_truncated_naive(&p, e, len, pow), power);
            assert_eq!((&p).mod_power_of_2_pow(e, pow).truncate(len), power);
            assert_eq!((&p).pow_truncated(e, len).mod_power_of_2(pow), power);
            if e != 0 {
                assert_eq!(
                    (&p).mod_power_of_2_pow_truncated(e - 1, len, pow)
                        .mod_power_of_2_mul_truncated(&p, len, pow),
                    power
                );
            }
            if e == 2 {
                assert_eq!((&p).mod_power_of_2_square_truncated(len, pow), power);
            }
        },
    );
}
