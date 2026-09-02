// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::Float;
use malachite_float::test_util::generators::{
    float_gaussian_rational_pair_gen, float_rational_pair_gen,
};
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_gaussian_rational() {
    let test = |s, t, cmp: Option<Ordering>| {
        let x = Float::from_str(s).unwrap();
        let y = GaussianRational::from_str(t).unwrap();
        assert_eq!(x.partial_cmp_abs(&y), cmp);
        assert_eq!(y.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.lt_abs(&y), cmp == Some(Less));
        assert_eq!(x.gt_abs(&y), cmp == Some(Greater));
        assert_eq!(x.eq_abs(&y), cmp == Some(Equal));
    };
    test("0.0", "0", Some(Equal));
    test("-0.0", "0", Some(Equal));
    test("1.0", "i", Some(Equal));
    test("-0.5", "i/2", Some(Equal));
    test("1.0", "3/5+4i/5", Some(Equal));
    test("0.75", "3/5+4i/5", Some(Less));
    test("1.25", "3/5+4i/5", Some(Greater));
    test("2.5", "3/2+2i", Some(Equal));
    test("2.0", "3/2+2i", Some(Less));
    test("-3.0", "3/2+2i", Some(Greater));
    test("0.5", "3/10+2i/5", Some(Equal));
    test("5.0", "3+4i", Some(Equal));
    test("NaN", "0", None);
    test("NaN", "3/2+2i", None);
    test("Infinity", "3/2+2i", Some(Greater));
    test("-Infinity", "0", Some(Greater));
}

#[test]
fn partial_cmp_abs_gaussian_rational_properties() {
    float_gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp_abs(&y);
        assert_eq!(y.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.eq_abs(&y), cmp == Some(Equal));
        assert_eq!((-&x).partial_cmp_abs(&y), cmp);
        assert_eq!(x.partial_cmp_abs(&-&y), cmp);
        assert_eq!(x.partial_cmp_abs(&(&y).conjugate()), cmp);
        if x.is_nan() {
            assert_eq!(cmp, None);
        } else if !x.is_finite() {
            assert_eq!(cmp, Some(Greater));
        } else {
            assert_eq!(
                Rational::exact_from(&x)
                    .abs_squared()
                    .partial_cmp(&(&y).abs_squared()),
                cmp
            );
        }
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(x.partial_cmp_abs(&Float::NAN), None);
        assert_eq!(Float::NAN.partial_cmp_abs(&x), None);
        assert!(x.ge_abs(&Float::ZERO));
        assert!(x.lt_abs(&Float::INFINITY));
        assert!(x.lt_abs(&Float::NEGATIVE_INFINITY));
    });

    float_rational_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            x.partial_cmp_abs(&GaussianRational::from(y.clone())),
            x.partial_cmp_abs(&y)
        );
    });
}
