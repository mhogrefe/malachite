// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_power_of_2_neg() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: &str) {
        let p = UnsignedPolynomial::<T>::from_str(s).unwrap();
        let q = (&p).mod_power_of_2_neg(pow);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_neg(pow), q);
        let mut r = p;
        r.mod_power_of_2_neg_assign(pow);
        assert_eq!(r, q);
    }
    test::<u8>("0", 0, "0");
    test::<u8>("0", 3, "0");
    test::<u8>("1", 1, "1");
    test::<u8>("1", 8, "255");
    test::<u8>("5*x^2+x+3", 3, "3*x^2+7*x+5");
    test::<u8>("x", 8, "255*x");
    test::<u8>("x^2+2", 2, "3*x^2+2");
    test::<u8>("255*x+1", 8, "x+255");
    test::<u8>("4*x^3+4", 3, "4*x^3+4");
    test::<u64>(
        "3*x^2+5",
        64,
        "18446744073709551613*x^2+18446744073709551611",
    );
    test::<u64>("18446744073709551615*x+1", 64, "x+18446744073709551615");
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail_1() {
    // pow is wider than a u8.
    (&UnsignedPolynomial::<u8>::from_str("x+1").unwrap()).mod_power_of_2_neg(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail_2() {
    // A coefficient is not reduced.
    (&UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap()).mod_power_of_2_neg(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_assign_fail() {
    // A coefficient is not reduced.
    let mut p = UnsignedPolynomial::<u8>::from_str("8*x+1").unwrap();
    p.mod_power_of_2_neg_assign(3);
}

fn mod_power_of_2_neg_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<T>().test_properties(|(p, _, pow)| {
        let q = (&p).mod_power_of_2_neg(pow);
        assert!(q.is_valid());
        assert_eq!(p.clone().mod_power_of_2_neg(pow), q);
        let mut r = p.clone();
        r.mod_power_of_2_neg_assign(pow);
        assert_eq!(r, q);

        // The result is reduced, has the same degree, and is the coefficient-wise negation.
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert_eq!(q.degree(), p.degree());
        for i in 0..p.len() {
            let c = p.coefficient(i);
            let d = q.coefficient(i);
            assert_eq!(d, c.mod_power_of_2_neg(pow));
            assert_eq!(c.mod_power_of_2_add(d, pow), T::ZERO);
        }
        // Negating twice gives the polynomial back, and only zero is its own negation modulo 2^pow
        // when pow > 1.
        assert_eq!((&q).mod_power_of_2_neg(pow), p);
        if pow == 0 {
            assert_eq!(q, UnsignedPolynomial::ZERO);
        }
        // Modulo 2^pow, reducing -c is the same as reducing the wrapping negation.
        for i in 0..p.len() {
            assert_eq!(
                q.coefficient(i),
                p.coefficient(i).wrapping_neg().mod_power_of_2(pow)
            );
        }
    });
}

#[test]
fn mod_power_of_2_neg_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_neg_properties_helper);
}
