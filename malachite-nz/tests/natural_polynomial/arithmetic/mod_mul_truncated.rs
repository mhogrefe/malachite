// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, Mod, ModIsReduced, ModMul};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{
    ModMulTruncated, ModMulTruncatedAssign, ModPowerOf2MulTruncated, MulTruncated, Polynomial,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_mul_truncated::{
    mod_mul_truncated_full, mod_mul_truncated_word,
};
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::{
    natural_mod_generated_coefficients, natural_test_moduli,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul_truncated::*;

// Checks every form of `mod_mul_truncated` and `mod_mul_truncated_assign` against `r`.
fn check_forms(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
    r: &NaturalPolynomial,
) {
    assert_eq!(&p.clone().mod_mul_truncated(q.clone(), len, m.clone()), r);
    assert_eq!(&p.clone().mod_mul_truncated(q.clone(), len, m), r);
    assert_eq!(&p.clone().mod_mul_truncated(q, len, m.clone()), r);
    assert_eq!(&p.clone().mod_mul_truncated(q, len, m), r);
    assert_eq!(&p.mod_mul_truncated(q.clone(), len, m.clone()), r);
    assert_eq!(&p.mod_mul_truncated(q.clone(), len, m), r);
    assert_eq!(&p.mod_mul_truncated(q, len, m.clone()), r);
    assert_eq!(&p.mod_mul_truncated(q, len, m), r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q.clone(), len, m.clone());
    assert!(s.is_valid());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q.clone(), len, m);
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q, len, m.clone());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_mul_truncated_assign(q, len, m);
    assert_eq!(&s, r);
}

#[test]
fn test_mod_mul_truncated() {
    let test = |s, t, len: u64, m: u32, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let m = Natural::from(m);
        let r = (&p).mod_mul_truncated(&q, len, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        check_forms(&p, &q, len, &m, &r);
        assert_eq!(mod_mul_truncated_naive(&p, &q, len, &m), r);
    };
    test("x^2+3*x+2", "2*x+5", 0, 7, "0");
    test("x^2+3*x+2", "2*x+5", 2, 7, "5*x+3");
    test("x^2+3*x+2", "2*x+5", 10, 7, "2*x^3+4*x^2+5*x+3");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "4", 2, 7, "5*x+1");
    // The linear coefficient vanishes, and is trimmed.
    test("x+6", "x+1", 2, 7, "6");
}

#[test]
#[should_panic]
fn mod_mul_truncated_fail_1() {
    NaturalPolynomial::from_str("x+7")
        .unwrap()
        .mod_mul_truncated(
            NaturalPolynomial::from_str("x+1").unwrap(),
            2,
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_mul_truncated_fail_2() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_mul_truncated(
            NaturalPolynomial::from_str("x+1").unwrap(),
            2,
            Natural::ZERO,
        );
}

#[test]
fn mod_mul_truncated_properties() {
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(p, q, len, m)| {
            let r = (&p).mod_mul_truncated(&q, len, &m);
            assert!(r.is_valid());
            assert!(r.mod_is_reduced(&m));
            check_forms(&p, &q, len, &m, &r);
            assert_eq!(mod_mul_truncated_naive(&p, &q, len, &m), r);
            assert_eq!((&p).mul_truncated(&q, len).mod_op(&m), r);
            assert_eq!((&p).mod_mul(&q, &m).truncate(len), r);
            assert_eq!((&q).mod_mul_truncated(&p, len, &m), r);
            if let Some(pow) = (&m).checked_log_base_2() {
                assert_eq!((&p).mod_power_of_2_mul_truncated(&q, len, pow), r);
            }
            assert!(r.len() <= len);
        },
    );
}

