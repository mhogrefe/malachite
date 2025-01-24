// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_q::exhaustive::exhaustive_rational_range_to_negative_infinity;
use malachite_q::Rational;

fn exhaustive_rational_range_to_negative_infinity_helper(a: Rational, values: &str) {
    let xs = exhaustive_rational_range_to_negative_infinity(a)
        .take(50)
        .collect_vec()
        .to_debug_string();
    assert_eq!(xs, values);
}

#[test]
fn test_exhaustive_rational_range_to_negative_infinity() {
    exhaustive_rational_range_to_negative_infinity_helper(
        Rational::ZERO,
        "[0, -1/2, -1, -1/3, -2, -3/2, -3, -1/4, -4, -5/2, -5, -2/3, -6, -7/2, -7, -1/5, -8, \
        -9/2, -9, -4/3, -10, -11/2, -11, -3/4, -12, -13/2, -13, -5/3, -14, -15/2, -15, -1/6, \
        -16, -17/2, -17, -7/3, -18, -19/2, -19, -5/4, -20, -21/2, -21, -8/3, -22, -23/2, -23, \
        -2/5, -24, -25/2]",
    );
    exhaustive_rational_range_to_negative_infinity_helper(
        Rational::from(5),
        "[0, 1/2, 1, 1/3, -1, -1/2, 2, 1/4, -2, 3/2, 3, -1/3, -3, -3/2, 4, 1/5, -4, 5/2, 5, 2/3, \
        -5, -5/2, -6, -1/4, -7, 7/2, -8, -2/3, -9, -7/2, -10, 1/6, -11, 9/2, -12, 4/3, -13, -9/2, \
        -14, 3/4, -15, -11/2, -16, -4/3, -17, -13/2, -18, -1/5, -19, -15/2]",
    );
    exhaustive_rational_range_to_negative_infinity_helper(
        Rational::from(-5),
        "[-5, -11/2, -6, -16/3, -7, -13/2, -8, -21/4, -9, -15/2, -10, -17/3, -11, -17/2, -12, \
        -26/5, -13, -19/2, -14, -19/3, -15, -21/2, -16, -23/4, -17, -23/2, -18, -20/3, -19, \
        -25/2, -20, -31/6, -21, -27/2, -22, -22/3, -23, -29/2, -24, -25/4, -25, -31/2, -26, \
        -23/3, -27, -33/2, -28, -27/5, -29, -35/2]",
    );
    exhaustive_rational_range_to_negative_infinity_helper(
        Rational::exact_from(std::f64::consts::PI),
        "[0, 1/2, 1, 1/3, -1, -1/2, 2, 1/4, -2, 3/2, 3, -1/3, -3, -3/2, -4, 1/5, -5, 5/2, -6, \
        2/3, -7, -5/2, -8, -1/4, -9, -7/2, -10, -2/3, -11, -9/2, -12, 1/6, -13, -11/2, -14, 4/3, \
        -15, -13/2, -16, 3/4, -17, -15/2, -18, -4/3, -19, -17/2, -20, -1/5, -21, -19/2]",
    );
    exhaustive_rational_range_to_negative_infinity_helper(
        -Rational::exact_from(std::f64::consts::PI),
        "[-4, -7/2, -5, -10/3, -6, -9/2, -7, -13/4, -8, -11/2, -9, -11/3, -10, -13/2, -11, -16/5, \
        -12, -15/2, -13, -13/3, -14, -17/2, -15, -15/4, -16, -19/2, -17, -14/3, -18, -21/2, -19, \
        -19/6, -20, -23/2, -21, -16/3, -22, -25/2, -23, -17/4, -24, -27/2, -25, -17/3, -26, \
        -29/2, -27, -17/5, -28, -31/2]",
    );
}
