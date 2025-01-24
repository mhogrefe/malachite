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

fn is_negative_zero_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.is_negative_zero(), out);
    };
    test(T::ZERO, false);
    test(T::NEGATIVE_ZERO, true);
    test(T::NAN, false);
    test(T::INFINITY, false);
    test(T::NEGATIVE_INFINITY, false);
    test(T::ONE, false);
    test(T::NEGATIVE_ONE, false);
    test(T::from(1.234), false);
    test(T::from(-1.234), false);
}

#[test]
fn test_is_negative_zero() {
    apply_fn_to_primitive_floats!(is_negative_zero_helper);
}

fn is_negative_zero_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|x| {
        assert_eq!(
            x.is_negative_zero(),
            NiceFloat(x) != NiceFloat(x.abs_negative_zero())
        );
    });
}

#[test]
fn is_negative_zero_properties() {
    apply_fn_to_primitive_floats!(is_negative_zero_properties_helper);
}
