// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::is_unique;

fn is_unique_helper(xs: &[u8], result: bool) {
    assert_eq!(is_unique(xs.iter()), result);
    assert_eq!(is_unique(xs.iter().rev()), result);
}

#[test]
fn test_is_unique() {
    is_unique_helper(&[], true);
    is_unique_helper(&[5], true);
    is_unique_helper(&[5, 6], true);
    is_unique_helper(&[5, 5], false);
    is_unique_helper(&[5, 4], true);
    is_unique_helper(&[1, 2, 3, 4], true);
    is_unique_helper(&[1, 2, 2, 4], false);
    is_unique_helper(&[1, 2, 3, 1], false);
    is_unique_helper(&[4; 100], false);

    assert_eq!(is_unique([1, 2].iter().cycle()), false);
}
