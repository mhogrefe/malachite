// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    AddDotReversed, ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddDot, ModPowerOf2AddDotReversed,
    ModPowerOf2AddDotReversedAssign, ModPowerOf2Dot, ModPowerOf2SubDotReversed,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::SignificantBits;
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
fn test_mod_power_of_2_add_dot_reversed() {
    let test = |x, v, w, pow, out| {
        let x = Natural::from_str(x).unwrap();
        let v = NaturalVector::from_str(v).unwrap();
        let w = NaturalVector::from_str(w).unwrap();
        let r = (&x).mod_power_of_2_add_dot_reversed(&v, &w, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(
            x.clone()
                .mod_power_of_2_add_dot_reversed(v.clone(), w.clone(), pow),
            r
        );
        assert_eq!(
            x.clone()
                .mod_power_of_2_add_dot_reversed(v.clone(), &w, pow),
            r
        );
        assert_eq!(
            x.clone()
                .mod_power_of_2_add_dot_reversed(&v, w.clone(), pow),
            r
        );
        assert_eq!(x.clone().mod_power_of_2_add_dot_reversed(&v, &w, pow), r);
        let mut y = x.clone();
        y.mod_power_of_2_add_dot_reversed_assign(v.clone(), w.clone(), pow);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_power_of_2_add_dot_reversed_assign(v.clone(), &w, pow);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_power_of_2_add_dot_reversed_assign(&v, w.clone(), pow);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_power_of_2_add_dot_reversed_assign(&v, &w, pow);
        assert_eq!(y, r);
    };
    test("0", "()", "()", 0, "0");
    test("5", "(1, 2, 3)", "(4, 5, 7)", 4, "2");
    test(
        "1",
        "(1267650600228229401496703205375, 2)",
        "(3, 1267650600228229401496703205375)",
        100,
        "8",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_dot_reversed_fail_1() {
    // The dimensions differ.
    Natural::ONE.mod_power_of_2_add_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1)").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_dot_reversed_fail_2() {
    // An element is not reduced.
    Natural::ONE.mod_power_of_2_add_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1, 8)").unwrap(),
        3,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_dot_reversed_fail_3() {
    // `x` is not reduced.
    Natural::from(8u32).mod_power_of_2_add_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1, 2)").unwrap(),
        3,
    );
}

#[test]
fn mod_power_of_2_add_dot_reversed_properties() {
    natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1().test_properties(
        |(v, w, x, pow)| {
            let r = (&x).mod_power_of_2_add_dot_reversed(&v, &w, pow);
            assert!(r.is_valid());
            assert!(r.significant_bits() <= pow);
            assert_eq!(
                x.clone()
                    .mod_power_of_2_add_dot_reversed(v.clone(), w.clone(), pow),
                r
            );
            assert_eq!(
                x.clone()
                    .mod_power_of_2_add_dot_reversed(v.clone(), &w, pow),
                r
            );
            assert_eq!(
                x.clone()
                    .mod_power_of_2_add_dot_reversed(&v, w.clone(), pow),
                r
            );
            assert_eq!(x.clone().mod_power_of_2_add_dot_reversed(&v, &w, pow), r);
            let mut y = x.clone();
            y.mod_power_of_2_add_dot_reversed_assign(&v, &w, pow);
            assert_eq!(y, r);
            // It is the reversed dot product added to `x`.
            assert_eq!(
                (&x).mod_power_of_2_add((&v).mod_power_of_2_dot(&reversed(&w), pow), pow),
                r
            );
            assert_eq!((&x).mod_power_of_2_add_dot(&v, &reversed(&w), pow), r);
            // Subtracting it back recovers `x`.
            assert_eq!((&r).mod_power_of_2_sub_dot_reversed(&v, &w, pow), x);
            // It is the non-modular result, reduced.
            assert_eq!((&x).add_dot_reversed(&v, &w).mod_power_of_2(pow), r);
        },
    );

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<Limb>()
        .test_properties(|(v, w, x, pow)| {
            // It agrees with the `UnsignedVector` implementation.
            assert_eq!(
                Natural::from(x).mod_power_of_2_add_dot_reversed(
                    NaturalVector::from(v.clone()),
                    NaturalVector::from(w.clone()),
                    pow
                ),
                Natural::from(x.mod_power_of_2_add_dot_reversed(v, w, pow))
            );
        });
}
