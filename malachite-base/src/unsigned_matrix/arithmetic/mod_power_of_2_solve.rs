// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Solve};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::TrailingZeros;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_smith_normal_form::*;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;
use alloc::vec::Vec;

// Solves AX = B modulo 2^pow, where A is the `height` by `width` matrix with elements `xs` and B is
// the `height` by `k` matrix with elements `bs`, returning the elements of a `width` by `k`
// solution X, or `None` if there is none. A is reduced to diag(2^v_0, ..., 2^v_(r-1), 0, ...) =
// UAV, with the row operations U also applied to B and the column operations V accumulated; then DY
// = UB has a solution exactly when, for each i < r, 2^v_i divides row i of UB and the rows of UB
// from r on are zero, and X = VY.
pub(crate) fn solve_mod_power_of_2<T: PrimitiveUnsigned>(
    mut xs: Vec<T>,
    height: usize,
    width: usize,
    mut bs: Vec<T>,
    k: usize,
    pow: u64,
) -> Option<Vec<T>> {
    let mut v = vec![T::ZERO; width * width];
    for i in 0..width {
        v[i * (width + 1)] = T::ONE.mod_power_of_2(pow);
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
    if bs[r * k..].iter().any(|&x| x != T::ZERO) {
        return None;
    }
    let mut ys = vec![T::ZERO; width * k];
    for (i, &val) in valuations.iter().enumerate() {
        for (y, &b) in ys[i * k..(i + 1) * k]
            .iter_mut()
            .zip(&bs[i * k..(i + 1) * k])
        {
            if b != T::ZERO {
                if TrailingZeros::trailing_zeros(b) < val {
                    return None;
                }
                *y = b >> val;
            }
        }
    }
    // X = VY.
    let mut out = vec![T::ZERO; width * k];
    if k != 0 && width != 0 {
        for (out_row, v_row) in out.chunks_exact_mut(k).zip(v.chunks_exact(width)) {
            for (&c, y_row) in v_row.iter().zip(ys.chunks_exact(k)) {
                if c != T::ZERO {
                    for (o, &y) in out_row.iter_mut().zip(y_row) {
                        *o = o.mod_power_of_2_add(c.mod_power_of_2_mul(y, pow), pow);
                    }
                }
            }
        }
    }
    Some(out)
}

fn assert_arguments<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    b: &[T],
    b_height: usize,
    pow: u64,
) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most the width of the element type, {}, but it is {pow}",
        T::WIDTH
    );
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

fn mod_power_of_2_solve_matrix<T: PrimitiveUnsigned>(
    a: UnsignedMatrix<T>,
    b: Vec<T>,
    b_height: usize,
    b_width: usize,
    pow: u64,
) -> Option<UnsignedMatrix<T>> {
    assert_arguments(&a, &b, b_height, pow);
    let (h, w) = (a.height, a.width);
    solve_mod_power_of_2(a.elements, h, w, b, b_width, pow).map(|elements| UnsignedMatrix {
        elements,
        height: w,
        width: b_width,
    })
}

