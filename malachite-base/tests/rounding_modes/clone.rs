// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{rounding_mode_gen, rounding_mode_pair_gen};

#[test]
#[allow(clippy::clone_on_copy)]
fn test_clone() {
    let test = |rm: RoundingMode| {
        let cloned = rm.clone();
        assert_eq!(cloned, rm);
    };
    test(Down);
    test(Up);
    test(Floor);
    test(Ceiling);
    test(Nearest);
    test(Exact);
}

#[test]
fn test_clone_from() {
    let test = |mut x: RoundingMode, y: RoundingMode| {
        x.clone_from(&y);
        assert_eq!(x, y);
    };
    test(Exact, Floor);
    test(Up, Ceiling);
}

#[test]
fn clone_and_clone_from_properties() {
    rounding_mode_gen().test_properties(|rm| {
        assert_eq!(rm.clone(), rm);
    });

    rounding_mode_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x;
        mut_x.clone_from(&y);
        assert_eq!(mut_x, y);
    });
}
