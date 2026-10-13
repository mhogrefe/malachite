// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Dot, ModPowerOf2DotReversed};
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;

fn reversed(w: &NaturalVector) -> NaturalVector {
    NaturalVector::from_owned_elements(w.elements.iter().rev().cloned().collect())
}

#[test]
fn test_mod_power_of_2_dot_reversed() {
    let test = |v, w, pow, out| {
        let v = NaturalVector::from_str(v).unwrap();
        let w = NaturalVector::from_str(w).unwrap();
        let d = (&v).mod_power_of_2_dot_reversed(&w, pow);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!(v.clone().mod_power_of_2_dot_reversed(w.clone(), pow), d);
        assert_eq!(v.clone().mod_power_of_2_dot_reversed(&w, pow), d);
        assert_eq!((&v).mod_power_of_2_dot_reversed(w.clone(), pow), d);
        assert_eq!((&v).mod_power_of_2_dot_reversed(&w, pow), d);
    };
    test("()", "()", 0, "0");
    test("(1, 2, 3)", "(4, 5, 7)", 4, "13");
    test(
        "(1267650600228229401496703205375, 2)",
        "(3, 1267650600228229401496703205375)",
        100,
        "7",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_dot_reversed_fail_1() {
    // The dimensions differ.
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_power_of_2_dot_reversed(NaturalVector::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_dot_reversed_fail_2() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_power_of_2_dot_reversed(NaturalVector::from_str("(1, 8)").unwrap(), 3);
}

#[test]
fn mod_power_of_2_dot_reversed_properties() {
    natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1().test_properties(
        |(v, w, _, pow)| {
            let d = (&v).mod_power_of_2_dot_reversed(&w, pow);
            assert!(d.is_valid());
            assert_eq!((&v).mod_power_of_2_dot_reversed(w.clone(), pow), d);
            assert_eq!(v.clone().mod_power_of_2_dot_reversed(&w, pow), d);
            // It is the dot product with the second vector reversed, and it is symmetric.
            assert_eq!((&v).mod_power_of_2_dot(&reversed(&w), pow), d);
            assert_eq!((&w).mod_power_of_2_dot_reversed(&v, pow), d);
        },
    );

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<Limb>()
        .test_properties(|(v, w, _, pow)| {
            // It agrees with the `UnsignedVector` implementation.
            assert_eq!(
                NaturalVector::from(v.clone())
                    .mod_power_of_2_dot_reversed(NaturalVector::from(w.clone()), pow),
                Natural::from(v.mod_power_of_2_dot_reversed(w, pow))
            );
        });
}
