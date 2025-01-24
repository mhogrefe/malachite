// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::common::rle_encode;

pub(crate) fn generate_rle_encoding() {
    // Example xs
    let xs = &[1, 1, 1, 0, 0, 0, 0, 0, 0, 2, 2];
    println!("{:?}", rle_encode(xs.iter()));
}
