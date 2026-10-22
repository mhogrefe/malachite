// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2012 Fredrik Johansson
//
//      Copyright © 2014 Alex J. Best
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;

// Whether `a` is in fraction-free reduced row echelon form with denominator `den` and rank `rank`:
// its rows from `rank` on are zero, the first nonzero element of each of the first `rank` rows is
// strictly to the right of that of the row above, and in each such element's column, that element
// is `den` and the elements of the other first `rank` rows are zero.
//
// This is equivalent to `fmpz_mat_is_in_rref_with_rank` from `fmpz_mat/is_in_rref_with_rank.c`,
// FLINT 3.6.0.
crate_test_fn! {
is_in_reduced_row_echelon_form_with_rank(a: &IntegerMatrix, den: &Integer, rank: usize) -> bool {
    let width = a.width;
    if width == 0 {
        return true;
    }
    let rows: alloc::vec::Vec<&[Integer]> = a.elements.chunks_exact(width).collect();
    // The rows from `rank` on must be zero.
    if rows.iter().skip(rank).any(|row| row.iter().any(|x| *x != 0u32)) {
        return false;
    }
    let mut previous_pivot = None;
    for (i, row) in rows.iter().enumerate().take(rank) {
        if let Some(j) = row.iter().position(|x| *x != 0u32) {
            // The pivot must be to the right of the previous one.
            if previous_pivot.is_some_and(|p| j <= p) {
                return false;
            }
            // The column must be 0 ... 0 den 0 ... 0 in the first `rank` rows.
            for (k, other) in rows.iter().enumerate().take(rank) {
                if if i == k { other[j] != *den } else { other[j] != 0u32 } {
                    return false;
                }
            }
            previous_pivot = Some(j);
        }
    }
    true
}}
