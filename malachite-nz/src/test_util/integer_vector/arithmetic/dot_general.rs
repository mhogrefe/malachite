// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2024 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{AddMulAssign, SubMulAssign};
use malachite_base::num::basic::traits::Zero;

// Accumulates the products one at a time, starting from `initial`, or from the first product.
//
// This is equivalent to `_fmpz_vec_dot_general_naive` from `fmpz_vec/dot.c`, FLINT 3.6.0.
pub fn vec_dot_general_naive(
    initial: Option<&Integer>,
    subtract: bool,
    xs: &[Integer],
    ys: &[Integer],
    reverse: bool,
) -> Integer {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    let y = |i: usize| if reverse { &ys[len - 1 - i] } else { &ys[i] };
    let (mut result, start) = if let Some(initial) = initial {
        (initial.clone(), 0)
    } else {
        if len == 0 {
            return Integer::ZERO;
        }
        let first = &xs[0] * y(0);
        (if subtract { -first } else { first }, 1)
    };
    for (i, x) in xs.iter().enumerate().skip(start) {
        if subtract {
            result.sub_mul_assign(x, y(i));
        } else {
            result.add_mul_assign(x, y(i));
        }
    }
    result
}
