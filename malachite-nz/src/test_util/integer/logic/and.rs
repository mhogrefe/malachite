// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::test_util::integer::logic::{integer_op_bits, integer_op_limbs};

pub fn integer_and_alt_1(x: &Integer, y: &Integer) -> Integer {
    integer_op_bits(&|a, b| a && b, x, y)
}

pub fn integer_and_alt_2(x: &Integer, y: &Integer) -> Integer {
    integer_op_limbs(&|a, b| a & b, x, y)
}
