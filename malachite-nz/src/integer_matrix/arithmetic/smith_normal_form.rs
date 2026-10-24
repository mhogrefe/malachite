// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 Alex J. Best
//
//      Copyright © 2026 Edgar Costa
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::hermite_normal_form::{
    RowCombination, extended_gcd_reduced_cofactor, neg_row, two_elements, two_rows,
};
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::Natural;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::swap_rows;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    AbsAssign, AddMulAssign, DivExact, DivisibleBy, ExtendedGcd, Gcd, HermiteNormalForm,
    HermiteNormalFormAndTransform, HermiteNormalFormAssign, ModAssign, MulAddMul, NegAssign,
    SaturatingAddAssign, SaturatingMulAssign, SmithNormalForm, SmithNormalFormAssign, SubMul,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Replaces x and y, which are not both zero, with gcd(x, y) and xy/gcd(x, y), which is ±lcm(x, y).
fn replace_with_gcd_and_lcm(x: &mut Integer, y: &mut Integer) {
    let g = Integer::from(x.unsigned_abs_ref().gcd(y.unsigned_abs_ref()));
    *y *= (&*x).div_exact(&g);
    *x = g;
}

// Replaces x with gcd(x, m).
fn gcd_with_modulus_assign(x: &mut Integer, m: &Integer) {
    *x = Integer::from(x.unsigned_abs_ref().gcd(m.unsigned_abs_ref()));
}

// The Smith normal form of `a`, which is diagonal: only its diagonal elements are read. Their
// absolute values are replaced, pair by pair, with their GCDs and LCMs.
//
// This is equivalent to `fmpz_mat_snf_diagonal` from `fmpz_mat/snf_diagonal.c`, FLINT 3.6.0.
crate_test_fn! {
snf_diagonal(a: &IntegerMatrix) -> IntegerMatrix {
    let width = a.width;
    let n = a.height.min(width);
    let mut s = a.clone();
    let xs = &mut s.elements;
    for i in 0..n {
        xs[i * (width + 1)].abs_assign();
    }
    for j in (0..n).rev() {
        for i in 0..j {
            let (x, y) = two_elements(xs, i * (width + 1), (i + 1) * (width + 1));
            if x != y {
                replace_with_gcd_and_lcm(x, y);
            }
        }
    }
    s
}}

// A bound on the number of alternating row and column Hermite normal forms that reduce `a` to a
// diagonal matrix: 16 + 4 n² (b + ⌊log₂ n⌋ + 2), where n is the maximum of the height and
// width and b the maximum number of significant bits of the elements, saturating at the largest
// `u64`. It is four times the worst case of Kannan and Bachem, so that only a runaway loop exceeds
// it.
//
// This is equivalent to `_fmpz_mat_snf_iter_bound` from `fmpz_mat/snf_iter_bound.c`, FLINT 3.6.0,
// except that the bound is computed exactly rather than in floating point, and saturates at the
// largest `u64` rather than the largest signed word.
pub(crate) fn snf_iteration_bound(a: &IntegerMatrix) -> u64 {
    let n = u64::exact_from(a.height.max(a.width));
    let mut bound = vec_max_bits(&a.elements).0;
    bound.saturating_add_assign(n.significant_bits() + 1);
    bound.saturating_mul_assign(n);
    bound.saturating_mul_assign(n);
    bound.saturating_mul_assign(4);
    bound.saturating_add_assign(16);
    bound
}

// The unimodular transforms U and V of a Smith normal form S = UAV.
pub(crate) struct SmithTransforms {
    pub(crate) u: IntegerMatrix,
    pub(crate) v: IntegerMatrix,
}

