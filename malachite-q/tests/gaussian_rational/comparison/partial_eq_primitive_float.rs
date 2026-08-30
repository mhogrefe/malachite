// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_primitive_float_pair_gen,
};
use std::str::FromStr;

#[test]
fn test_partial_eq_f32() {
    let test = |s, v: f32, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x == v, out);
        assert_eq!(v == x, out);
    };
    test("0", 0.0, true);
    test("123", 123.0, true);
    test("-123", -123.0, true);
    test("1/2", 0.5, true);
    test("-1/2", 0.5, false);
    test("1/3", 0.333, false);
    test("123+i", 123.0, false);
    test("1/2+i", 0.5, false);
    test("i", 0.0, false);
    test("0", f32::NAN, false);
    test("0", f32::INFINITY, false);
}

#[test]
fn test_partial_eq_f64() {
    let test = |s, v: f64, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x == v, out);
        assert_eq!(v == x, out);
    };
    test("0", 0.0, true);
    test("1000000000000", 1.0e12, true);
    test("1/1024", 0.0009765625, true);
    test("1000000000000+i", 1.0e12, false);
    test("0", f64::NAN, false);
    test("0", f64::NEG_INFINITY, false);
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_float_properties_helper<T: PartialEq<GaussianRational> + PrimitiveFloat>()
where
    GaussianRational: TryFrom<T> + PartialEq<T>,
{
    gaussian_rational_primitive_float_pair_gen::<T>().test_properties(|(n, f)| {
        let eq = n == f;
        assert_eq!(f == n, eq);
        if let Ok(g) = GaussianRational::try_from(f) {
            assert_eq!(&n == &g, eq);
        } else {
            assert!(!eq);
        }
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_ne!(x, T::NAN);
        assert_ne!(T::NAN, x);
        assert_ne!(x, T::INFINITY);
        assert_ne!(x, T::NEGATIVE_INFINITY);
    });
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_eq_primitive_float_properties_helper);
}
