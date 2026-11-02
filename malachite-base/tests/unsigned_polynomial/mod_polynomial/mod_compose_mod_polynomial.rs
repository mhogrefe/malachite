// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModComposeModPolynomial, ModComposeModPolynomialAssign, ModComposeModPolynomialPrecomputed,
};
use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::str::FromStr;

#[test]
fn test_mod_compose_mod_polynomial() {
    fn test<T: PrimitiveUnsigned>(f: &str, g: &str, h: &str, m: T, out: &str) {
        let f = UnsignedPolynomial::<T>::from_str(f).unwrap();
        let g = UnsignedPolynomial::<T>::from_str(g).unwrap();
        let h = UnsignedPolynomial::<T>::from_str(h).unwrap();
        let r = (&f).mod_compose_mod_polynomial(&g, &h, m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(
            f.clone()
                .mod_compose_mod_polynomial(g.clone(), h.clone(), m),
            r
        );
        let mut f_alt = f.clone();
        f_alt.mod_compose_mod_polynomial_assign(&g, &h, m);
        assert_eq!(f_alt, r);
        assert_eq!(mod_compose_mod_polynomial_naive(&f, &g, &h, m), r);
        assert_eq!(mod_compose_mod_polynomial_horner(&f, &g, &h, m), r);
        assert_eq!(mod_compose_mod_polynomial_brent_kung(&f, &g, &h, m), r);
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed(&g, &h, m, &data),
            r
        );
        let powers =
            UnsignedPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed_powers(&h, m, &powers),
            r
        );
    }
    // - m == 1
    test::<u8>("0", "0", "0", 1, "0");
    // - h constant
    test::<u8>("3*x^2+2*x+1", "0", "3", 7, "0");
    // - f zero
    test::<u8>("0", "x+1", "x^3+2*x^2+1", 7, "0");
    // - f constant
    test::<u8>("5", "x", "x^2+1", 7, "5");
    // - g zero
    test::<u8>("3*x^2+2*x+1", "0", "x^3+2*x^2+1", 7, "1");
    // - h linear
    test::<u8>("7*x^2+6*x+5", "3", "x+4", 101, "86");
    // - Horner, h short
    test::<u8>("3*x^2+2*x+1", "x+1", "x^3+2*x^2+1", 7, "3*x^2+x+6");
    // - f at least as long as h
    test::<u8>(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+2",
        "x^3+2*x^2+1",
        7,
        "5*x^2+4*x+3",
    );
    // - Brent–Kung
    test::<u8>(
        "8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "5*x^4+6*x^3+7*x^2+8*x+9",
        "x^8+6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        101,
        "49*x^7+98*x^6+25*x^5+93*x^4+92*x^3+59*x^2+6*x+88",
    );
}

// Deterministic polynomials long enough for the rectangular evaluation of the Brent–Kung blocks,
// checked against the naive composition.
#[test]
fn test_mod_compose_mod_polynomial_long() {
    fn coefficients(len: u64, seed: u64, m: u64) -> UnsignedPolynomial<u64> {
        UnsignedPolynomial::from_coefficients_asc(
            (0..len)
                .map(|i| (i * i * seed + 3 * i + seed) % m)
                .collect(),
        )
    }
    let m = 1000003u64;
    // - rectangular evaluation, with a nonempty last block (deg h = 120)
    // - rectangular evaluation, with an empty last block (deg h = 150)
    for n in [120, 150] {
        let mut h = coefficients(n, 7, m).into_coefficients_asc();
        h.push(1);
        let h = UnsignedPolynomial::from_coefficients_asc(h);
        let f = coefficients(n - 3, 11, m);
        let g = coefficients(n - 1, 13, m);
        let r = (&f).mod_compose_mod_polynomial(&g, &h, m);
        assert_eq!(r, mod_compose_mod_polynomial_naive(&f, &g, &h, m));
        assert_eq!(mod_compose_mod_polynomial_brent_kung(&f, &g, &h, m), r);
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed(&g, &h, m, &data),
            r
        );
        let powers =
            UnsignedPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed_powers(&h, m, &powers),
            r
        );
    }
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_fail_1() {
    // g not reduced mod h
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_compose_mod_polynomial(
            UnsignedPolynomial::from_str("x^2").unwrap(),
            UnsignedPolynomial::from_str("x^2+1").unwrap(),
            7,
        );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_fail_2() {
    // h zero
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_compose_mod_polynomial(
            UnsignedPolynomial::from_str("0").unwrap(),
            UnsignedPolynomial::from_str("0").unwrap(),
            7,
        );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_fail_3() {
    // leading coefficient of h not invertible
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_compose_mod_polynomial(
            UnsignedPolynomial::from_str("x").unwrap(),
            UnsignedPolynomial::from_str("2*x^2+1").unwrap(),
            4,
        );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_fail_4() {
    // coefficient not reduced mod m
    UnsignedPolynomial::<u8>::from_str("9*x+1")
        .unwrap()
        .mod_compose_mod_polynomial(
            UnsignedPolynomial::from_str("x").unwrap(),
            UnsignedPolynomial::from_str("x^2+1").unwrap(),
            7,
        );
}

fn mod_compose_mod_polynomial_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(f, g, h, m): (
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        T,
    )| {
        let r = (&f).mod_compose_mod_polynomial(&g, &h, m);
        assert!(r.is_valid());
        assert_eq!(r, mod_compose_mod_polynomial_naive(&f, &g, &h, m));
        assert_eq!(mod_compose_mod_polynomial_horner(&f, &g, &h, m), r);
        assert_eq!(mod_compose_mod_polynomial_brent_kung(&f, &g, &h, m), r);
        let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed(&g, &h, m, &data),
            r
        );
        let powers =
            UnsignedPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed_powers(&h, m, &powers),
            r
        );
    };
    unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>().test_properties(test);
    // Long enough for Brent–Kung
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_polynomial_triple_unsigned_quadruple_gen_var_3::<T>()
        .test_properties_with_config(&config, test);
}

#[test]
fn mod_compose_mod_polynomial_properties() {
    apply_fn_to_unsigneds!(mod_compose_mod_polynomial_properties_helper);
}
