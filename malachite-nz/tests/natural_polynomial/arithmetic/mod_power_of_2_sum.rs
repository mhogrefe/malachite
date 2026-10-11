// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Sum,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;

#[test]
fn test_mod_power_of_2_sum() {
    let test = |ss: &[&str], pow, out| {
        let ps: Vec<NaturalPolynomial> = ss
            .iter()
            .map(|s| NaturalPolynomial::from_str(s).unwrap())
            .collect();
        let sum = NaturalPolynomial::mod_power_of_2_sum(ps.iter(), pow);
        assert_eq!(sum.to_string(), out);
        assert_eq!(
            NaturalPolynomial::mod_power_of_2_sum(ps.into_iter(), pow),
            sum
        );
    };
    test(&[], 3, "0");
    test(&["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"], 3, "x^3+7*x");
    test(
        &["x+18446744073709551615", "18446744073709551615*x+1"],
        64,
        "0",
    );
    test(
        &["1267650600228229401496703205375", "633825300114114700748351602688*x+1"],
        100,
        "633825300114114700748351602688*x",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sum_fail_1() {
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    NaturalPolynomial::mod_power_of_2_sum([&p].into_iter(), 3);
}

#[test]
fn mod_power_of_2_sum_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            let sum = NaturalPolynomial::mod_power_of_2_sum([&p, &q].into_iter(), pow);
            assert!(sum.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_sum([p.clone(), q.clone()].into_iter(), pow),
                sum
            );
            // It is `mod_power_of_2_add`, in either order, and summing more terms keeps adding.
            assert_eq!((&p).mod_power_of_2_add(&q, pow), sum);
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_sum([&q, &p].into_iter(), pow),
                sum
            );
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_sum([&p, &q, &p].into_iter(), pow),
                (&sum).mod_power_of_2_add(&p, pow)
            );
            // The sum of one polynomial is that polynomial, and the sum of none is zero.
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_sum([&p].into_iter(), pow),
                p
            );
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_sum(
                    core::iter::empty::<NaturalPolynomial>(),
                    pow
                ),
                NaturalPolynomial::ZERO
            );
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<Limb>().test_properties(
        |(p, q, pow)| {
            // It agrees with the UnsignedPolynomial implementation.
            let sum = UnsignedPolynomial::mod_power_of_2_sum([&p, &q].into_iter(), pow);
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_sum(
                    [NaturalPolynomial::from(p), NaturalPolynomial::from(q)].into_iter(),
                    pow
                ),
                NaturalPolynomial::from(sum)
            );
        },
    );
}
