// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018, 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_lu_decomposition::{assert_reduced, lu_in_place};
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::to_limb_matrix;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{
    AddMulAssign, ModAdd, ModAddAssign, ModDeterminant, ModMul, ModMulAssign, ModNeg, ModNegAssign,
    ModSub, ModSubAssign, Parity,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::NotAssign;
use malachite_base::unsigned_matrix::arithmetic::berkowitz_over_ring::{
    BerkowitzRing, charpoly_berkowitz_over_ring,
};

// The value ab - cd modulo m.
//
// This is equivalent to `gr_fmms` from `gr_mat/det_cofactor.c`, FLINT 3.6.0.
fn fmms(a: &Natural, b: &Natural, c: &Natural, d: &Natural, m: &Natural) -> Natural {
    a.mod_mul(b, m).mod_sub(c.mod_mul(d, m), m)
}

// The determinant modulo `m` of the n by n matrix with elements `xs`, where n is at most 4.
//
// This is equivalent to `gr_mat_det_cofactor` from `gr_mat/det_cofactor.c`, FLINT 3.6.0.
fn det_cofactor(xs: &[Natural], n: usize, m: &Natural) -> Natural {
    let e = |i: usize, j: usize| &xs[i * n + j];
    match n {
        0 => Natural::ONE,
        1 => xs[0].clone(),
        2 => fmms(e(0, 0), e(1, 1), e(0, 1), e(1, 0), m),
        3 => fmms(e(1, 0), e(2, 1), e(1, 1), e(2, 0), m)
            .mod_mul(e(0, 2), m)
            .mod_add(
                fmms(e(1, 2), e(2, 0), e(1, 0), e(2, 2), m).mod_mul(e(0, 1), m),
                m,
            )
            .mod_add(
                fmms(e(1, 1), e(2, 2), e(1, 2), e(2, 1), m).mod_mul(e(0, 0), m),
                m,
            ),
        _ => {
            let pairs = [
                ((0, 3, 1, 2, 0, 2, 1, 3), (2, 1, 3, 0, 2, 0, 3, 1)),
                ((0, 1, 1, 3, 0, 3, 1, 1), (2, 2, 3, 0, 2, 0, 3, 2)),
                ((0, 2, 1, 1, 0, 1, 1, 2), (2, 3, 3, 0, 2, 0, 3, 3)),
                ((0, 3, 1, 0, 0, 0, 1, 3), (2, 2, 3, 1, 2, 1, 3, 2)),
                ((0, 0, 1, 2, 0, 2, 1, 0), (2, 3, 3, 1, 2, 1, 3, 3)),
                ((0, 1, 1, 0, 0, 0, 1, 1), (2, 3, 3, 2, 2, 2, 3, 3)),
            ];
            let mut det = Natural::ZERO;
            for ((a0, a1, a2, a3, a4, a5, a6, a7), (b0, b1, b2, b3, b4, b5, b6, b7)) in pairs {
                let a = fmms(e(a0, a1), e(a2, a3), e(a4, a5), e(a6, a7), m);
                let b = fmms(e(b0, b1), e(b2, b3), e(b4, b5), e(b6, b7), m);
                det.mod_add_assign(a.mod_mul(b, m), m);
            }
            det
        }
    }
}

// The integers modulo a positive `m`, as `Natural`s, for `charpoly_berkowitz_over_ring`. The dot
// products are computed exactly and reduced once.
pub(crate) struct NaturalModBerkowitzRing<'a> {
    pub(crate) m: &'a Natural,
}

impl BerkowitzRing for NaturalModBerkowitzRing<'_> {
    type Element = Natural;

    #[inline]
    fn zero(&self) -> Natural {
        Natural::ZERO
    }

    #[inline]
    fn one(&self) -> Natural {
        Natural::ONE % self.m
    }

    #[inline]
    fn neg(&self, x: &Natural) -> Natural {
        x.mod_neg(self.m)
    }

    #[inline]
    fn add(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_add(y, self.m)
    }

    #[inline]
    fn sub_assign(&self, x: &mut Natural, y: &Natural) {
        x.mod_sub_assign(y, self.m);
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_mul(y, self.m)
    }

    fn dot(&self, xs: &[Natural], ys: &[Natural]) -> Natural {
        let mut s = Natural::ZERO;
        for (x, y) in xs.iter().zip(ys) {
            s.add_mul_assign(x, y);
        }
        s % self.m
    }

    fn dot_rev(&self, xs: &[Natural], ys: &[Natural]) -> Natural {
        let mut s = Natural::ZERO;
        for (x, y) in xs.iter().zip(ys.iter().rev()) {
            s.add_mul_assign(x, y);
        }
        s % self.m
    }
}

