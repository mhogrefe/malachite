// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::first_and_last;

fn first_and_last_helper(xs: &[u8], result: Option<(u8, u8)>) {
    assert_eq!(first_and_last(&mut xs.iter().copied()), result);
}

#[test]
fn test_first_and_last() {
    first_and_last_helper(&[1, 2, 10, 11, 12, 7, 8, 16, 5], Some((1, 5)));
    first_and_last_helper(&[5], Some((5, 5)));
    first_and_last_helper(&[], None);
}