// The word kernels agree with the full truncated product, for factors and truncation lengths of
// various sizes, including some past the Karatsuba threshold, and moduli of various sizes up to a
// limb.
#[test]
fn test_mod_mul_truncated_word_algorithms() {
    for &(n, k, len) in &[
        (1, 1, 1),
        (3, 2, 2),
        (8, 8, 8),
        (9, 8, 16),
        (20, 7, 30),
        (90, 85, 90),
        (200, 81, 250),
        (200, 190, 199),
    ] {
        for m in natural_test_moduli() {
            if m.significant_bits() > Limb::WIDTH {
                continue;
            }
            let xs = natural_mod_generated_coefficients(n, &m);
            let ys = natural_mod_generated_coefficients(k, &m);
            // The full product's coefficients may be trimmed, so the results are compared as
            // polynomials.
            let expected =
                NaturalPolynomial::from_coefficients_asc(mod_mul_truncated_full(&xs, &ys, len, &m));
            let to_polynomial = |out: Vec<Limb>| {
                NaturalPolynomial::from_coefficients_asc(
                    out.into_iter().map(Natural::from).collect(),
                )
            };
            assert_eq!(
                to_polynomial(mod_mul_truncated_word(&xs, &ys, len, &m)),
                expected
            );
            assert_eq!(
                to_polynomial(mod_mul_truncated_word(&ys, &xs, len, &m)),
                expected
            );
        }
    }
}

#[test]
fn mod_mul_truncated_word_properties() {
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(p, q, len, m)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() || len == 0 || m.significant_bits() > Limb::WIDTH {
                return;
            }
            let out_len = usize::try_from(len).unwrap().min(xs.len() + ys.len() - 1);
            assert_eq!(
                NaturalPolynomial::from_coefficients_asc(
                    mod_mul_truncated_word(xs, ys, out_len, &m)
                        .into_iter()
                        .map(Natural::from)
                        .collect()
                ),
                (&p).mod_mul_truncated(&q, len, &m)
            );
        },
    );
}

// The dispatcher's choice between the word kernels and the full truncated product.
#[test]
fn test_mod_mul_truncated_dispatch() {
    let test = |len1: usize, len2: usize, len: u64, m: Natural| {
        let p =
            NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(len1, &m));
        let q =
            NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(len2, &m));
        let r = (&p).mod_mul_truncated(&q, len, &m);
        assert_eq!(mod_mul_truncated_naive(&p, &q, len, &m), r);
        check_forms(&p, &q, len, &m, &r);
    };
    // - word kernels
    test(10, 9, 12, Natural::from(Limb::MAX >> 1));
    // - word kernels, a length past the end of the product
    test(10, 9, 100, Natural::from(Limb::MAX >> 1));
    // - full product, too long for the window
    test(3000, 2100, 2500, Natural::from(Limb::MAX >> 1));
    // - full product, a modulus too large for a limb
    test(10, 9, 12, Natural::from(Limb::MAX) * Natural::from(3u32));
    // - full product, a constant factor
    test(10, 1, 12, Natural::from(Limb::MAX >> 1));
    // - full product, a length of zero
    test(10, 9, 0, Natural::from(Limb::MAX >> 1));
}

// The dispatcher at the edges of the word kernels' windows, against the full product, which shares
// nothing with the word kernels. Each length is the largest in a window, or one past it, for the
// largest modulus the window covers. The edges are those measured in 2026-10; if they move, these
// still check agreement, only less sharply.
#[test]
fn test_mod_mul_truncated_window_edges() {
    for &(bits, edge) in &[(24, 64), (40, 192), (60, 320), (64, 192)] {
        let m = (Natural::ONE << bits) - Natural::from(59u32);
        for n in [edge, edge + 1] {
            let xs = natural_mod_generated_coefficients(n, &m);
            let mut ys = natural_mod_generated_coefficients(n, &m);
            ys.reverse();
            let p = NaturalPolynomial::from_coefficients_asc(xs.clone());
            let q = NaturalPolynomial::from_coefficients_asc(ys.clone());
            assert_eq!(
                (&p).mod_mul_truncated(&q, u64::exact_from(n), &m),
                NaturalPolynomial::from_coefficients_asc(mod_mul_truncated_full(&xs, &ys, n, &m))
            );
        }
    }
}
