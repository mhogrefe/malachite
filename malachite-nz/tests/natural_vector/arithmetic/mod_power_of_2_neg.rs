// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_3;

#[test]
fn test_mod_power_of_2_neg() {
    let test = |s, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = (&v).mod_power_of_2_neg(pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_power_of_2_neg(pow), w);
        let mut x = v;
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, w);
    };
    test("()", 0, "()");
    test("()", 3, "()");
    test("(0)", 0, "(0)");
    test("(1)", 1, "(1)");
    test("(5, 1, 3)", 3, "(3, 7, 5)");
    // A zero element stays zero, wherever it is.
    test("(0, 1)", 8, "(0, 255)");
    test("(1, 0)", 8, "(255, 0)");
    test("(4, 4)", 3, "(4, 4)");
    test(
        "(3, 5)",
        100,
        "(1267650600228229401496703205373, 1267650600228229401496703205371)",
    );
    test(
        "(1000000000000000000000, 1)",
        70,
        "(180591620717411303424, 1180591620717411303423)",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail() {
    // An element is not reduced.
    (&NaturalVector::from_str("(8, 1)").unwrap()).mod_power_of_2_neg(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_assign_fail() {
    // An element is not reduced.
    let mut v = NaturalVector::from_str("(8, 1)").unwrap();
    v.mod_power_of_2_neg_assign(3);
}

#[test]
fn mod_power_of_2_neg_properties() {
    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        let w = (&v).mod_power_of_2_neg(pow);
        assert_eq!(v.clone().mod_power_of_2_neg(pow), w);
        let mut x = v.clone();
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, w);

        // The result is reduced, has the same dimension, and is the elementwise negation.
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_power_of_2_neg(pow));
            assert_eq!(x.mod_power_of_2_add(y, pow), 0u32);
        }
        // Negating twice gives the vector back.
        assert_eq!((&w).mod_power_of_2_neg(pow), v);
        // It is the negation over the integers, reduced.
        let negated = -IntegerVector::from(v.clone());
        assert_eq!(
            w.elements,
            negated
                .elements
                .iter()
                .map(|x| x.mod_power_of_2(pow))
                .collect::<Vec<Natural>>()
        );
    });

    unsigned_vector_unsigned_pair_gen_var_4::<u64>().test_properties(|(v, pow)| {
        // The `u64` and `Natural` versions agree.
        assert_eq!(
            NaturalVector::from((&v).mod_power_of_2_neg(pow)),
            NaturalVector::from(v).mod_power_of_2_neg(pow)
        );
    });
}
