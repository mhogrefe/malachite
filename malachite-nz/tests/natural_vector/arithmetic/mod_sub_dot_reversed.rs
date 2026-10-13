// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAddDotReversed, ModDot, ModSub, ModSubDot, ModSubDotReversed, ModSubDotReversedAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
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
fn test_mod_sub_dot_reversed() {
    let test = |x, v, w, m, out| {
        let x = Natural::from_str(x).unwrap();
        let v = NaturalVector::from_str(v).unwrap();
        let w = NaturalVector::from_str(w).unwrap();
        let m = Natural::from_str(m).unwrap();
        let r = (&x).mod_sub_dot_reversed(&v, &w, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(
            x.clone()
                .mod_sub_dot_reversed(v.clone(), w.clone(), m.clone()),
            r
        );
        assert_eq!(x.clone().mod_sub_dot_reversed(v.clone(), w.clone(), &m), r);
        assert_eq!(x.clone().mod_sub_dot_reversed(v.clone(), &w, m.clone()), r);
        assert_eq!(x.clone().mod_sub_dot_reversed(v.clone(), &w, &m), r);
        assert_eq!(x.clone().mod_sub_dot_reversed(&v, w.clone(), m.clone()), r);
        assert_eq!(x.clone().mod_sub_dot_reversed(&v, w.clone(), &m), r);
        assert_eq!(x.clone().mod_sub_dot_reversed(&v, &w, m.clone()), r);
        assert_eq!(x.clone().mod_sub_dot_reversed(&v, &w, &m), r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(v.clone(), w.clone(), m.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(v.clone(), w.clone(), &m);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(v.clone(), &w, m.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(v.clone(), &w, &m);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(&v, w.clone(), m.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(&v, w.clone(), &m);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(&v, &w, m.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.mod_sub_dot_reversed_assign(&v, &w, &m);
        assert_eq!(y, r);
    };
    test("0", "()", "()", "1", "0");
    test("3", "(1, 2, 3)", "(4, 5, 7)", "11", "7");
    test(
        "5",
        "(1000000000000000000000000000056, 2)",
        "(3, 1000000000000000000000000000056)",
        "1000000000000000000000000000057",
        "1000000000000000000000000000055",
    );
}

#[test]
#[should_panic]
fn mod_sub_dot_reversed_fail_1() {
    // The modulus is zero.
    Natural::ONE.mod_sub_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1, 2)").unwrap(),
        Natural::ZERO,
    );
}

#[test]
#[should_panic]
fn mod_sub_dot_reversed_fail_2() {
    // The dimensions differ.
    Natural::ONE.mod_sub_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_sub_dot_reversed_fail_3() {
    // An element is not reduced.
    Natural::ONE.mod_sub_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1, 7)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_sub_dot_reversed_fail_4() {
    // `x` is not reduced.
    Natural::from(7u32).mod_sub_dot_reversed(
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1, 2)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_sub_dot_reversed_properties() {
    natural_vector_natural_vector_natural_natural_quadruple_gen_var_1().test_properties(
        |(v, w, x, m)| {
            let r = (&x).mod_sub_dot_reversed(&v, &w, &m);
            assert!(r.is_valid());
            assert!(r < m);
            assert_eq!(
                x.clone()
                    .mod_sub_dot_reversed(v.clone(), w.clone(), m.clone()),
                r
            );
            assert_eq!(x.clone().mod_sub_dot_reversed(v.clone(), w.clone(), &m), r);
            assert_eq!(x.clone().mod_sub_dot_reversed(v.clone(), &w, m.clone()), r);
            assert_eq!(x.clone().mod_sub_dot_reversed(v.clone(), &w, &m), r);
            assert_eq!(x.clone().mod_sub_dot_reversed(&v, w.clone(), m.clone()), r);
            assert_eq!(x.clone().mod_sub_dot_reversed(&v, w.clone(), &m), r);
            assert_eq!(x.clone().mod_sub_dot_reversed(&v, &w, m.clone()), r);
            assert_eq!(x.clone().mod_sub_dot_reversed(&v, &w, &m), r);
            let mut y = x.clone();
            y.mod_sub_dot_reversed_assign(&v, &w, &m);
            assert_eq!(y, r);
            // It is the reversed dot product subtracted from `x`.
            assert_eq!((&x).mod_sub((&v).mod_dot(&reversed(&w), &m), &m), r);
            assert_eq!((&x).mod_sub_dot(&v, &reversed(&w), &m), r);
            // Adding it back recovers `x`.
            assert_eq!((&r).mod_add_dot_reversed(&v, &w, &m), x);
        },
    );

    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_2::<Limb>()
        .test_properties(|(v, w, x, m)| {
            // It agrees with the `UnsignedVector` implementation.
            assert_eq!(
                Natural::from(x).mod_sub_dot_reversed(
                    NaturalVector::from(v.clone()),
                    NaturalVector::from(w.clone()),
                    Natural::from(m)
                ),
                Natural::from(x.mod_sub_dot_reversed(v, w, m))
            );
        });
}
