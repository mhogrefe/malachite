// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014, 2015 Alex J. Best
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::determinant::balanced_crt_word;
use crate::integer_matrix::arithmetic::hermite_normal_form::{
    combine_rows, extended_gcd_reduced_cofactor, hnf_classical, hnf_minors, hnf_modular,
    neg_row, reduce_row,
};
use crate::integer_matrix::arithmetic::mul::multi_mod::{OPTIMAL_MODULUS_MIN, next_prime};
use crate::integer_matrix::arithmetic::solve::solve_nonsingular;
use crate::integer_matrix::arithmetic::word_lu::WordLu;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::swap_rows;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, CeilingSqrt, Determinant, DivExact, DivRound, Gcd, LcmAssign, ModInverse,
    ModMul, Nullspace, Square,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;

// The largest dimension of the nonsingular matrix C for which `extend_columns` solves with C
// directly; for larger ones, the solve with Bᵤ, whose last row is small, is faster.
const DIRECT_EXTENSION_MAX_DIMENSION: usize = 40;

// The number of bits above which an fmpz is stored as a GMP integer, so that a determinant of more
// bits makes the recursion worthwhile.
const SMALL_FMPZ_BITS: u64 = 62;

// Given the (r - 1) by r matrix `b` whose first r - 2 rows, together with a last row, form a
// nonsingular r - 1 by r - 1 matrix `h1`'s preimage, extends the Hermite normal form `h1` of the
// first r - 1 columns of `b` to that of `b`. A nonzero vector k in the nullspace of all rows of `b`
// but the last, restricted to its first r - 1 columns, is found; the last row of a matrix Bᵤ is
// the unit vector e_j for the first j with k_j ≠ 0, which makes Bᵤ nonsingular; Bᵤx = c is
// solved for the remaining columns c of `b`; x is corrected by multiples of k so that the last row
// of `b` is satisfied too; and the remaining columns of the result are `h1`x.
//
// This is equivalent to `add_columns` from `fmpz_mat/hnf_pernet_stein.c`, FLINT 3.6.0, except that
// the last row of Bᵤ is e_j rather than random, and that the rational arithmetic is done with
// integers over a common denominator.
fn add_columns(b: &IntegerMatrix, h1: &IntegerMatrix) -> IntegerMatrix {
    let n = b.height;
    let width = b.width;
    let extra = width - n;
    let mut cols = Vec::with_capacity(n * extra);
    for row in b.elements.chunks_exact(width) {
        cols.extend_from_slice(&row[n..]);
    }
    let cols = IntegerMatrix {
        elements: cols,
        height: n,
        width: extra,
    };
    let mut bu = Vec::with_capacity(n * n);
    for row in b.elements.chunks_exact(width).take(n - 1) {
        bu.extend_from_slice(&row[..n]);
    }
    // Find a basis vector of the nullspace.
    let k = IntegerMatrix {
        elements: bu.clone(),
        height: n - 1,
        width: n,
    }
    .nullspace();
    assert_eq!(k.width, 1, "the nullspace was not of dimension one");
    let k = k.elements;
    // Make the last row of Bᵤ a unit vector that makes Bᵤ nonsingular.
    let j0 = k.iter().position(|x| *x != 0u32).unwrap();
    bu.extend((0..n).map(|j| if j == j0 { Integer::ONE } else { Integer::ZERO }));
    let bu = IntegerMatrix {
        elements: bu,
        height: n,
        width: n,
    };
    // Solve Bᵤx = cols, as BᵤX = e·cols.
    let (x, e) = solve_nonsingular(&bu, &cols).unwrap();
    let last = &b.elements[(n - 1) * width..n * width];
    let mut den = Integer::ZERO;
    for (b, k) in last.iter().zip(&k) {
        den.add_mul_assign(b, k);
    }
    // The columns of the corrected x are those of y, divided by e·den.
    let mut y = x;
    for j in 0..extra {
        let mut last_dot = Integer::ZERO;
        for (b, x) in last.iter().zip(y.elements[j..].iter().step_by(extra)) {
            last_dot.add_mul_assign(b, x);
        }
        let alpha = (&last[n + j] * &e) - last_dot;
        for (x, k) in y.elements[j..].iter_mut().step_by(extra).zip(&k) {
            *x *= &den;
            x.add_mul_assign(&alpha, k);
        }
    }
    join_columns(h1, &(h1 * y).div_exact(e * den))
}

