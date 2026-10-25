// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_power_of_2_smith_normal_form::*;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Solve,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Solves AX = B modulo 2^pow, where A is the `height` by `width` matrix with elements `xs` and B is
// the `height` by `k` matrix with elements `bs`, returning the elements of a `width` by `k`
// solution X, or `None` if there is none, as for `UnsignedMatrix`s.
fn solve_mod_power_of_2(
    mut xs: Vec<Natural>,
    height: usize,
    width: usize,
    mut bs: Vec<Natural>,
    k: usize,
    pow: u64,
) -> Option<Vec<Natural>> {
    let mut v = vec![Natural::ZERO; width * width];
    for i in 0..width {
        v[i * (width + 1)] = Natural::ONE;
    }
    let valuations = smith_eliminate_mod_power_of_2(
        &mut xs,
        height,
        width,
        pow,
        Some((&mut bs, k)),
        Some(&mut v),
    );
    let r = valuations.len();
    if bs[r * k..].iter().any(|x| *x != 0u32) {
        return None;
    }
    let mut ys = vec![Natural::ZERO; width * k];
    for (i, &val) in valuations.iter().enumerate() {
        for (y, b) in ys[i * k..(i + 1) * k]
            .iter_mut()
            .zip(&bs[i * k..(i + 1) * k])
        {
            if let Some(b_val) = b.trailing_zeros() {
                if b_val < val {
                    return None;
                }
                *y = b >> val;
            }
        }
    }
    // X = VY.
    let mut out = vec![Natural::ZERO; width * k];
    if k != 0 && width != 0 {
        for (out_row, v_row) in out.chunks_exact_mut(k).zip(v.chunks_exact(width)) {
            for (c, y_row) in v_row.iter().zip(ys.chunks_exact(k)) {
                if *c != 0u32 {
                    for (o, y) in out_row.iter_mut().zip(y_row) {
                        o.mod_power_of_2_add_assign(c.mod_power_of_2_mul(y, pow), pow);
                    }
                }
            }
        }
    }
    Some(out)
}

fn assert_arguments(a: &NaturalMatrix, b: &[Natural], b_height: usize, pow: u64) {
    assert_eq!(
        a.height, b_height,
        "the matrix and the right-hand side must have the same height"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    assert!(
        b.iter().all(|x| x.significant_bits() <= pow),
        "the right-hand side must be reduced mod 2^{pow}"
    );
}

fn limb_matrix(height: usize, width: usize, xs: &[Natural]) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(
        u64::exact_from(height),
        u64::exact_from(width),
        xs.iter().map(Limb::wrapping_from).collect(),
    )
}

// Solves AX = B modulo 2^pow, where B is `b_height` by `b_width` with elements `b`, returning the
// elements of a solution.
fn solve_helper(
    a: NaturalMatrix,
    b: Vec<Natural>,
    b_height: usize,
    b_width: usize,
    pow: u64,
) -> Option<Vec<Natural>> {
    assert_arguments(&a, &b, b_height, pow);
    if pow <= Limb::WIDTH {
        to_limb_matrix(&a)
            .mod_power_of_2_solve(limb_matrix(b_height, b_width, &b), pow)
            .map(|x| x.elements.into_iter().map(Natural::from).collect())
    } else {
        let (h, w) = (a.height, a.width);
        solve_mod_power_of_2(a.elements, h, w, b, b_width, pow)
    }
}

fn solve_matrix(a: NaturalMatrix, b: NaturalMatrix, pow: u64) -> Option<NaturalMatrix> {
    let (w, bh, bw) = (a.width, b.height, b.width);
    solve_helper(a, b.elements, bh, bw, pow).map(|elements| NaturalMatrix {
        elements,
        height: w,
        width: bw,
    })
}

fn solve_vector(a: NaturalMatrix, b: NaturalVector, pow: u64) -> Option<NaturalVector> {
    let n = b.elements.len();
    solve_helper(a, b.elements, n, 1, pow).map(|elements| NaturalVector { elements })
}

impl ModPowerOf2Solve<Self> for NaturalMatrix {
    type Output = Self;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`NaturalMatrix`] and $B$ is an
    /// [`NaturalMatrix`], taking the matrix by value and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 6]").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: Self, pow: u64) -> Option<Self> {
        solve_matrix(self, b, pow)
    }
}

impl ModPowerOf2Solve<&Self> for NaturalMatrix {
    type Output = Self;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`NaturalMatrix`] and $B$ is an
    /// [`NaturalMatrix`], taking the matrix by value and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 6]").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &Self, pow: u64) -> Option<Self> {
        solve_matrix(self, b.clone(), pow)
    }
}

impl ModPowerOf2Solve<NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`NaturalMatrix`] and $B$ is an
    /// [`NaturalMatrix`], taking the matrix by reference and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 6]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: NaturalMatrix, pow: u64) -> Option<NaturalMatrix> {
        solve_matrix(self.clone(), b, pow)
    }
}

impl ModPowerOf2Solve<&NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`NaturalMatrix`] and $B$ is an
    /// [`NaturalMatrix`], taking the matrix by reference and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalMatrix::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalMatrix::from_str("[1; 6]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &NaturalMatrix, pow: u64) -> Option<NaturalMatrix> {
        solve_matrix(self.clone(), b.clone(), pow)
    }
}

impl ModPowerOf2Solve<NaturalVector> for NaturalMatrix {
    type Output = NaturalVector;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`NaturalMatrix`] and $b$ is an
    /// [`NaturalVector`], taking the matrix by value and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalVector::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(1, 6)").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: NaturalVector, pow: u64) -> Option<NaturalVector> {
        solve_vector(self, b, pow)
    }
}

impl ModPowerOf2Solve<&NaturalVector> for NaturalMatrix {
    type Output = NaturalVector;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`NaturalMatrix`] and $b$ is an
    /// [`NaturalVector`], taking the matrix by value and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalVector::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(1, 6)").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &NaturalVector, pow: u64) -> Option<NaturalVector> {
        solve_vector(self, b.clone(), pow)
    }
}

impl ModPowerOf2Solve<NaturalVector> for &NaturalMatrix {
    type Output = NaturalVector;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`NaturalMatrix`] and $b$ is an
    /// [`NaturalVector`], taking the matrix by reference and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalVector::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(1, 6)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: NaturalVector, pow: u64) -> Option<NaturalVector> {
        solve_vector(self.clone(), b, pow)
    }
}

impl ModPowerOf2Solve<&NaturalVector> for &NaturalMatrix {
    type Output = NaturalVector;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`NaturalMatrix`] and $b$ is an
    /// [`NaturalVector`], taking the matrix by reference and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2SmithNormalForm)), with the row
    /// operations $U$ also applied to $B$ and the column operations $V$ accumulated. Then $DY = UB$
    /// has a solution exactly when each row $i < r$ of $UB$ is divisible by $2^{v_i}$ and the later
    /// rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k, n) = O((\min(h, w)(hw + hk + w^2) + w^2k) n \log n \log\log n)$
    ///
    /// $M(h, w, k, n) = O((hw + hk + w^2) n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, $k$ is the width of the right-hand side, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if the height of the matrix is not that of the right-hand side, or if any element of
    /// either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = NaturalVector::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = NaturalVector::from_str("(1, 6)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &NaturalVector, pow: u64) -> Option<NaturalVector> {
        solve_vector(self.clone(), b.clone(), pow)
    }
}
