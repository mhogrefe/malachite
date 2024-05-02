// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_positive_finite_primitive_floats;
use malachite_base::test_util::num::exhaustive::exhaustive_primitive_floats_helper_helper;

fn exhaustive_positive_finite_primitive_floats_helper<T: PrimitiveFloat>(out: &[T]) {
    exhaustive_primitive_floats_helper_helper(
        exhaustive_positive_finite_primitive_floats::<T>(),
        out,
    );
}

#[test]
fn test_exhaustive_positive_finite_primitive_floats() {
    exhaustive_positive_finite_primitive_floats_helper::<f32>(&[
        1.0, 2.0, 1.5, 0.5, 1.25, 3.0, 1.75, 4.0, 1.125, 2.5, 1.375, 0.75, 1.625, 3.5, 1.875, 0.25,
        1.0625, 2.25, 1.1875, 0.625, 1.3125, 2.75, 1.4375, 6.0, 1.5625, 3.25, 1.6875, 0.875,
        1.8125, 3.75, 1.9375, 8.0, 1.03125, 2.125, 1.09375, 0.5625, 1.15625, 2.375, 1.21875, 5.0,
        1.28125, 2.625, 1.34375, 0.6875, 1.40625, 2.875, 1.46875, 0.375, 1.53125, 3.125,
    ]);
    exhaustive_positive_finite_primitive_floats_helper::<f64>(&[
        1.0, 2.0, 1.5, 0.5, 1.25, 3.0, 1.75, 4.0, 1.125, 2.5, 1.375, 0.75, 1.625, 3.5, 1.875, 0.25,
        1.0625, 2.25, 1.1875, 0.625, 1.3125, 2.75, 1.4375, 6.0, 1.5625, 3.25, 1.6875, 0.875,
        1.8125, 3.75, 1.9375, 8.0, 1.03125, 2.125, 1.09375, 0.5625, 1.15625, 2.375, 1.21875, 5.0,
        1.28125, 2.625, 1.34375, 0.6875, 1.40625, 2.875, 1.46875, 0.375, 1.53125, 3.125,
    ]);
}
