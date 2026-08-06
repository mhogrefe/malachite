// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::test_util::natural::arithmetic::mod_div::mod_div_simple;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd, Mod};

// A simple reference implementation of `ModDivList`, deriving the progression from any single
// quotient: the solutions of `qc ≡ b mod m` are spaced `m / gcd(c, m)` apart, and the smallest is
// any quotient reduced modulo the spacing. The result is canonical, so this agrees exactly with the
// implementation in `mod_div_list.rs`.
pub fn mod_div_list_simple(
    b: Natural,
    c: Natural,
    m: Natural,
) -> Option<(Natural, Natural, Natural)> {
    let q = mod_div_simple(b, c.clone(), m.clone())?;
    let length = c.gcd(&m);
    let stride = m.div_exact(&length);
    Some((q.mod_op(&stride), stride, length))
}
