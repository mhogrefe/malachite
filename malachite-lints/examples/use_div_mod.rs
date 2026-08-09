// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivMod, DivRem};
use malachite_nz::natural::Natural;

const X: Natural = Natural::const_from(97);
const Y: Natural = Natural::const_from(7);

fn main() {
    // A quotient and a remainder of the same unsigned operands: flagged.
    let a = 97u64;
    let b = 7u64;
    let q = a / b;
    let r = a % b;
    println!("{q} {r}");
    // The remainder first: flagged.
    let r = a % b;
    let q = a / b;
    println!("{q} {r}");
    // Signed operands, where `/` and `%` truncate: flagged, suggesting `div_rem`.
    let c = -97i32;
    let d = 7i32;
    let q = c / d;
    let r = c % d;
    println!("{q} {r}");
    // Bignums: flagged.
    let q = (&X) / (&Y);
    let r = (&X) % (&Y);
    println!("{q} {r}");

    // The paired call: fine.
    let (q, r) = a.div_mod(b);
    println!("{q} {r}");
    let (q, r) = c.div_rem(d);
    println!("{q} {r}");
    // Only one of the two: fine.
    let q = a / b;
    println!("{q}");
    // Different operands: fine.
    let q = a / b;
    let r = b % a;
    println!("{q} {r}");
    // Separated by a statement that could change the operands: still flagged only when adjacent,
    // so this is fine.
    let mut e = 97u64;
    let q = e / b;
    e += 1;
    let r = e % b;
    println!("{q} {r}");
}
