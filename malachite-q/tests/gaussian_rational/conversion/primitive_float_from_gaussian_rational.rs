// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsReal};
use malachite_base::num::float::NiceFloat;
use malachite_base::strings::ToDebugString;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::gaussian_rational::conversion::primitive_float_from_gaussian_rational::*;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::fmt::Debug;
use std::str::FromStr;

fn test_try_from_gaussian_rational_helper<T>(s: &str, out: &str)
where
    T: Debug
        + for<'a> TryFrom<&'a GaussianRational, Error = PrimitiveFloatFromGaussianRationalError>,
{
    let x = GaussianRational::from_str(s).unwrap();
    assert_eq!(T::try_from(&x).to_debug_string(), out);
}

#[test]
fn test_try_from_gaussian_rational() {
    let test = test_try_from_gaussian_rational_helper::<f32>;
    test("0", "Ok(0.0)");
    test("123", "Ok(123.0)");
    test("-123", "Ok(-123.0)");
    test("1/2", "Ok(0.5)");
    test_try_from_gaussian_rational_helper::<f64>("-3/2", "Ok(-1.5)");
    test("1/3", "Err(PrimitiveFloatFromGaussianRationalError)");
    test_try_from_gaussian_rational_helper::<f64>(
        "1/3",
        "Err(PrimitiveFloatFromGaussianRationalError)",
    );
    test("16777217", "Err(PrimitiveFloatFromGaussianRationalError)");
    test_try_from_gaussian_rational_helper::<f64>("16777217", "Ok(16777217.0)");
    test("i", "Err(PrimitiveFloatFromGaussianRationalError)");
    test("i/2", "Err(PrimitiveFloatFromGaussianRationalError)");
    test("2-3i", "Err(PrimitiveFloatFromGaussianRationalError)");
}

#[test]
fn test_exact_from_gaussian_rational() {
    let x = GaussianRational::from_str("1/2").unwrap();
    assert_eq!(NiceFloat(f32::exact_from(&x)), NiceFloat(0.5));
    assert_eq!(NiceFloat(f64::exact_from(&x)), NiceFloat(0.5));
}

#[test]
#[should_panic]
fn f32_exact_from_gaussian_rational_fail() {
    f32::exact_from(&GaussianRational::from_str("1/3").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_gaussian_rational_fail() {
    f64::exact_from(&GaussianRational::from_str("i/2").unwrap());
}

#[test]
fn test_convertible_from_gaussian_rational() {
    fn test<T: for<'a> ConvertibleFrom<&'a GaussianRational>>(s: &str, out: bool) {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(T::convertible_from(&x), out);
    }
    test::<f32>("0", true);
    test::<f32>("123", true);
    test::<f32>("1/2", true);
    test::<f32>("1/3", false);
    test::<f32>("16777217", false);
    test::<f64>("16777217", true);
    test::<f32>("i", false);
    test::<f32>("i/2", false);
    test::<f32>("2-3i", false);
}

fn try_from_gaussian_rational_properties_helper<T>()
where
    T: PrimitiveFloat
        + for<'a> TryFrom<&'a GaussianRational, Error = PrimitiveFloatFromGaussianRationalError>
        + for<'a> TryFrom<&'a Rational>
        + for<'a> ConvertibleFrom<&'a GaussianRational>
        + for<'a> ConvertibleFrom<&'a Rational>,
    GaussianRational: TryFrom<T> + ConvertibleFrom<T>,
{
    gaussian_rational_gen().test_properties(|x| {
        let ot = T::try_from(&x);
        assert_eq!(T::convertible_from(&x), ot.is_ok());
        assert_eq!(ot.is_ok(), x.is_real() && T::convertible_from(&x.real));
        if let Ok(t) = ot {
            assert_eq!(NiceFloat(t), NiceFloat(T::try_from(&x.real).ok().unwrap()));
            assert_eq!(GaussianRational::try_from(t).ok().unwrap(), x);
        }
    });
}

#[test]
fn try_from_gaussian_rational_properties() {
    apply_fn_to_primitive_floats!(try_from_gaussian_rational_properties_helper);
}
