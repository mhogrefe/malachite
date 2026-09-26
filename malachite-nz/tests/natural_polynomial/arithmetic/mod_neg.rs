// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Mod, ModAdd, ModIsReduced, ModNeg, ModNegAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_natural_triple_gen_var_1;

#[test]
fn test_mod_neg() {
    let test = |s, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let q = (&p).mod_neg(&m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).mod_neg(m.clone()), q);
        assert_eq!(p.clone().mod_neg(&m), q);
        assert_eq!(p.clone().mod_neg(m.clone()), q);
        let mut r = p.clone();
        r.mod_neg_assign(&m);
        assert_eq!(r, q);
        let mut r = p;
        r.mod_neg_assign(m);
        assert_eq!(r, q);
    };
    test("0", "1", "0");
    test("0", "7", "0");
    test("1", "2", "1");
    test("5*x^2+x+3", "7", "2*x^2+6*x+4");
    test("x", "255", "254*x");
    test("6*x^3+6", "7", "x^3+1");
    test(
        "3*x^2+5",
        "1000000000000000000000000000057",
        "1000000000000000000000000000054*x^2+1000000000000000000000000000052",
    );
    test(
        "1000000000000000000000000000056*x+1",
        "1000000000000000000000000000057",
        "x+1000000000000000000000000000056",
    );
}

#[test]
#[should_panic]
fn mod_neg_fail_1() {
    // m is 0.
    (&NaturalPolynomial::ZERO).mod_neg(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_neg_fail_2() {
    // A coefficient is not reduced.
    (&NaturalPolynomial::from_str("7*x+1").unwrap()).mod_neg(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_neg_assign_fail() {
    // A coefficient is not reduced.
    let mut p = NaturalPolynomial::from_str("7*x+1").unwrap();
    p.mod_neg_assign(Natural::from(7u32));
}

#[test]
fn mod_neg_properties() {
    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, _, m)| {
        let q = (&p).mod_neg(&m);
        assert!(q.is_valid());
        assert_eq!((&p).mod_neg(m.clone()), q);
        assert_eq!(p.clone().mod_neg(&m), q);
        assert_eq!(p.clone().mod_neg(m.clone()), q);
        let mut r = p.clone();
        r.mod_neg_assign(&m);
        assert_eq!(r, q);
        let mut r = p.clone();
        r.mod_neg_assign(m.clone());
        assert_eq!(r, q);

        // The result is reduced, has the same degree, and is the coefficient-wise negation.
        assert!(q.mod_is_reduced(&m));
        assert_eq!(q.degree(), p.degree());
        for i in 0..p.len() {
            let c = p.coefficient(i);
            let d = q.coefficient(i);
            assert_eq!(*d, c.mod_neg(&m));
            assert_eq!(c.mod_add(d, &m), Natural::ZERO);
        }
        // Negating twice gives the polynomial back.
        assert_eq!((&q).mod_neg(&m), p);
        // It is the negation over the integers, reduced.
        assert_eq!((-IntegerPolynomial::from(p.clone())).mod_op(m), q);
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().test_properties(|(p, _, m)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            NaturalPolynomial::from((&p).mod_neg(m)),
            NaturalPolynomial::from(p).mod_neg(Natural::from(m))
        );
    });
}
