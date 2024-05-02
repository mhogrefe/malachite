// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::test_util::natural::logic::{natural_op_bits, natural_op_limbs};

pub fn natural_and_alt_1(x: &Natural, y: &Natural) -> Natural {
    natural_op_bits(&|a, b| a && b, x, y)
}

pub fn natural_and_alt_2(x: &Natural, y: &Natural) -> Natural {
    natural_op_limbs(&|a, b| a & b, x, y)
}
