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
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_unsigned_triple_gen_var_1;

#[test]
fn test_mod_power_of_2_neg() {
    let test = |s, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = (&p).mod_power_of_2_neg(pow);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().mod_power_of_2_neg(pow), q);
        let mut r = p;
        r.mod_power_of_2_neg_assign(pow);
        assert_eq!(r, q);
    };
    test("0", 0, "0");
    test("0", 3, "0");
    test("1", 1, "1");
    test("5*x^2+x+3", 3, "3*x^2+7*x+5");
    test("x", 8, "255*x");
    test("x^2+2", 2, "3*x^2+2");
    test("4*x^3+4", 3, "4*x^3+4");
    test(
        "3*x^2+5",
        100,
        "1267650600228229401496703205373*x^2+1267650600228229401496703205371",
    );
    test(
        "1000000000000000000000*x+1",
        70,
        "180591620717411303424*x+1180591620717411303423",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail() {
    // A coefficient is not reduced.
    (&NaturalPolynomial::from_str("8*x+1").unwrap()).mod_power_of_2_neg(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_assign_fail() {
    // A coefficient is not reduced.
    let mut p = NaturalPolynomial::from_str("8*x+1").unwrap();
    p.mod_power_of_2_neg_assign(3);
}

#[test]
fn mod_power_of_2_neg_properties() {
    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, _, pow)| {
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
            assert_eq!(*d, c.mod_power_of_2_neg(pow));
            assert_eq!(c.mod_power_of_2_add(d, pow), Natural::ZERO);
        }
        // Negating twice gives the polynomial back.
        assert_eq!((&q).mod_power_of_2_neg(pow), p);
        // It is the negation over the integers, reduced.
        assert_eq!((-IntegerPolynomial::from(p.clone())).mod_power_of_2(pow), q);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_1::<u64>().test_properties(
        |(p, _, pow)| {
            // The u64 and Natural versions agree.
            assert_eq!(
                NaturalPolynomial::from((&p).mod_power_of_2_neg(pow)),
                NaturalPolynomial::from(p).mod_power_of_2_neg(pow)
            );
        },
    );
}
