// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use alloc::collections::BTreeSet;
use alloc::vec;
use alloc::vec::Vec;

// The rows of the Howell form of `a` modulo `m`, indexed by the columns of their pivots, computed
// from the definition: the row span `S` is enumerated, the pivot in column `j` generates the ideal
// formed by the elements in column `j` of the vectors in `S` that are zero in every earlier column,
// and the row with that pivot is the unique such vector whose element in each later pivot column is
// less than that column's pivot. The span has up to `m^w` elements, where `w` is the width, so this
// is only practical for small matrices and moduli.
pub(crate) fn howell_rows_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> Vec<Option<Vec<T>>> {
    let w = usize::exact_from(a.width());
    let mut span = BTreeSet::new();
    span.insert(vec![T::ZERO; w]);
    for i in 0..a.height() {
        let row: Vec<T> = a.row_iter(i).copied().collect();
        let mut next = BTreeSet::new();
        for v in &span {
            let mut c = T::ZERO;
            loop {
                next.insert(
                    v.iter()
                        .zip(&row)
                        .map(|(&x, &y)| x.mod_add(c.mod_mul(y, m), m))
                        .collect::<Vec<T>>(),
                );
                c += T::ONE;
                if c == m {
                    break;
                }
            }
        }
        span = next;
    }
    let mut pivots: Vec<Option<T>> = vec![None; w];
    for (j, pivot) in pivots.iter_mut().enumerate() {
        let mut d = m;
        for v in &span {
            if v[..j].iter().all(|&x| x == T::ZERO) {
                d.gcd_assign(v[j]);
            }
        }
        if d != m {
            *pivot = Some(d);
        }
    }
    (0..w)
        .map(|j| {
            let d = pivots[j]?;
            let mut candidates = span.iter().filter(|v| {
                v[..j].iter().all(|&x| x == T::ZERO)
                    && v[j] == d
                    && (j + 1..w).all(|k| pivots[k].is_none_or(|e| v[k] < e))
            });
            let row = candidates.next().unwrap().clone();
            assert!(candidates.next().is_none());
            Some(row)
        })
        .collect()
}

/// The strong echelon form of `a` modulo `m`, computed from the definition by enumerating the row
/// span. It is only practical for small matrices and moduli.
pub fn unsigned_matrix_mod_strong_echelon_form_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let w = a.width();
    let h = a.height().max(w);
    let mut elements = Vec::new();
    for row in howell_rows_naive(a, m) {
        elements.extend(row.unwrap_or_else(|| vec![T::ZERO; usize::exact_from(w)]));
    }
    elements.resize(usize::exact_from(h * w), T::ZERO);
    UnsignedMatrix::from_owned_elements(h, w, elements)
}

/// A matrix with the same row span as `a` modulo `m`: the rows in reverse order, then each row plus
/// a multiple of the row above it.
pub fn unsigned_matrix_mod_row_equivalent<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    let w = a.width();
    let mut rows: Vec<Vec<T>> = (0..a.height())
        .rev()
        .map(|i| a.row_iter(i).copied().collect())
        .collect();
    for i in 1..rows.len() {
        let c = T::wrapping_from(i + 1) % m;
        let above = rows[i - 1].clone();
        for (x, y) in rows[i].iter_mut().zip(above) {
            *x = x.mod_add(c.mod_mul(y, m), m);
        }
    }
    let elements = if w == 0 { Vec::new() } else { rows.concat() };
    UnsignedMatrix::from_owned_elements(a.height(), a.width(), elements)
}
