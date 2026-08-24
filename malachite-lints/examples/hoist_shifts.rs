// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

fn main() {
    let n = const { Natural::const_from(100) };
    let m = const { Natural::const_from(3) };
    let i = const { Integer::const_from_signed(-100) };
    let j = const { Integer::const_from_signed(3) };
    let q = const { Rational::const_from_unsigneds(1, 3) };
    let r = const { Rational::const_from_unsigneds(2, 5) };
    // A left-shifted operand of a Natural multiplication, on either side: flagged.
    let _ = (&n << 5u64) * &m;
    let _ = &n * (&m << 5u64);
    // The same for Integer, whose left shift is also an exact multiplication by a power of 2:
    // flagged.
    let _ = (&i << 5u64) * &j;
    // Natural and Integer right shifts are floor divisions, and their `/` truncates, so neither
    // commutes: fine.
    let _ = (&n >> 5u64) * &m;
    let _ = (&n << 5u64) / &m;
    let _ = (&i >> 5u64) * &j;
    // For Rational, both shift directions commute with both operations, in either operand:
    // flagged.
    let _ = (&q << 5u64) * &r;
    let _ = (&q >> 5u64) * &r;
    let _ = (&q << 5u64) / &r;
    let _ = (&q >> 5u64) / &r;
    let _ = &q / (&r << 5u64);
    let _ = &q / (&r >> 5u64);
    // Float shifts saturate at the exponent-range boundaries, so hoisting is not
    // value-preserving: fine.
    let f = const { Float::const_from_unsigned(3) };
    let _ = (&f << 5u64) * Float::const_from_unsigned(5);
    // Primitive integers overflow at different points under the two spellings: fine.
    let p = 100u64;
    let _ = (p << 5) * p;
    // The shifted result itself: fine.
    let _ = (&n * &m) << 5u64;
}
