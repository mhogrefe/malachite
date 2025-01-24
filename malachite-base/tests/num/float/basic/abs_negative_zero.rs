// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_gen;

fn abs_negative_zero_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        let out = NiceFloat(out);

        assert_eq!(NiceFloat(n.abs_negative_zero()), out);

        let mut n = n;
        n.abs_negative_zero_assign();
        assert_eq!(NiceFloat(n), out);
    };
    test(T::ZERO, T::ZERO);
    test(T::NEGATIVE_ZERO, T::ZERO);
    test(T::NAN, T::NAN);
    test(T::INFINITY, T::INFINITY);
    test(T::NEGATIVE_INFINITY, T::NEGATIVE_INFINITY);
    test(T::ONE, T::ONE);
    test(T::NEGATIVE_ONE, T::NEGATIVE_ONE);
    test(T::from(1.234), T::from(1.234));
    test(T::from(-1.234), T::from(-1.234));
}

#[test]
fn test_abs_negative_zero() {
    apply_fn_to_primitive_floats!(abs_negative_zero_helper);
}

fn abs_negative_zero_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        let y = x.abs_negative_zero();
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        assert!(x == y || x == NiceFloat(T::NEGATIVE_ZERO) && y == NiceFloat(T::ZERO));
        assert_eq!(NiceFloat(y.0.abs_negative_zero()), y);

        let mut x = x.0;
        x.abs_negative_zero_assign();
        assert_eq!(NiceFloat(x), y);
    });
}

#[test]
fn abs_negative_zero_properties() {
    apply_fn_to_primitive_floats!(abs_negative_zero_properties_helper);
}
