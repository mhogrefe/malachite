// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::test_util::generators::rational_primitive_float_pair_gen;
use malachite_q::Rational;
use rug;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_partial_eq_primitive_float() {
    let test = |u, v: f32, out| {
        assert_eq!(Rational::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Rational::from_str(u).unwrap() == v, out);

        assert_eq!(v == Rational::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Rational::from_str(u).unwrap(), out);

        let v = f64::from(v);
        assert_eq!(Rational::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Rational::from_str(u).unwrap() == v, out);

        assert_eq!(v == Rational::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Rational::from_str(u).unwrap(), out);
    };
    test("2/3", f32::NAN, false);
    test("2/3", f32::INFINITY, false);
    test("2/3", f32::NEGATIVE_INFINITY, false);
    test("-2/3", f32::NAN, false);
    test("-2/3", f32::INFINITY, false);
    test("-2/3", f32::NEGATIVE_INFINITY, false);

    test("0", 0.0, true);
    test("0", -0.0, true);
    test("0", 5.0, false);
    test("0", -5.0, false);
    test("3/2", 1.5, true);
    test("3/2", 5.0, false);
    test("3/2", -1.5, false);
    test("-3/2", 1.5, false);
    test("-3/2", 5.0, false);
    test("-3/2", -1.5, true);
}

#[allow(clippy::cmp_owned, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_float_properties_helper<
    T: PartialEq<Rational> + PartialEq<rug::Rational> + PrimitiveFloat,
>()
where
    Rational: TryFrom<T> + PartialEq<T> + PartialOrd<T>,
    rug::Rational: PartialEq<T>,
{
    rational_primitive_float_pair_gen::<T>().test_properties(|(n, f)| {
        let eq = n == f;
        assert_eq!(rug::Rational::from(&n) == f, eq);
        assert_eq!(f == n, eq);
        assert_eq!(f == rug::Rational::from(&n), eq);
        assert_eq!(n.partial_cmp(&f) == Some(Equal), eq);
        if f.is_finite() {
            assert_eq!(PartialEq::<Rational>::eq(&n, &Rational::exact_from(f)), eq);
        }
    });
}

#[test]
fn partial_eq_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_eq_primitive_float_properties_helper);
}
