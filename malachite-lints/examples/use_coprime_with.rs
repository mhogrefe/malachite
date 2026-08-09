// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CoprimeWith, Gcd};
use malachite_nz::natural::Natural;

const X: Natural = Natural::const_from(56);
const Y: Natural = Natural::const_from(33);

fn main() {
    // Comparing a gcd with 1: flagged.
    let _ = (&X).gcd(&Y) == 1u32;
    // The negated form: flagged.
    let _ = (&X).gcd(&Y) != 1u32;
    // The constant on the left: flagged.
    let _ = 1u32 == (&X).gcd(&Y);
    // On primitives: flagged.
    let a = 56u64;
    let b = 33u64;
    let _ = a.gcd(b) == 1;

    // The idiomatic forms: fine.
    let _ = (&X).coprime_with(&Y);
    let _ = !a.coprime_with(b);
    // Comparing a gcd with something other than 1: fine.
    let _ = (&X).gcd(&Y) == 11u32;
    // A stored gcd compared with 1 may be reused; only direct comparisons are flagged: fine.
    let g = (&X).gcd(&Y);
    let _ = g == 1u32;
    println!("{g}");
}
