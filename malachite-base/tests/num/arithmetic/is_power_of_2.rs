// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::test_util::generators::primitive_float_gen;

fn is_power_of_2_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(n.is_power_of_2(), out);
    };
    test(T::ZERO, false);
    test(T::NEGATIVE_ZERO, false);
    test(T::INFINITY, false);
    test(T::NEGATIVE_INFINITY, false);
    test(T::NAN, false);
    test(T::NEGATIVE_ONE, false);
    test(T::from(1.5f32), false);
    test(T::from(-1.5f32), false);

    test(T::ONE, true);
    test(T::TWO, true);
    test(T::from(4.0f32), true);
    test(T::from(0.5f32), true);
    test(T::from(0.25f32), true);
}

#[test]
fn test_is_power_of_2() {
    apply_fn_to_primitive_floats!(is_power_of_2_helper);
}

fn is_power_of_2_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        if f.is_power_of_2() {
            assert_eq!(f.precision(), 1);
            assert_eq!(T::power_of_2(f.checked_log_base_2().unwrap()), f);
        }
    });
}

#[test]
fn is_power_of_2_properties() {
    apply_fn_to_primitive_floats!(is_power_of_2_properties_helper);
}
