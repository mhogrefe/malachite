// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::comparison::{
    is_strictly_ascending, is_strictly_descending, is_weakly_ascending, is_weakly_descending,
};

fn is_strictly_descending_helper(xs: &[u8], result: bool) {
    assert_eq!(is_strictly_descending(xs.iter()), result);
    assert_eq!(is_strictly_ascending(xs.iter().rev()), result);
    if xs.len() < 2 {
        assert!(result);
    }
    if result {
        assert!(is_weakly_descending(xs.iter()));
        if xs.len() > 1 {
            assert!(!is_weakly_ascending(xs.iter()));
        }
    }
}

#[test]
fn test_is_strictly_descending() {
    is_strictly_descending_helper(&[], true);
    is_strictly_descending_helper(&[5], true);
    is_strictly_descending_helper(&[6, 5], true);
    is_strictly_descending_helper(&[6, 6], false);
    is_strictly_descending_helper(&[6, 7], false);
    is_strictly_descending_helper(&[4, 3, 2, 1], true);
    is_strictly_descending_helper(&[4, 2, 2, 1], false);
    is_strictly_descending_helper(&[4, 2, 3, 1], false);
    is_strictly_descending_helper(&[3, 1, 4, 1, 5, 9], false);
}
