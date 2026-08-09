// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves, SplitInHalf, WrappingFrom};

fn manual_join(hi: u32, lo: u32) -> u64 {
    // Assembling the halves with a shift and an or: flagged.
    (u64::from(hi) << u32::WIDTH) | u64::from(lo)
}

fn manual_join_add(hi: u32, lo: u32) -> u64 {
    // The addition form: flagged.
    (u64::from(hi) << 32) + u64::from(lo)
}

fn manual_join_reversed(hi: u32, lo: u32) -> u64 {
    // The low half written first: flagged.
    u64::from(lo) | (u64::from(hi) << 32)
}

fn manual_upper(x: u64) -> u32 {
    // Shifting and converting down: flagged.
    u32::wrapping_from(x >> 32)
}

fn manual_upper_exact(x: u64) -> u32 {
    // The exact_from form: flagged.
    u32::exact_from(x >> u32::WIDTH)
}

fn fine(hi: u32, lo: u32, x: u64) -> (u64, u32, u32, u64) {
    // The named functions: fine.
    let joined = u64::join_halves(hi, lo);
    let upper = x.upper_half();
    // A shift that is not half the width: fine.
    let small_shift = u32::wrapping_from(x >> 8);
    // A shifted operand that is not a half-width conversion: fine.
    let not_a_half = (x << 32) | u64::from(lo);
    (joined, upper, small_shift, not_a_half)
}

fn main() {
    println!("{}", manual_join(3, 4));
    println!("{}", manual_join_add(3, 4));
    println!("{}", manual_join_reversed(3, 4));
    println!("{}", manual_upper(1 + (5 << 33)));
    println!("{}", manual_upper_exact(1 + (5 << 33)));
    println!("{:?}", fine(3, 4, 1 + (5 << 33)));
}
