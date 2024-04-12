// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::exhaustive::negative_primitive_floats_increasing;
use malachite_base::test_util::num::exhaustive::*;

fn negative_primitive_floats_increasing_helper<T: PrimitiveFloat>(first_20: &[T], last_20: &[T]) {
    exhaustive_primitive_floats_helper_helper_with_reverse(
        negative_primitive_floats_increasing::<T>(),
        first_20,
        last_20,
    );
}

#[test]
fn test_negative_primitive_floats_increasing() {
    negative_primitive_floats_increasing_helper::<f32>(
        &[
            f32::NEGATIVE_INFINITY,
            -3.4028235e38,
            -3.4028233e38,
            -3.402823e38,
            -3.4028229e38,
            -3.4028227e38,
            -3.4028225e38,
            -3.4028222e38,
            -3.402822e38,
            -3.4028218e38,
            -3.4028216e38,
            -3.4028214e38,
            -3.4028212e38,
            -3.402821e38,
            -3.4028208e38,
            -3.4028206e38,
            -3.4028204e38,
            -3.4028202e38,
            -3.40282e38,
            -3.4028198e38,
        ],
        &[
            -2.8e-44, -2.7e-44, -2.5e-44, -2.4e-44, -2.2e-44, -2.1e-44, -2.0e-44, -1.8e-44,
            -1.7e-44, -1.5e-44, -1.4e-44, -1.3e-44, -1.1e-44, -1.0e-44, -8.0e-45, -7.0e-45,
            -6.0e-45, -4.0e-45, -3.0e-45, -1.0e-45,
        ],
    );
    negative_primitive_floats_increasing_helper::<f64>(
        &[
            f64::NEGATIVE_INFINITY,
            -1.7976931348623157e308,
            -1.7976931348623155e308,
            -1.7976931348623153e308,
            -1.7976931348623151e308,
            -1.797693134862315e308,
            -1.7976931348623147e308,
            -1.7976931348623145e308,
            -1.7976931348623143e308,
            -1.7976931348623141e308,
            -1.797693134862314e308,
            -1.7976931348623137e308,
            -1.7976931348623135e308,
            -1.7976931348623133e308,
            -1.7976931348623131e308,
            -1.797693134862313e308,
            -1.7976931348623127e308,
            -1.7976931348623125e308,
            -1.7976931348623123e308,
            -1.7976931348623121e308,
        ],
        &[
            -1.0e-322, -9.4e-323, -9.0e-323, -8.4e-323, -8.0e-323, -7.4e-323, -7.0e-323, -6.4e-323,
            -6.0e-323, -5.4e-323, -5.0e-323, -4.4e-323, -4.0e-323, -3.5e-323, -3.0e-323, -2.5e-323,
            -2.0e-323, -1.5e-323, -1.0e-323, -5.0e-324,
        ],
    );
}
