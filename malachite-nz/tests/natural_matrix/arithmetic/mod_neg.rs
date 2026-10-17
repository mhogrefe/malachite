// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Mod, ModIsReduced, ModNeg, ModNegAssign, ModPowerOf2Neg, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_5;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_2, natural_matrix_unsigned_pair_gen_var_4,
};

#[test]
fn test_mod_neg() {
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let b = (&a).mod_neg(&m);
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);
        assert_eq!((&a).mod_neg(m.clone()), b);
        assert_eq!(a.clone().mod_neg(&m), b);
        assert_eq!(a.clone().mod_neg(m.clone()), b);
        let mut x = a.clone();
        x.mod_neg_assign(&m);
        assert_eq!(x, b);
        let mut x = a;
        x.mod_neg_assign(m);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "1", "[]_{0,0}");
    test("[]_{0,0}", "7", "[]_{0,0}");
    test("[0]", "1", "[0]");
    test("[5, 1, 0]", "7", "[2, 6, 0]");
    // A zero element stays zero, wherever it is.
    test("[0, 1]", "255", "[0, 254]");
    test("[1, 0]", "255", "[254, 0]");
    test("[3, 3]", "6", "[3, 3]");
    test(
        "[1, 999999999999999999999]",
        "1000000000000000000000",
        "[999999999999999999999, 1]",
    );
}

#[test]
#[should_panic]
fn mod_neg_fail_1() {
    // An element is not reduced.
    (&NaturalMatrix::from_str("[7, 1]").unwrap()).mod_neg(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_neg_fail_2() {
    // The modulus is zero, even though there are no elements to check.
    NaturalMatrix::from_str("[]_{0,0}")
        .unwrap()
        .mod_neg(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_neg_assign_fail() {
    // An element is not reduced.
    let mut a = NaturalMatrix::from_str("[7, 1]").unwrap();
    a.mod_neg_assign(&Natural::from(7u32));
}

#[test]
fn mod_neg_properties() {
    natural_matrix_natural_pair_gen_var_2().test_properties(|(a, m)| {
        let b = (&a).mod_neg(&m);
        assert_eq!((&a).mod_neg(m.clone()), b);
        assert_eq!(a.clone().mod_neg(&m), b);
        assert_eq!(a.clone().mod_neg(m.clone()), b);
        let mut x = a.clone();
        x.mod_neg_assign(&m);
        assert_eq!(x, b);
        let mut x = a.clone();
        x.mod_neg_assign(m.clone());
        assert_eq!(x, b);

        // The result is reduced, has the same shape, and is the elementwise negation.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x.mod_neg(&m));
        }
        // Negating twice gives the matrix back, and negation commutes with transposition.
        assert_eq!((&b).mod_neg(&m), a);
        assert_eq!(a.transpose().mod_neg(&m), b.transpose());
        // It is the negation over the integers, reduced.
        let m_i = Integer::from(&m);
        let negated = -IntegerMatrix::from(a.clone());
        assert!(
            negated
                .elements
                .iter()
                .zip(&b.elements)
                .all(|(x, y)| x.mod_op(&m_i) == *y)
        );
    });

    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_neg`.
        assert_eq!(
            (&a).mod_neg(Natural::power_of_2(pow)),
            (&a).mod_power_of_2_neg(pow)
        );
    });

    unsigned_matrix_unsigned_pair_gen_var_5().test_properties(|(a, m)| {
        // The `u64` and `Natural` versions agree.
        assert_eq!(
            NaturalMatrix::from((&a).mod_neg(m)),
            NaturalMatrix::from(a).mod_neg(Natural::from(m))
        );
    });
}
