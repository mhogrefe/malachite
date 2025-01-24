// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

pub fn slice_move_left_naive<T: Copy>(xs: &mut [T], amount: usize) {
    let slice = xs[amount..].to_vec();
    let limit = xs.len() - amount;
    xs[..limit].copy_from_slice(&slice);
}
