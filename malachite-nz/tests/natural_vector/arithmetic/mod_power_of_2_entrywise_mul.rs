// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    EntrywiseMul, ModEntrywiseMul, ModPowerOf2, ModPowerOf2Add, ModPowerOf2EntrywiseMul,
    ModPowerOf2EntrywiseMulAssign, ModPowerOf2IsReduced, ModPowerOf2Mul, PowerOf2,
};
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_natural_vector_unsigned_quadruple_gen_var_1,
    natural_vector_natural_vector_unsigned_triple_gen_var_1,
};

#[test]
fn test_mod_power_of_2_entrywise_mul() {
    let test = |s, t, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
        let r = (&v).mod_power_of_2_entrywise_mul(&w, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_power_of_2_entrywise_mul(w.clone(), pow), r);
        assert_eq!(v.clone().mod_power_of_2_entrywise_mul(&w, pow), r);
        assert_eq!(v.clone().mod_power_of_2_entrywise_mul(w.clone(), pow), r);
        let mut x = v.clone();
        x.mod_power_of_2_entrywise_mul_assign(&w, pow);
        assert_eq!(x, r);
        let mut x = v;
        x.mod_power_of_2_entrywise_mul_assign(w, pow);
        assert_eq!(x, r);
    };
    test("()", "()", 0, "()");
    test("(0)", "(0)", 0, "(0)");
    test("(5, 1, 3)", "(4, 7, 6)", 3, "(4, 7, 2)");
    test(
        "(18446744073709551621, 3)",
        "(36893488147419103231, 5)",
        65,
        "(18446744073709551611, 15)",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_1() {
    // An element of self is not reduced.
    (&NaturalVector::from_str("(8, 1)").unwrap())
        .mod_power_of_2_entrywise_mul(&NaturalVector::from_str("(1, 1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_2() {
    // An element of other is not reduced.
    (&NaturalVector::from_str("(1, 1)").unwrap())
        .mod_power_of_2_entrywise_mul(&NaturalVector::from_str("(1, 8)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_3() {
    // The dimensions differ.
    (&NaturalVector::from_str("(1, 1)").unwrap())
        .mod_power_of_2_entrywise_mul(&NaturalVector::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_assign_fail() {
    let mut v = NaturalVector::from_str("(1, 1)").unwrap();
    v.mod_power_of_2_entrywise_mul_assign(NaturalVector::from_str("(1, 8)").unwrap(), 3);
}

#[test]
fn mod_power_of_2_entrywise_mul_properties() {
    natural_vector_natural_vector_unsigned_triple_gen_var_1().test_properties(|(v, w, pow)| {
        let r = (&v).mod_power_of_2_entrywise_mul(&w, pow);
        assert_eq!((&v).mod_power_of_2_entrywise_mul(w.clone(), pow), r);
        assert_eq!(v.clone().mod_power_of_2_entrywise_mul(&w, pow), r);
        assert_eq!(v.clone().mod_power_of_2_entrywise_mul(w.clone(), pow), r);
        let mut x = v.clone();
        x.mod_power_of_2_entrywise_mul_assign(&w, pow);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.mod_power_of_2_entrywise_mul_assign(w.clone(), pow);
        assert_eq!(x, r);

        // The result is reduced, has the same dimension, and is the elementwise modular product.
        assert!(r.mod_power_of_2_is_reduced(pow));
        assert_eq!(r.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(*z, x.mod_power_of_2_mul(y, pow));
        }
        // It is the ordinary entrywise product, reduced, and it is `mod_entrywise_mul` with modulus
        // $2^k$.
        assert_eq!((&v).entrywise_mul(&w).mod_power_of_2(pow), r);
        assert_eq!((&v).mod_entrywise_mul(&w, Natural::power_of_2(pow)), r);
        // It is commutative.
        assert_eq!((&w).mod_power_of_2_entrywise_mul(&v, pow), r);
        // The vector of 1s is the identity, and the zero vector absorbs everything.
        let ones = NaturalVector {
            elements: vec![Natural::ONE.mod_power_of_2(pow); v.elements.len()],
        };
        assert_eq!((&v).mod_power_of_2_entrywise_mul(&ones, pow), v);
        let zero = NaturalVector::zero(v.dimension());
        assert_eq!((&v).mod_power_of_2_entrywise_mul(&zero, pow), zero);
    });

    natural_vector_natural_vector_natural_vector_unsigned_quadruple_gen_var_1().test_properties(
        |(u, v, w, pow)| {
            // It is associative, and it distributes over modular addition.
            assert_eq!(
                (&u).mod_power_of_2_entrywise_mul(&v, pow)
                    .mod_power_of_2_entrywise_mul(&w, pow),
                (&u).mod_power_of_2_entrywise_mul((&v).mod_power_of_2_entrywise_mul(&w, pow), pow)
            );
            assert_eq!(
                (&u).mod_power_of_2_entrywise_mul((&v).mod_power_of_2_add(&w, pow), pow),
                (&u).mod_power_of_2_entrywise_mul(&v, pow)
                    .mod_power_of_2_add((&u).mod_power_of_2_entrywise_mul(&w, pow), pow)
            );
        },
    );

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<u64>().test_properties(
        |(v, w, pow)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalVector::from((&v).mod_power_of_2_entrywise_mul(&w, pow)),
                NaturalVector::from(v).mod_power_of_2_entrywise_mul(NaturalVector::from(w), pow)
            );
        },
    );
}
