// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, WrappingAddAssign};
use malachite_base::num::basic::traits::{One, Two};
use malachite_float::Float;
use malachite_nz::integer::Integer;

fn rebind_then_abs(t: Integer) -> Integer {
    // Rebinding as mutable only to call the in-place variant: flagged.
    let mut t = t;
    t.abs_assign();
    t += Integer::ONE;
    t
}

fn rebind_then_atan(u: Float) -> Float {
    // An in-place method that returns an `Ordering`: flagged, suggesting `.0`.
    let mut u = u;
    u.atan_prec_assign(10);
    u <<= 1u32;
    u
}

fn rebind_primitive(n: u64) -> u64 {
    // A primitive whose by-value variant is a trait method: flagged.
    let mut n = n;
    n.wrapping_add_assign(3);
    n * n
}

fn by_value(t: Integer) -> Integer {
    // The suggested form: fine, even though it shadows `t` with a method on itself.
    let mut t = t.abs();
    t += Integer::ONE;
    t
}

fn mutable_from_the_start(t: Integer) -> Integer {
    // In place on a binding that was mutable to begin with: fine.
    let mut t = t;
    t += Integer::TWO;
    t.abs_assign();
    t *= Integer::TWO;
    t
}

fn rebind_with_other_initializer(x: &Integer) -> Integer {
    // The rebind is of a computed value, not a bare local: not this lint's shape.
    let mut t = x.clone();
    t.abs_assign();
    t += Integer::ONE;
    t
}

const MINUS_FIVE: Integer = Integer::const_from_signed(-5);
const THREE: Float = Float::const_from_unsigned(3);

fn main() {
    let x = MINUS_FIVE;
    println!("{}", rebind_then_abs(x.clone()));
    println!("{}", rebind_then_atan(THREE));
    println!("{}", rebind_primitive(4));
    println!("{}", by_value(x.clone()));
    println!("{}", mutable_from_the_start(x.clone()));
    println!("{}", rebind_with_other_initializer(&x));
}
