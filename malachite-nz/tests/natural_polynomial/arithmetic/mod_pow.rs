// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Mod, ModIsReduced, ModMul, ModPow, ModPowAssign, ModPowerOf2Pow, Pow, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_pow::*;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_natural_triple_gen_var_3;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_pow::mod_pow_naive;
use std::panic::catch_unwind;

// Checks both kernels against `out` when they apply: a polynomial of length at least 2 with a
// nonzero constant term, an exponent of at least 3, and a modulus greater than 1.
fn verify_kernels(xs: &[Natural], e: u64, m: &Natural, out: &[Natural]) {
    if xs.len() >= 2 && xs[0] != 0u32 && e >= 3 && *m > 1u32 {
        assert_eq!(mod_pow_binexp(xs, e, m), out);
        assert_eq!(mod_pow_exact(xs, e, m), out);
    }
}

fn verify(p: &NaturalPolynomial, e: u64, m: &Natural, power: &NaturalPolynomial) {
    assert!(power.is_valid());
    assert_eq!(p.clone().mod_pow(e, m.clone()), *power);
    assert_eq!(p.clone().mod_pow(e, m), *power);
    assert_eq!(p.mod_pow(e, m.clone()), *power);
    assert_eq!(p.mod_pow(e, m), *power);
    let mut q = p.clone();
    q.mod_pow_assign(e, m.clone());
    assert_eq!(q, *power);
    let mut q = p.clone();
    q.mod_pow_assign(e, m);
    assert_eq!(q, *power);
    assert_eq!(mod_pow_naive(p, e, m), *power);
    verify_kernels(p.coefficients_asc(), e, m, power.coefficients_asc());
}

#[test]
fn test_mod_pow() {
    let test = |s, e: u64, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let power = (&p).mod_pow(e, &m);
        assert_eq!(power.to_string(), out);
        verify(&p, e, &m, &power);
    };
    // - m == 1
    test("0", 3, "1", "0");
    test("0", 0, "1", "0");
    // - e == 0
    test("x+1", 0, "7", "1");
    test("0", 0, "7", "1");
    // - the polynomial is zero
    test("0", 3, "7", "0");
    // - the polynomial is a constant
    test("3", 5, "7", "5");
    test("2", 3, "8", "0");
    // - the polynomial is a monomial
    test("3*x^2", 3, "7", "6*x^6");
    test("2*x^2", 3, "8", "0");
    // - e == 1
    test("x^2+x", 1, "6", "x^2+x");
    // - e == 2
    test("2*x+1", 2, "4", "1");
    // - the power needs no reduction
    test("x+1", 3, "1000", "x^3+3*x^2+3*x+1");
    test(
        "5*x^2+3*x+7",
        13,
        "1000000000000000000000000000057",
        "1220703125*x^26+9521484375*x^25+56494140625*x^24+235371093750*x^23+827607421875*x^22\
        +2409605859375*x^21+6176069453125*x^20+13825986281250*x^19+27837936912500*x^18\
        +50142782868750*x^17+82185868908000*x^16+121939345119150*x^15+165607556473515*x^14\
        +204622367798943*x^13+231850579062921*x^12+239001116433534*x^11+225518024283552*x^10\
        +192628514668590*x^9+149719105820284*x^8+104103261440178*x^7+65104112200127*x^6\
        +35560699565391*x^5+17099235662145*x^4+6808218451398*x^3+2287767041651*x^2\
        +539810200839*x+96889010407",
    );
    // - binary exponentiation modulo m
    test("x+1", 5, "7", "x^5+5*x^4+3*x^3+3*x^2+5*x+1");
    test(
        "3*x^2+5*x+4",
        9,
        "7",
        "6*x^18+6*x^17+x^15+6*x^14+3*x^11+3*x^10+4*x^8+3*x^7+x^4+x^3+6*x+1",
    );
    // - leading coefficients vanish modulo m
    test(
        "2*x^2+x+1",
        7,
        "6",
        "2*x^14+4*x^13+4*x^12+2*x^11+4*x^10+4*x^9+4*x^8+5*x^7+5*x^6+x^5+5*x^4+5*x^3+5*x^2+x+1",
    );
    // - the power vanishes after a square
    test("2*x+2", 3, "4", "0");
    // - the power vanishes after a multiplication
    test("2*x+2", 3, "8", "0");
    // - a factor of x^2 removed first
    test("x^3+x^2", 5, "6", "x^15+5*x^14+4*x^13+4*x^12+5*x^11+x^10");
    // Generated coefficients, compared with the naive power, for moduli of several sizes.
    for m in natural_test_moduli() {
        for (len, e) in [(2, 9), (3, 13), (8, 7), (30, 5)] {
            let p = NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(
                len, &m,
            ));
            let power = (&p).mod_pow(e, &m);
            verify(&p, e, &m, &power);
        }
    }
}

#[test]
fn mod_pow_fail() {
    let p = NaturalPolynomial::from_str("x+4").unwrap();
    assert_panic!((&p).mod_pow(3, Natural::from(3u32)));
    assert_panic!((&p).mod_pow(3, Natural::ZERO));
    assert_panic!(p.clone().mod_pow(3, &Natural::from(3u32)));
    assert_panic!({
        let mut q = p.clone();
        q.mod_pow_assign(3, Natural::from(3u32));
    });
}

#[test]
fn mod_pow_properties() {
    natural_polynomial_unsigned_natural_triple_gen_var_3().test_properties(|(p, e, m)| {
        let power = (&p).mod_pow(e, &m);
        assert!(power.mod_is_reduced(&m));
        verify(&p, e, &m, &power);
        assert_eq!((&p).pow(e).mod_op(&m), power);
        let half = e >> 1;
        assert_eq!(
            (&p).mod_pow(half, &m)
                .mod_mul((&p).mod_pow(e - half, &m), &m),
            power
        );
        // agreement with powering modulo the power of 2 with as many bits as m
        let k = m.significant_bits();
        assert_eq!(
            (&p).mod_pow(e, Natural::power_of_2(k)),
            (&p).mod_power_of_2_pow(e, k)
        );
    });
}
