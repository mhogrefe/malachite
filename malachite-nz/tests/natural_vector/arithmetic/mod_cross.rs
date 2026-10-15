// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModCross, ModCrossAssign, ModDot, ModIsReduced, ModMul, ModNeg, ModSquare, ModSub,
};
use malachite_base::test_util::generators::*;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen_var_4, natural_vector_natural_vector_natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural_vector::arithmetic::mod_cross::*;

fn check(v: &NaturalVector, w: &NaturalVector, m: &Natural) -> NaturalVector {
    let r = v.mod_cross(w, m);
    assert!(r.mod_is_reduced(m));
    assert_eq!(v.mod_cross(w.clone(), m), r);
    assert_eq!(v.mod_cross(w, m.clone()), r);
    assert_eq!(v.clone().mod_cross(w, m), r);
    assert_eq!(v.clone().mod_cross(w.clone(), m.clone()), r);
    let mut s = v.clone();
    s.mod_cross_assign(w, m);
    assert_eq!(s, r);
    let mut s = v.clone();
    s.mod_cross_assign(w.clone(), m.clone());
    assert_eq!(s, r);
    r
}

#[test]
fn test_mod_cross() {
    let test = |s, t, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
        let m = Natural::from_str(m).unwrap();
        let r = check(&v, &w, &m);
        assert_eq!(r.to_string(), out);
        assert_eq!(natural_vector_mod_cross_naive(&v, &w, &m), r);
    };
    test("(0, 0, 0)", "(1, 2, 3)", "5", "(0, 0, 0)");
    test("(0, 0, 0)", "(0, 0, 0)", "1", "(0, 0, 0)");
    test("(1, 0, 0)", "(0, 1, 0)", "2", "(0, 0, 1)");
    test("(1, 0, 0)", "(0, 1, 0)", "7", "(0, 0, 1)");
    test("(0, 1, 0)", "(1, 0, 0)", "7", "(0, 0, 6)");
    test("(1, 2, 3)", "(4, 5, 6)", "7", "(4, 6, 4)");
    test("(1, 2, 3)", "(4, 5, 6)", "12", "(9, 6, 9)");
    test("(3, 5, 7)", "(7, 5, 3)", "10", "(0, 0, 0)");
    test("(1, 2, 3)", "(2, 4, 6)", "251", "(0, 0, 0)");
    test("(254, 254, 254)", "(1, 2, 3)", "255", "(254, 2, 254)");
    test("(200, 100, 50)", "(25, 75, 125)", "251", "(216, 95, 201)");
    test("(1, 1, 1)", "(1, 1, 1)", "2", "(0, 0, 0)");
    test("(5, 0, 3)", "(2, 4, 1)", "6", "(0, 1, 2)");
    test(
        "(18446744073709551616, 18446744073709551615, 1)",
        "(3, 18446744073709551621, 7)",
        "1267650600228229401496703205379",
        "(110680464442257309684, 1267650600099102192980736344070, 36893488146613796867)",
    );
    test(
        "(633825300114114700748351602688, 1, 316912650057057350374175801344)",
        "(158456325028528675187087900672, 633825300114114700748351602691, 5)",
        "1267650600228229401496703205653",
        "(1109194275199700726309615295568, 1069580193942568557512843332860, \
        158456325028528675187087919508)",
    );
    test(
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        "(1, 2, 3)",
        "18446744073709551615",
        "(18446744073709551614, 2, 18446744073709551614)",
    );
    test(
        "(18446744073709551615, 2, 18446744073709551613)",
        "(18446744073709551611, 18446744073709551609, 9)",
        "18446744073709551617",
        "(18446744073709551603, 42, 28)",
    );
    test(
        "(1000000000000000000000000, 3, 7)",
        "(5, 100000000000000000000000, 11)",
        "10000000000000000000000000",
        "(9300000000000000000000033, 9000000000000000000000035, 9999999999999999999999985)",
    );
}

#[test]
#[should_panic]
fn mod_cross_fail_1() {
    // The vectors are not 3-dimensional.
    NaturalVector::from_str("(1, 2)").unwrap().mod_cross(
        NaturalVector::from_str("(1, 2)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_cross_fail_2() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 2, 7)").unwrap().mod_cross(
        NaturalVector::from_str("(1, 2, 3)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_cross_assign_fail() {
    let mut v = NaturalVector::from_str("(1, 2, 3, 4)").unwrap();
    v.mod_cross_assign(
        NaturalVector::from_str("(1, 2, 3, 4)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_cross_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_3().test_properties(|(v, w, m)| {
        let r = check(&v, &w, &m);
        // It is the exact cross product, reduced.
        assert_eq!(natural_vector_mod_cross_naive(&v, &w, &m), r);
        // Swapping the vectors negates the product.
        assert_eq!((&w).mod_cross(&v, &m), (&r).mod_neg(&m));
        // The product is perpendicular to both vectors.
        assert_eq!((&r).mod_dot(&v, &m), 0u32);
        assert_eq!((&r).mod_dot(&w, &m), 0u32);
        // Lagrange's identity: |v x w|^2 = |v|^2 |w|^2 - (v . w)^2, modulo m.
        let vw = (&v).mod_dot(&w, &m);
        assert_eq!(
            (&r).mod_dot(&r, &m),
            (&v).mod_dot(&v, &m)
                .mod_mul((&w).mod_dot(&w, &m), &m)
                .mod_sub(vw.mod_square(&m), &m)
        );
    });

    natural_vector_natural_pair_gen_var_4().test_properties(|(v, m)| {
        // A vector's product with itself, or with zero, is zero.
        let zero = NaturalVector::zero(3);
        assert!((&v).mod_cross(&v, &m).is_zero());
        assert!((&v).mod_cross(&zero, &m).is_zero());
        assert!((&zero).mod_cross(&v, &m).is_zero());
    });

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_4::<Limb>().test_properties(
        |(v, w, m)| {
            // It agrees with the UnsignedVector implementation.
            let r = (&v).mod_cross(&w, m);
            assert_eq!(
                NaturalVector::from(v).mod_cross(NaturalVector::from(w), Natural::from(m)),
                NaturalVector::from(r)
            );
        },
    );
}
