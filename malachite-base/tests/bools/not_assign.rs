// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::NotAssign;
use malachite_base::test_util::generators::bool_gen;

#[test]
fn test_not_assign() {
    let test = |mut b: bool, out| {
        b.not_assign();
        assert_eq!(b, out);
    };
    test(false, true);
    test(true, false);
}

#[test]
fn not_assign_properties() {
    bool_gen().test_properties(|b| {
        let mut mut_b = b;
        mut_b.not_assign();
        assert_ne!(mut_b, b);
        assert_eq!(mut_b, !b);
        mut_b.not_assign();
        assert_eq!(mut_b, b);
    });
}
