// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CeilingDivMod, CeilingDivNegMod, DivMod, DivRem};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

fn main() {
    let x = const { Natural::const_from(100) };
    let y = const { Natural::const_from(7) };
    // Projecting the quotient: flagged.
    let _ = (&x).div_mod(&y).0;
    // Projecting the remainder: flagged.
    let _ = (&x).div_rem(&y).1;
    // The other families: flagged.
    let a = const { Integer::const_from_signed(100) };
    let b = const { Integer::const_from_signed(7) };
    let _ = (&a).ceiling_div_mod(&b).0;
    let _ = (&x).ceiling_div_neg_mod(&y).1;
    // Using both components: fine.
    let (q, r) = (&x).div_mod(&y);
    println!("{q} {r}");
    // Projecting an unrelated tuple result: fine.
    let p = (x, y);
    let _ = p.0;
}
