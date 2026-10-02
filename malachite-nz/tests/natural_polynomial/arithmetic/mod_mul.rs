// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, Mod, ModIsReduced, ModMul, ModMulAssign, ModPowerOf2Mul,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_mul::{mod_mul_full, mod_mul_word};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::{
    mod_mul_naive, natural_mod_generated_coefficients, natural_test_moduli,
};

// Checks every form of `mod_mul` and `mod_mul_assign` against `r`.
fn check_forms(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural, r: &NaturalPolynomial) {
    assert_eq!(&p.clone().mod_mul(q.clone(), m.clone()), r);
    assert_eq!(&p.clone().mod_mul(q.clone(), m), r);
    assert_eq!(&p.clone().mod_mul(q, m.clone()), r);
    assert_eq!(&p.clone().mod_mul(q, m), r);
    assert_eq!(&p.mod_mul(q.clone(), m.clone()), r);
    assert_eq!(&p.mod_mul(q.clone(), m), r);
    assert_eq!(&p.mod_mul(q, m.clone()), r);
    assert_eq!(&p.mod_mul(q, m), r);
    let mut s = p.clone();
    s.mod_mul_assign(q.clone(), m.clone());
    assert!(s.is_valid());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_assign(q.clone(), m);
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_assign(q, m.clone());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_assign(q, m);
    assert_eq!(&s, r);
}

#[test]
fn test_mod_mul() {
    let test = |s, t, m: u32, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let m = Natural::from(m);
        let r = (&p).mod_mul(&q, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        check_forms(&p, &q, &m, &r);
        assert_eq!(mod_mul_naive(&p, &q, &m), r);
    };
    test("0", "x+1", 7, "0");
    test("x^2+3*x+2", "2*x+5", 7, "2*x^3+4*x^2+5*x+3");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "4", 7, "4*x^2+5*x+1");
    // Modulo 6, the leading coefficient vanishes and the degree drops.
    test("2*x+1", "3*x+1", 6, "5*x+1");
    // Every coefficient vanishes.
    test("2*x+2", "3", 6, "0");
    test("0", "0", 1, "0");
}

#[test]
#[should_panic]
fn mod_mul_fail_1() {
    NaturalPolynomial::from_str("x+7").unwrap().mod_mul(
        NaturalPolynomial::from_str("x+1").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_fail_2() {
    NaturalPolynomial::from_str("x+1").unwrap().mod_mul(
        NaturalPolynomial::from_str("7*x+1").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_mul_fail_3() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_mul(NaturalPolynomial::from_str("x+1").unwrap(), Natural::ZERO);
}

#[test]
fn mod_mul_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let r = (&p).mod_mul(&q, &m);
            assert!(r.is_valid());
            assert!(r.mod_is_reduced(&m));
            check_forms(&p, &q, &m, &r);
            assert_eq!(mod_mul_naive(&p, &q, &m), r);
            assert_eq!((&p * &q).mod_op(&m), r);
            // Multiplication is commutative.
            assert_eq!((&q).mod_mul(&p, &m), r);
            // Modulo a power of 2, it agrees with `mod_power_of_2_mul`.
            if let Some(pow) = (&m).checked_log_base_2() {
                assert_eq!((&p).mod_power_of_2_mul(&q, pow), r);
            }
        },
    );
}

// The word kernels agree with the full product, for factors of various lengths, including some past
// the Karatsuba threshold, and moduli of various sizes up to a limb.
#[test]
fn test_mod_mul_word_algorithms() {
    for &(n, k) in &[(1, 1), (3, 2), (8, 8), (9, 8), (20, 7), (90, 85), (200, 81)] {
        for m in natural_test_moduli() {
            if m.significant_bits() > Limb::WIDTH {
                continue;
            }
            let xs = natural_mod_generated_coefficients(n, &m);
            let ys = natural_mod_generated_coefficients(k, &m);
            let expected = mod_mul_full(&xs, &ys, &m);
            let to_naturals =
                |out: Vec<Limb>| -> Vec<Natural> { out.into_iter().map(Natural::from).collect() };
            assert_eq!(to_naturals(mod_mul_word(&xs, &ys, &m)), expected);
            assert_eq!(to_naturals(mod_mul_word(&ys, &xs, &m)), expected);
        }
    }
}

#[test]
fn mod_mul_word_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() || m.significant_bits() > Limb::WIDTH {
                return;
            }
            assert_eq!(
                NaturalPolynomial::from_coefficients_asc(
                    mod_mul_word(xs, ys, &m)
                        .into_iter()
                        .map(Natural::from)
                        .collect()
                ),
                (&p).mod_mul(&q, &m)
            );
        },
    );
}

// The dispatcher's choice between the word kernels and the full product.
#[test]
fn test_mod_mul_dispatch() {
    let test = |len1: usize, len2: usize, m: Natural| {
        let p =
            NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(len1, &m));
        let q =
            NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(len2, &m));
        let r = (&p).mod_mul(&q, &m);
        assert_eq!(mod_mul_naive(&p, &q, &m), r);
        check_forms(&p, &q, &m, &r);
    };
    // - word kernels
    test(10, 9, Natural::from(Limb::MAX >> 1));
    // - full product, too long for the window
    test(3000, 2100, Natural::from(Limb::MAX >> 1));
    // - full product, a modulus too large for a limb
    test(10, 9, Natural::from(Limb::MAX) * Natural::from(3u32));
    // - full product, a constant factor
    test(10, 1, Natural::from(Limb::MAX >> 1));
}

// The dispatcher at the edges of the word kernels' windows, against the full product, which shares
// nothing with the word kernels. Each length is the largest in a window, or one past it, for the
// largest modulus the window covers. The edges are those measured in 2026-10; if they move, these
// still check agreement, only less sharply.
#[test]
fn test_mod_mul_window_edges() {
    for &(bits, edge) in &[(26, 80), (31, 96), (40, 192), (60, 320), (64, 160)] {
        let m = (Natural::ONE << bits) - Natural::from(59u32);
        for n in [edge, edge + 1] {
            let xs = natural_mod_generated_coefficients(n, &m);
            let mut ys = natural_mod_generated_coefficients(n, &m);
            ys.reverse();
            let p = NaturalPolynomial::from_coefficients_asc(xs.clone());
            let q = NaturalPolynomial::from_coefficients_asc(ys.clone());
            assert_eq!(
                (&p).mod_mul(&q, &m),
                NaturalPolynomial::from_coefficients_asc(mod_mul_full(&xs, &ys, &m))
            );
        }
    }
}
