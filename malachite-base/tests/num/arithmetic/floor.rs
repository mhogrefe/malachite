// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_gen;

fn floor_assign_primitive_float_helper<T: PrimitiveFloat>() {
    let test = |n: T, out| {
        assert_eq!(NiceFloat(n.floor()), NiceFloat(out));

        let mut n = n;
        n.floor_assign();
        assert_eq!(NiceFloat(n), NiceFloat(out));
    };
    test(T::ZERO, T::ZERO);
    test(T::NEGATIVE_ZERO, T::NEGATIVE_ZERO);
    test(T::INFINITY, T::INFINITY);
    test(T::NEGATIVE_INFINITY, T::NEGATIVE_INFINITY);
    test(T::NAN, T::NAN);
    test(T::ONE, T::ONE);
    test(T::NEGATIVE_ONE, T::NEGATIVE_ONE);
    test(T::from(1.5f32), T::from(1.0f32));
    test(T::from(-1.5f32), T::from(-2.0f32));
}

#[test]
fn test_floor_assign() {
    apply_fn_to_primitive_floats!(floor_assign_primitive_float_helper);
}

fn floor_assign_properties_helper<T: PrimitiveFloat>() {
    primitive_float_gen::<T>().test_properties(|f| {
        let mut floor = f;
        floor.floor_assign();
        assert_eq!(NiceFloat(floor), NiceFloat(f.floor()));
        assert_eq!(NiceFloat(floor.floor()), NiceFloat(floor));
        assert_eq!(NiceFloat(-floor), NiceFloat((-f).ceiling()));
        assert_eq!(f.sign(), floor.sign());
    });
}

#[test]
fn floor_assign_properties() {
    apply_fn_to_primitive_floats!(floor_assign_properties_helper);
}
