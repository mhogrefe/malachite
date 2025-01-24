// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::comparison::{
    is_strictly_ascending, is_weakly_ascending, is_weakly_descending,
};

fn is_weakly_ascending_helper(xs: &[u8], result: bool) {
    assert_eq!(is_weakly_ascending(xs.iter()), result);
    assert_eq!(is_weakly_descending(xs.iter().rev()), result);
    if xs.len() < 2 {
        assert!(result);
    }
    if !result {
        assert!(!is_strictly_ascending(xs.iter()));
    }
}

#[test]
fn test_is_weakly_ascending() {
    is_weakly_ascending_helper(&[], true);
    is_weakly_ascending_helper(&[5], true);
    is_weakly_ascending_helper(&[5, 6], true);
    is_weakly_ascending_helper(&[5, 5], true);
    is_weakly_ascending_helper(&[5, 4], false);
    is_weakly_ascending_helper(&[1, 2, 3, 4], true);
    is_weakly_ascending_helper(&[1, 2, 2, 4], true);
    is_weakly_ascending_helper(&[1, 3, 2, 4], false);
    is_weakly_ascending_helper(&[3, 1, 4, 1, 5, 9], false);
}
