// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, Mod, ModIsReduced, ModSquare};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ModMulTruncated, ModPowerOf2SquareTruncated, ModSquareTruncated, ModSquareTruncatedAssign,
    Polynomial, SquareTruncated,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_square_truncated::*;

// Checks every form of `mod_square_truncated` and `mod_square_truncated_assign` against `r`.
fn check_forms(p: &NaturalPolynomial, len: u64, m: &Natural, r: &NaturalPolynomial) {
    assert_eq!(&p.clone().mod_square_truncated(len, m.clone()), r);
    assert_eq!(&p.clone().mod_square_truncated(len, m), r);
    assert_eq!(&p.mod_square_truncated(len, m.clone()), r);
    assert_eq!(&p.mod_square_truncated(len, m), r);
    let mut s = p.clone();
    s.mod_square_truncated_assign(len, m.clone());
    assert!(s.is_valid());
    assert_eq!(&s, r);
    let mut s = p.clone();
    s.mod_square_truncated_assign(len, m);
    assert_eq!(&s, r);
}

#[test]
fn test_mod_square_truncated() {
    let test = |s, len: u64, m: u32, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from(m);
        let r = (&p).mod_square_truncated(len, &m);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        check_forms(&p, len, &m, &r);
        assert_eq!(mod_square_truncated_naive(&p, len, &m), r);
    };
    test("x^2+3*x+2", 0, 7, "0");
    test("x^2+3*x+2", 3, 7, "6*x^2+5*x+4");
    test("x^2+3*x+2", 10, 7, "x^4+6*x^3+6*x^2+5*x+4");
    // A constant, which `mod_square_truncated_assign` squares in place.
    test("5", 1, 7, "4");
    test("5", 0, 7, "0");
    // A constant whose square vanishes.
    test("3", 1, 9, "0");
    test("3*x+1", 3, 9, "6*x+1");
}

#[test]
#[should_panic]
fn mod_square_truncated_fail_1() {
    NaturalPolynomial::from_str("x+7")
        .unwrap()
        .mod_square_truncated(2, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_square_truncated_fail_2() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_square_truncated(2, Natural::ZERO);
}

#[test]
fn mod_square_truncated_properties() {
    natural_polynomial_unsigned_natural_triple_gen_var_1::<u64>().test_properties(|(p, len, m)| {
        let r = (&p).mod_square_truncated(len, &m);
        assert!(r.is_valid());
        assert!(r.mod_is_reduced(&m));
        check_forms(&p, len, &m, &r);
        assert_eq!(mod_square_truncated_naive(&p, len, &m), r);
        assert_eq!((&p).square_truncated(len).mod_op(&m), r);
        assert_eq!((&p).mod_square(&m).truncate(len), r);
        assert_eq!((&p).mod_mul_truncated(&p, len, &m), r);
        if let Some(pow) = (&m).checked_log_base_2() {
            assert_eq!((&p).mod_power_of_2_square_truncated(len, pow), r);
        }
        assert!(r.len() <= len);
    });
}
