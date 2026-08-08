// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, ExtendedGcd, Gcd, GcdAssign, Lcm};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::IsInteger;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen};
use std::str::FromStr;

#[test]
fn test_gcd() {
    let test = |s, t, out| {
        let x = Rational::from_str(s).unwrap();
        let y = Rational::from_str(t).unwrap();
        let g = Rational::from_str(out).unwrap();

        assert_eq!(x.clone().gcd(y.clone()), g);
        assert_eq!(x.clone().gcd(&y), g);
        assert_eq!((&x).gcd(y.clone()), g);
        assert_eq!((&x).gcd(&y), g);

        let mut z = x.clone();
        z.gcd_assign(y.clone());
        assert_eq!(z, g);
        let mut z = x;
        z.gcd_assign(&y);
        assert_eq!(z, g);
    };
    // - both zero
    test("0", "0", "0");
    // - one zero: the GCD is the absolute value of the other
    test("0", "-22/7", "22/7");
    test("5/3", "0", "5/3");
    // - integers agree with the integer GCD
    test("12", "-18", "6");
    // - coprime numerators, coprime denominators
    test("2/3", "-3/4", "1/12");
    // - shared factors in both components
    test("4/9", "6/15", "2/45");
    // - equal values
    test("-7/2", "-7/2", "7/2");
    // - multi-limb components
    test(
        "98765432123456789012345678990/12345678987654321012345678901",
        "36925814703692581470369258146/12345678987654321012345678901",
        "2/12345678987654321012345678901",
    );
}

#[test]
fn test_extended_gcd() {
    let test = |s, t, out| {
        let x = Rational::from_str(s).unwrap();
        let y = Rational::from_str(t).unwrap();

        assert_eq!(format!("{:?}", x.clone().extended_gcd(y.clone())), out);
        assert_eq!(format!("{:?}", x.clone().extended_gcd(&y)), out);
        assert_eq!(format!("{:?}", (&x).extended_gcd(y.clone())), out);
        assert_eq!(format!("{:?}", (&x).extended_gcd(&y)), out);
    };
    // - both zero
    test("0", "0", "(0, 0, 0)");
    // - one zero
    test("0", "-22/7", "(22/7, 0, -1)");
    test("5/3", "0", "(5/3, 1, 0)");
    // - integers
    test("12", "-18", "(6, -1, -1)");
    // - the doctest pair
    test("2/3", "-3/4", "(1/12, -1, -1)");
    // - equal values
    test("-7/2", "-7/2", "(7/2, 0, -1)");
}

#[test]
fn gcd_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let g = (&x).gcd(&y);
        assert_eq!(x.clone().gcd(y.clone()), g);
        assert_eq!(x.clone().gcd(&y), g);
        assert_eq!((&x).gcd(y.clone()), g);
        let mut z = x.clone();
        z.gcd_assign(&y);
        assert_eq!(z, g);

        // nonnegative, and zero only when both inputs are zero
        assert!(g >= 0u32);
        assert_eq!(g == 0u32, x == 0u32 && y == 0u32);
        // commutative
        assert_eq!((&y).gcd(&x), g);
        // the GCD divides both into integers
        if g != 0u32 {
            assert!((&x / &g).is_integer());
            assert!((&y / &g).is_integer());
        }
        // sign-blind
        assert_eq!((-&x).gcd(&y), g);
        // component formula
        assert_eq!(
            g,
            Rational::from_naturals(
                x.to_numerator().gcd(y.to_numerator()),
                x.to_denominator().lcm(y.to_denominator()),
            )
        );
        // the GCD with zero is the absolute value
        assert_eq!((&x).gcd(Rational::ZERO), (&x).abs());
    });

    rational_gen().test_properties(|x| {
        assert_eq!((&x).gcd(&x), (&x).abs());
        assert_eq!((&x).gcd(Rational::ONE) == 1u32, x.is_integer());
    });
}

#[test]
fn extended_gcd_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let (g, u, v) = (&x).extended_gcd(&y);
        assert_eq!(
            x.clone().extended_gcd(y.clone()),
            (g.clone(), u.clone(), v.clone())
        );

        // the GCD agrees with Gcd
        assert_eq!(g, (&x).gcd(&y));
        // the Bézout identity, with integer cofactors
        assert_eq!(Rational::from(&u) * &x + Rational::from(&v) * &y, g);
        if g != 0u32 {
            // the quotients are coprime integers, as FLINT's cofactor function returns
            let xq = &x / &g;
            let yq = &y / &g;
            assert!(xq.is_integer());
            assert!(yq.is_integer());
            assert_eq!(xq.into_numerator().gcd(yq.into_numerator()), Natural::ONE);
        }
    });
}
