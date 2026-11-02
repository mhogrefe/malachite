// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::mod_polynomial::traits::*;
use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2_compose_mod_polynomial() {
    fn test<T: PrimitiveUnsigned>(f: &str, g: &str, h: &str, pow: u64, out: &str) {
        let f = UnsignedPolynomial::<T>::from_str(f).unwrap();
        let g = UnsignedPolynomial::<T>::from_str(g).unwrap();
        let h = UnsignedPolynomial::<T>::from_str(h).unwrap();
        let r = (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(
            f.clone()
                .mod_power_of_2_compose_mod_polynomial(g.clone(), h.clone(), pow),
            r
        );
        let mut f_alt = f.clone();
        f_alt.mod_power_of_2_compose_mod_polynomial_assign(&g, &h, pow);
        assert_eq!(f_alt, r);
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
        let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        assert_eq!(
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data),
            r
        );
        let powers =
            UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
                &g, &h, &pow,
            );
        assert_eq!(
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, pow, &powers),
            r
        );
    }
    // - pow == 0
    test::<u8>("0", "0", "0", 0, "0");
    // - h constant
    test::<u8>("3*x^2+2*x+1", "0", "3", 8, "0");
    // - f at least as long as h
    test::<u16>(
        "5*x^4+4*x^3+3*x^2+2*x+1",
        "x^2+2",
        "x^3+2*x^2+1",
        8,
        "127*x^2+89*x+15",
    );
    // - Brent–Kung
    test::<u16>(
        "8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "5*x^4+6*x^3+7*x^2+8*x+9",
        "x^8+6*x^7+2*x^6+9*x^5+5*x^4+x^3+4*x^2+x+3",
        8,
        "246*x^7+189*x^6+18*x^5+97*x^4+10*x^3+239*x^2+84*x+233",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_compose_mod_polynomial_fail() {
    // leading coefficient of h even
    UnsignedPolynomial::<u8>::from_str("x+1")
        .unwrap()
        .mod_power_of_2_compose_mod_polynomial(
            UnsignedPolynomial::from_str("x").unwrap(),
            UnsignedPolynomial::from_str("2*x^2+1").unwrap(),
            3,
        );
}

fn mod_power_of_2_compose_mod_polynomial_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(f, g, h, pow): (
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        UnsignedPolynomial<T>,
        u64,
    )| {
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
        let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&h, pow);
        assert_eq!(
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, pow, &data),
            r
        );
        let powers =
            UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
                &g, &h, &pow,
            );
        assert_eq!(
            (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, pow, &powers),
            r
        );
    };
    unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>().test_properties(test);
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_polynomial_triple_unsigned_quadruple_gen_var_4::<T>()
        .test_properties_with_config(&config, test);
}

#[test]
fn mod_power_of_2_compose_mod_polynomial_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_compose_mod_polynomial_properties_helper);
}
