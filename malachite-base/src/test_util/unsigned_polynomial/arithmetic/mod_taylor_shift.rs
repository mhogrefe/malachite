// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;
use alloc::vec::Vec;

// Expands $\sum_i a_i (x + c)^i$ directly, keeping the coefficients of $(x + c)^i$ modulo `m` as a
// rolling row of Pascal's triangle scaled by powers of $c$. Nothing is shared with the
// implementation.
pub fn mod_taylor_shift_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    c: T,
    m: T,
) -> UnsignedPolynomial<T> {
    let xs = p.coefficients_asc();
    let n = xs.len();
    let mut out = vec![T::ZERO; n];
    // The coefficients of (x + c)^i modulo m.
    let mut row: Vec<T> = Vec::with_capacity(n);
    for (i, &a) in xs.iter().enumerate() {
        if i == 0 {
            row.push(T::ONE % m);
        } else {
            // (x + c)^i = x (x + c)^(i - 1) + c (x + c)^(i - 1)
            row.push(T::ZERO);
            for j in (0..i).rev() {
                let shifted = row[j];
                row[j + 1].mod_add_assign(shifted, m);
                row[j].mod_mul_assign(c, m);
            }
        }
        for (o, &r) in out.iter_mut().zip(row.iter()) {
            o.mod_add_assign(a.mod_mul(r, m), m);
        }
    }
    UnsignedPolynomial::from_coefficients_asc(out)
}
