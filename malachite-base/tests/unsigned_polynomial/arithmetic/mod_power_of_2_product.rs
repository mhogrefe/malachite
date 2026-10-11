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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_product() {
    let test = |ss: &[&str], pow: u64, out| {
        let ps: Vec<UnsignedPolynomial<u8>> = ss
            .iter()
            .map(|s| UnsignedPolynomial::from_str(s).unwrap())
            .collect();
        let product = UnsignedPolynomial::mod_power_of_2_product(ps.iter(), pow);
        assert_eq!(product.to_string(), out);
        assert_eq!(
            UnsignedPolynomial::mod_power_of_2_product(ps.into_iter(), pow),
            product
        );
    };
    test(&[], 3, "1");
    test(&[], 0, "0");
    test(&["1"], 1, "1");
    test(
        &["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"],
        3,
        "7*x^6+4*x^5+6*x^4+2*x^3+3*x^2+2*x",
    );
    test(&["x+1", "x+7"], 3, "x^2+7");
    // Two nonzero polynomials multiply to zero, and the factor after them is not examined.
    test(&["2*x", "4"], 3, "0");
    test(&["2*x", "4", "9*x"], 3, "0");
}

fn mod_power_of_2_product_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(UnsignedPolynomial::<T>::mod_power_of_2_product(
        core::iter::empty::<UnsignedPolynomial<T>>(),
        T::WIDTH + 1
    ));
    let p = UnsignedPolynomial::<T>::from_str("8*x+1").unwrap();
    assert_panic!(UnsignedPolynomial::mod_power_of_2_product(
        [&p].into_iter(),
        3
    ));
}

#[test]
fn mod_power_of_2_product_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_product_fail_helper);
}

fn mod_power_of_2_product_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_1::<T>().test_properties(
        |(p, q, pow)| {
            let product = UnsignedPolynomial::mod_power_of_2_product([&p, &q].into_iter(), pow);
            assert!(product.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_product([p.clone(), q.clone()].into_iter(), pow),
                product
            );
            // It is `mod_power_of_2_add`, in either order, and more factors keep multiplying.
            assert_eq!((&p).mod_power_of_2_mul(&q, pow), product);
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_product([&q, &p].into_iter(), pow),
                product
            );
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_product([&p, &q, &p].into_iter(), pow),
                (&product).mod_power_of_2_mul(&p, pow)
            );
            // The product of one polynomial is that polynomial, and the product of none is the
            // constant 1, reduced.
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_product([&p].into_iter(), pow),
                p
            );
            assert_eq!(
                UnsignedPolynomial::mod_power_of_2_product(
                    core::iter::empty::<UnsignedPolynomial<T>>(),
                    pow
                ),
                if pow == 0 {
                    UnsignedPolynomial::ZERO
                } else {
                    UnsignedPolynomial::one()
                }
            );
        },
    );
}

#[test]
fn mod_power_of_2_product_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_product_properties_helper);
}