// Replaces `x` with its Smith normal form, found by alternating row and column Hermite normal forms
// until the matrix is diagonal, then making each diagonal element divide the next, and finally
// making the diagonal elements nonnegative. If `transforms` is present, it starts as (U, V) and
// becomes (U'U, VV'), where U' and V' are the unimodular transforms taking `x` to its Smith normal
// form, so that, starting from identity matrices, U·x·V is the Smith normal form. For each pair
// of diagonal elements a and b that does not satisfy the divisibility condition, with d = gcd(a, b)
// = sa + tb, a = dp, and b = dq, the row operations [1, t; q, tq - 1] and the column operations [s,
// 1
// - sp; 1, -p], both with determinant -1, take diag(a, b) to diag(d, pqd); and the rows of U whose
// diagonal elements are negative are negated.
//
// This is equivalent to `_fmpz_mat_snf_iterative_hermite` from `fmpz_mat/snf.c`, FLINT 3.6.0, and,
// with `transforms` present, to `fmpz_mat_snf_transform` from `fmpz_mat/snf_transform.c`, FLINT
// 3.6.0, with neither U nor V `NULL`.
pub(crate) fn snf_iterative_in_place(
    x: &mut IntegerMatrix,
    mut transforms: Option<&mut SmithTransforms>,
) {
    let (m, n) = (x.height, x.width);
    let d = m.min(n);
    if d == 0 {
        return;
    }
    let max_iterations = snf_iteration_bound(x);
    let mut iteration = 0;
    while !x.is_diagonal() {
        assert!(
            iteration < max_iterations,
            "the alternating Hermite normal forms exceeded the iteration bound {max_iterations}"
        );
        // Row Hermite normal form: X becomes Mr·X and U becomes Mr·U.
        if let Some(t) = transforms.as_deref_mut() {
            let (h, mr) = (&*x).hermite_normal_form_and_transform();
            *x = h;
            t.u = mr * &t.u;
        } else {
            x.hermite_normal_form_assign();
        }
        if x.is_diagonal() {
            break;
        }
        // Column Hermite normal form, by transposition: X becomes X·Mcᵀ and V becomes V·Mcᵀ.
        if let Some(t) = transforms.as_deref_mut() {
            let (h, mc) = x.transpose().hermite_normal_form_and_transform();
            *x = h.transpose();
            t.v *= mc.transpose();
        } else {
            *x = x.transpose().hermite_normal_form().transpose();
        }
        iteration += 1;
    }
    // Make each diagonal element divide the next.
    let xs = &mut x.elements;
    for j in 0..d {
        if xs[j * (n + 1)] == 1u32 {
            continue;
        }
        for k in j + 1..d {
            let (a, b) = two_elements(xs, j * (n + 1), k * (n + 1));
            if *b == 0u32 || (*a != 0u32 && (&*b).divisible_by(&*a)) {
                continue;
            }
            let Some(t) = transforms.as_deref_mut() else {
                replace_with_gcd_and_lcm(a, b);
                continue;
            };
            let (g, s, v) = (&*a).extended_gcd(&*b);
            let g = Integer::from(g);
            let p = (&*a).div_exact(&g);
            let q = (&*b).div_exact(&g);
            let vq_minus_1 = &v * &q - Integer::ONE;
            let one_minus_sp = Integer::ONE.sub_mul(&s, &p);
            // Rows j and k of U: row j + t·row k, and q·row j + (tq - 1)·row k.
            let (row_j, row_k) = two_rows(&mut t.u.elements, m, j, k);
            for (x, y) in row_j.iter_mut().zip(row_k.iter_mut()) {
                let new_y = (&q).mul_add_mul(&*x, &vq_minus_1, &*y);
                x.add_mul_assign(&v, &*y);
                *y = new_y;
            }
            // Columns j and k of V: s·column j + column k, and (1 - sp)·column j - p·column k.
            for row in t.v.elements.chunks_exact_mut(n) {
                let (x, y) = two_elements(row, j, k);
                let new_y = (&one_minus_sp * &*x).sub_mul(&p, &*y);
                *x *= &s;
                *x += &*y;
                *y = new_y;
            }
            *a = g;
            *b = q * p * &*a;
        }
    }
    // Make the diagonal elements nonnegative, negating the corresponding rows of U.
    for j in 0..d {
        let x = &mut xs[j * (n + 1)];
        if *x < 0u32 {
            x.neg_assign();
            if let Some(t) = transforms.as_deref_mut() {
                neg_row(&mut t.u.elements[j * m..(j + 1) * m]);
            }
        }
    }
}

