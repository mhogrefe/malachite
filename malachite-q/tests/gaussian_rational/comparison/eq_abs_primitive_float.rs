// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_primitive_float_pair_gen,
};
use std::str::FromStr;

#[test]
fn test_eq_abs_f32() {
    let test = |s, v: f32, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x.eq_abs(&v), out);
        assert_eq!(v.eq_abs(&x), out);
    };
    test("0", 0.0, true);
    test("0", -0.0, true);
    test("i", 1.0, true);
    test("-1/2", 0.5, true);
    test("3/5+4i/5", 1.0, true);
    test("3/5+4i/5", -1.0, true);
    test("3/10+2i/5", 0.5, true);
    test("3/10+2i/5", -0.5, true);
    test("3/10+2i/5", 0.4, false);
    test("3+4i", 5.0, true);
    test("2+2i", 3.0, false);
    test("0", f32::NAN, false);
    test("0", f32::INFINITY, false);
    test("3+4i", f32::NEG_INFINITY, false);
}

#[test]
fn test_eq_abs_f64() {
    let test = |s, v: f64, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(x.eq_abs(&v), out);
        assert_eq!(v.eq_abs(&x), out);
    };
    test("0", 0.0, true);
    test("3/2+2i", 2.5, true);
    test("3/2+2i", -2.5, true);
    test("3/2+2i", 2.0, false);
    test("5+12i", 13.0, true);
    test("0", f64::NAN, false);
    test("0", f64::NEG_INFINITY, false);
}

fn eq_abs_primitive_float_properties_helper<T: EqAbs<GaussianRational> + PrimitiveFloat>()
where
    GaussianRational: EqAbs<T>,
    Rational: TryFrom<T>,
{
    gaussian_rational_primitive_float_pair_gen::<T>().test_properties(|(x, f)| {
        let eq = x.eq_abs(&f);
        assert_eq!(f.eq_abs(&x), eq);
        assert_eq!(x.eq_abs(&-f), eq);
        assert_eq!((&x).conjugate().eq_abs(&f), eq);
        assert_eq!((-&x).eq_abs(&f), eq);
        if let Ok(y) = Rational::try_from(f) {
            assert_eq!((&x).abs_squared() == y.abs_squared(), eq);
        } else {
            assert!(!eq);
        }
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(x.eq_abs(&T::NAN), false);
        assert_eq!(T::NAN.eq_abs(&x), false);
        assert_eq!(x.eq_abs(&T::INFINITY), false);
        assert_eq!(x.eq_abs(&T::NEGATIVE_INFINITY), false);
    });
}

#[test]
fn eq_abs_primitive_float_properties() {
    apply_fn_to_primitive_floats!(eq_abs_primitive_float_properties_helper);
}
