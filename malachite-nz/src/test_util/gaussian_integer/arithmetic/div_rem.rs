// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::DivMod;

// Rounds t / n to the nearest integer, rounding ties up: the floor, plus one when the remainder is
// at least half the divisor.
fn round_half_up(t: Integer, n: &Integer) -> Integer {
    let (q, r) = t.div_mod(n);
    if r << 1u32 >= *n {
        q + Integer::from(1u32)
    } else {
        q
    }
}

// Schoolbook: the exact quotient is ((ac + bd) + (bc - ad)i) / (c^2 + d^2); round each part to the
// nearest integer, ties up, and take the remainder.
pub fn gaussian_integer_div_rem_naive(
    x: &GaussianInteger,
    y: &GaussianInteger,
) -> (GaussianInteger, GaussianInteger) {
    assert!(y.real != 0u32 || y.imaginary != 0u32, "division by zero");
    let norm: Integer = &y.real * &y.real + &y.imaginary * &y.imaginary;
    let q = GaussianInteger {
        real: round_half_up(&x.real * &y.real + &x.imaginary * &y.imaginary, &norm),
        imaginary: round_half_up(&x.imaginary * &y.real - &x.real * &y.imaginary, &norm),
    };
    let r = GaussianInteger {
        real: &x.real - (&q.real * &y.real - &q.imaginary * &y.imaginary),
        imaginary: &x.imaginary - (&q.real * &y.imaginary + &q.imaginary * &y.real),
    };
    (q, r)
}
