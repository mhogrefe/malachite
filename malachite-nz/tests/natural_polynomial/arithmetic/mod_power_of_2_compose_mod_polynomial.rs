// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::*;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use std::str::FromStr;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_compose_mod_polynomial::*;

#[test]
fn test_mod_power_of_2_compose_mod_polynomial() {
    let test = |f, g, h, pow, out| {
        let f = NaturalPolynomial::from_str(f).unwrap();
        let g = NaturalPolynomial::from_str(g).unwrap();
        let h = NaturalPolynomial::from_str(h).unwrap();
        let r = (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(
            mod_power_of_2_compose_mod_polynomial_naive(&f, &g, &h, pow),
            r
        );
        assert_eq!(
            mod_power_of_2_compose_mod_polynomial_horner(&f, &g, &h, pow),
            r
        );
        assert_eq!(
            mod_power_of_2_compose_mod_polynomial_brent_kung(&f, &g, &h, pow),
            r
        );
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        assert_eq!(
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data),
            r
        );
        let powers =
            NaturalPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
                &g, &h, &pow,
            );
        assert_eq!(
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, pow, &powers),
            r
        );
    };
    // - pow == 0
    test("0", "0", "0", 0, "0");
    test(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+2",
        "x^3+2*x^2+1",
        8,
        "127*x^2+89*x+15",
    );
    // - Brent–Kung, modulus larger than a word
    test(
        "8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "5*x^4+6*x^3+7*x^2+8*x+9",
        "x^8+6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        100,
        "1267650597691089270587358200566*x^7+1267650599963288330231385355965*x^6+\
        1267650596715929073645930871826*x^5+1267650598100997933679077014113*x^4+\
        1267650600083781229155248675338*x^3+1267650598542694292100930774255*x^2+\
        1267650600016867388866038032468*x+1267650598937189850450224248553",
    );
}

#[test]
fn mod_power_of_2_compose_mod_polynomial_properties() {
    let test = |(f, g, h, pow): (NaturalPolynomial, NaturalPolynomial, NaturalPolynomial, u64)| {
        let r = (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow);
        assert!(r.is_valid());
        assert_eq!(
            r,
            mod_power_of_2_compose_mod_polynomial_naive(&f, &g, &h, pow)
        );
        assert_eq!(
            mod_power_of_2_compose_mod_polynomial_horner(&f, &g, &h, pow),
            r
        );
        assert_eq!(
            mod_power_of_2_compose_mod_polynomial_brent_kung(&f, &g, &h, pow),
            r
        );
        let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        assert_eq!(
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data),
            r
        );
        let powers =
            NaturalPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
                &g, &h, &pow,
            );
        assert_eq!(
            f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, pow, &powers),
            r
        );
    };
    natural_polynomial_triple_unsigned_quadruple_gen_var_2().test_properties(test);
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    natural_polynomial_triple_unsigned_quadruple_gen_var_2()
        .test_properties_with_config(&config, test);
}
