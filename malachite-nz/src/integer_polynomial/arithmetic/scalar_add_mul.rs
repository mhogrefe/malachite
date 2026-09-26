// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::add::add_assign_ref;
use crate::integer_polynomial::arithmetic::sub::sub_assign_ref;
use alloc::vec::Vec;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::AddMulAssign;

// Adds `ys` multiplied by `c` into `xs`. Where `ys` is longer than `xs`, `xs` is extended by the
// products of the remaining elements of `ys` and `c`, so that the elements of `xs` and `ys` at the
// same index are always added: this is the operation $p \gets p + cq$ on coefficients.
//
// Nothing is trimmed, since leading coefficients can cancel, and a caller holding the coefficients
// of a polynomial must trim them. When `c` is zero, `xs` is left alone and not extended.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements
// of `xs` and `ys` and of `c`.
//
// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar_addmul_fmpz.c`, FLINT
// 3.6.0, extended to vectors of different lengths.
#[doc(hidden)]
pub fn integers_add_mul_scalar_assign(xs: &mut Vec<Integer>, ys: &[Integer], c: &Integer) {
    if *c == 1u32 {
        add_assign_ref(xs, ys);
    } else if *c == -1i32 {
        sub_assign_ref(xs, ys);
    } else if *c != 0u32 {
        let common = min(xs.len(), ys.len());
        for (x, y) in xs.iter_mut().zip(&ys[..common]) {
            x.add_mul_assign(y, c);
        }
        xs.extend(ys[common..].iter().map(|y| y * c));
    }
}
