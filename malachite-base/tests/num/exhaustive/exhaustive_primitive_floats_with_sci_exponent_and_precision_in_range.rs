// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::*;
use malachite_base::test_util::num::exhaustive::*;
use std::panic::catch_unwind;

fn exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper<
    T: PrimitiveFloat,
>(
    a: T,
    b: T,
    sci_exponent: i64,
    precision: u64,
    out: &[T],
) {
    exhaustive_primitive_floats_helper_helper_with_limit(
        20,
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            a,
            b,
            sci_exponent,
            precision,
        ),
        out,
    );
}

#[test]
fn test_exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range() {
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        1,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        2,
        &[3.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        3,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        4,
        &[2.75],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        5,
        &[2.875, 3.125],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        6,
        &[2.8125, 2.9375, 3.0625],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        7,
        &[2.71875, 2.78125, 2.84375, 2.90625, 2.96875, 3.03125, 3.09375],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        1,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        2,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        3,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        4,
        &[1920.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        5,
        &[1984.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        6,
        &[1952.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        7,
        &[1904.0, 1936.0, 1968.0, 2000.0],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        7.0e-45,
        1.0e-44,
        -147,
        1,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        7.0e-45,
        1.0e-44,
        -147,
        2,
        &[8.0e-45],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        7.0e-45,
        1.0e-44,
        -147,
        3,
        &[7.0e-45, 1.0e-44],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1.0,
        1.99,
        0,
        1,
        &[1.0],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        1,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        2,
        &[3.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        3,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        4,
        &[2.75],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        5,
        &[2.875, 3.125],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        6,
        &[2.8125, 2.9375, 3.0625],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        7,
        &[2.71875, 2.78125, 2.84375, 2.90625, 2.96875, 3.03125, 3.09375],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        1,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        2,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        3,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        4,
        &[1920.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        5,
        &[1984.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        6,
        &[1952.0],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        7,
        &[1904.0, 1936.0, 1968.0, 2000.0],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        7.0e-45,
        1.0e-44,
        -147,
        1,
        &[],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        7.0e-45,
        1.0e-44,
        -147,
        2,
        &[8.407790785948902e-45],
    );
    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f64>(
        7.0e-45,
        1.0e-44,
        -147,
        3,
        &[7.006492321624085e-45, 9.80908925027372e-45],
    );

    exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_helper::<f32>(
        1.0,
        1.99,
        0,
        1,
        &[1.0],
    );
}

fn exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_fail_helper<
    T: PrimitiveFloat,
>() {
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(1.1),
            T::from(1.2),
            0,
            0
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(1.1),
            T::from(1.2),
            0,
            100
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(1.1),
            T::from(1.2),
            10000,
            1
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(1.1),
            T::from(1.2),
            -10000,
            1
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::MIN_POSITIVE_SUBNORMAL,
            T::MIN_POSITIVE_SUBNORMAL,
            T::MIN_EXPONENT,
            2
        )
    );

    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(1.2),
            T::from(1.1),
            0,
            1
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(1.1),
            T::from(1.2),
            1,
            1
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(0.1),
            T::from(1.2),
            1,
            1
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::from(-1.2),
            T::from(1.1),
            0,
            1
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::ONE,
            T::INFINITY,
            0,
            1
        )
    );
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range::<T>(
            T::ONE,
            T::NAN,
            0,
            1
        )
    );
}

#[test]
fn exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_fail() {
    apply_fn_to_primitive_floats!(
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range_fail_helper
    );
}
