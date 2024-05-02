// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::common::test_eq_helper;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_pair_gen, primitive_float_triple_gen,
};

const TEST_STRINGS: [&str; 7] = ["-Infinity", "-5.0e5", "-0.0", "NaN", "0.0", "0.123", "Infinity"];

#[test]
pub fn test_eq() {
    test_eq_helper::<NiceFloat<f32>>(&TEST_STRINGS);
    test_eq_helper::<NiceFloat<f64>>(&TEST_STRINGS);
}

#[allow(clippy::eq_op)]
fn eq_properties_helper<T: PrimitiveFloat>() {
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        assert_eq!(x == y, y == x);
    });

    primitive_float_gen::<T>().test_properties(|x| {
        let x = NiceFloat(x);
        assert_eq!(x, x);
    });

    primitive_float_triple_gen::<T>().test_properties(|(x, y, z)| {
        let x = NiceFloat(x);
        let y = NiceFloat(y);
        let z = NiceFloat(z);
        if x == y && x == z {
            assert_eq!(x, z);
        }
    });
}

#[test]
pub fn eq_properties() {
    apply_fn_to_primitive_floats!(eq_properties_helper);
}
