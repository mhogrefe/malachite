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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use std::panic::catch_unwind;

#[test]
fn test_mod_product() {
    let test = |ss: &[&str], m: u8, out| {
        let ps: Vec<UnsignedPolynomial<u8>> = ss
            .iter()
            .map(|s| UnsignedPolynomial::from_str(s).unwrap())
            .collect();
        let product = UnsignedPolynomial::mod_product(ps.iter(), m);
        assert_eq!(product.to_string(), out);
        assert_eq!(UnsignedPolynomial::mod_product(ps.into_iter(), m), product);
    };
    test(&[], 7, "1");
    test(&[], 1, "0");
    test(
        &["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"],
        9,
        "7*x^6+5*x^5+8*x^4+4*x^3+3*x^2",
    );
    test(&["x+1", "x+254"], 255, "x^2+254");
    // Two nonzero polynomials multiply to zero, and the factor after them is not examined.
    test(&["3*x", "3"], 9, "0");
    test(&["3*x", "3", "10*x"], 9, "0");
}

fn mod_product_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(UnsignedPolynomial::<T>::mod_product(
        core::iter::empty::<UnsignedPolynomial<T>>(),
        T::ZERO
    ));
    let p = UnsignedPolynomial::<T>::from_str("7*x+1").unwrap();
    assert_panic!(UnsignedPolynomial::mod_product(
        [&p].into_iter(),
        T::exact_from(7u8)
    ));
}

#[test]
fn mod_product_fail() {
    apply_fn_to_unsigneds!(mod_product_fail_helper);
}

fn mod_product_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<T>().test_properties(
        |(p, q, m)| {
            let product = UnsignedPolynomial::mod_product([&p, &q].into_iter(), m);
            assert!(product.mod_is_reduced(&m));
            assert_eq!(
                UnsignedPolynomial::mod_product([p.clone(), q.clone()].into_iter(), m),
                product
            );
            // It is `mod_add`, in either order, and more factors keep multiplying.
            assert_eq!((&p).mod_mul(&q, m), product);
            assert_eq!(
                UnsignedPolynomial::mod_product([&q, &p].into_iter(), m),
                product
            );
            assert_eq!(
                UnsignedPolynomial::mod_product([&p, &q, &p].into_iter(), m),
                (&product).mod_mul(&p, m)
            );
            // The product of one polynomial is that polynomial, and the product of none is the
            // constant 1, reduced.
            assert_eq!(UnsignedPolynomial::mod_product([&p].into_iter(), m), p);
            assert_eq!(
                UnsignedPolynomial::mod_product(core::iter::empty::<UnsignedPolynomial<T>>(), m),
                if m == T::ONE {
                    UnsignedPolynomial::ZERO
                } else {
                    UnsignedPolynomial::one()
                }
            );
        },
    );
}

#[test]
fn mod_product_properties() {
    apply_fn_to_unsigneds!(mod_product_properties_helper);
}
