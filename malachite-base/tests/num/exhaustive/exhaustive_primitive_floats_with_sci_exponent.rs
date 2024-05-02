// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_primitive_floats_with_sci_exponent;
use malachite_base::test_util::num::exhaustive::*;
use std::panic::catch_unwind;

fn exhaustive_primitive_floats_with_sci_exponent_helper<T: PrimitiveFloat>(
    sci_exponent: i64,
    out: &[T],
) {
    exhaustive_primitive_floats_helper_helper_with_limit(
        20,
        exhaustive_primitive_floats_with_sci_exponent::<T>(sci_exponent),
        out,
    );
}

#[test]
fn test_exhaustive_primitive_floats_with_sci_exponent() {
    exhaustive_primitive_floats_with_sci_exponent_helper::<f32>(
        0,
        &[
            1.0, 1.5, 1.25, 1.75, 1.125, 1.375, 1.625, 1.875, 1.0625, 1.1875, 1.3125, 1.4375,
            1.5625, 1.6875, 1.8125, 1.9375, 1.03125, 1.09375, 1.15625, 1.21875,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_helper::<f32>(
        4,
        &[
            16.0, 24.0, 20.0, 28.0, 18.0, 22.0, 26.0, 30.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0,
            29.0, 31.0, 16.5, 17.5, 18.5, 19.5,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_helper::<f32>(
        -4,
        &[
            0.0625,
            0.09375,
            0.078125,
            0.109375,
            0.0703125,
            0.0859375,
            0.1015625,
            0.1171875,
            0.06640625,
            0.07421875,
            0.08203125,
            0.08984375,
            0.09765625,
            0.10546875,
            0.11328125,
            0.12109375,
            0.064453125,
            0.068359375,
            0.072265625,
            0.076171875,
        ],
    );

    exhaustive_primitive_floats_with_sci_exponent_helper::<f32>(-149, &[1.0e-45]);
    exhaustive_primitive_floats_with_sci_exponent_helper::<f32>(-148, &[3.0e-45, 4.0e-45]);
    exhaustive_primitive_floats_with_sci_exponent_helper::<f32>(
        -147,
        &[6.0e-45, 8.0e-45, 7.0e-45, 1.0e-44],
    );
    exhaustive_primitive_floats_with_sci_exponent_helper::<f32>(
        127,
        &[
            1.7014118e38,
            2.5521178e38,
            2.1267648e38,
            2.9774707e38,
            1.9140883e38,
            2.3394413e38,
            2.7647942e38,
            3.1901472e38,
            1.80775e38,
            2.0204266e38,
            2.233103e38,
            2.4457795e38,
            2.658456e38,
            2.8711325e38,
            3.083809e38,
            3.2964854e38,
            1.754581e38,
            1.8609192e38,
            1.9672574e38,
            2.0735957e38,
        ],
    );

    exhaustive_primitive_floats_with_sci_exponent_helper::<f64>(
        0,
        &[
            1.0, 1.5, 1.25, 1.75, 1.125, 1.375, 1.625, 1.875, 1.0625, 1.1875, 1.3125, 1.4375,
            1.5625, 1.6875, 1.8125, 1.9375, 1.03125, 1.09375, 1.15625, 1.21875,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_helper::<f64>(
        4,
        &[
            16.0, 24.0, 20.0, 28.0, 18.0, 22.0, 26.0, 30.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0,
            29.0, 31.0, 16.5, 17.5, 18.5, 19.5,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_helper::<f64>(
        -4,
        &[
            0.0625,
            0.09375,
            0.078125,
            0.109375,
            0.0703125,
            0.0859375,
            0.1015625,
            0.1171875,
            0.06640625,
            0.07421875,
            0.08203125,
            0.08984375,
            0.09765625,
            0.10546875,
            0.11328125,
            0.12109375,
            0.064453125,
            0.068359375,
            0.072265625,
            0.076171875,
        ],
    );

    exhaustive_primitive_floats_with_sci_exponent_helper::<f64>(-1074, &[5.0e-324]);
    exhaustive_primitive_floats_with_sci_exponent_helper::<f64>(-1073, &[1.0e-323, 1.5e-323]);
    exhaustive_primitive_floats_with_sci_exponent_helper::<f64>(
        -1072,
        &[2.0e-323, 3.0e-323, 2.5e-323, 3.5e-323],
    );
    exhaustive_primitive_floats_with_sci_exponent_helper::<f64>(
        1023,
        &[
            8.98846567431158e307,
            1.348269851146737e308,
            1.1235582092889474e308,
            1.5729814930045264e308,
            1.0112023883600527e308,
            1.2359140302178422e308,
            1.4606256720756317e308,
            1.6853373139334212e308,
            9.550244778956053e307,
            1.0673802988245e308,
            1.1797361197533948e308,
            1.2920919406822896e308,
            1.4044477616111843e308,
            1.516803582540079e308,
            1.6291594034689738e308,
            1.7415152243978685e308,
            9.269355226633816e307,
            9.83113433127829e307,
            1.0392913435922764e308,
            1.0954692540567238e308,
        ],
    );
}

fn exhaustive_primitive_floats_with_sci_exponent_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent::<T>(10000));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent::<T>(-10000));
}

#[test]
fn exhaustive_primitive_floats_with_sci_exponent_fail() {
    apply_fn_to_primitive_floats!(exhaustive_primitive_floats_with_sci_exponent_fail_helper);
}
