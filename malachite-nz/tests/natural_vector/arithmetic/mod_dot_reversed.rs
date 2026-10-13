// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModDot, ModDotReversed};
use malachite_base::num::basic::traits::Zero;
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
fn test_mod_dot_reversed() {
    let test = |v, w, m, out| {
        let v = NaturalVector::from_str(v).unwrap();
        let w = NaturalVector::from_str(w).unwrap();
        let m = Natural::from_str(m).unwrap();
        let d = (&v).mod_dot_reversed(&w, &m);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!(v.clone().mod_dot_reversed(w.clone(), m.clone()), d);
        assert_eq!(v.clone().mod_dot_reversed(w.clone(), &m), d);
        assert_eq!(v.clone().mod_dot_reversed(&w, m.clone()), d);
        assert_eq!(v.clone().mod_dot_reversed(&w, &m), d);
        assert_eq!((&v).mod_dot_reversed(w.clone(), m.clone()), d);
        assert_eq!((&v).mod_dot_reversed(w.clone(), &m), d);
        assert_eq!((&v).mod_dot_reversed(&w, m.clone()), d);
        assert_eq!((&v).mod_dot_reversed(&w, &m), d);
    };
    test("()", "()", "1", "0");
    test("(1, 2, 3)", "(4, 5, 7)", "11", "7");
    test(
        "(1000000000000000000000000000056, 2)",
        "(3, 1000000000000000000000000000056)",
        "1000000000000000000000000000057",
        "7",
    );
}

#[test]
#[should_panic]
fn mod_dot_reversed_fail_1() {
    // The modulus is zero.
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_dot_reversed(NaturalVector::from_str("(1, 2)").unwrap(), Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_dot_reversed_fail_2() {
    // The dimensions differ.
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_dot_reversed(NaturalVector::from_str("(1)").unwrap(), Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_dot_reversed_fail_3() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 2)").unwrap().mod_dot_reversed(
        NaturalVector::from_str("(1, 7)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_dot_reversed_properties() {
    natural_vector_natural_vector_natural_natural_quadruple_gen_var_1().test_properties(
        |(v, w, _, m)| {
            let d = (&v).mod_dot_reversed(&w, &m);
            assert!(d.is_valid());
            assert_eq!((&v).mod_dot_reversed(w.clone(), &m), d);
            assert_eq!(v.clone().mod_dot_reversed(&w, &m), d);
            // It is the dot product with the second vector reversed, and it is symmetric.
            assert_eq!((&v).mod_dot(&reversed(&w), &m), d);
            assert_eq!((&w).mod_dot_reversed(&v, &m), d);
        },
    );

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_2::<Limb>()
        .test_properties(|(v, w, _, m)| {
            // It agrees with the `UnsignedVector` implementation.
            assert_eq!(
                NaturalVector::from(v.clone())
                    .mod_dot_reversed(NaturalVector::from(w.clone()), Natural::from(m)),
                Natural::from(v.mod_dot_reversed(w, m))
            );
        });
}
