// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Product,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;

#[test]
fn test_mod_power_of_2_product() {
    let test = |ss: &[&str], pow, out| {
        let ps: Vec<NaturalPolynomial> = ss
            .iter()
            .map(|s| NaturalPolynomial::from_str(s).unwrap())
            .collect();
        let product = NaturalPolynomial::mod_power_of_2_product(ps.iter(), pow);
        assert_eq!(product.to_string(), out);
        assert_eq!(
            NaturalPolynomial::mod_power_of_2_product(ps.into_iter(), pow),
            product
        );
    };
    test(&[], 3, "1");
    test(&[], 0, "0");
    test(
        &["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"],
        3,
        "7*x^6+4*x^5+6*x^4+2*x^3+3*x^2+2*x",
    );
    // Two nonzero polynomials multiply to zero, and the factor after them is not examined.
    test(&["2*x", "4", "9*x"], 3, "0");
    test(&["1125899906842624*x", "1125899906842624"], 100, "0");
    test(
        &["1152921504606846976*x+1", "1152921504606846976*x+3"],
        100,
        "4611686018427387904*x+3",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_product_fail_1() {
    let p = NaturalPolynomial::from_str("8*x+1").unwrap();
    NaturalPolynomial::mod_power_of_2_product([&p].into_iter(), 3);
}

#[test]
fn mod_power_of_2_product_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            let product = NaturalPolynomial::mod_power_of_2_product([&p, &q].into_iter(), pow);
            assert!(product.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_product([p.clone(), q.clone()].into_iter(), pow),
                product
            );
            // It is `mod_power_of_2_add`, in either order, and more factors keep multiplying.
            assert_eq!((&p).mod_power_of_2_mul(&q, pow), product);
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_product([&q, &p].into_iter(), pow),
                product
            );
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_product([&p, &q, &p].into_iter(), pow),
                (&product).mod_power_of_2_mul(&p, pow)
            );
            // The product of one polynomial is that polynomial, and the product of none is the
            // constant 1, reduced.
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_product([&p].into_iter(), pow),
                p
            );
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_product(
                    core::iter::empty::<NaturalPolynomial>(),
                    pow
                ),
                if pow == 0 {
                    NaturalPolynomial::ZERO
                } else {
                    NaturalPolynomial::one()
                }
            );
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<Limb>().test_properties(
        |(p, q, pow)| {
            // It agrees with the UnsignedPolynomial implementation.
            let product = UnsignedPolynomial::mod_power_of_2_product([&p, &q].into_iter(), pow);
            assert_eq!(
                NaturalPolynomial::mod_power_of_2_product(
                    [NaturalPolynomial::from(p), NaturalPolynomial::from(q)].into_iter(),
                    pow
                ),
                NaturalPolynomial::from(product)
            );
        },
    );
}
