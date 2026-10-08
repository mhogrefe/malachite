// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Mod, ModIsReduced, ModNeg, ModNegAssign, ModPowerOf2Neg, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen_var_2, natural_vector_unsigned_pair_gen_var_3,
};

#[test]
fn test_mod_neg() {
    let test = |s, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let w = (&v).mod_neg(&m);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).mod_neg(m.clone()), w);
        assert_eq!(v.clone().mod_neg(&m), w);
        assert_eq!(v.clone().mod_neg(m.clone()), w);
        let mut x = v.clone();
        x.mod_neg_assign(&m);
        assert_eq!(x, w);
        let mut x = v;
        x.mod_neg_assign(m);
        assert_eq!(x, w);
    };
    test("()", "1", "()");
    test("()", "7", "()");
    test("(0)", "1", "(0)");
    test("(5, 1, 0)", "7", "(2, 6, 0)");
    // A zero element stays zero, wherever it is.
    test("(0, 1)", "255", "(0, 254)");
    test("(1, 0)", "255", "(254, 0)");
    test("(3, 3)", "6", "(3, 3)");
    test(
        "(1, 999999999999999999999)",
        "1000000000000000000000",
        "(999999999999999999999, 1)",
    );
}

#[test]
#[should_panic]
fn mod_neg_fail_1() {
    // An element is not reduced.
    (&NaturalVector::from_str("(7, 1)").unwrap()).mod_neg(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_neg_fail_2() {
    // The modulus is zero, even though there are no elements to check.
    NaturalVector::from_str("()")
        .unwrap()
        .mod_neg(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_neg_assign_fail() {
    // An element is not reduced.
    let mut v = NaturalVector::from_str("(7, 1)").unwrap();
    v.mod_neg_assign(&Natural::from(7u32));
}

#[test]
fn mod_neg_properties() {
    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        let w = (&v).mod_neg(&m);
        assert_eq!((&v).mod_neg(m.clone()), w);
        assert_eq!(v.clone().mod_neg(&m), w);
        assert_eq!(v.clone().mod_neg(m.clone()), w);
        let mut x = v.clone();
        x.mod_neg_assign(&m);
        assert_eq!(x, w);
        let mut x = v.clone();
        x.mod_neg_assign(m.clone());
        assert_eq!(x, w);

        // The result is reduced, has the same dimension, and is the elementwise negation.
        assert!(w.mod_is_reduced(&m));
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.mod_neg(&m));
        }
        // Negating twice gives the vector back.
        assert_eq!((&w).mod_neg(&m), v);
        // It is the negation over the integers, reduced.
        let m_i = Integer::from(&m);
        let negated = -IntegerVector::from(v.clone());
        assert!(
            negated
                .elements
                .iter()
                .zip(&w.elements)
                .all(|(x, y)| x.mod_op(&m_i) == *y)
        );
    });

    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_neg`.
        assert_eq!(
            (&v).mod_neg(Natural::power_of_2(pow)),
            (&v).mod_power_of_2_neg(pow)
        );
    });

    unsigned_vector_unsigned_pair_gen_var_5::<u64>().test_properties(|(v, m)| {
        // The `u64` and `Natural` versions agree.
        assert_eq!(
            NaturalVector::from((&v).mod_neg(m)),
            NaturalVector::from(v).mod_neg(Natural::from(m))
        );
    });
}
