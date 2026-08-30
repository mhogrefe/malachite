// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::Float;
use malachite_float::test_util::generators::{
    float_gaussian_integer_pair_gen, float_integer_pair_gen,
};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use std::str::FromStr;

#[test]
fn test_eq_abs_gaussian_integer() {
    let test = |s, t, out| {
        let x = Float::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();
        assert_eq!(x.eq_abs(&y), out);
        assert_eq!(y.eq_abs(&x), out);
    };
    test("0.0", "0", true);
    test("-0.0", "0", true);
    test("1.0", "i", true);
    test("-1.0", "i", true);
    test("-123.0", "123", true);
    test("5.0", "3+4i", true);
    test("-5.0", "3+4i", true);
    test("4.0", "3+4i", false);
    test("5.5", "3+4i", false);
    test("13.0", "5+12i", true);
    test("3.0", "2+2i", false);
    test("0.5", "123", false);
    test("NaN", "0", false);
    test("Infinity", "3+4i", false);
    test("-Infinity", "0", false);
}

#[test]
fn eq_abs_gaussian_integer_properties() {
    float_gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let eq = x.eq_abs(&y);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!((-&x).eq_abs(&y), eq);
        assert_eq!(x.eq_abs(&-&y), eq);
        assert_eq!(x.eq_abs(&(&y).conjugate()), eq);
        assert_eq!(x.eq_abs(&GaussianRational::from(&y)), eq);
        if x.is_finite() {
            assert_eq!(
                Rational::exact_from(&x).abs_squared() == (&y).abs_squared(),
                eq
            );
        } else {
            assert!(!eq);
        }
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(x.eq_abs(&Float::NAN), false);
        assert_eq!(Float::NAN.eq_abs(&x), false);
        assert_eq!(x.eq_abs(&Float::INFINITY), false);
        assert_eq!(x.eq_abs(&Float::NEGATIVE_INFINITY), false);
    });

    float_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(x.eq_abs(&GaussianInteger::from(y.clone())), x.eq_abs(&y));
        assert_eq!(GaussianInteger::from(y.clone()).eq_abs(&x), y.eq_abs(&x));
    });
}
