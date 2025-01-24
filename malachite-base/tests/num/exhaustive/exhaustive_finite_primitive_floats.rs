// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_finite_primitive_floats;
use malachite_base::test_util::num::exhaustive::exhaustive_primitive_floats_helper_helper;

fn exhaustive_finite_primitive_floats_helper<T: PrimitiveFloat>(out: &[T]) {
    exhaustive_primitive_floats_helper_helper(exhaustive_finite_primitive_floats::<T>(), out);
}

#[test]
fn test_exhaustive_finite_primitive_floats() {
    exhaustive_finite_primitive_floats_helper::<f32>(&[
        0.0, -0.0, 1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, -1.75,
        4.0, -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, 3.5, -3.5,
        1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, 0.625, -0.625,
        1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0,
    ]);
    exhaustive_finite_primitive_floats_helper::<f64>(&[
        0.0, -0.0, 1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, -1.75,
        4.0, -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, 3.5, -3.5,
        1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, 0.625, -0.625,
        1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0,
    ]);
}
