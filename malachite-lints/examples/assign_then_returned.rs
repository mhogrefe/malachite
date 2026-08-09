// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    FloorSqrt, FloorSqrtAssign, NextPowerOf2Assign, Parity, WrappingAddAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;

const K: Natural = Natural::const_from(100);

fn balanced(m: &Natural) -> Natural {
    let mut b = m >> 1u32;
    if m.even() {
        b -= Natural::ONE;
    }
    // An in-place call followed by the receiver as the tail expression: flagged.
    b.floor_sqrt_assign();
    b
}

fn bump(mut n: Natural) -> Natural {
    // The same shape on a parameter: flagged.
    n.next_power_of_2_assign();
    n
}

fn fine(m: &Natural) -> Natural {
    let mut b = m >> 1u32;
    if m.even() {
        b -= Natural::ONE;
    }
    // The by-value form: fine.
    b.floor_sqrt()
}

fn copy_type(mut c: u64) -> u64 {
    // A Copy receiver may still be read after the block's value is copied out: fine.
    c.wrapping_add_assign(1);
    c
}

struct Wrapper(Natural);

impl Wrapper {
    fn double_assign(&mut self) {
        self.0 <<= 1u32;
    }

    // The house delegation idiom: the by-value variant is implemented by the in-place one. The
    // enclosing function is the method's own family: fine.
    fn double(mut self) -> Wrapper {
        self.double_assign();
        self
    }
}

fn main() {
    println!("{}", balanced(&K));
    println!("{}", bump(K));
    println!("{}", fine(&K));
    println!("{}", copy_type(5));
    println!("{}", Wrapper(K).double().0);
}
