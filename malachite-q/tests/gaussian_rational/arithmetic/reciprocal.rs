// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    AbsSquared, Conjugate, DivI, MulI, Reciprocal, ReciprocalAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{gaussian_rational_gen_var_3, rational_gen_var_1};
use std::str::FromStr;

#[test]
fn test_reciprocal() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let reciprocal = x.clone().reciprocal();
        assert!(reciprocal.real.is_valid());
        assert!(reciprocal.imaginary.is_valid());
        assert_eq!(reciprocal.to_string(), out);

        let reciprocal = (&x).reciprocal();
        assert!(reciprocal.real.is_valid());
        assert!(reciprocal.imaginary.is_valid());
        assert_eq!(reciprocal.to_string(), out);

        let mut x = x;
        x.reciprocal_assign();
        assert!(x.real.is_valid());
        assert!(x.imaginary.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("1", "1");
    test("-1", "-1");
    test("i", "-i");
    test("-i", "i");
    test("2", "1/2");
    test("2i", "-i/2");
    test("22/7", "7/22");
    test("-22i/7", "7i/22");
    test("1+i", "1/2-i/2");
    test("1-i", "1/2+i/2");
    test("3+4i", "3/25-4i/25");
    test("3/5+4i/5", "3/5-4i/5");
    test("1/2+i/3", "18/13-12i/13");
    test("-2+3i", "-2/13-3i/13");
}

#[test]
#[should_panic]
fn reciprocal_fail() {
    GaussianRational::ZERO.reciprocal();
}

#[test]
#[should_panic]
fn reciprocal_ref_fail() {
    (&GaussianRational::ZERO).reciprocal();
}

#[test]
#[should_panic]
fn reciprocal_assign_fail() {
    let mut x = GaussianRational::ZERO;
    x.reciprocal_assign();
}

#[test]
fn reciprocal_properties() {
    gaussian_rational_gen_var_3().test_properties(|x| {
        let reciprocal = x.clone().reciprocal();
        assert!(reciprocal.real.is_valid());
        assert!(reciprocal.imaginary.is_valid());
        assert_eq!((&x).reciprocal(), reciprocal);
        let mut x_alt = x.clone();
        x_alt.reciprocal_assign();
        assert_eq!(x_alt, reciprocal);

        assert_eq!(&x * &reciprocal, GaussianRational::ONE);
        assert_eq!((&reciprocal).reciprocal(), x);
        assert_eq!((&reciprocal).abs_squared(), (&x).abs_squared().reciprocal());
        assert_eq!((&reciprocal).conjugate(), (&x).conjugate().reciprocal());
        assert_eq!((-&x).reciprocal(), -&reciprocal);
        assert_eq!((&x).mul_i().reciprocal(), (&reciprocal).div_i());
        assert_eq!(
            reciprocal,
            (&x).conjugate() * GaussianRational::from((&x).abs_squared().reciprocal())
        );
    });

    rational_gen_var_1().test_properties(|x| {
        assert_eq!(
            GaussianRational::from(x.clone()).reciprocal(),
            GaussianRational::from((&x).reciprocal())
        );
        assert_eq!(
            GaussianRational::from(x.clone()).mul_i().reciprocal(),
            GaussianRational::from(Rational::from(0u32) - x.reciprocal()).mul_i()
        );
    });
}
