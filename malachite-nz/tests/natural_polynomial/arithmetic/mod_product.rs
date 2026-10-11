// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModProduct};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;

#[test]
fn test_mod_product() {
    let test = |ss: &[&str], m, out| {
        let ps: Vec<NaturalPolynomial> = ss
            .iter()
            .map(|s| NaturalPolynomial::from_str(s).unwrap())
            .collect();
        let m = Natural::from_str(m).unwrap();
        let product = NaturalPolynomial::mod_product(ps.iter(), &m);
        assert_eq!(product.to_string(), out);
        assert_eq!(
            NaturalPolynomial::mod_product(ps.iter(), m.clone()),
            product
        );
        assert_eq!(NaturalPolynomial::mod_product(ps.into_iter(), &m), product);
    };
    test(&[], "7", "1");
    test(&[], "1", "0");
    test(
        &["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"],
        "9",
        "7*x^6+5*x^5+8*x^4+4*x^3+3*x^2",
    );
    // Two nonzero polynomials multiply to zero, and the factor after them is not examined.
    test(&["3*x", "3", "10*x"], "9", "0");
    test(
        &["x+1", "x+1000000000000000000000000000056"],
        "1000000000000000000000000000057",
        "x^2+1000000000000000000000000000056",
    );
}

#[test]
#[should_panic]
fn mod_product_fail_1() {
    let p = NaturalPolynomial::from_str("7*x+1").unwrap();
    NaturalPolynomial::mod_product([&p].into_iter(), &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_product_fail_2() {
    NaturalPolynomial::mod_product(core::iter::empty::<NaturalPolynomial>(), Natural::ZERO);
}

#[test]
fn mod_product_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let product = NaturalPolynomial::mod_product([&p, &q].into_iter(), &m);
            assert!(product.mod_is_reduced(&m));
            assert_eq!(
                NaturalPolynomial::mod_product([p.clone(), q.clone()].into_iter(), &m),
                product
            );
            // It is `mod_add`, in either order, and more factors keep multiplying.
            assert_eq!((&p).mod_mul(&q, &m), product);
            assert_eq!(
                NaturalPolynomial::mod_product([&q, &p].into_iter(), &m),
                product
            );
            assert_eq!(
                NaturalPolynomial::mod_product([&p, &q, &p].into_iter(), &m),
                (&product).mod_mul(&p, &m)
            );
            // The product of one polynomial is that polynomial, and the product of none is the
            // constant 1, reduced.
            assert_eq!(NaturalPolynomial::mod_product([&p].into_iter(), &m), p);
            assert_eq!(
                NaturalPolynomial::mod_product(core::iter::empty::<NaturalPolynomial>(), &m),
                if m == 1u32 {
                    NaturalPolynomial::ZERO
                } else {
                    NaturalPolynomial::one()
                }
            );
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<Limb>().test_properties(
        |(p, q, m)| {
            // It agrees with the UnsignedPolynomial implementation.
            let product = UnsignedPolynomial::mod_product([&p, &q].into_iter(), m);
            assert_eq!(
                NaturalPolynomial::mod_product(
                    [NaturalPolynomial::from(p), NaturalPolynomial::from(q)].into_iter(),
                    Natural::from(m)
                ),
                NaturalPolynomial::from(product)
            );
        },
    );
}
