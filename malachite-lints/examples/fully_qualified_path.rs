// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Mod;
use malachite_nz::integer;
use malachite_nz::natural::Natural;

fn main() {
    // Full paths to Malachite items in code: flagged.
    let _ = malachite_nz::natural::Natural::from(3u32);
    let _: malachite_nz::integer::Integer = malachite_nz::integer::Integer::from(3);
    let _ = malachite_base::num::arithmetic::traits::Mod::mod_op(5u32, 3u32);
    let _ = malachite_q::Rational::from(3u32);
    // Imported items, and paths through an imported module: fine.
    let _ = Natural::from(3u32);
    let _ = integer::Integer::from(3);
    let _ = 5u32.mod_op(3u32);
    // Paths into other crates are out of scope: fine.
    let _ = core::cmp::max(1u32, 2u32);
    let _ = std::mem::size_of::<Natural>();
}
