// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::is_in_reduced_row_echelon_form_with_rank::*;
use malachite_nz::test_util::generators::integer_matrix_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_integer_matrix_is_in_reduced_row_echelon_form_with_rank
    );
}

fn print_check(a: &IntegerMatrix, den: &Integer, rank: u64) {
    println!(
        "is_in_reduced_row_echelon_form_with_rank({a}, {den}, {rank}) = {}",
        is_in_reduced_row_echelon_form_with_rank(a, den, usize::try_from(rank).unwrap())
    );
}

// Prints each generated matrix with the denominator of its first nonzero element (or 1) and the
// number of its nonzero rows, and also a copy forced toward a fraction-free reduced row echelon
// form: in each nonzero row's first nonzero column, the element is set to that denominator and the
// elements of the other nonzero rows are set to 0. The copy is in the form when the leading columns
// of its nonzero rows increase.
fn demo_integer_matrix_is_in_reduced_row_echelon_form_with_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let den = a
            .elements
            .iter()
            .find(|x| **x != 0u32)
            .cloned()
            .unwrap_or(Integer::ONE);
        let rank = (0..a.height()).filter(|&i| !a.is_zero_row(i)).count();
        print_check(&a, &den, u64::try_from(rank).unwrap());
        let mut b = a.clone();
        let w = usize::try_from(a.width()).unwrap();
        let nonzero_rows: Vec<usize> = (0..usize::try_from(a.height()).unwrap())
            .filter(|&i| b.elements[i * w..(i + 1) * w].iter().any(|x| *x != 0u32))
            .collect();
        // Move the nonzero rows to the top.
        let mut elements = Vec::with_capacity(b.elements.len());
        for &i in &nonzero_rows {
            elements.extend_from_slice(&b.elements[i * w..(i + 1) * w]);
        }
        elements.resize(b.elements.len(), Integer::ZERO);
        b = IntegerMatrix::from_owned_elements(a.height(), a.width(), elements);
        let leading_columns: Vec<usize> = (0..nonzero_rows.len())
            .map(|i| {
                b.elements[i * w..(i + 1) * w]
                    .iter()
                    .position(|x| *x != 0u32)
                    .unwrap()
            })
            .collect();
        for (i, &j) in leading_columns.iter().enumerate() {
            for k in 0..nonzero_rows.len() {
                b.elements[k * w + j] = if k == i { den.clone() } else { Integer::ZERO };
            }
        }
        print_check(&b, &den, u64::try_from(nonzero_rows.len()).unwrap());
    }
}
