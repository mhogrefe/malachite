// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_primitive_floats_with_sci_exponent_and_precision;
use malachite_base::test_util::num::exhaustive::*;
use std::panic::catch_unwind;

fn exhaustive_primitive_floats_with_sci_exponent_and_precision_helper<T: PrimitiveFloat>(
    sci_exponent: i64,
    precision: u64,
    out: &[T],
) {
    exhaustive_primitive_floats_helper_helper_with_limit(
        20,
        exhaustive_primitive_floats_with_sci_exponent_and_precision::<T>(sci_exponent, precision),
        out,
    );
}

#[test]
fn test_exhaustive_primitive_floats_with_sci_exponent_and_precision() {
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(0, 1, &[1.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(0, 2, &[1.5]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(0, 3, &[1.25, 1.75]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        0,
        4,
        &[1.125, 1.375, 1.625, 1.875],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        0,
        5,
        &[1.0625, 1.1875, 1.3125, 1.4375, 1.5625, 1.6875, 1.8125, 1.9375],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(4, 1, &[16.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(4, 2, &[24.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(4, 3, &[20.0, 28.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        4,
        4,
        &[18.0, 22.0, 26.0, 30.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        4,
        5,
        &[17.0, 19.0, 21.0, 23.0, 25.0, 27.0, 29.0, 31.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(-4, 1, &[0.0625]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(-4, 2, &[0.09375]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        -4,
        3,
        &[0.078125, 0.109375],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        -4,
        4,
        &[0.0703125, 0.0859375, 0.1015625, 0.1171875],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        -4,
        5,
        &[
            0.06640625, 0.07421875, 0.08203125, 0.08984375, 0.09765625, 0.10546875, 0.11328125,
            0.12109375,
        ],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(-149, 1, &[1.0e-45]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(-148, 1, &[3.0e-45]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(-148, 2, &[4.0e-45]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(-147, 1, &[6.0e-45]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(-147, 2, &[8.0e-45]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f32>(
        -147,
        3,
        &[7.0e-45, 1.0e-44],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(0, 1, &[1.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(0, 2, &[1.5]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(0, 3, &[1.25, 1.75]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        0,
        4,
        &[1.125, 1.375, 1.625, 1.875],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        0,
        5,
        &[1.0625, 1.1875, 1.3125, 1.4375, 1.5625, 1.6875, 1.8125, 1.9375],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(4, 1, &[16.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(4, 2, &[24.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(4, 3, &[20.0, 28.0]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        4,
        4,
        &[18.0, 22.0, 26.0, 30.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        4,
        5,
        &[17.0, 19.0, 21.0, 23.0, 25.0, 27.0, 29.0, 31.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(-4, 1, &[0.0625]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(-4, 2, &[0.09375]);
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -4,
        3,
        &[0.078125, 0.109375],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -4,
        4,
        &[0.0703125, 0.0859375, 0.1015625, 0.1171875],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -4,
        5,
        &[
            0.06640625, 0.07421875, 0.08203125, 0.08984375, 0.09765625, 0.10546875, 0.11328125,
            0.12109375,
        ],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -1074,
        1,
        &[5.0e-324],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -1073,
        1,
        &[1.0e-323],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -1073,
        2,
        &[1.5e-323],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -1072,
        1,
        &[2.0e-323],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -1072,
        2,
        &[3.0e-323],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_helper::<f64>(
        -1072,
        3,
        &[2.5e-323, 3.5e-323],
    );
}

fn exhaustive_primitive_floats_with_sci_exponent_and_precision_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_and_precision::<T>(0, 0));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_and_precision::<T>(0, 100));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_and_precision::<T>(10000, 1));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_and_precision::<T>(-10000, 1));
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision::<T>(T::MIN_EXPONENT, 2)
    );
}

#[test]
fn exhaustive_primitive_floats_with_sci_exponent_and_precision_fail() {
    apply_fn_to_primitive_floats!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_fail_helper
    );
}
