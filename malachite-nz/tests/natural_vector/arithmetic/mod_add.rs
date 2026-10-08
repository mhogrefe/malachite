// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced, ModNeg};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen_var_2, natural_vector_natural_vector_natural_triple_gen_var_1,
    natural_vector_natural_vector_natural_vector_natural_quadruple_gen_var_1,
};

#[test]
fn test_mod_add() {
    let test = |s, t, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        let r = (&v).mod_add(&w, &m);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_add(&w, m.clone()), r);
        assert_eq!((&v).mod_add(w.clone(), &m), r);
        assert_eq!((&v).mod_add(w.clone(), m.clone()), r);
        assert_eq!(v.clone().mod_add(&w, &m), r);
        assert_eq!(v.clone().mod_add(&w, m.clone()), r);
        assert_eq!(v.clone().mod_add(w.clone(), &m), r);
        assert_eq!(v.clone().mod_add(w.clone(), m.clone()), r);
        let mut x = v.clone();
        x.mod_add_assign(&w, &m);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_add_assign(&w, m.clone());
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_add_assign(w.clone(), &m);
        assert_eq!(x, r);
        let mut x = v;
        x.mod_add_assign(w, m);
        assert_eq!(x, r);
    };
    test("()", "()", "1", "()");
    test("()", "()", "5", "()");
    test("(0)", "(0)", "1", "(0)");
    test("(5, 1, 3)", "(4, 7, 0)", "8", "(1, 0, 3)");
    test("(1, 2)", "(3, 4)", "7", "(4, 6)");
    test("(4, 5)", "(4, 6)", "7", "(1, 4)");
    test(
        "(999999999999999999999, 1)",
        "(1, 999999999999999999999)",
        "1000000000000000000000",
        "(0, 0)",
    );
}

#[test]
#[should_panic]
fn mod_add_fail_1() {
    // An element of self is not reduced.
    (&NaturalVector::from_str("(7, 1)").unwrap()).mod_add(
        &NaturalVector::from_str("(1, 1)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_fail_2() {
    // An element of other is not reduced.
    (&NaturalVector::from_str("(1, 1)").unwrap()).mod_add(
        &NaturalVector::from_str("(1, 7)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_fail_3() {
    // The dimensions differ.
    (&NaturalVector::from_str("(1, 1)").unwrap()).mod_add(
        &NaturalVector::from_str("(1)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_fail_4() {
    // The modulus is zero.
    (&NaturalVector::from_str("()").unwrap())
        .mod_add(&NaturalVector::from_str("()").unwrap(), Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_add_fail_5() {
    NaturalVector::from_str("(7, 1)").unwrap().mod_add(
        NaturalVector::from_str("(1, 1)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_fail_6() {
    (&NaturalVector::from_str("(1, 1)").unwrap()).mod_add(
        NaturalVector::from_str("(1, 7)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_assign_fail_1() {
    let mut v = NaturalVector::from_str("(1, 1)").unwrap();
    v.mod_add_assign(
        NaturalVector::from_str("(1, 7)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_add_assign_fail_2() {
    let mut v = NaturalVector::from_str("(1, 1)").unwrap();
    v.mod_add_assign(
        &NaturalVector::from_str("(1)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_add_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_1().test_properties(|(v, w, m)| {
        let r = (&v).mod_add(&w, &m);
        assert_eq!((&v).mod_add(&w, m.clone()), r);
        assert_eq!((&v).mod_add(w.clone(), &m), r);
        assert_eq!((&v).mod_add(w.clone(), m.clone()), r);
        assert_eq!(v.clone().mod_add(&w, &m), r);
        assert_eq!(v.clone().mod_add(&w, m.clone()), r);
        assert_eq!(v.clone().mod_add(w.clone(), &m), r);
        assert_eq!(v.clone().mod_add(w.clone(), m.clone()), r);
        let mut x = v.clone();
        x.mod_add_assign(&w, &m);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_add_assign(&w, m.clone());
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_add_assign(w.clone(), &m);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_add_assign(w.clone(), m.clone());
        assert_eq!(x, r);

        // The result is reduced, has the same dimension, and is the elementwise modular sum.
        assert!(r.mod_is_reduced(&m));
        assert_eq!(r.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(*z, x.mod_add(y, &m));
        }
        // It is the ordinary sum, reduced.
        assert_eq!(&(&v + &w) % &m, r);
        // Addition is commutative, and adding the negation gives zero.
        assert_eq!((&w).mod_add(&v, &m), r);
        assert!(
            (&v).mod_add((&v).mod_neg(&m), &m)
                .elements
                .iter()
                .all(|x| *x == 0u32)
        );
    });

    natural_vector_natural_vector_natural_vector_natural_quadruple_gen_var_1().test_properties(
        |(u, v, w, m)| {
            // Addition is associative.
            assert_eq!(
                (&u).mod_add(&v, &m).mod_add(&w, &m),
                (&u).mod_add((&v).mod_add(&w, &m), &m)
            );
        },
    );

    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        // The zero vector of the same dimension is the identity.
        let zero = NaturalVector {
            elements: vec![Natural::ZERO; v.elements.len()],
        };
        assert_eq!((&v).mod_add(&zero, &m), v);
    });

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<u64>().test_properties(
        |(v, w, m)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalVector::from((&v).mod_add(&w, m)),
                NaturalVector::from(v).mod_add(NaturalVector::from(w), Natural::from(m))
            );
        },
    );
}