// Sets `cp`, of length n + 1, to the coefficients of the characteristic polynomial modulo `m` of
// the n by n matrix with elements `xs`, from the constant coefficient up, with Berkowitz's
// division-free algorithm.
//
// This is equivalent to `_gr_mat_charpoly_berkowitz` from `gr_mat/charpoly_berkowitz.c`, FLINT
// 3.6.0.
pub(crate) fn charpoly_berkowitz(cp: &mut [Natural], xs: &[Natural], n: usize, m: &Natural) {
    for (c, x) in cp.iter_mut().zip(charpoly_berkowitz_over_ring(
        &NaturalModBerkowitzRing { m },
        xs,
        n,
    )) {
        *c = x;
    }
}

// Whether the permutation `perm` of `0..perm.len()` is odd.
//
// This is equivalent to `_perm_parity` from `perm/parity.c`, FLINT 3.6.0.
pub(crate) fn permutation_is_odd(perm: &[usize]) -> bool {
    let mut visited = vec![false; perm.len()];
    let mut odd = false;
    for start in 0..perm.len() {
        if visited[start] {
            continue;
        }
        // A cycle of length l is a product of l - 1 transpositions.
        let mut i = start;
        let mut length = 0u64;
        while !visited[i] {
            visited[i] = true;
            i = perm[i];
            length += 1;
        }
        if length.even() {
            odd.not_assign();
        }
    }
    odd
}

// The determinant modulo `m` of the n by n matrix `a`, from its LU decomposition, or `None` if a
// pivot is not invertible.
//
// This is equivalent to `gr_mat_det_lu` from `gr_mat/det_lu.c`, FLINT 3.6.0, with `None` standing
// for a status other than `GR_SUCCESS`.
pub(crate) fn det_lu(mut a: NaturalMatrix, m: &Natural) -> Option<Natural> {
    let n = a.height;
    if n == 0 {
        return Some(Natural::ONE % m);
    }
    let (perm, rank) = lu_in_place(&mut a, m, true)?;
    if rank == 0 {
        return Some(Natural::ZERO);
    }
    let mut det = a.elements[0].clone();
    for i in 1..n {
        det.mod_mul_assign(&a.elements[i * n + i], m);
    }
    if permutation_is_odd(&perm) {
        det.mod_neg_assign(m);
    }
    Some(det)
}

// The determinant modulo `m` of the n by n matrix with elements `xs`, from its characteristic
// polynomial.
//
// This is equivalent to `gr_mat_det_berkowitz` from `gr_mat/det_berkowitz.c`, FLINT 3.6.0.
pub(crate) fn det_berkowitz(xs: &[Natural], n: usize, m: &Natural) -> Natural {
    let mut cp = vec![Natural::ZERO; n + 1];
    charpoly_berkowitz(&mut cp, xs, n, m);
    let d = core::mem::take(&mut cp[0]);
    if n.odd() { d.mod_neg(m) } else { d }
}

// This is equivalent to `fmpz_mod_mat_det` from `fmpz_mod_mat/det.c`, FLINT 3.6.0, which calls
// `_gr_fmpz_mod_mat_det` from `gr/fmpz_mod.c`.
fn mod_determinant_helper(a: &NaturalMatrix, m: &Natural) -> Natural {
    assert_reduced(a, m);
    assert_eq!(a.height, a.width, "the matrix must be square");
    if let Natural(Small(small_m)) = *m {
        return Natural::from(to_limb_matrix(a).mod_determinant(small_m));
    }
    let n = a.height;
    if n <= 4 {
        return det_cofactor(&a.elements, n, m);
    }
    // LU decomposition works on a copy, as in FLINT, so that the original is still available for
    // the division-free algorithm, which is used if a pivot is not invertible.
    det_lu(a.clone(), m).unwrap_or_else(|| det_berkowitz(&a.elements, n, m))
}