// The Smith normal form of `a`, found by alternating row and column Hermite normal forms until the
// matrix is diagonal, and then making each diagonal element divide the next and nonnegative.
//
// This is equivalent to `_fmpz_mat_snf_iterative_hermite` from `fmpz_mat/snf.c`, FLINT 3.6.0.
crate_test_fn! {
snf_iterative_hermite(a: &IntegerMatrix) -> IntegerMatrix {
    let mut x = a.clone();
    snf_iterative_in_place(&mut x, None);
    x
}}

// Clears column `k` of the `width`-wide matrix with elements `xs` below row `k` by combining rows
// `j` and `j + 1` with extended GCDs, from the bottom up, and moves the last row to row `k`.
fn kannan_bachem_clear_column(xs: &mut [Integer], m: usize, n: usize, k: usize) {
    for i in k + 1..m {
        // Reduce row i - 1 with row i.
        if xs[(i - 1) * n + k] == 0u32 {
            continue;
        }
        let (x, y) = two_rows(xs, n, i, i - 1);
        if x[k].unsigned_abs_ref() == y[k].unsigned_abs_ref() {
            if x[k] == y[k] {
                for (y, x) in y[k..].iter_mut().zip(&x[k..]) {
                    *y -= x;
                }
            } else {
                for (y, x) in y[k..].iter_mut().zip(&x[k..]) {
                    *y += x;
                }
            }
            continue;
        }
        RowCombination::new(&x[k], &y[k]).apply(&mut x[k..], &mut y[k..]);
    }
    swap_rows(xs, n, m - 1, k);
}

// Clears row `k` of the `width`-wide matrix with elements `xs` to the right of column `k` by
// combining column `k` with each later column with extended GCDs.
fn kannan_bachem_clear_row(xs: &mut [Integer], n: usize, k: usize) {
    for j in k + 1..n {
        // Reduce column j with column k.
        if xs[k * n + j] == 0u32 {
            continue;
        }
        if xs[k * n + k].unsigned_abs_ref() == xs[k * n + j].unsigned_abs_ref() {
            let equal = xs[k * n + k] == xs[k * n + j];
            for row in xs.chunks_exact_mut(n).skip(k) {
                let (y, x) = two_elements(row, j, k);
                if equal {
                    *y -= &*x;
                } else {
                    *y += &*x;
                }
            }
            continue;
        }
        let c = RowCombination::new(&xs[k * n + k], &xs[k * n + j]);
        c.apply_to_columns(xs, n, k, k, j);
    }
}

// The Smith normal form of `a`, by the algorithm of Kannan and Bachem: for each diagonal position,
// the elements below and to the right are cleared by row and column operations with extended GCDs
// until the column stays clear, and then the diagonal is put into Smith normal form.
//
// This is equivalent to `fmpz_mat_snf_kannan_bachem` from `fmpz_mat/snf_kannan_bachem.c`, FLINT
// 3.6.0.
crate_test_fn! {
snf_kannan_bachem(a: &IntegerMatrix) -> IntegerMatrix {
    let (m, n) = (a.height, a.width);
    let mut s = a.clone();
    let xs = &mut s.elements;
    for k in 0..m.min(n) {
        loop {
            kannan_bachem_clear_column(xs, m, n, k);
            kannan_bachem_clear_row(xs, n, k);
            if (0..m).all(|i| i == k || xs[i * n + k] == 0u32) {
                break;
            }
        }
        if xs[k * n + k] < 0u32 {
            xs[k * n + k].neg_assign();
        }
    }
    snf_diagonal(&s)
}}

// The GCD of the elements x of `xs`, which are not all absent, and coefficients t with Σ tⱼxⱼ
// = gcd(x), as computed by successive extended GCDs.
fn successive_extended_gcd<'a>(
    mut xs: impl Iterator<Item = &'a Integer>,
) -> (Integer, Vec<Integer>) {
    let mut g = xs.next().unwrap().clone();
    let mut t = vec![Integer::ONE];
    for x in xs {
        let (g_new, u, v) = extended_gcd_reduced_cofactor(&g, x);
        g = g_new;
        for t in &mut t {
            *t *= &u;
        }
        t.push(v);
    }
    (g, t)
}