// Given the (r - 1) by r matrix `b` whose first r - 1 columns form a nonsingular matrix C, extends
// the Hermite normal form `h1` of C to that of `b`: CX = d·c is solved for the remaining columns c
// of `b`, and the remaining columns of the result are `h1`X/d.
fn add_columns_direct(b: &IntegerMatrix, h1: &IntegerMatrix) -> IntegerMatrix {
    let n = b.height;
    let width = b.width;
    let extra = width - n;
    let mut c = Vec::with_capacity(n * n);
    let mut cols = Vec::with_capacity(n * extra);
    for row in b.elements.chunks_exact(width) {
        c.extend_from_slice(&row[..n]);
        cols.extend_from_slice(&row[n..]);
    }
    let c = IntegerMatrix {
        elements: c,
        height: n,
        width: n,
    };
    let cols = IntegerMatrix {
        elements: cols,
        height: n,
        width: extra,
    };
    let (x, d) = solve_nonsingular(&c, &cols).unwrap();
    join_columns(h1, &(h1 * x).div_exact(d))
}

// The matrix whose rows are those of `h1` followed by those of `cols`; both have the same height.
fn join_columns(h1: &IntegerMatrix, cols: &IntegerMatrix) -> IntegerMatrix {
    let n = h1.height;
    let extra = cols.width;
    let width = h1.width + extra;
    let mut h = Vec::with_capacity(n * width);
    for (h1_row, cols_row) in h1
        .elements
        .chunks_exact(h1.width)
        .zip(cols.elements.chunks_exact(extra))
    {
        h.extend_from_slice(h1_row);
        h.extend_from_slice(cols_row);
    }
    IntegerMatrix {
        elements: h,
        height: n,
        width,
    }
}

// The index of the first nonzero element of `xs`, or the length of `xs` if there is none.
fn first_nonzero(xs: &[Integer]) -> usize {
    xs.iter().position(|x| *x != 0u32).unwrap_or(xs.len())
}

// Takes `h`, whose rows before `start_row` are in Hermite normal form with the pivot columns
// `pivots[..num_pivots]`, to Hermite normal form, adding the remaining rows one at a time: each is
// reduced by the existing rows with extended GCDs, moved to its place, and the rows above the
// pivots are reduced.
//
// This is equivalent to `add_rows` from `fmpz_mat/hnf_pernet_stein.c`, FLINT 3.6.0.
fn add_rows(h: &mut IntegerMatrix, start_row: usize, pivots: &mut [usize], mut num_pivots: usize) {
    let (m, n) = (h.height, h.width);
    let xs = &mut h.elements;
    for row in start_row..m {
        // Reduce the row being added with the existing rows.
        let mut j = 0;
        for (i, &pivot) in pivots[..num_pivots].iter().enumerate() {
            // Check whether the added row can still be reduced: it cannot when it has a nonzero
            // element before the next pivot, which happens when p loses a pivot.
            while j < pivot && xs[row * n + j] == 0u32 {
                j += 1;
            }
            if j < pivot {
                break;
            }
            if xs[row * n + j] == 0u32 {
                continue;
            }
            combine_rows(xs, n, i, row, j);
        }
        // Move the added row, if it is nonzero, to its place.
        let j = first_nonzero(&xs[row * n..(row + 1) * n]);
        let mut new_row = row;
        if j != n {
            if xs[row * n + j] < 0u32 {
                neg_row(&mut xs[row * n + j..(row + 1) * n]);
            }
            loop {
                if new_row < row {
                    swap_rows(xs, n, new_row, new_row + 1);
                }
                if new_row == 0 {
                    break;
                }
                new_row -= 1;
                if first_nonzero(&xs[new_row * n..(new_row + 1) * n]) <= j {
                    break;
                }
            }
        }
        // Recompute the pivots.
        let mut i = new_row;
        let mut j = 0;
        while i <= row && i < n {
            while j < n && xs[i * n + j] == 0u32 {
                j += 1;
            }
            if j == n {
                break;
            }
            pivots[i] = j;
            num_pivots = i + 1;
            i += 1;
            j += 1;
        }
        // Reduce the elements above the pivots.
        for (i, &p) in pivots[..num_pivots].iter().enumerate() {
            for i2 in 0..i {
                reduce_row(xs, n, i2, i, p);
            }
        }
    }
}

