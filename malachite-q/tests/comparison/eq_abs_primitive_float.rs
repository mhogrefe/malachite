// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_q::test_util::generators::{rational_gen, rational_primitive_float_pair_gen};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_eq_abs_f32() {
    let test = |u, v: f32, out| {
        assert_eq!(Rational::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Rational::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Rational::from_str(u).unwrap()), out);
    };
    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("3/2", 1.5, true);
    test("3/2", -1.5, true);
    test("3/2", 2.5, false);
    test("3/2", -2.5, false);
    test("1000000000000", 123.0, false);
    test("1000000000000", -123.0, false);
    test("1", 0.5, false);
    test("1", -0.5, false);
    test("1", f32::INFINITY, false);
    test("1", f32::NEGATIVE_INFINITY, false);
    test("1", f32::NAN, false);
    test("-3/2", 1.5, true);
    test("-3/2", -1.5, true);
    test("-3/2", 2.5, false);
    test("-3/2", -2.5, false);
    test("-1000000000000", 123.0, false);
    test("-1000000000000", -123.0, false);
    test("-1", 0.5, false);
    test("-1", -0.5, false);
    test("-1", f32::INFINITY, false);
    test("-1", f32::NEGATIVE_INFINITY, false);
    test("-1", f32::NAN, false);
}

#[test]
fn test_eq_abs_f64() {
    let test = |u, v: f64, out| {
        assert_eq!(Rational::from_str(u).unwrap().eq_abs(&v), out);
        assert_eq!(!Rational::from_str(u).unwrap().ne_abs(&v), out);
        assert_eq!(v.eq_abs(&Rational::from_str(u).unwrap()), out);
    };
    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("3/2", 1.5, true);
    test("3/2", -1.5, true);
    test("3/2", 2.5, false);
    test("3/2", -2.5, false);
    test("1000000000000", 123.0, false);
    test("1000000000000", -123.0, false);
    test("1", 0.5, false);
    test("1", -0.5, false);
    test("1", f64::INFINITY, false);
    test("1", f64::NEGATIVE_INFINITY, false);
    test("1", f64::NAN, false);
    test("-3/2", 1.5, true);
    test("-3/2", -1.5, true);
    test("-3/2", 2.5, false);
    test("-3/2", -2.5, false);
    test("-1000000000000", 123.0, false);
    test("-1000000000000", -123.0, false);
    test("-1", 0.5, false);
    test("-1", -0.5, false);
    test("-1", f64::INFINITY, false);
    test("-1", f64::NEGATIVE_INFINITY, false);
    test("-1", f64::NAN, false);
}

fn eq_abs_primitive_float_properties_helper<
    T: EqAbs<Rational> + PartialEq<Rational> + PrimitiveFloat,
>()
where
    Rational: EqAbs<T> + PartialEq<T> + PartialOrdAbs<T>,
{
    rational_primitive_float_pair_gen::<T>().test_properties(|(n, x)| {
        let eq = n.eq_abs(&x);
        assert_ne!(n.ne_abs(&x), eq);

        assert_eq!(x.eq_abs(&n), eq);
        assert_eq!(n.partial_cmp_abs(&x) == Some(Equal), eq);
    });

    rational_gen().test_properties(|n| {
        assert_ne!(n, T::NAN);
        assert_ne!(n, T::INFINITY);
        assert_ne!(n, T::NEGATIVE_INFINITY);
    });
}

#[test]
fn eq_abs_primitive_float_properties() {
    apply_fn_to_primitive_floats!(eq_abs_primitive_float_properties_helper);
}
