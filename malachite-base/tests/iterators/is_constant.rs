// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::comparison::{is_weakly_ascending, is_weakly_descending};
use malachite_base::iterators::is_constant;

fn is_constant_helper(xs: &[u8], result: bool) {
    assert_eq!(is_constant(xs.iter()), result);
    assert_eq!(is_constant(xs.iter().rev()), result);
    assert_eq!(
        is_weakly_ascending(xs.iter()) && is_weakly_descending(xs.iter()),
        result
    );
    if xs.len() < 2 {
        assert!(result);
    }
}

#[test]
fn test_is_constant() {
    is_constant_helper(&[], true);
    is_constant_helper(&[5], true);
    is_constant_helper(&[5, 6], false);
    is_constant_helper(&[5, 5], true);
    is_constant_helper(&[5, 4], false);
    is_constant_helper(&[1; 4], true);
    is_constant_helper(&[1, 2, 3, 4], false);
    is_constant_helper(&[1, 2, 2, 4], false);
    is_constant_helper(&[4; 100], true);

    assert_eq!(is_constant([1, 2].iter().cycle()), false);
}
