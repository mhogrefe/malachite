// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_positive_finite_primitive_floats_in_range;
use malachite_base::test_util::num::exhaustive::exhaustive_primitive_floats_helper_helper;
use std::panic::catch_unwind;

fn exhaustive_positive_finite_primitive_floats_in_range_helper<T: PrimitiveFloat>(
    a: T,
    b: T,
    out: &[T],
) {
    exhaustive_primitive_floats_helper_helper(
        exhaustive_positive_finite_primitive_floats_in_range::<T>(a, b),
        out,
    );
}

#[test]
fn test_exhaustive_positive_finite_primitive_floats_in_range() {
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        &[
            3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625,
            2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625,
            2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625,
            2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375,
            2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375,
            2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875,
        ],
    );
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f32>(5.0, 5.0, &[5.0]);
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f32>(
        1.0,
        6.0,
        &[
            1.0, 2.0, 1.5, 4.0, 1.25, 3.0, 1.75, 1.125, 1.375, 2.5, 1.625, 6.0, 1.875, 3.5, 1.0625,
            2.25, 1.1875, 2.75, 1.3125, 5.0, 1.4375, 3.25, 1.5625, 1.6875, 1.8125, 3.75, 1.9375,
            4.5, 1.03125, 2.125, 1.09375, 5.5, 1.15625, 2.375, 1.21875, 4.25, 1.28125, 2.625,
            1.34375, 1.40625, 1.46875, 2.875, 1.53125, 4.75, 1.59375, 3.125, 1.65625, 3.375,
            1.71875, 3.625,
        ],
    );
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f32>(
        0.01,
        100.0,
        &[
            1.0, 2.0, 1.5, 0.5, 1.25, 3.0, 1.75, 4.0, 1.125, 2.5, 1.375, 0.75, 1.625, 3.5, 1.875,
            0.25, 1.0625, 2.25, 1.1875, 0.625, 1.3125, 2.75, 1.4375, 6.0, 1.5625, 3.25, 1.6875,
            0.875, 1.8125, 3.75, 1.9375, 8.0, 1.03125, 2.125, 1.09375, 0.5625, 1.15625, 2.375,
            1.21875, 5.0, 1.28125, 2.625, 1.34375, 0.6875, 1.40625, 2.875, 1.46875, 0.375, 1.53125,
            3.125,
        ],
    );
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f32>(
        255.9,
        256.1,
        &[
            255.9375, 256.0, 255.90625, 255.96875, 255.92188, 256.0625, 255.95312, 256.03125,
            255.98438, 256.09375, 255.91406, 255.92969, 255.94531, 256.01562, 255.96094, 255.97656,
            255.99219, 256.04688, 255.90234, 255.91016, 255.91797, 256.07812, 255.92578, 256.0078,
            255.9336, 256.02344, 255.9414, 255.94922, 255.95703, 256.03906, 255.96484, 256.0547,
            255.97266, 256.0703, 255.98047, 255.98828, 255.9961, 256.08594, 255.90039, 256.0039,
            255.9043, 256.01172, 255.9082, 255.91211, 255.91602, 256.01953, 255.91992, 255.92383,
            255.92773, 256.02734,
        ],
    );

    exhaustive_positive_finite_primitive_floats_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        &[
            3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625,
            2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625,
            2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625,
            2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375,
            2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375,
            2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875,
        ],
    );
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f64>(5.0, 5.0, &[5.0]);
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f64>(
        1.0,
        6.0,
        &[
            1.0, 2.0, 1.5, 4.0, 1.25, 3.0, 1.75, 1.125, 1.375, 2.5, 1.625, 6.0, 1.875, 3.5, 1.0625,
            2.25, 1.1875, 2.75, 1.3125, 5.0, 1.4375, 3.25, 1.5625, 1.6875, 1.8125, 3.75, 1.9375,
            4.5, 1.03125, 2.125, 1.09375, 5.5, 1.15625, 2.375, 1.21875, 4.25, 1.28125, 2.625,
            1.34375, 1.40625, 1.46875, 2.875, 1.53125, 4.75, 1.59375, 3.125, 1.65625, 3.375,
            1.71875, 3.625,
        ],
    );
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f64>(
        0.01,
        100.0,
        &[
            1.0, 2.0, 1.5, 0.5, 1.25, 3.0, 1.75, 4.0, 1.125, 2.5, 1.375, 0.75, 1.625, 3.5, 1.875,
            0.25, 1.0625, 2.25, 1.1875, 0.625, 1.3125, 2.75, 1.4375, 6.0, 1.5625, 3.25, 1.6875,
            0.875, 1.8125, 3.75, 1.9375, 8.0, 1.03125, 2.125, 1.09375, 0.5625, 1.15625, 2.375,
            1.21875, 5.0, 1.28125, 2.625, 1.34375, 0.6875, 1.40625, 2.875, 1.46875, 0.375, 1.53125,
            3.125,
        ],
    );
    exhaustive_positive_finite_primitive_floats_in_range_helper::<f64>(
        255.9,
        256.1,
        &[
            255.9375,
            256.0,
            255.90625,
            255.96875,
            255.921875,
            256.0625,
            255.953125,
            256.03125,
            255.984375,
            256.09375,
            255.9140625,
            255.9296875,
            255.9453125,
            256.015625,
            255.9609375,
            255.9765625,
            255.9921875,
            256.046875,
            255.90234375,
            255.91015625,
            255.91796875,
            256.078125,
            255.92578125,
            256.0078125,
            255.93359375,
            256.0234375,
            255.94140625,
            255.94921875,
            255.95703125,
            256.0390625,
            255.96484375,
            256.0546875,
            255.97265625,
            256.0703125,
            255.98046875,
            255.98828125,
            255.99609375,
            256.0859375,
            255.900390625,
            256.00390625,
            255.904296875,
            256.01171875,
            255.908203125,
            255.912109375,
            255.916015625,
            256.01953125,
            255.919921875,
            255.923828125,
            255.927734375,
            256.02734375,
        ],
    );
}

fn exhaustive_positive_finite_primitive_floats_in_range_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(exhaustive_positive_finite_primitive_floats_in_range::<T>(
        T::from(1.2),
        T::from(1.1),
    ));
    assert_panic!(exhaustive_positive_finite_primitive_floats_in_range::<T>(
        T::from(-1.1),
        T::from(1.1),
    ));
    assert_panic!(exhaustive_positive_finite_primitive_floats_in_range::<T>(
        T::ONE,
        T::INFINITY,
    ));
    assert_panic!(exhaustive_positive_finite_primitive_floats_in_range::<T>(
        T::ONE,
        T::NAN,
    ));
}

#[test]
fn exhaustive_positive_finite_primitive_floats_in_range_fail() {
    apply_fn_to_primitive_floats!(exhaustive_positive_finite_primitive_floats_in_range_fail_helper);
}
