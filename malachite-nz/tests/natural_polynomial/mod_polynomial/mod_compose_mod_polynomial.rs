// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModComposeModPolynomial, ModComposeModPolynomialAssign, ModComposeModPolynomialPrecomputed,
};
use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use std::str::FromStr;

#[test]
fn test_mod_compose_mod_polynomial() {
    let test = |f, g, h, m, out| {
        let f = NaturalPolynomial::from_str(f).unwrap();
        let g = NaturalPolynomial::from_str(g).unwrap();
        let h = NaturalPolynomial::from_str(h).unwrap();
        let m = Natural::from_str(m).unwrap();
        let r = (&f).mod_compose_mod_polynomial(&g, &h, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(
            f.clone()
                .mod_compose_mod_polynomial(g.clone(), h.clone(), m.clone()),
            r
        );
        let mut f_alt = f.clone();
        f_alt.mod_compose_mod_polynomial_assign(&g, &h, &m);
        assert_eq!(f_alt, r);
        assert_eq!(mod_compose_mod_polynomial_naive(&f, &g, &h, &m), r);
        assert_eq!(mod_compose_mod_polynomial_horner(&f, &g, &h, &m), r);
        assert_eq!(mod_compose_mod_polynomial_brent_kung(&f, &g, &h, &m), r);
        let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed(&g, &h, &m, &data),
            r
        );
        let powers =
            NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed_powers(&h, &m, &powers),
            r
        );
    };
    // - m == 1
    test("0", "0", "0", "1", "0");
    // - h constant
    test("3*x^2+2*x+1", "0", "3", "7", "0");
    // - f constant
    test("5", "x", "x^2+1", "7", "5");
    // - h linear
    test("7*x^2+6*x+5", "3", "x+4", "101", "86");
    // - Horner, word-sized modulus
    test("3*x^2+2*x+1", "x+1", "x^3+2*x^2+1", "7", "3*x^2+x+6");
    // - Horner, large modulus
    test(
        "3*x^2+2*x+1",
        "x+1",
        "x^3+2*x^2+1",
        "1000000000000000000000000000057",
        "3*x^2+8*x+6",
    );
    // - f at least as long as h
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+2",
        "x^3+2*x^2+1",
        "1000000000000000000000000000057",
        "2175*x^2+999999999999999999999999999634*x+1039",
    );
    // - Brent–Kung
    test(
        "8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "5*x^4+6*x^3+7*x^2+8*x+9",
        "x^8+6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        "1000000000000000000000000000057",
        "999999997462859869090654995247*x^7+999999999735058928734682150646*x^6+\
        999999996487699672149227666507*x^5+999999997872768532182373808794*x^4+\
        999999999855551827658545470019*x^3+999999998314464890604227568936*x^2+\
        999999999788637987369334827149*x+999999998708960448953521043234",
    );
}

// Deterministic polynomials long enough for the rectangular evaluation of the Brent–Kung blocks,
// checked against the naive composition.
#[test]
fn test_mod_compose_mod_polynomial_long() {
    let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    let coefficients = |len: u64, seed: u64| {
        NaturalPolynomial::from_coefficients_asc(
            (0..len)
                .map(|i| Natural::from(i * i * seed + 3 * i + seed).pow(3) % &m)
                .collect(),
        )
    };
    // - rectangular evaluation (deg h = 120)
    let mut h = coefficients(120, 7).into_coefficients_asc();
    h.push(Natural::ONE);
    let h = NaturalPolynomial::from_coefficients_asc(h);
    let f = coefficients(117, 11);
    let g = coefficients(119, 13);
    let r = (&f).mod_compose_mod_polynomial(&g, &h, &m);
    assert_eq!(r, mod_compose_mod_polynomial_naive(&f, &g, &h, &m));
    let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    assert_eq!(
        (&f).mod_compose_mod_polynomial_precomputed(&g, &h, &m, &data),
        r
    );
}

#[test]
#[should_panic]
fn mod_compose_mod_polynomial_fail() {
    // g not reduced mod h
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_compose_mod_polynomial(
            NaturalPolynomial::from_str("x^2").unwrap(),
            NaturalPolynomial::from_str("x^2+1").unwrap(),
            Natural::from(7u32),
        );
}

#[test]
fn mod_compose_mod_polynomial_properties() {
    let test = |(f, g, h, m): (
        NaturalPolynomial,
        NaturalPolynomial,
        NaturalPolynomial,
        Natural,
    )| {
        let r = (&f).mod_compose_mod_polynomial(&g, &h, &m);
        assert!(r.is_valid());
        assert_eq!(r, mod_compose_mod_polynomial_naive(&f, &g, &h, &m));
        assert_eq!(mod_compose_mod_polynomial_horner(&f, &g, &h, &m), r);
        assert_eq!(mod_compose_mod_polynomial_brent_kung(&f, &g, &h, &m), r);
        let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
        assert_eq!(
            (&f).mod_compose_mod_polynomial_precomputed(&g, &h, &m, &data),
            r
        );
        let powers =
            NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
        assert_eq!(
            f.mod_compose_mod_polynomial_precomputed_powers(h, m, &powers),
            r
        );
    };
    natural_polynomial_triple_natural_quadruple_gen_var_2().test_properties(test);
    // Long enough for Brent–Kung
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    natural_polynomial_triple_natural_quadruple_gen_var_2()
        .test_properties_with_config(&config, test);
}
