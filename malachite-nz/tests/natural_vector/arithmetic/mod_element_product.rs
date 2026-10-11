// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ElementProduct, Mod, ModElementProduct, ModIsReduced, ModPowerOf2ElementProduct, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen_var_2, natural_vector_unsigned_pair_gen_var_3,
};

#[test]
fn test_mod_element_product() {
    let test = |s, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let product = v.to_mod_element_product(&m);
        assert_eq!(product.to_string(), out);
        assert_eq!(v.to_mod_element_product(m.clone()), product);
        assert_eq!(v.clone().into_mod_element_product(&m), product);
        assert_eq!(v.into_mod_element_product(m), product);
    };
    test("()", "1", "0");
    test("()", "7", "1");
    test("(3, 5, 7)", "10", "5");
    // The partial product becomes zero, and the unreduced element after it is not examined.
    test("(2, 5, 11)", "10", "0");
    test(
        "(1000000000000000000000000000056, 1000000000000000000000000000055)",
        "1000000000000000000000000000057",
        "2",
    );
}

#[test]
#[should_panic]
fn mod_element_product_fail_1() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 7)")
        .unwrap()
        .to_mod_element_product(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_element_product_fail_2() {
    // The modulus is zero, even though there are no elements to check.
    NaturalVector::from_str("()")
        .unwrap()
        .to_mod_element_product(Natural::ZERO);
}

#[test]
fn mod_element_product_properties() {
    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        let product = v.to_mod_element_product(&m);
        assert_eq!(v.to_mod_element_product(m.clone()), product);
        assert_eq!(v.clone().into_mod_element_product(&m), product);
        assert_eq!(v.clone().into_mod_element_product(m.clone()), product);
        assert!(product.mod_is_reduced(&m));
        // It is the product of the elements, reduced.
        assert_eq!(v.into_element_product().mod_op(&m), product);
    });

    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_element_product`.
        assert_eq!(
            v.to_mod_element_product(Natural::power_of_2(pow)),
            v.to_mod_power_of_2_element_product(pow)
        );
    });

    unsigned_vector_unsigned_pair_gen_var_5::<Limb>().test_properties(|(v, m)| {
        // It agrees with the UnsignedVector implementation.
        assert_eq!(
            NaturalVector::from(v.clone()).into_mod_element_product(Natural::from(m)),
            Natural::from(v.into_mod_element_product(m))
        );
    });
}
