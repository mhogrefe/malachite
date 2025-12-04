// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_pair_gen, primitive_float_triple_gen,
};
use std::cmp::Ordering::*;

#[test]
pub fn test_cmp_abs() {
    let test = |s, t, cmp| {
        let u = NiceFloat(s);
        let v = NiceFloat(t);
        assert_eq!(u.cmp_abs(&v), cmp);
        assert_eq!(v.cmp_abs(&u), cmp.reverse());

        assert_eq!(u.lt_abs(&v), cmp == Less);
        assert_eq!(u.gt_abs(&v), cmp == Greater);
        assert_eq!(u.le_abs(&v), cmp != Greater);
        assert_eq!(u.ge_abs(&v), cmp != Less);
    };
    test(0.0, 0.0, Equal);
    test(0.0, f64::NAN, Greater);
    test(0.0, f64::INFINITY, Less);
    test(0.0, 5.0, Less);
    test(123.0, 123.0, Equal);
    test(123.0, 124.0, Less);
    test(123.0, 122.0, Greater);
    test(1000000000000.0, 123.0, Greater);
    test(123.0, 1000000000000.0, Less);
    test(1000000000000.0, 1000000000000.0, Equal);
    test(1000000000000.0, 0.0, Greater);
    test(0.5, 0.0, Greater);
    test(0.5, 0.5, Equal);

    test(0.0, -0.0, Equal);
    test(0.0, f64::NEGATIVE_INFINITY, Less);
    test(0.0, -5.0, Less);
    test(123.0, -123.0, Equal);
    test(123.0, -124.0, Less);
    test(123.0, -122.0, Greater);
    test(1000000000000.0, -123.0, Greater);
    test(123.0, -1000000000000.0, Less);
    test(1000000000000.0, -1000000000000.0, Equal);
    test(1000000000000.0, -0.0, Greater);
    test(0.5, -0.0, Greater);
    test(0.5, -0.5, Equal);

    test(-0.0, 0.0, Equal);
    test(-0.0, f64::NAN, Greater);
    test(-0.0, f64::INFINITY, Less);
    test(-0.0, 5.0, Less);
    test(-123.0, 123.0, Equal);
    test(-123.0, 124.0, Less);
    test(-123.0, 122.0, Greater);
    test(-1000000000000.0, 123.0, Greater);
    test(-123.0, 1000000000000.0, Less);
    test(-1000000000000.0, 1000000000000.0, Equal);
    test(-1000000000000.0, 0.0, Greater);
    test(-0.5, 0.0, Greater);
    test(-0.5, 0.5, Equal);

    test(-0.0, -0.0, Equal);
    test(-0.0, f64::NEGATIVE_INFINITY, Less);
    test(-0.0, -5.0, Less);
    test(-123.0, -123.0, Equal);
    test(-123.0, -124.0, Less);
    test(-123.0, -122.0, Greater);
    test(-1000000000000.0, -123.0, Greater);
    test(-123.0, -1000000000000.0, Less);
    test(-1000000000000.0, -1000000000000.0, Equal);
    test(-1000000000000.0, -0.0, Greater);
    test(-0.5, -0.0, Greater);
    test(-0.5, -0.5, Equal);
}

fn cmp_abs_properties_helper<T: PrimitiveFloat>() {
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let cmp = x.cmp_abs(&y);
        assert_eq!(y.cmp_abs(&x).reverse(), cmp);
        assert_eq!(NiceFloat(-x.0).cmp_abs(&NiceFloat(-y.0)), cmp);

        assert_eq!(x.lt_abs(&y), cmp == Less);
        assert_eq!(x.gt_abs(&y), cmp == Greater);
        assert_eq!(x.le_abs(&y), cmp != Greater);
        assert_eq!(x.ge_abs(&y), cmp != Less);
    });

    primitive_float_gen::<T>().test_properties(|x| {
        let x = NiceFloat(x);
        assert_eq!(x.cmp_abs(&x), Equal);
    });

    primitive_float_triple_gen::<T>().test_properties(|(x, y, z)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let z = NiceFloat(z);
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });
}

#[test]
pub fn cmp_abs_properties() {
    apply_fn_to_primitive_floats!(cmp_abs_properties_helper);
}
