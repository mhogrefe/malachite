// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Mod, ModIsReduced, ModPow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ModMulTruncated, ModPowTruncated, ModPowTruncatedAssign, ModSquareTruncated, Polynomial,
    PowTruncated,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_pow_truncated::mod_pow_truncated_binexp;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_pow_truncated::*;
use std::panic::catch_unwind;

fn verify(p: &NaturalPolynomial, e: u64, len: u64, m: &Natural, power: &NaturalPolynomial) {
    assert!(power.is_valid());
    assert_eq!(p.clone().mod_pow_truncated(e, len, m.clone()), *power);
    assert_eq!(p.clone().mod_pow_truncated(e, len, m), *power);
    assert_eq!(p.mod_pow_truncated(e, len, m.clone()), *power);
    assert_eq!(p.mod_pow_truncated(e, len, m), *power);
    let mut q = p.clone();
    q.mod_pow_truncated_assign(e, len, m.clone());
    assert_eq!(q, *power);
    let mut q = p.clone();
    q.mod_pow_truncated_assign(e, len, m);
    assert_eq!(q, *power);
    assert_eq!(mod_pow_truncated_naive(p, e, len, m), *power);
    let xs = p.coefficients_asc();
    if xs.len() >= 2 && xs[0] != 0u32 && e >= 3 && len >= 2 && *m > 1u32 {
        assert_eq!(
            mod_pow_truncated_binexp(xs, e, len, m),
            power.coefficients_asc()
        );
    }
}

#[test]
fn test_mod_pow_truncated() {
    let test = |s, e: u64, len: u64, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let power = (&p).mod_pow_truncated(e, len, &m);
        assert_eq!(power.to_string(), out);
        verify(&p, e, len, &m, &power);
    };
    // - len == 0
    test("x+1", 3, 0, "7", "0");
    // - m == 1
    test("0", 3, 4, "1", "0");
    // - e == 0
    test("x+1", 0, 5, "7", "1");
    test("0", 0, 3, "7", "1");
    // - the polynomial is zero
    test("0", 3, 2, "7", "0");
    // - the polynomial is zero modulo x^n
    test("x^2+x", 3, 1, "7", "0");
    // - the polynomial is a constant
    test("2", 3, 2, "8", "0");
    // - only the constant term of q^e is kept
    test("3*x^2+x+2", 3, 1, "7", "1");
    // - e == 1
    test("x^2+x+1", 1, 2, "7", "x+1");
    // - e == 2
    test("x^2+x+1", 2, 3, "7", "3*x^2+2*x+1");
    // - the power needs no reduction
    test("x+1", 3, 1000, "1000", "x^3+3*x^2+3*x+1");
    test(
        "5*x^2+3*x+7",
        13,
        20,
        "1000000000000000000000000000057",
        "13825986281250*x^19+27837936912500*x^18+50142782868750*x^17+82185868908000*x^16\
        +121939345119150*x^15+165607556473515*x^14+204622367798943*x^13+231850579062921*x^12\
        +239001116433534*x^11+225518024283552*x^10+192628514668590*x^9+149719105820284*x^8\
        +104103261440178*x^7+65104112200127*x^6+35560699565391*x^5+17099235662145*x^4\
        +6808218451398*x^3+2287767041651*x^2+539810200839*x+96889010407",
    );
    // - binary exponentiation modulo m
    test("x+1", 5, 3, "7", "3*x^2+5*x+1");
    test("3*x^2+5*x+4", 9, 10, "7", "4*x^8+3*x^7+x^4+x^3+6*x+1");
    // - leading coefficients vanish modulo m
    test("2*x^2+x+1", 7, 6, "6", "x^5+5*x^4+5*x^3+5*x^2+x+1");
    // - the power vanishes after a square
    test("2*x+2", 3, 5, "4", "0");
    // - the power vanishes after a multiplication
    test("2*x+2", 3, 5, "8", "0");
    // - low != 0, e * low < n
    test("x^2+x", 4, 5, "7", "x^4");
    test("x^3+x^2", 3, 8, "6", "3*x^7+x^6");
    // - e * low >= n
    test("x^2+x", 4, 4, "7", "0");
    // Generated coefficients, compared with the naive truncated power, for moduli of several sizes.
    for m in natural_test_moduli() {
        for (len, e, n) in [(2, 9, 5), (3, 13, 20), (8, 7, 30), (30, 5, 40)] {
            let p = NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(
                len, &m,
            ));
            let power = (&p).mod_pow_truncated(e, n, &m);
            verify(&p, e, n, &m, &power);
        }
    }
}

#[test]
fn mod_pow_truncated_fail() {
    let p = NaturalPolynomial::from_str("x+4").unwrap();
    assert_panic!((&p).mod_pow_truncated(3, 2, Natural::from(3u32)));
    assert_panic!((&p).mod_pow_truncated(3, 2, Natural::ZERO));
    assert_panic!({
        let mut q = p.clone();
        q.mod_pow_truncated_assign(3, 2, Natural::from(3u32));
    });
}

#[test]
fn mod_pow_truncated_properties() {
    natural_polynomial_unsigned_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(p, e, len, m)| {
            let power = (&p).mod_pow_truncated(e, len, &m);
            assert!(power.mod_is_reduced(&m));
            assert!(power.len() <= len);
            verify(&p, e, len, &m, &power);
            assert_eq!((&p).mod_pow(e, &m).truncate(len), power);
            assert_eq!((&p).pow_truncated(e, len).mod_op(&m), power);
            if e != 0 {
                assert_eq!(
                    (&p).mod_pow_truncated(e - 1, len, &m)
                        .mod_mul_truncated(&p, len, &m),
                    power
                );
            }
            if e == 2 {
                assert_eq!((&p).mod_square_truncated(len, &m), power);
            }
        },
    );
}
