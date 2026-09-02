// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate, Sign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_primitive_float_pair_gen,
};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_f32() {
    let test = |s, v: f32, cmp: Option<Ordering>| {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(x.partial_cmp_abs(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.lt_abs(&v), cmp == Some(Less));
        assert_eq!(x.gt_abs(&v), cmp == Some(Greater));
        assert_eq!(x.eq_abs(&v), cmp == Some(Equal));
    };
    test("0", 0.0, Some(Equal));
    test("0", -0.0, Some(Equal));
    test("0", 0.5, Some(Less));
    test("i", 1.0, Some(Equal));
    test("i", -1.5, Some(Less));
    test("-123", 123.0, Some(Equal));
    test("3+4i", 5.0, Some(Equal));
    test("3+4i", -5.0, Some(Equal));
    test("3+4i", 4.5, Some(Greater));
    test("3+4i", 5.5, Some(Less));
    test("3+4i", 0.0, Some(Greater));
    test("2+2i", 3.0, Some(Less));
    test("2+2i", 2.75, Some(Greater));
    test("1+i", 1.5, Some(Less));
    test("1+i", 1.25, Some(Greater));
    test("0", f32::NAN, None);
    test("3+4i", f32::NAN, None);
    test("0", f32::INFINITY, Some(Less));
    test("3+4i", f32::NEG_INFINITY, Some(Less));
}

#[test]
fn test_partial_cmp_abs_f64() {
    let test = |s, v: f64, cmp: Option<Ordering>| {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(x.partial_cmp_abs(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
    };
    test("0", 0.0, Some(Equal));
    test("5+12i", 13.0, Some(Equal));
    test("5+12i", 12.999, Some(Greater));
    test("5+12i", 13.001, Some(Less));
    test("1000000000000+i", 1.0e12, Some(Greater));
    test("1000000000000+i", 1.0000000001e12, Some(Less));
    test("0", f64::NAN, None);
    test("1+i", f64::NEG_INFINITY, Some(Less));
}

fn partial_cmp_abs_primitive_float_properties_helper<
    T: PartialOrdAbs<GaussianInteger> + PrimitiveFloat,
>()
where
    GaussianInteger: EqAbs<T> + PartialOrdAbs<T>,
{
    gaussian_integer_primitive_float_pair_gen::<T>().test_properties(|(x, f)| {
        let cmp = x.partial_cmp_abs(&f);
        assert_eq!(f.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
        assert_eq!(x.eq_abs(&f), cmp == Some(Equal));
        assert_eq!(x.partial_cmp_abs(&-f), cmp);
        assert_eq!((&x).conjugate().partial_cmp_abs(&f), cmp);
        assert_eq!((-&x).partial_cmp_abs(&f), cmp);
        if f.is_nan() {
            assert_eq!(cmp, None);
        } else if !f.is_finite() {
            assert_eq!(cmp, Some(Less));
        } else if f == T::ZERO {
            assert_eq!(cmp, Some((&x).abs_squared().sign()));
        } else {
            // Exact oracle: compare |x|^2 * 2^(2k) against m^2 * 2^(2k + 2e) with k large enough
            // that both sides are integers.
            let (m, e) = f.abs().integer_mantissa_and_exponent();
            let m_squared = Natural::from(u128::from(m) * u128::from(m));
            let abs_squared = Natural::exact_from(&(&x).abs_squared());
            let expected = if e >= 0 {
                abs_squared.cmp(&(m_squared << (e.unsigned_abs() << 1)))
            } else {
                (abs_squared << (e.unsigned_abs() << 1)).cmp(&m_squared)
            };
            assert_eq!(cmp, Some(expected));
        }
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(x.partial_cmp_abs(&T::NAN), None);
        assert_eq!(T::NAN.partial_cmp_abs(&x), None);
        assert!(x.ge_abs(&T::ZERO));
        assert!(x.lt_abs(&T::INFINITY));
        assert!(x.lt_abs(&T::NEGATIVE_INFINITY));
    });
}

#[test]
fn partial_cmp_abs_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_abs_primitive_float_properties_helper);
}