// Clears column `i` below row `i`, modulo `modulus`, for the algorithm of Iliopoulos, in the `rows`
// by `cols` view of the matrix with elements `xs` whose element (r, c) has index `at(r, c)`: the
// matrix itself, to clear a column, or its transpose, to clear a row.
//
// This is equivalent to `_eliminate_col` from `fmpz_mat/snf_iliopoulos.c`, FLINT 3.6.0, and, on the
// transpose, to `_eliminate_row`, except that the elements of row `i` to the right of the pivot are
// also reduced modulo `modulus`, as they are by `_eliminate_col` but not by `_eliminate_row`.
fn iliopoulos_eliminate(
    xs: &mut [Integer],
    rows: usize,
    cols: usize,
    at: impl Fn(usize, usize) -> usize,
    i: usize,
    modulus: &Integer,
) {
    if i == rows - 1 {
        gcd_with_modulus_assign(&mut xs[at(i, i)], modulus);
        return;
    }
    // Replace rows i + 1 and i with combinations that make the element of row i in column i zero.
    if xs[at(i, i)] != 0u32 {
        let c = RowCombination::new(&xs[at(i + 1, i)], &xs[at(i, i)]);
        for col in i..cols {
            let (x, y) = two_elements(xs, at(i + 1, col), at(i, col));
            c.apply_to_pair(x, y);
        }
    }
    // Compute the extended GCD of the elements of column i below row i.
    let (g, mut t) = successive_extended_gcd((i + 1..rows).map(|k| &xs[at(k, i)]));
    // Add combinations of the rows below row i to row i, to give it the GCD in column i.
    for (k, t) in (i + 1..rows).zip(t.iter_mut()) {
        t.mod_assign(modulus);
        for col in i..cols {
            let (x, y) = two_elements(xs, at(i, col), at(k, col));
            x.add_mul_assign(&*t, &*y);
        }
    }
    // Reduce each row below row i with row i.
    if g != 0u32 {
        for k in i + 1..rows {
            let r = -(&xs[at(k, i)]).div_exact(&g);
            for col in i..cols {
                let (x, y) = two_elements(xs, at(k, col), at(i, col));
                x.add_mul_assign(&r, &*y);
            }
        }
        for k in i + 1..rows {
            xs[at(k, i)].mod_assign(modulus);
        }
    }
    for r in i..rows {
        for c in i + 1..cols {
            xs[at(r, c)].mod_assign(modulus);
        }
    }
    gcd_with_modulus_assign(&mut xs[at(i, i)], modulus);
}

// The Smith normal form of `a`, which is nonsingular, given `modulus`, a positive multiple of its
// largest elementary divisor, by the algorithm of Iliopoulos, which clears each row and column
// modulo `modulus` until the diagonal element divides the rest of its row, and then puts the
// diagonal into Smith normal form.
//
// This is equivalent to `fmpz_mat_snf_iliopoulos` from `fmpz_mat/snf_iliopoulos.c`, FLINT 3.6.0,
// except that, when a row is cleared, its elements to the right of the pivot are also reduced
// modulo `modulus`.
crate_test_fn! {
snf_iliopoulos(a: &IntegerMatrix, modulus: &Natural) -> IntegerMatrix {
    let (m, n) = (a.height, a.width);
    let modulus = Integer::from(modulus);
    let mut s = a.clone();
    let xs = &mut s.elements;
    for x in xs.iter_mut() {
        x.mod_assign(&modulus);
    }
    for k in 0..m.min(n) {
        loop {
            iliopoulos_eliminate(xs, n, m, |r, c| c * n + r, k, &modulus);
            iliopoulos_eliminate(xs, m, n, |r, c| r * n + c, k, &modulus);
            let pivot = &xs[k * n + k];
            let rest = &xs[k * n + k + 1..(k + 1) * n];
            let done = if *pivot == 0u32 {
                rest.iter().all(|x| *x == 0u32)
            } else {
                rest.iter().all(|x| x.divisible_by(pivot))
            };
            if done {
                break;
            }
        }
        for x in &mut xs[k * n + k + 1..(k + 1) * n] {
            *x = Integer::ZERO;
        }
    }
    snf_diagonal(&s)
}}

// The largest smaller dimension of the matrices whose Smith normal form is found by the algorithm
// of Kannan and Bachem, which is faster than alternating Hermite normal forms for them but whose
// intermediate elements grow quickly in larger matrices.
const KANNAN_BACHEM_MAX_DIMENSION: usize = 4;

