// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, Reciprocal, ReciprocalAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_q::test_util::generators::rational_gen_var_1;
use malachite_q::Rational;
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_reciprocal() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();

        let reciprocal = x.clone().reciprocal();
        assert!(reciprocal.is_valid());
        assert_eq!(reciprocal.to_string(), out);

        let reciprocal = (&x).reciprocal();
        assert!(reciprocal.is_valid());
        assert_eq!(reciprocal.to_string(), out);

        assert_eq!(BigRational::from_str(s).unwrap().recip().to_string(), out);
        assert_eq!(rug::Rational::from_str(s).unwrap().recip().to_string(), out);

        let mut x = x;
        x.reciprocal_assign();
        assert!(reciprocal.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("1", "1");
    test("-1", "-1");
    test("123", "1/123");
    test("22/7", "7/22");
    test("-22/7", "-7/22");
}

#[test]
#[should_panic]
fn reciprocal_fail() {
    Rational::ZERO.reciprocal();
}

#[test]
#[should_panic]
fn reciprocal_ref_fail() {
    (&Rational::ZERO).reciprocal();
}

#[test]
#[should_panic]
fn reciprocal_assign_fail() {
    let mut q = Rational::ZERO;
    q.reciprocal_assign();
}

#[test]
fn reciprocal_properties() {
    rational_gen_var_1().test_properties(|x| {
        let reciprocal = x.clone().reciprocal();
        assert!(reciprocal.is_valid());

        assert_eq!(Rational::from(&BigRational::from(&x).recip()), reciprocal);

        assert_eq!(Rational::from(&rug::Rational::from(&x).recip()), reciprocal);

        let reciprocal_alt = (&x).reciprocal();
        assert!(reciprocal_alt.is_valid());
        assert_eq!(reciprocal_alt, reciprocal);

        let mut reciprocal_alt = x.clone();
        reciprocal_alt.reciprocal_assign();
        assert!(reciprocal_alt.is_valid());
        assert_eq!(reciprocal_alt, reciprocal);

        assert_ne!(reciprocal, 0);
        assert_eq!(reciprocal, -(-&x).reciprocal());
        assert_eq!(reciprocal == x, (&x).abs() == 1);
        assert_eq!((&reciprocal).reciprocal(), x);
    });
}
