// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::integer::Integer;

#[test]
fn test_default() {
    let default = Integer::default();
    assert!(default.is_valid());
    assert_eq!(default, 0);
    assert_eq!(default.to_string(), "0");
}
