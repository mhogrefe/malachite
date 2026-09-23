// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

extern crate malachite_base;

// Another crate reached through `crate::`: flagged, once for the braced list.
use crate::malachite_base::num::arithmetic::traits::Mod;
use crate::malachite_base::num::basic::traits::{One, Zero};
// Another crate named directly: fine.
use malachite_base::num::arithmetic::traits::Parity;

mod inner {
    // A local item reached through `crate::`: fine, the prefix is needed here.
    pub(crate) use crate::helper;
}

fn helper() -> u32 {
    // Flagged in an expression too.
    crate::malachite_base::num::arithmetic::traits::Mod::mod_op(5u32, 3)
}

fn main() {
    let _ = inner::helper() + u32::ONE + u32::ZERO;
    let _ = 5u32.mod_op(3).even();
}
