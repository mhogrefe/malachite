// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_q::exhaustive::exhaustive_rational_range;
use malachite_q::Rational;
use std::str::FromStr;

fn exhaustive_rational_range_helper(a: &str, b: &str, values: &str) {
    let a = Rational::from_str(a).unwrap();
    let b = Rational::from_str(b).unwrap();
    let xs = exhaustive_rational_range(a, b)
        .take(50)
        .collect_vec()
        .to_debug_string();
    assert_eq!(xs, values);
}

#[test]
fn test_exhaustive_rational_range() {
    exhaustive_rational_range_helper(
        "0",
        "2",
        "[0, 1/2, 1, 1/3, 3/2, 2/3, 4/3, 1/6, 5/3, 1/4, 3/4, 5/6, 5/4, 1/5, 7/4, 1/8, 2/5, 7/6, \
        3/5, 1/7, 4/5, 11/6, 6/5, 3/8, 7/5, 2/7, 8/5, 5/8, 9/5, 3/7, 4/7, 1/12, 5/7, 7/8, 6/7, \
        1/9, 8/7, 9/8, 9/7, 1/10, 10/7, 11/8, 11/7, 2/9, 12/7, 13/8, 13/7, 1/11, 15/8, 4/9]",
    );
    exhaustive_rational_range_helper(
        "1/3",
        "1/2",
        "[1/3, 2/5, 3/7, 4/9, 3/8, 4/11, 5/11, 5/14, 5/12, 5/13, 6/13, 7/15, 7/16, 6/17, 7/17, \
        8/21, 8/17, 7/18, 7/19, 10/21, 8/19, 7/20, 9/19, 9/22, 9/20, 8/23, 9/23, 9/25, 10/23, \
        11/24, 11/23, 11/28, 11/25, 9/26, 12/25, 10/27, 11/26, 11/27, 13/27, 11/30, 13/28, 10/29, \
        11/29, 11/31, 12/29, 13/30, 13/29, 13/33, 14/29, 12/31]",
    );
    exhaustive_rational_range_helper(
        "99/100",
        "101/100",
        "[1, 99/100, 100/101, 102/103, 102/101, 101/102, 103/102, 104/105, 104/103, 103/104, \
        105/104, 105/106, 106/105, 107/106, 106/107, 110/111, 108/107, 107/108, 109/108, 109/110, \
        108/109, 111/110, 110/109, 111/112, 112/111, 113/112, 112/113, 114/115, 114/113, 113/114, \
        115/114, 118/119, 116/115, 115/116, 117/116, 117/118, 116/117, 119/118, 118/117, 119/120, \
        120/119, 121/120, 120/121, 122/123, 122/121, 121/122, 123/122, 125/126, 124/123, 123/124]",
    );
    exhaustive_rational_range_helper(
        "268876667/98914198",
        "245850922/78256779",
        "[3, 11/4, 14/5, 20/7, 17/6, 23/8, 25/8, 30/11, 25/9, 29/10, 26/9, 31/11, 28/9, 31/10, \
        32/11, 41/15, 34/11, 35/12, 37/12, 39/14, 36/13, 41/14, 37/13, 45/16, 38/13, 43/14, \
        40/13, 43/15, 44/15, 47/16, 46/15, 57/20, 47/15, 49/16, 47/17, 52/19, 48/17, 49/18, \
        49/17, 59/20, 50/17, 53/18, 52/17, 53/19, 53/17, 55/18, 54/19, 63/23, 55/19, 61/20]",
    );

    exhaustive_rational_range_helper(
        "-2",
        "0",
        "[-1, -1/2, -2, -1/3, -3/2, -2/3, -4/3, -1/6, -5/3, -1/4, -3/4, -5/6, -5/4, -1/5, -7/4, \
        -1/8, -2/5, -7/6, -3/5, -1/7, -4/5, -11/6, -6/5, -3/8, -7/5, -2/7, -8/5, -5/8, -9/5, \
        -3/7, -4/7, -1/12, -5/7, -7/8, -6/7, -1/9, -8/7, -9/8, -9/7, -1/10, -10/7, -11/8, -11/7, \
        -2/9, -12/7, -13/8, -13/7, -1/11, -15/8, -4/9]",
    );
    exhaustive_rational_range_helper(
        "-1/2",
        "-1/3",
        "[-1/2, -2/5, -3/7, -4/9, -3/8, -4/11, -5/11, -5/14, -5/12, -5/13, -6/13, -7/15, -7/16, \
        -6/17, -7/17, -8/21, -8/17, -7/18, -7/19, -10/21, -8/19, -7/20, -9/19, -9/22, -9/20, \
        -8/23, -9/23, -9/25, -10/23, -11/24, -11/23, -11/28, -11/25, -9/26, -12/25, -10/27, \
        -11/26, -11/27, -13/27, -11/30, -13/28, -10/29, -11/29, -11/31, -12/29, -13/30, -13/29, \
        -13/33, -14/29, -12/31]",
    );
    exhaustive_rational_range_helper(
        "-101/100",
        "-99/100",
        "[-1, -101/100, -100/101, -102/103, -102/101, -101/102, -103/102, -104/105, -104/103, \
        -103/104, -105/104, -105/106, -106/105, -107/106, -106/107, -110/111, -108/107, -107/108, \
        -109/108, -109/110, -108/109, -111/110, -110/109, -111/112, -112/111, -113/112, -112/113, \
        -114/115, -114/113, -113/114, -115/114, -118/119, -116/115, -115/116, -117/116, -117/118, \
        -116/117, -119/118, -118/117, -119/120, -120/119, -121/120, -120/121, -122/123, -122/121, \
        -121/122, -123/122, -125/126, -124/123, -123/124]",
    );
    exhaustive_rational_range_helper(
        "-245850922/78256779",
        "-268876667/98914198",
        "[-3, -11/4, -14/5, -20/7, -17/6, -23/8, -25/8, -30/11, -25/9, -29/10, -26/9, -31/11, \
        -28/9, -31/10, -32/11, -41/15, -34/11, -35/12, -37/12, -39/14, -36/13, -41/14, -37/13, \
        -45/16, -38/13, -43/14, -40/13, -43/15, -44/15, -47/16, -46/15, -57/20, -47/15, -49/16, \
        -47/17, -52/19, -48/17, -49/18, -49/17, -59/20, -50/17, -53/18, -52/17, -53/19, -53/17, \
        -55/18, -54/19, -63/23, -55/19, -61/20]",
    );

    exhaustive_rational_range_helper(
        "-2",
        "3",
        "[0, 1/2, 1, 1/3, -1, -1/2, 2, 1/4, -2, 3/2, -3/2, -1/4, 5/2, -1/3, 2/3, 1/7, -2/3, 3/4, \
        4/3, 1/5, -4/3, -3/4, 5/3, 1/6, -5/3, 5/4, 7/3, -1/5, 8/3, -5/4, 7/4, 1/9, -7/4, 2/5, \
        9/4, -1/6, 11/4, -2/5, 3/5, 1/8, -3/5, 5/6, 4/5, -1/7, -4/5, -5/6, 6/5, -1/9, -6/5, 7/6]",
    );
    exhaustive_rational_range_helper(
        "-1/2",
        "1/3",
        "[0, -1/2, -1/3, 1/5, 1/4, -1/5, -1/4, 1/7, -2/5, 1/6, -1/6, 1/8, -1/7, -1/8, 2/7, 1/11, \
        -2/7, -3/8, -3/7, 1/9, -1/9, 1/10, 2/9, 1/12, -2/9, -1/10, -4/9, -1/11, 3/10, 2/11, \
        -3/10, 1/15, -2/11, -1/12, 3/11, 1/13, -3/11, -5/12, -4/11, 1/14, -5/11, -1/13, 2/13, \
        -1/15, -2/13, -1/14, 3/13, 1/17, -3/13, 3/14]",
    );
    exhaustive_rational_range_helper(
        "-101/100",
        "99/100",
        "[0, 1/2, -1, 1/3, -1/2, -1/3, 2/3, 1/6, -2/3, 1/4, -1/4, -1/6, 3/4, 1/5, -3/4, 1/8, \
        -1/5, 5/6, 2/5, 1/7, -2/5, -5/6, 3/5, -1/8, -3/5, -1/7, 4/5, 3/8, -4/5, 2/7, -2/7, 1/12, \
        3/7, -3/8, -3/7, 1/9, 4/7, 5/8, -4/7, 1/10, 5/7, -5/8, -5/7, -1/9, 6/7, 7/8, -6/7, 1/11, \
        -7/8, 2/9]",
    );
    exhaustive_rational_range_helper(
        "-245850922/78256779",
        "268876667/98914198",
        "[0, 1/2, 1, 1/3, -1, -1/2, 2, 1/4, -2, 3/2, -3, -1/3, -3/2, 2/3, 5/2, 1/6, -5/2, -2/3, \
        4/3, 1/5, -4/3, -1/4, 5/3, -1/6, -5/3, 3/4, 7/3, -1/5, -7/3, -3/4, 8/3, 1/8, -8/3, 5/4, \
        -5/4, 5/6, 7/4, 2/5, -7/4, 1/7, 9/4, -2/5, -9/4, -5/6, -11/4, 3/5, -3/5, 1/9, 4/5, 7/6]",
    );
    exhaustive_rational_range_helper("0", "0", "[]");
}

#[test]
#[should_panic]
fn exhaustive_rational_range_fail() {
    exhaustive_rational_range(Rational::ONE, Rational::ZERO);
}
