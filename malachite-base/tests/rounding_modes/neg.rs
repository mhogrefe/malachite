// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::rounding_mode_gen;

#[test]
fn test_neg() {
    let test = |mut rm: RoundingMode, out| {
        assert_eq!(-rm, out);
        rm.neg_assign();
        assert_eq!(rm, out);
    };
    test(Down, Down);
    test(Up, Up);
    test(Floor, Ceiling);
    test(Ceiling, Floor);
    test(Nearest, Nearest);
    test(Exact, Exact);
}

#[test]
fn neg_properties() {
    rounding_mode_gen().test_properties(|rm| {
        assert_eq!(-(-rm), rm);
        let mut rm_alt = rm;
        rm_alt.neg_assign();
        assert_eq!(rm_alt, -rm);
    });
}
