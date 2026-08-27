// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::logic::traits::NotAssign;
use malachite_nz::integer::Integer;

const THREE: Integer = Integer::const_from_signed(3);
const SEVEN: Integer = Integer::const_from_signed(7);

fn main() {
    // Flipping a bool by reassignment: flagged.
    let mut parity = true;
    parity = !parity;
    println!("{parity}");
    // Negating a primitive by reassignment: flagged.
    let mut x = 5i64;
    x = -x;
    println!("{x}");
    // The same patterns on a bignum: flagged.
    let mut n = THREE;
    n = -n;
    println!("{n}");
    let mut m = SEVEN;
    m = !m;
    println!("{m}");

    // The in-place variants: fine.
    let mut c = false;
    c.not_assign();
    println!("{c}");
    let mut y = 3i8;
    y.neg_assign();
    println!("{y}");
    // Assigning the negation of a different value: fine.
    let w = 9i32;
    let mut z = 4i32;
    println!("{z}");
    z = -w;
    println!("{z}");
    // Negating a float, which has NegAssign: flagged.
    let mut f = 2.5f64;
    f = -f;
    println!("{f}");
}
