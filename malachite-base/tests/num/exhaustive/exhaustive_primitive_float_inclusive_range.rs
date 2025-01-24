// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::exhaustive::exhaustive_primitive_float_inclusive_range;
use malachite_base::test_util::num::exhaustive::exhaustive_primitive_floats_helper_helper;
use std::panic::catch_unwind;

fn exhaustive_primitive_float_inclusive_range_helper<T: PrimitiveFloat>(a: T, b: T, out: &[T]) {
    exhaustive_primitive_floats_helper_helper(
        exhaustive_primitive_float_inclusive_range::<T>(a, b),
        out,
    );
}

#[allow(clippy::approx_constant)]
#[test]
fn test_exhaustive_primitive_float_inclusive_range() {
    exhaustive_primitive_float_inclusive_range_helper::<f32>(1.0, 1.0, &[1.0]);
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        1.0,
        2.0,
        &[
            1.0, 2.0, 1.5, 1.25, 1.75, 1.125, 1.375, 1.625, 1.875, 1.0625, 1.1875, 1.3125, 1.4375,
            1.5625, 1.6875, 1.8125, 1.9375, 1.03125, 1.09375, 1.15625, 1.21875, 1.28125, 1.34375,
            1.40625, 1.46875, 1.53125, 1.59375, 1.65625, 1.71875, 1.78125, 1.84375, 1.90625,
            1.96875, 1.015625, 1.046875, 1.078125, 1.109375, 1.140625, 1.171875, 1.203125,
            1.234375, 1.265625, 1.296875, 1.328125, 1.359375, 1.390625, 1.421875, 1.453125,
            1.484375, 1.515625,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        -0.1,
        0.1,
        &[
            0.0,
            -0.0,
            0.0625,
            -0.0625,
            0.03125,
            -0.03125,
            0.09375,
            -0.09375,
            0.015625,
            -0.015625,
            0.078125,
            -0.078125,
            0.046875,
            -0.046875,
            0.0703125,
            -0.0703125,
            0.0078125,
            -0.0078125,
            0.0859375,
            -0.0859375,
            0.0390625,
            -0.0390625,
            0.06640625,
            -0.06640625,
            0.0234375,
            -0.0234375,
            0.07421875,
            -0.07421875,
            0.0546875,
            -0.0546875,
            0.08203125,
            -0.08203125,
            0.00390625,
            -0.00390625,
            0.08984375,
            -0.08984375,
            0.03515625,
            -0.03515625,
            0.09765625,
            -0.09765625,
            0.01953125,
            -0.01953125,
            0.064453125,
            -0.064453125,
            0.04296875,
            -0.04296875,
            0.068359375,
            -0.068359375,
            0.01171875,
            -0.01171875,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
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
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        100.0,
        101.0,
        &[
            100.0, 101.0, 100.5, 100.25, 100.75, 100.125, 100.375, 100.625, 100.875, 100.0625,
            100.1875, 100.3125, 100.4375, 100.5625, 100.6875, 100.8125, 100.9375, 100.03125,
            100.09375, 100.15625, 100.21875, 100.28125, 100.34375, 100.40625, 100.46875, 100.53125,
            100.59375, 100.65625, 100.71875, 100.78125, 100.84375, 100.90625, 100.96875,
            100.015625, 100.046875, 100.078125, 100.109375, 100.140625, 100.171875, 100.203125,
            100.234375, 100.265625, 100.296875, 100.328125, 100.359375, 100.390625, 100.421875,
            100.453125, 100.484375, 100.515625,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        1.0e38,
        f32::INFINITY,
        &[
            f32::INFINITY,
            1.2760589e38,
            1.7014118e38,
            1.0633824e38,
            1.4887354e38,
            1.1697206e38,
            2.5521178e38,
            1.3823971e38,
            2.1267648e38,
            1.5950736e38,
            2.9774707e38,
            1.0102133e38,
            1.1165515e38,
            1.2228898e38,
            1.9140883e38,
            1.329228e38,
            1.4355662e38,
            1.5419045e38,
            2.3394413e38,
            1.6482427e38,
            1.0367978e38,
            1.089967e38,
            2.7647942e38,
            1.1431361e38,
            3.1901472e38,
            1.1963052e38,
            1.80775e38,
            1.2494743e38,
            1.3026434e38,
            1.3558126e38,
            2.0204266e38,
            1.4089817e38,
            2.233103e38,
            1.4621508e38,
            2.4457795e38,
            1.5153199e38,
            1.568489e38,
            1.6216582e38,
            2.658456e38,
            1.6748273e38,
            2.8711325e38,
            1.0235056e38,
            3.083809e38,
            1.0500901e38,
            1.0766747e38,
            1.1032592e38,
            3.2964854e38,
            1.1298438e38,
            1.1564284e38,
            1.1830129e38,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        f32::MIN_POSITIVE_SUBNORMAL,
        &[0.0, -0.0, 1.0e-45, -1.0e-45],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        -0.0,
        f32::MIN_POSITIVE_SUBNORMAL,
        &[0.0, -0.0, 1.0e-45],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        0.0,
        f32::MIN_POSITIVE_SUBNORMAL,
        &[0.0, 1.0e-45],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        -0.0,
        &[-0.0, -1.0e-45],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        0.0,
        &[0.0, -0.0, -1.0e-45],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        f32::NEGATIVE_INFINITY,
        f32::INFINITY,
        &[
            f32::INFINITY,
            f32::NEGATIVE_INFINITY,
            0.0,
            -0.0,
            1.0,
            -1.0,
            2.0,
            -2.0,
            1.5,
            -1.5,
            0.5,
            -0.5,
            1.25,
            -1.25,
            3.0,
            -3.0,
            1.75,
            -1.75,
            4.0,
            -4.0,
            1.125,
            -1.125,
            2.5,
            -2.5,
            1.375,
            -1.375,
            0.75,
            -0.75,
            1.625,
            -1.625,
            3.5,
            -3.5,
            1.875,
            -1.875,
            0.25,
            -0.25,
            1.0625,
            -1.0625,
            2.25,
            -2.25,
            1.1875,
            -1.1875,
            0.625,
            -0.625,
            1.3125,
            -1.3125,
            2.75,
            -2.75,
            1.4375,
            -1.4375,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        f32::NEGATIVE_INFINITY,
        f32::NEGATIVE_INFINITY,
        &[f32::NEGATIVE_INFINITY],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(
        f32::INFINITY,
        f32::INFINITY,
        &[f32::INFINITY],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f32>(0.0, 0.0, &[0.0]);
    exhaustive_primitive_float_inclusive_range_helper::<f32>(-0.0, -0.0, &[-0.0]);
    exhaustive_primitive_float_inclusive_range_helper::<f32>(-0.0, 0.0, &[0.0, -0.0]);

    exhaustive_primitive_float_inclusive_range_helper::<f64>(1.0, 1.0, &[1.0]);
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        1.0,
        2.0,
        &[
            1.0, 2.0, 1.5, 1.25, 1.75, 1.125, 1.375, 1.625, 1.875, 1.0625, 1.1875, 1.3125, 1.4375,
            1.5625, 1.6875, 1.8125, 1.9375, 1.03125, 1.09375, 1.15625, 1.21875, 1.28125, 1.34375,
            1.40625, 1.46875, 1.53125, 1.59375, 1.65625, 1.71875, 1.78125, 1.84375, 1.90625,
            1.96875, 1.015625, 1.046875, 1.078125, 1.109375, 1.140625, 1.171875, 1.203125,
            1.234375, 1.265625, 1.296875, 1.328125, 1.359375, 1.390625, 1.421875, 1.453125,
            1.484375, 1.515625,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        -0.1,
        0.1,
        &[
            0.0,
            -0.0,
            0.0625,
            -0.0625,
            0.03125,
            -0.03125,
            0.09375,
            -0.09375,
            0.015625,
            -0.015625,
            0.078125,
            -0.078125,
            0.046875,
            -0.046875,
            0.0703125,
            -0.0703125,
            0.0078125,
            -0.0078125,
            0.0859375,
            -0.0859375,
            0.0390625,
            -0.0390625,
            0.06640625,
            -0.06640625,
            0.0234375,
            -0.0234375,
            0.07421875,
            -0.07421875,
            0.0546875,
            -0.0546875,
            0.08203125,
            -0.08203125,
            0.00390625,
            -0.00390625,
            0.08984375,
            -0.08984375,
            0.03515625,
            -0.03515625,
            0.09765625,
            -0.09765625,
            0.01953125,
            -0.01953125,
            0.064453125,
            -0.064453125,
            0.04296875,
            -0.04296875,
            0.068359375,
            -0.068359375,
            0.01171875,
            -0.01171875,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
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
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        100.0,
        101.0,
        &[
            100.0, 101.0, 100.5, 100.25, 100.75, 100.125, 100.375, 100.625, 100.875, 100.0625,
            100.1875, 100.3125, 100.4375, 100.5625, 100.6875, 100.8125, 100.9375, 100.03125,
            100.09375, 100.15625, 100.21875, 100.28125, 100.34375, 100.40625, 100.46875, 100.53125,
            100.59375, 100.65625, 100.71875, 100.78125, 100.84375, 100.90625, 100.96875,
            100.015625, 100.046875, 100.078125, 100.109375, 100.140625, 100.171875, 100.203125,
            100.234375, 100.265625, 100.296875, 100.328125, 100.359375, 100.390625, 100.421875,
            100.453125, 100.484375, 100.515625,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        1.0e308,
        f64::INFINITY,
        &[
            f64::INFINITY,
            1.348269851146737e308,
            1.1235582092889474e308,
            1.5729814930045264e308,
            1.0112023883600527e308,
            1.2359140302178422e308,
            1.4606256720756317e308,
            1.6853373139334212e308,
            1.0673802988245e308,
            1.1797361197533948e308,
            1.2920919406822896e308,
            1.4044477616111843e308,
            1.516803582540079e308,
            1.6291594034689738e308,
            1.7415152243978685e308,
            1.0392913435922764e308,
            1.0954692540567238e308,
            1.1516471645211711e308,
            1.2078250749856185e308,
            1.2640029854500659e308,
            1.3201808959145132e308,
            1.3763588063789606e308,
            1.432536716843408e308,
            1.4887146273078554e308,
            1.5448925377723027e308,
            1.6010704482367501e308,
            1.6572483587011975e308,
            1.7134262691656448e308,
            1.7696041796300922e308,
            1.0252468659761645e308,
            1.0533358212083882e308,
            1.081424776440612e308,
            1.1095137316728356e308,
            1.1376026869050593e308,
            1.165691642137283e308,
            1.1937805973695067e308,
            1.2218695526017303e308,
            1.249958507833954e308,
            1.2780474630661777e308,
            1.3061364182984014e308,
            1.334225373530625e308,
            1.3623143287628488e308,
            1.3904032839950725e308,
            1.4184922392272961e308,
            1.4465811944595198e308,
            1.4746701496917435e308,
            1.5027591049239672e308,
            1.5308480601561909e308,
            1.5589370153884146e308,
            1.5870259706206383e308,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        f64::MIN_POSITIVE_SUBNORMAL,
        &[0.0, -0.0, 5.0e-324, -5.0e-324],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        -0.0,
        f64::MIN_POSITIVE_SUBNORMAL,
        &[0.0, -0.0, 5.0e-324],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        0.0,
        f64::MIN_POSITIVE_SUBNORMAL,
        &[0.0, 5.0e-324],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        -0.0,
        &[-0.0, -5.0e-324],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        0.0,
        &[0.0, -0.0, -5.0e-324],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        f64::NEGATIVE_INFINITY,
        f64::INFINITY,
        &[
            f64::INFINITY,
            f64::NEGATIVE_INFINITY,
            0.0,
            -0.0,
            1.0,
            -1.0,
            2.0,
            -2.0,
            1.5,
            -1.5,
            0.5,
            -0.5,
            1.25,
            -1.25,
            3.0,
            -3.0,
            1.75,
            -1.75,
            4.0,
            -4.0,
            1.125,
            -1.125,
            2.5,
            -2.5,
            1.375,
            -1.375,
            0.75,
            -0.75,
            1.625,
            -1.625,
            3.5,
            -3.5,
            1.875,
            -1.875,
            0.25,
            -0.25,
            1.0625,
            -1.0625,
            2.25,
            -2.25,
            1.1875,
            -1.1875,
            0.625,
            -0.625,
            1.3125,
            -1.3125,
            2.75,
            -2.75,
            1.4375,
            -1.4375,
        ],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        f64::NEGATIVE_INFINITY,
        f64::NEGATIVE_INFINITY,
        &[f64::NEGATIVE_INFINITY],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(
        f64::INFINITY,
        f64::INFINITY,
        &[f64::INFINITY],
    );
    exhaustive_primitive_float_inclusive_range_helper::<f64>(0.0, 0.0, &[0.0]);
    exhaustive_primitive_float_inclusive_range_helper::<f64>(-0.0, -0.0, &[-0.0]);
    exhaustive_primitive_float_inclusive_range_helper::<f64>(-0.0, 0.0, &[0.0, -0.0]);
}

fn exhaustive_primitive_float_inclusive_range_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(exhaustive_primitive_float_inclusive_range::<T>(
        T::ONE,
        T::ZERO
    ));
    assert_panic!(exhaustive_primitive_float_inclusive_range::<T>(
        T::ONE,
        T::NAN
    ));
}

#[test]
fn exhaustive_primitive_float_inclusive_range_fail() {
    apply_fn_to_primitive_floats!(exhaustive_primitive_float_inclusive_range_fail_helper);
}
