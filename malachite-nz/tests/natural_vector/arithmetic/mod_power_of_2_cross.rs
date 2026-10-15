// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Cross, ModPowerOf2CrossAssign, ModPowerOf2Dot, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2Neg, ModPowerOf2Square, ModPowerOf2Sub,
};
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_unsigned_triple_gen_var_2, natural_vector_unsigned_pair_gen_var_5,
};
use malachite_nz::test_util::natural_vector::arithmetic::mod_power_of_2_cross::*;

fn check(v: &NaturalVector, w: &NaturalVector, pow: u64) -> NaturalVector {
    let r = v.mod_power_of_2_cross(w, pow);
    assert!(r.mod_power_of_2_is_reduced(pow));
    assert_eq!(v.mod_power_of_2_cross(w.clone(), pow), r);
    assert_eq!(v.clone().mod_power_of_2_cross(w, pow), r);
    assert_eq!(v.clone().mod_power_of_2_cross(w.clone(), pow), r);
    let mut s = v.clone();
    s.mod_power_of_2_cross_assign(w, pow);
    assert_eq!(s, r);
    let mut s = v.clone();
    s.mod_power_of_2_cross_assign(w.clone(), pow);
    assert_eq!(s, r);
    r
}

#[test]
fn test_mod_power_of_2_cross() {
    let test = |s, t, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
        let r = check(&v, &w, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!(natural_vector_mod_power_of_2_cross_naive(&v, &w, pow), r);
    };
    test("(0, 0, 0)", "(1, 2, 3)", 2, "(0, 0, 0)");
    test("(0, 0, 0)", "(0, 0, 0)", 0, "(0, 0, 0)");
    test("(1, 0, 0)", "(0, 1, 0)", 1, "(0, 0, 1)");
    test("(1, 0, 0)", "(0, 1, 0)", 8, "(0, 0, 1)");
    test("(0, 1, 0)", "(1, 0, 0)", 8, "(0, 0, 255)");
    test("(1, 2, 3)", "(4, 5, 6)", 4, "(13, 6, 13)");
    test("(1, 2, 3)", "(4, 5, 6)", 8, "(253, 6, 253)");
    test("(3, 5, 7)", "(7, 5, 3)", 3, "(4, 0, 4)");
    test("(1, 2, 3)", "(2, 4, 6)", 8, "(0, 0, 0)");
    test("(255, 255, 255)", "(1, 2, 3)", 8, "(255, 2, 255)");
    test("(200, 100, 50)", "(25, 75, 125)", 8, "(46, 58, 212)");
    test("(1, 1, 1)", "(1, 1, 1)", 1, "(0, 0, 0)");
    test(
        "(18446744073709551616, 18446744073709551615, 1)",
        "(3, 18446744073709551621, 7)",
        100,
        "(110680464442257309684, 1267650600099102192980736344067, 36893488147419103235)",
    );
    test(
        "(633825300114114700748351602688, 1, 316912650057057350374175801344)",
        "(158456325028528675187087900672, 633825300114114700748351602691, 5)",
        100,
        "(316912650057057350374175801349, 633825300114114700748351602688, \
        475368975085586025561263702016)",
    );
    test(
        "(18446744073709551615, 18446744073709551615, 18446744073709551615)",
        "(1, 2, 3)",
        64,
        "(18446744073709551615, 2, 18446744073709551615)",
    );
    test(
        "(18446744073709551615, 2, 18446744073709551613)",
        "(18446744073709551611, 18446744073709551609, 9)",
        65,
        "(36893488147419103229, 18446744073709551640, 17)",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_cross_fail_1() {
    // The vectors are not 3-dimensional.
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_power_of_2_cross(NaturalVector::from_str("(1, 2)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_cross_fail_2() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 2, 8)")
        .unwrap()
        .mod_power_of_2_cross(NaturalVector::from_str("(1, 2, 3)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_cross_assign_fail() {
    let mut v = NaturalVector::from_str("(1, 2, 3, 4)").unwrap();
    v.mod_power_of_2_cross_assign(NaturalVector::from_str("(1, 2, 3, 4)").unwrap(), 3);
}

#[test]
fn mod_power_of_2_cross_properties() {
    natural_vector_natural_vector_unsigned_triple_gen_var_2().test_properties(|(v, w, pow)| {
        let r = check(&v, &w, pow);
        // It is the exact cross product, reduced.
        assert_eq!(natural_vector_mod_power_of_2_cross_naive(&v, &w, pow), r);
        // Swapping the vectors negates the product.
        assert_eq!(
            (&w).mod_power_of_2_cross(&v, pow),
            (&r).mod_power_of_2_neg(pow)
        );
        // The product is perpendicular to both vectors.
        assert_eq!((&r).mod_power_of_2_dot(&v, pow), 0u32);
        assert_eq!((&r).mod_power_of_2_dot(&w, pow), 0u32);
        // Lagrange's identity: |v x w|^2 = |v|^2 |w|^2 - (v . w)^2, modulo 2^pow.
        let vw = (&v).mod_power_of_2_dot(&w, pow);
        assert_eq!(
            (&r).mod_power_of_2_dot(&r, pow),
            (&v).mod_power_of_2_dot(&v, pow)
                .mod_power_of_2_mul((&w).mod_power_of_2_dot(&w, pow), pow)
                .mod_power_of_2_sub(vw.mod_power_of_2_square(pow), pow)
        );
    });

    natural_vector_unsigned_pair_gen_var_5().test_properties(|(v, pow)| {
        // A vector's product with itself, or with zero, is zero.
        let zero = NaturalVector::zero(3);
        assert!((&v).mod_power_of_2_cross(&v, pow).is_zero());
        assert!((&v).mod_power_of_2_cross(&zero, pow).is_zero());
        assert!((&zero).mod_power_of_2_cross(&v, pow).is_zero());
    });

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_3::<Limb>().test_properties(
        |(v, w, pow)| {
            // It agrees with the UnsignedVector implementation.
            let r = (&v).mod_power_of_2_cross(&w, pow);
            assert_eq!(
                NaturalVector::from(v).mod_power_of_2_cross(NaturalVector::from(w), pow),
                NaturalVector::from(r)
            );
        },
    );
}
