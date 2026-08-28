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
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;

#[test]
fn test_try_from_primitive_float() {
    fn test_ok<T: PrimitiveFloat>(f: T, out: &str)
    where
        GaussianInteger: TryFrom<T> + ConvertibleFrom<T>,
    {
        assert_eq!(GaussianInteger::try_from(f).ok().unwrap().to_string(), out);
        assert!(GaussianInteger::convertible_from(f));
    }
    fn test_err<T: PrimitiveFloat>(f: T)
    where
        GaussianInteger: TryFrom<T> + ConvertibleFrom<T>,
    {
        assert!(GaussianInteger::try_from(f).is_err());
        assert!(!GaussianInteger::convertible_from(f));
    }
    test_ok(0.0f32, "0");
    test_ok(-0.0f32, "0");
    test_ok(123.0f32, "123");
    test_ok(-123.0f32, "-123");
    test_ok(1.0e9f32, "1000000000");
    test_ok(123.0f64, "123");
    test_ok(1.0e9f64, "1000000000");

    test_err(0.5f32);
    test_err(-100.1f64);
    test_err(f32::NAN);
    test_err(f64::NAN);
    test_err(f32::INFINITY);
    test_err(f64::NEGATIVE_INFINITY);
}

fn from_primitive_float_properties_helper<T: PrimitiveFloat>()
where
    GaussianInteger: TryFrom<T> + ConvertibleFrom<T>,
    Integer: TryFrom<T> + ConvertibleFrom<T>,
{
    primitive_float_gen::<T>().test_properties(|f| {
        let og = GaussianInteger::try_from(f);
        assert_eq!(og.is_ok(), Integer::convertible_from(f));
        assert_eq!(og.is_ok(), f.is_integer());
        assert_eq!(GaussianInteger::convertible_from(f), og.is_ok());
        if let Ok(g) = og {
            assert_eq!(g.imaginary, 0u32);
            assert_eq!(g.real, Integer::exact_from(f));
        }
    });
}

#[test]
fn from_primitive_float_properties() {
    apply_fn_to_primitive_floats!(from_primitive_float_properties_helper);
}
