// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

#[test]
fn test_try_from_primitive_float() {
    fn test_ok<T: PrimitiveFloat>(f: T, out: &str)
    where
        GaussianRational: TryFrom<T> + ConvertibleFrom<T>,
    {
        assert_eq!(GaussianRational::try_from(f).ok().unwrap().to_string(), out);
        assert!(GaussianRational::convertible_from(f));
    }
    fn test_err<T: PrimitiveFloat>(f: T)
    where
        GaussianRational: TryFrom<T> + ConvertibleFrom<T>,
    {
        assert!(GaussianRational::try_from(f).is_err());
        assert!(!GaussianRational::convertible_from(f));
    }
    test_ok(0.0f32, "0");
    test_ok(-0.0f32, "0");
    test_ok(123.0f32, "123");
    test_ok(-123.0f32, "-123");
    test_ok(0.5f32, "1/2");
    test_ok(-1.5f64, "-3/2");
    test_ok(0.1f32, "13421773/134217728");

    test_err(f32::NAN);
    test_err(f64::NAN);
    test_err(f32::INFINITY);
    test_err(f64::NEGATIVE_INFINITY);
}

fn from_primitive_float_properties_helper<T: PrimitiveFloat>()
where
    GaussianRational: TryFrom<T> + ConvertibleFrom<T>,
    Rational: TryFrom<T> + ConvertibleFrom<T>,
{
    primitive_float_gen::<T>().test_properties(|f| {
        let og = GaussianRational::try_from(f);
        assert_eq!(og.is_ok(), Rational::convertible_from(f));
        assert_eq!(og.is_ok(), f.is_finite());
        assert_eq!(GaussianRational::convertible_from(f), og.is_ok());
        if let Ok(g) = og {
            assert_eq!(g.imaginary, 0u32);
            assert_eq!(g.real, Rational::exact_from(f));
        }
    });
}

#[test]
fn from_primitive_float_properties() {
    apply_fn_to_primitive_floats!(from_primitive_float_properties_helper);
}
