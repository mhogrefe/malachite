// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_pair_gen, primitive_float_triple_gen,
};

#[test]
fn test_eq_abs() {
    let test = |s, t, eq| {
        let u = NiceFloat(s);
        let v = NiceFloat(t);
        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test(0.0, 0.0, true);
    test(0.0, f64::NAN, false);
    test(0.0, f64::NEGATIVE_INFINITY, false);
    test(0.0, 5.0, false);
    test(123.0, 123.0, true);
    test(123.0, 124.0, false);
    test(123.0, 122.0, false);
    test(1000000000000.0, 123.0, false);
    test(123.0, 1000000000000.0, false);
    test(1000000000000.0, 1000000000000.0, true);
    test(1000000000000.0, 0.0, false);
    test(0.5, 0.0, false);
    test(0.5, 0.5, true);

    test(0.0, -0.0, true);
    test(0.0, -f64::NAN, false);
    test(0.0, -f64::NEGATIVE_INFINITY, false);
    test(0.0, -5.0, false);
    test(123.0, -123.0, true);
    test(123.0, -124.0, false);
    test(123.0, -122.0, false);
    test(1000000000000.0, -123.0, false);
    test(123.0, -1000000000000.0, false);
    test(1000000000000.0, -1000000000000.0, true);
    test(1000000000000.0, -0.0, false);
    test(0.5, -0.0, false);
    test(0.5, -0.5, true);

    test(-0.0, 0.0, true);
    test(-0.0, f64::NAN, false);
    test(-0.0, f64::NEGATIVE_INFINITY, false);
    test(-0.0, 5.0, false);
    test(-123.0, 123.0, true);
    test(-123.0, 124.0, false);
    test(-123.0, 122.0, false);
    test(-1000000000000.0, 123.0, false);
    test(-123.0, 1000000000000.0, false);
    test(-1000000000000.0, 1000000000000.0, true);
    test(-1000000000000.0, 0.0, false);
    test(-0.5, 0.0, false);
    test(-0.5, 0.5, true);

    test(-0.0, -0.0, true);
    test(-0.0, -f64::NAN, false);
    test(-0.0, -f64::NEGATIVE_INFINITY, false);
    test(-0.0, -5.0, false);
    test(-123.0, -123.0, true);
    test(-123.0, -124.0, false);
    test(-123.0, -122.0, false);
    test(-1000000000000.0, -123.0, false);
    test(-123.0, -1000000000000.0, false);
    test(-1000000000000.0, -1000000000000.0, true);
    test(-1000000000000.0, -0.0, false);
    test(-0.5, -0.0, false);
    test(-0.5, -0.5, true);
}

#[allow(clippy::eq_op)]
fn eq_abs_properties_helper<T: PrimitiveFloat>() {
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let eq = x.eq_abs(&y);
        assert_eq!(y.eq_abs(&x), eq);
        assert_eq!(x.ne_abs(&y), !eq);
        assert_eq!(NiceFloat(x.0.abs()) == NiceFloat(y.0.abs()), eq);
    });

    primitive_float_gen::<T>().test_properties(|x| {
        let x = NiceFloat(x);
        assert!(x.eq_abs(&x));
    });

    primitive_float_triple_gen::<T>().test_properties(|(x, y, z)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let z = NiceFloat(z);
        if x.eq_abs(&y) && x.eq_abs(&z) {
            assert!(x.eq_abs(&z));
        }
    });
}

#[test]
pub fn eq_properties() {
    apply_fn_to_primitive_floats!(eq_abs_properties_helper);
}
