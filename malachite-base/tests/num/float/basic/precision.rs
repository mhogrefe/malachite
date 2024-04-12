// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::test_util::generators::primitive_float_gen_var_12;
use std::panic::catch_unwind;

fn precision_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.precision(), out);
    };
    test(T::ONE, 1);
    test(T::NEGATIVE_ONE, 1);
    test(T::from(3.0), 2);
    test(T::from(1.5), 2);
    test(T::from(1.234), 23);
    test(T::from(-1.234), 23);
}

#[test]
fn test_precision() {
    apply_fn_to_primitive_floats!(precision_helper);
}

fn precision_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(T::NAN.precision());
    assert_panic!(T::INFINITY.precision());
    assert_panic!(T::NEGATIVE_INFINITY.precision());
    assert_panic!(T::ZERO.precision());
    assert_panic!(T::NEGATIVE_ZERO.precision());
}

#[test]
pub fn precision_fail() {
    apply_fn_to_primitive_floats!(precision_fail_helper);
}

fn precision_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen_var_12::<T>().test_properties(|x| {
        let precision = x.precision();
        assert_ne!(precision, 0);
        assert!(precision <= T::max_precision_for_sci_exponent(x.sci_exponent()));
        assert_eq!((-x).precision(), precision);
    });
}

#[test]
fn precision_properties() {
    apply_fn_to_primitive_floats!(precision_properties_helper);
}
