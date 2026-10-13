// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    EntrywiseMul, ModAdd, ModEntrywiseMul, ModEntrywiseMulAssign, ModIsReduced, ModMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_natural_triple_gen_var_1,
    natural_vector_natural_vector_natural_vector_natural_quadruple_gen_var_1,
};

#[test]
fn test_mod_entrywise_mul() {
    let test = |s, t, m: Natural, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
        let r = (&v).mod_entrywise_mul(&w, &m);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_entrywise_mul(&w, m.clone()), r);
        assert_eq!((&v).mod_entrywise_mul(w.clone(), &m), r);
        assert_eq!((&v).mod_entrywise_mul(w.clone(), m.clone()), r);
        assert_eq!(v.clone().mod_entrywise_mul(&w, &m), r);
        assert_eq!(v.clone().mod_entrywise_mul(&w, m.clone()), r);
        assert_eq!(v.clone().mod_entrywise_mul(w.clone(), &m), r);
        assert_eq!(v.clone().mod_entrywise_mul(w.clone(), m.clone()), r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(&w, &m);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(&w, m.clone());
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(w.clone(), &m);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(w.clone(), m.clone());
        assert_eq!(x, r);
    };
    test("()", "()", Natural::ONE, "()");
    test("(0)", "(0)", Natural::ONE, "(0)");
    test("(5, 1, 3)", "(4, 6, 5)", Natural::from(7u32), "(6, 6, 1)");
    test(
        "(18446744073709551616, 3)",
        "(18446744073709551617, 5)",
        Natural::from_str("18446744073709551629").unwrap(),
        "(156, 15)",
    );
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_1() {
    // An element of self is not reduced.
    (&NaturalVector::from_str("(7, 1)").unwrap()).mod_entrywise_mul(
        &NaturalVector::from_str("(1, 1)").unwrap(),
        &Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_2() {
    // An element of other is not reduced.
    (&NaturalVector::from_str("(1, 1)").unwrap()).mod_entrywise_mul(
        &NaturalVector::from_str("(1, 7)").unwrap(),
        &Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_3() {
    // The dimensions differ.
    (&NaturalVector::from_str("(1, 1)").unwrap()).mod_entrywise_mul(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_4() {
    // The modulus is zero.
    (&NaturalVector::from_str("()").unwrap())
        .mod_entrywise_mul(&NaturalVector::from_str("()").unwrap(), &Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_5() {
    NaturalVector::from_str("(7, 1)")
        .unwrap()
        .mod_entrywise_mul(
            NaturalVector::from_str("(1, 1)").unwrap(),
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_entrywise_mul_assign_fail() {
    let mut v = NaturalVector::from_str("(1, 1)").unwrap();
    v.mod_entrywise_mul_assign(
        NaturalVector::from_str("(1, 7)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_entrywise_mul_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_1().test_properties(|(v, w, m)| {
        let r = (&v).mod_entrywise_mul(&w, &m);
        assert_eq!((&v).mod_entrywise_mul(&w, m.clone()), r);
        assert_eq!((&v).mod_entrywise_mul(w.clone(), &m), r);
        assert_eq!((&v).mod_entrywise_mul(w.clone(), m.clone()), r);
        assert_eq!(v.clone().mod_entrywise_mul(&w, &m), r);
        assert_eq!(v.clone().mod_entrywise_mul(&w, m.clone()), r);
        assert_eq!(v.clone().mod_entrywise_mul(w.clone(), &m), r);
        assert_eq!(v.clone().mod_entrywise_mul(w.clone(), m.clone()), r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(&w, &m);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(&w, m.clone());
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(w.clone(), &m);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(w.clone(), m.clone());
        assert_eq!(x, r);

        // The result is reduced, has the same dimension, and is the elementwise modular product.
        assert!(r.mod_is_reduced(&m));
        assert_eq!(r.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(*z, x.mod_mul(y, &m));
        }
        // It is the ordinary entrywise product, reduced.
        assert_eq!(&(&v).entrywise_mul(&w) % &m, r);
        // It is commutative.
        assert_eq!((&w).mod_entrywise_mul(&v, &m), r);
        // The vector of 1s is the identity, and the zero vector absorbs everything.
        let ones = NaturalVector {
            elements: vec![Natural::ONE % &m; v.elements.len()],
        };
        assert_eq!((&v).mod_entrywise_mul(&ones, &m), v);
        let zero = NaturalVector::zero(v.dimension());
        assert_eq!((&v).mod_entrywise_mul(&zero, &m), zero);
    });

    natural_vector_natural_vector_natural_vector_natural_quadruple_gen_var_1().test_properties(
        |(u, v, w, m)| {
            // It is associative, and it distributes over modular addition.
            assert_eq!(
                (&u).mod_entrywise_mul(&v, &m).mod_entrywise_mul(&w, &m),
                (&u).mod_entrywise_mul((&v).mod_entrywise_mul(&w, &m), &m)
            );
            assert_eq!(
                (&u).mod_entrywise_mul((&v).mod_add(&w, &m), &m),
                (&u).mod_entrywise_mul(&v, &m)
                    .mod_add((&u).mod_entrywise_mul(&w, &m), &m)
            );
        },
    );

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<u64>().test_properties(
        |(v, w, m)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalVector::from((&v).mod_entrywise_mul(&w, m)),
                NaturalVector::from(v).mod_entrywise_mul(NaturalVector::from(w), Natural::from(m))
            );
        },
    );
}