// The sum of the squares of `xs`.
fn sum_of_squares<'a>(xs: impl Iterator<Item = &'a Integer>) -> Natural {
    xs.map(|x| x.unsigned_abs_ref().square()).sum()
}

// Given an n - 1 by n matrix `b` and two vectors `c` and `d` of length n, returns the determinants
// of the n by n matrices formed by appending `c`, and `d`, to the rows of `b`. When the system
// whose matrix is the transpose of the first and whose right-hand side is d has a solution x with a
// nonzero last element, the denominators of x and of x/x_{n - 1} divide the two determinants, and
// the quotients are found by the Chinese remainder theorem; otherwise both determinants are
// computed directly.
//
// This is equivalent to `double_det` from `fmpz_mat/hnf_pernet_stein.c`, FLINT 3.6.0, except that
// primes dividing either denominator are skipped (FLINT skips only those dividing both, and then
// fails to invert the other modulo the prime).
fn double_det(b: &IntegerMatrix, c: &[Integer], d: &[Integer]) -> (Integer, Integer) {
    let n = b.width;
    let mut bt = vec![Integer::ZERO; n * n];
    for i in 0..n {
        for j in 0..n - 1 {
            bt[i * n + j] = b.elements[j * n + i].clone();
        }
        bt[i * n + n - 1] = c[i].clone();
    }
    let bt = IntegerMatrix {
        elements: bt,
        height: n,
        width: n,
    };
    let mut bt_d = bt.clone();
    for (row, x) in bt_d.elements.chunks_exact_mut(n).zip(d) {
        row[n - 1] = x.clone();
    }
    let dt = IntegerMatrix {
        elements: d.to_vec(),
        height: n,
        width: 1,
    };
    // Solve Bᵀx = dᵀ, as BᵀX = e·dᵀ.
    let solution = solve_nonsingular(&bt, &dt);
    let Some((x, e)) = solution.filter(|(x, _)| x.elements[n - 1] != 0u32) else {
        // The clever method can't be used, so compute both determinants directly.
        return (bt.determinant(), bt_d.determinant());
    };
    let x = &x.elements;
    let e = e.unsigned_abs_ref();
    let last = x[n - 1].unsigned_abs_ref();
    // The least common multiples of the denominators of x, and of x/x_{n - 1} and 1/x_{n - 1}.
    let mut u1 = Natural::ONE;
    let mut u2 = Natural::ONE;
    for xi in &x[..n - 1] {
        let xi = xi.unsigned_abs_ref();
        u1.lcm_assign(e.div_exact(e.gcd(xi)));
        u2.lcm_assign(last.div_exact(last.gcd(xi)));
    }
    let g = e.gcd(last);
    u1.lcm_assign(e.div_exact(&g));
    u2.lcm_assign(last.div_exact(&g));
    // Hadamard bounds for the quotients of the determinants by u1 and u2.
    let mut bound = Natural::ONE;
    for row in b.elements.chunks_exact(n) {
        bound *= sum_of_squares(row.iter()).ceiling_sqrt();
    }
    let s1 = (sum_of_squares(c.iter()).ceiling_sqrt() * &bound)
        .div_round(&u1, Ceiling)
        .0;
    let s2 = (sum_of_squares(d.iter()).ceiling_sqrt() * &bound)
        .div_round(&u2, Ceiling)
        .0;
    let bound = s1.max(s2) << 1u32;
    let mut v1 = Integer::ZERO;
    let mut v2 = Integer::ZERO;
    let mut product = Natural::ONE;
    let mut p = OPTIMAL_MODULUS_MIN;
    while product <= bound {
        p = next_prime(p);
        let u1_mod = u1.rem_limb_ref(p);
        let u2_mod = u2.rem_limb_ref(p);
        if u1_mod == 0 || u2_mod == 0 {
            continue;
        }
        let v1_mod = WordLu::new(&bt, p)
            .determinant()
            .mod_mul(u1_mod.mod_inverse(p).unwrap(), p);
        let v2_mod = WordLu::new(&bt_d, p)
            .determinant()
            .mod_mul(u2_mod.mod_inverse(p).unwrap(), p);
        v1 = balanced_crt_word(&v1, &product, v1_mod, p);
        v2 = balanced_crt_word(&v2, &product, v2_mod, p);
        product *= Natural::from(p);
    }
    (Integer::from(u1) * v1, Integer::from(u2) * v2)
}

