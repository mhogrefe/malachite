// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Dot, ModPowerOf2, ModPowerOf2Dot, ModPowerOf2IsReduced,
};
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_vector_natural_vector_unsigned_triple_gen_var_1;

#[test]
fn test_mod_power_of_2_dot() {
    let test = |a, b, pow, out| {
        let v = NaturalVector::from_str(a).unwrap();
        let w = NaturalVector::from_str(b).unwrap();
        let d = (&v).mod_power_of_2_dot(&w, pow);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!((&v).mod_power_of_2_dot(w.clone(), pow), d);
        assert_eq!(v.clone().mod_power_of_2_dot(&w, pow), d);
        assert_eq!(v.mod_power_of_2_dot(w, pow), d);
    };
    test("()", "()", 0, "0");
    test("()", "()", 100, "0");
    // - pow fits in a limb
    test("(1, 2, 3)", "(4, 5, 7)", 4, "3");
    test(
        "(18446744073709551615, 18446744073709551615)",
        "(18446744073709551615, 1)",
        64,
        "0",
    );
    // - pow is larger than a limb
    test(
        "(1267650600228229401496703205375, 1267650600228229401496703205375)",
        "(1267650600228229401496703205375, 3)",
        100,
        "1267650600228229401496703205374",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_dot_fail_1() {
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_power_of_2_dot(NaturalVector::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_dot_fail_2() {
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_power_of_2_dot(NaturalVector::from_str("(1, 8)").unwrap(), 3);
}

#[test]
fn mod_power_of_2_dot_properties() {
    natural_vector_natural_vector_unsigned_triple_gen_var_1().test_properties(|(v, w, pow)| {
        let d = (&v).mod_power_of_2_dot(&w, pow);
        assert!(d.is_valid());
        assert!(d.mod_power_of_2_is_reduced(pow));
        assert_eq!((&v).mod_power_of_2_dot(w.clone(), pow), d);
        assert_eq!(v.clone().mod_power_of_2_dot(&w, pow), d);
        assert_eq!(v.clone().mod_power_of_2_dot(w.clone(), pow), d);
        // It is symmetric, and it is the exact dot product, reduced.
        assert_eq!((&w).mod_power_of_2_dot(&v, pow), d);
        assert_eq!(v.dot(w).mod_power_of_2(pow), d);
    });

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<Limb>().test_properties(
        |(v, w, pow)| {
            // It agrees with the UnsignedVector implementation.
            assert_eq!(
                NaturalVector::from(v.clone())
                    .mod_power_of_2_dot(NaturalVector::from(w.clone()), pow),
                Natural::from(v.mod_power_of_2_dot(w, pow))
            );
        },
    );
}