// The Smith normal form of `a`: from its diagonal when it is diagonal, by the algorithm of Kannan
// and Bachem when its smaller dimension is at most 4, and by alternating row and column Hermite
// normal forms otherwise.
//
// This is equivalent to `fmpz_mat_snf` from `fmpz_mat/snf.c`, FLINT 3.6.0, except that the
// algorithm of Kannan and Bachem is used for small matrices.
fn snf_helper(a: &IntegerMatrix) -> IntegerMatrix {
    if a.is_diagonal() {
        snf_diagonal(a)
    } else if a.height.min(a.width) <= KANNAN_BACHEM_MAX_DIMENSION {
        snf_kannan_bachem(a)
    } else {
        snf_iterative_hermite(a)
    }
}

impl SmithNormalForm for IntegerMatrix {
    type Output = Self;

    /// Computes the Smith normal form of an [`IntegerMatrix`], taking it by value.
    ///
    /// The Smith normal form $S$ of an $h \times w$ matrix $A$ is the unique $h \times w$ diagonal
    /// matrix $UAV$, for unimodular $U$ and $V$, whose diagonal elements are nonnegative and each
    /// divide the next. Its nonzero diagonal elements, as many as the rank of $A$, are the
    /// elementary divisors of $A$.
    ///
    /// Diagonal matrices are reduced pair by pair, with GCDs and LCMs. Matrices whose smaller
    /// dimension is at most 4 are reduced by the algorithm of Kannan and Bachem, which clears each
    /// row and column of a diagonal element with extended GCDs until the column stays clear. Larger
    /// matrices are reduced by alternating row and column Hermite normal forms until the matrix is
    /// diagonal, as described by Sergeraert; then pairs of diagonal elements that violate the
    /// divisibility condition are replaced with their GCDs and LCMs.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^8 (b + \log n)^3 + n^8 (b + \log n)^2 \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: by the
    /// argument of Kannan and Bachem, $O(n^2 (b + \log n))$ alternations suffice, although usually
    /// only a few are needed; each Hermite normal form is of a matrix whose elements have $O(n(b +
    /// \log n))$ bits, since the determinantal divisors do not change, and is found within the
    /// bounds of
    /// [`hermite_normal_form`](malachite_base::num::arithmetic::traits::HermiteNormalForm).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalForm;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.smith_normal_form().to_string(), "[1, 0; 0, 2]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 4, 4; -6, 6, 12; 10, -4, -16]").unwrap();
    /// assert_eq!(
    ///     a.smith_normal_form().to_string(),
    ///     "[2, 0, 0; 0, 6, 0; 0, 0, 12]"
    /// );
    ///
    /// // A matrix of rank 2
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(
    ///     a.smith_normal_form().to_string(),
    ///     "[1, 0, 0; 0, 3, 0; 0, 0, 0]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// assert_eq!(a.smith_normal_form().to_string(), "[1, 0; 0, 2; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_snf` from `fmpz_mat/snf.c`, FLINT 3.6.0, except that the
    /// algorithm of Kannan and Bachem is used for matrices whose smaller dimension is at most 4.
    #[inline]
    fn smith_normal_form(self) -> Self {
        snf_helper(&self)
    }
}

impl SmithNormalForm for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Computes the Smith normal form of an [`IntegerMatrix`], taking it by reference.
    ///
    /// The Smith normal form $S$ of an $h \times w$ matrix $A$ is the unique $h \times w$ diagonal
    /// matrix $UAV$, for unimodular $U$ and $V$, whose diagonal elements are nonnegative and each
    /// divide the next. Its nonzero diagonal elements, as many as the rank of $A$, are the
    /// elementary divisors of $A$.
    ///
    /// Diagonal matrices are reduced pair by pair, with GCDs and LCMs. Matrices whose smaller
    /// dimension is at most 4 are reduced by the algorithm of Kannan and Bachem, which clears each
    /// row and column of a diagonal element with extended GCDs until the column stays clear. Larger
    /// matrices are reduced by alternating row and column Hermite normal forms until the matrix is
    /// diagonal, as described by Sergeraert; then pairs of diagonal elements that violate the
    /// divisibility condition are replaced with their GCDs and LCMs.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^8 (b + \log n)^3 + n^8 (b + \log n)^2 \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: by the
    /// argument of Kannan and Bachem, $O(n^2 (b + \log n))$ alternations suffice, although usually
    /// only a few are needed; each Hermite normal form is of a matrix whose elements have $O(n(b +
    /// \log n))$ bits, since the determinantal divisors do not change, and is found within the
    /// bounds of
    /// [`hermite_normal_form`](malachite_base::num::arithmetic::traits::HermiteNormalForm).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalForm;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).smith_normal_form().to_string(), "[1, 0; 0, 2]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 4, 4; -6, 6, 12; 10, -4, -16]").unwrap();
    /// assert_eq!(
    ///     (&a).smith_normal_form().to_string(),
    ///     "[2, 0, 0; 0, 6, 0; 0, 0, 12]"
    /// );
    ///
    /// // A matrix of rank 2
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(
    ///     (&a).smith_normal_form().to_string(),
    ///     "[1, 0, 0; 0, 3, 0; 0, 0, 0]"
    /// );
    ///
    /// let a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// assert_eq!((&a).smith_normal_form().to_string(), "[1, 0; 0, 2; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_snf` from `fmpz_mat/snf.c`, FLINT 3.6.0, except that the
    /// algorithm of Kannan and Bachem is used for matrices whose smaller dimension is at most 4.
    #[inline]
    fn smith_normal_form(self) -> IntegerMatrix {
        snf_helper(self)
    }
}

impl SmithNormalFormAssign for IntegerMatrix {
    /// Replaces an [`IntegerMatrix`] with its Smith normal form.
    ///
    /// The Smith normal form $S$ of an $h \times w$ matrix $A$ is the unique $h \times w$ diagonal
    /// matrix $UAV$, for unimodular $U$ and $V$, whose diagonal elements are nonnegative and each
    /// divide the next. Its nonzero diagonal elements, as many as the rank of $A$, are the
    /// elementary divisors of $A$.
    ///
    /// Diagonal matrices are reduced pair by pair, with GCDs and LCMs. Matrices whose smaller
    /// dimension is at most 4 are reduced by the algorithm of Kannan and Bachem, which clears each
    /// row and column of a diagonal element with extended GCDs until the column stays clear. Larger
    /// matrices are reduced by alternating row and column Hermite normal forms until the matrix is
    /// diagonal, as described by Sergeraert; then pairs of diagonal elements that violate the
    /// divisibility condition are replaced with their GCDs and LCMs.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^8 (b + \log n)^3 + n^8 (b + \log n)^2 \log (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 (b + \log n))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of its elements: by the
    /// argument of Kannan and Bachem, $O(n^2 (b + \log n))$ alternations suffice, although usually
    /// only a few are needed; each Hermite normal form is of a matrix whose elements have $O(n(b +
    /// \log n))$ bits, since the determinantal divisors do not change, and is found within the
    /// bounds of
    /// [`hermite_normal_form`](malachite_base::num::arithmetic::traits::HermiteNormalForm).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SmithNormalFormAssign;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.smith_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
    ///
    /// let mut a = IntegerMatrix::from_str("[2, 4, 4; -6, 6, 12; 10, -4, -16]").unwrap();
    /// a.smith_normal_form_assign();
    /// assert_eq!(a.to_string(), "[2, 0, 0; 0, 6, 0; 0, 0, 12]");
    ///
    /// // A matrix of rank 2
    /// let mut a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// a.smith_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0, 0; 0, 3, 0; 0, 0, 0]");
    ///
    /// let mut a = IntegerMatrix::from_str("[6, 4; 4, 6; 9, 3]").unwrap();
    /// a.smith_normal_form_assign();
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2; 0, 0]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_snf` from `fmpz_mat/snf.c`, FLINT 3.6.0, with `S` aliasing
    /// `A`, except that the algorithm of Kannan and Bachem is used for matrices whose smaller
    /// dimension is at most 4.
    #[inline]
    fn smith_normal_form_assign(&mut self) {
        *self = snf_helper(self);
    }
}
