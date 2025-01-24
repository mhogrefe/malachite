// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::Float;
use std::mem::size_of;

#[test]
fn test_size() {
    if size_of::<usize>() == size_of::<u64>() {
        assert_eq!(size_of::<Float>(), 40);
    }
}