impl ModDeterminant<Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the determinant of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$, which may be composite. The
    /// determinant of an $n \times n$ matrix $A$ is
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \operatorname{sgn}(\sigma) \prod_{i=0}^{n-1}
    /// A_{i, \sigma(i)} \right ) \bmod m,
    /// $$
    ///
    /// so the determinant of the $0 \times 0$ matrix is $1 \bmod m$.
    ///
    /// When $m$ fits in a limb, the determinant is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs. Otherwise,
    /// matrices up to $4 \times 4$ use cofactor expansion, and larger ones LU decomposition, unless
    /// a pivot turns out not to be invertible, which can only happen if $m$ is composite; then
    /// Berkowitz's division-free algorithm is used.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDeterminant;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_determinant(Natural::from(7u32)), 5);
    ///
    /// // The modulus may be composite.
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!(a.mod_determinant(Natural::from(6u32)), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_det` from `fmpz_mod_mat/det.c`, FLINT 3.6.0, except that
    /// it panics if an element is not reduced.
    #[inline]
    fn mod_determinant(self, m: Natural) -> Natural {
        mod_determinant_helper(&self, &m)
    }
}

impl ModDeterminant<&Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the determinant of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$, which may be composite. The
    /// determinant of an $n \times n$ matrix $A$ is
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \operatorname{sgn}(\sigma) \prod_{i=0}^{n-1}
    /// A_{i, \sigma(i)} \right ) \bmod m,
    /// $$
    ///
    /// so the determinant of the $0 \times 0$ matrix is $1 \bmod m$.
    ///
    /// When $m$ fits in a limb, the determinant is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs. Otherwise,
    /// matrices up to $4 \times 4$ use cofactor expansion, and larger ones LU decomposition, unless
    /// a pivot turns out not to be invertible, which can only happen if $m$ is composite; then
    /// Berkowitz's division-free algorithm is used.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDeterminant;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_determinant(&Natural::from(7u32)), 5);
    ///
    /// // The modulus may be composite.
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!(a.mod_determinant(&Natural::from(6u32)), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_det` from `fmpz_mod_mat/det.c`, FLINT 3.6.0, except that
    /// it panics if an element is not reduced.
    #[inline]
    fn mod_determinant(self, m: &Natural) -> Natural {
        mod_determinant_helper(&self, m)
    }
}

impl ModDeterminant<Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the determinant of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$, which may be composite. The
    /// determinant of an $n \times n$ matrix $A$ is
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \operatorname{sgn}(\sigma) \prod_{i=0}^{n-1}
    /// A_{i, \sigma(i)} \right ) \bmod m,
    /// $$
    ///
    /// so the determinant of the $0 \times 0$ matrix is $1 \bmod m$.
    ///
    /// When $m$ fits in a limb, the determinant is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs. Otherwise,
    /// matrices up to $4 \times 4$ use cofactor expansion, and larger ones LU decomposition, unless
    /// a pivot turns out not to be invertible, which can only happen if $m$ is composite; then
    /// Berkowitz's division-free algorithm is used.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDeterminant;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_determinant(Natural::from(7u32)), 5);
    ///
    /// // The modulus may be composite.
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_determinant(Natural::from(6u32)), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_det` from `fmpz_mod_mat/det.c`, FLINT 3.6.0, except that
    /// it panics if an element is not reduced.
    #[inline]
    fn mod_determinant(self, m: Natural) -> Natural {
        mod_determinant_helper(self, &m)
    }
}

impl ModDeterminant<&Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the determinant of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$, which may be composite. The
    /// determinant of an $n \times n$ matrix $A$ is
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \operatorname{sgn}(\sigma) \prod_{i=0}^{n-1}
    /// A_{i, \sigma(i)} \right ) \bmod m,
    /// $$
    ///
    /// so the determinant of the $0 \times 0$ matrix is $1 \bmod m$.
    ///
    /// When $m$ fits in a limb, the determinant is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs. Otherwise,
    /// matrices up to $4 \times 4$ use cofactor expansion, and larger ones LU decomposition, unless
    /// a pivot turns out not to be invertible, which can only happen if $m$ is composite; then
    /// Berkowitz's division-free algorithm is used.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^4 b \log b \log \log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDeterminant;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_determinant(&Natural::from(7u32)), 5);
    ///
    /// // The modulus may be composite.
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!((&a).mod_determinant(&Natural::from(6u32)), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_mat_det` from `fmpz_mod_mat/det.c`, FLINT 3.6.0, except that
    /// it panics if an element is not reduced.
    #[inline]
    fn mod_determinant(self, m: &Natural) -> Natural {
        mod_determinant_helper(self, m)
    }
}