fn mod_power_of_2_solve_vector<T: PrimitiveUnsigned>(
    a: UnsignedMatrix<T>,
    b: Vec<T>,
    pow: u64,
) -> Option<UnsignedVector<T>> {
    let n = b.len();
    assert_arguments(&a, &b, n, pow);
    let (h, w) = (a.height, a.width);
    solve_mod_power_of_2(a.elements, h, w, b, 1, pow).map(|elements| UnsignedVector { elements })
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<Self> for UnsignedMatrix<T> {
    type Output = Self;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`UnsignedMatrix`] and $B$ is an
    /// [`UnsignedMatrix`], taking the matrix by value and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 6]").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: Self, pow: u64) -> Option<Self> {
        let (h, w) = (b.height, b.width);
        mod_power_of_2_solve_matrix(self, b.elements, h, w, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<&Self> for UnsignedMatrix<T> {
    type Output = Self;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`UnsignedMatrix`] and $B$ is an
    /// [`UnsignedMatrix`], taking the matrix by value and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 6]").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &Self, pow: u64) -> Option<Self> {
        mod_power_of_2_solve_matrix(self, b.elements.clone(), b.height, b.width, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`UnsignedMatrix`] and $B$ is an
    /// [`UnsignedMatrix`], taking the matrix by reference and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 6]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: UnsignedMatrix<T>, pow: u64) -> Option<UnsignedMatrix<T>> {
        let (h, w) = (b.height, b.width);
        mod_power_of_2_solve_matrix(self.clone(), b.elements, h, w, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<&UnsignedMatrix<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;
    /// Solves $AX = B$ modulo $2^k$ for $X$, where $A$ is an [`UnsignedMatrix`] and $B$ is an
    /// [`UnsignedMatrix`], taking the matrix by reference and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[2; 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 0])"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[6; 2]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some([1; 1; 0])"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[1; 6]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &UnsignedMatrix<T>, pow: u64) -> Option<UnsignedMatrix<T>> {
        mod_power_of_2_solve_matrix(self.clone(), b.elements.clone(), b.height, b.width, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<UnsignedVector<T>> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking the matrix by value and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 6)").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: UnsignedVector<T>, pow: u64) -> Option<UnsignedVector<T>> {
        mod_power_of_2_solve_vector(self, b.elements, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<&UnsignedVector<T>> for UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking the matrix by value and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 6)").unwrap();
    /// assert_eq!(a.mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &UnsignedVector<T>, pow: u64) -> Option<UnsignedVector<T>> {
        mod_power_of_2_solve_vector(self, b.elements.clone(), pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<UnsignedVector<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking the matrix by reference and the right-hand side by value.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 6)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: UnsignedVector<T>, pow: u64) -> Option<UnsignedVector<T>> {
        mod_power_of_2_solve_vector(self.clone(), b.elements, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Solve<&UnsignedVector<T>> for &UnsignedMatrix<T> {
    type Output = UnsignedVector<T>;
    /// Solves $Ax = b$ modulo $2^k$ for $x$, where $A$ is an [`UnsignedMatrix`] and $b$ is an
    /// [`UnsignedVector`], taking the matrix by reference and the right-hand side by reference.
    ///
    /// The elements of both must already be reduced modulo $2^k$. If $A$ is $h \times w$, a
    /// solution is a matrix (or vector) $X$ with $w$ rows and $AX = B$ modulo $2^k$, and `None` is
    /// returned if there is none. Unlike modulo a general composite number, every system modulo
    /// $2^k$ can be solved this way.
    ///
    /// $A$ is reduced to $D = UAV = \operatorname{diag}(2^{v_0}, \ldots, 2^{v_{r-1}}, 0, \ldots)$
    /// by elimination with full pivoting on the 2-adic valuation, as for the Smith normal form (see
    /// [`ModPowerOf2SmithNormalForm`](crate::num::arithmetic::traits::ModPowerOf2SmithNormalForm)),
    /// with the row operations $U$ also applied to $B$ and the column operations $V$ accumulated.
    /// Then $DY = UB$ has a solution exactly when each row $i < r$ of $UB$ is divisible by
    /// $2^{v_i}$ and the later rows are zero, and $X = VY$.
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(\min(h, w)(hw + hk + w^2) + w^2k)$
    ///
    /// $M(h, w, k) = O(hw + hk + w^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is the width of the right-hand side.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, if the height of the matrix is not that of
    /// the right-hand side, or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Solve;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(2, 6)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 0))"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(6, 2)").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_solve(&b, 3).to_debug_string(),
    ///     "Some((1, 1, 0))"
    /// );
    ///
    /// // The right-hand side has an odd element, but the matrix has none
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// let b = UnsignedVector::<u8>::from_str("(1, 6)").unwrap();
    /// assert_eq!((&a).mod_power_of_2_solve(&b, 3).to_debug_string(), "None");
    /// ```
    #[inline]
    fn mod_power_of_2_solve(self, b: &UnsignedVector<T>, pow: u64) -> Option<UnsignedVector<T>> {
        mod_power_of_2_solve_vector(self.clone(), b.elements.clone(), pow)
    }
}