// The Hermite normal form of `a`, by the algorithm of Pernet and Stein. A prime p is chosen, and
// the LU decomposition of `a` modulo p gives its rank r, a row permutation, and the pivot columns.
// The first r - 2 permuted rows, restricted to the first r - 1 pivot columns, are completed by each
// of the next two rows to give square matrices with determinants d1 and d2; with g = gcd(d1, d2) =
// s·d1 + t·d2, the row combining the two with s and t completes a square matrix C with
// determinant g, whose Hermite normal form is found modulo g, or recursively when g is large. It is
// extended to all columns, and the remaining rows are added in. If the pivots turn out not to be
// those modulo p, the next prime is tried. Matrices of full column rank and small width are passed
// to the algorithm of Kannan and Bachem, and those of rank at most 2 modulo p to the classical
// algorithm.
//
// When C has dimension at most `direct_max`, the columns are extended by solving with C directly.
//
// This is equivalent to `fmpz_mat_hnf_pernet_stein` from `fmpz_mat/hnf_pernet_stein.c`, FLINT
// 3.6.0, except that the primes are taken in increasing order, starting just above $2^{W-5}$, where
// $W$ is the limb width, rather than at random, and that the columns may be extended directly.
fn hnf_pernet_stein_with_direct_extension_max(
    a: &IntegerMatrix,
    direct_max: usize,
) -> IntegerMatrix {
    let (m, n) = (a.height, a.width);
    if m == 0 || n == 0 {
        return a.clone();
    }
    let mut p = OPTIMAL_MODULUS_MIN;
    loop {
        p = next_prime(p);
        let lu = WordLu::new(a, p);
        let r = lu.rank;
        let perm = lu.perm;
        let mut pivots = lu.pivots_nonpivots;
        if r == 0 {
            // The rank is zero modulo p, so the matrix may be zero.
            if a.is_zero() {
                return a.clone();
            }
            continue;
        }
        // If the matrix has full column rank, the minors-based algorithm may be preferable.
        if r == n && n < 52 {
            let cutoff = match vec_max_bits(&a.elements).0 {
                0..=8 => 35,
                9..=32 => 44,
                33..=256 => 48,
                _ => 52,
            };
            if n < cutoff {
                return hnf_minors(a);
            }
        }
        // With rank at most 2 there is no decomposition. Such a matrix has less than full column
        // rank, since otherwise its width would be below the cutoff above.
        if r <= 2 {
            return hnf_classical(a);
        }
        let e = |i: usize, j: usize| &a.elements[perm[i] * n + pivots[j]];
        let mut bs = Vec::with_capacity((r - 2) * (r - 1));
        for i in 0..r - 2 {
            bs.extend((0..r - 1).map(|j| e(i, j).clone()));
        }
        let b = IntegerMatrix {
            elements: bs,
            height: r - 2,
            width: r - 1,
        };
        let c: Vec<Integer> = (0..r - 1).map(|j| e(r - 2, j).clone()).collect();
        let d: Vec<Integer> = (0..r - 1).map(|j| e(r - 1, j).clone()).collect();
        let (d1, d2) = double_det(&b, &c, &d);
        // d1 is nonzero modulo p, since it is a leading principal minor of the permuted matrix
        // restricted to its pivot columns, so g is nonzero.
        let (g, s, t) = extended_gcd_reduced_cofactor(&d1, &d2);
        // The rows of B: the first r - 2 permuted rows, and their combination with s and t.
        let combined_row = |j: usize| (&s * e(r - 2, j)).add_mul(&t, e(r - 1, j));
        let mut bs = Vec::with_capacity((r - 1) * n);
        for i in 0..r - 2 {
            bs.extend((0..n).map(|j| e(i, j).clone()));
        }
        bs.extend((0..n).map(combined_row));
        // C is the first r - 1 columns of B, which is nonsingular with determinant g.
        let mut cs = Vec::with_capacity((r - 1) * (r - 1));
        for row in bs.chunks_exact(n) {
            cs.extend_from_slice(&row[..r - 1]);
        }
        let c = IntegerMatrix {
            elements: cs,
            height: r - 1,
            width: r - 1,
        };
        let h1 = if g.significant_bits() > SMALL_FMPZ_BITS && c.height > 3 {
            // g is large, so recurse.
            hnf_pernet_stein_with_direct_extension_max(&c, direct_max)
        } else {
            // Compute the Hermite normal form of C modulo its determinant.
            hnf_modular(&c, g.unsigned_abs_ref())
        };
        let b = IntegerMatrix {
            elements: bs,
            height: r - 1,
            width: n,
        };
        let h2 = if r - 1 <= direct_max {
            add_columns_direct(&b, &h1)
        } else {
            add_columns(&b, &h1)
        };
        let mut h3 = vec![Integer::ZERO; (m + 1) * n];
        for (h3_row, h2_row) in h3.chunks_exact_mut(n).zip(h2.elements.chunks_exact(n)) {
            for (&col, x) in pivots.iter().zip(h2_row) {
                h3_row[col] = x.clone();
            }
        }
        for i in 1..=m - r + 2 {
            let source = perm[m - i];
            h3[(m + 1 - i) * n..(m + 2 - i) * n]
                .clone_from_slice(&a.elements[source * n..(source + 1) * n]);
        }
        // Check that the pivots of H3 are as expected; otherwise the prime was unlucky.
        if (0..r - 1).all(|i| first_nonzero(&h3[i * n..(i + 1) * n]) == pivots[i]) {
            let mut h3 = IntegerMatrix {
                elements: h3,
                height: m + 1,
                width: n,
            };
            // Add the remaining rows.
            add_rows(&mut h3, r - 1, &mut pivots, r - 1);
            h3.elements.truncate(m * n);
            h3.height = m;
            return h3;
        }
    }
}

// The Hermite normal form of `a`, by the algorithm of Pernet and Stein.
crate_test_fn! {
hnf_pernet_stein(a: &IntegerMatrix) -> IntegerMatrix {
    hnf_pernet_stein_with_direct_extension_max(a, DIRECT_EXTENSION_MAX_DIMENSION)
}}

// The Hermite normal form of `a`, by the algorithm of Pernet and Stein, always extending the
// columns with the solve with Bᵤ, as FLINT does.
crate_test_fn! {
hnf_pernet_stein_without_direct_extension(a: &IntegerMatrix) -> IntegerMatrix {
    hnf_pernet_stein_with_direct_extension_max(a, 0)
}}
