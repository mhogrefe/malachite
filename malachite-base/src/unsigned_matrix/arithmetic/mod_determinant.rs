// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018 Fredrik Johansson
//
//      Copyright © 2015 Tommy Hofmann
//
//      Copyright © 2019 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ExtendedGcd, ModDeterminant, Parity};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::factorization::traits::IsPrime;
use crate::num::logic::traits::NotAssign;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::berkowitz_over_ring::{
    UnsignedModBerkowitzRing, charpoly_berkowitz_over_ring,
};
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::{assert_reduced, lu_in_place};
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::{
    is_divisible, signed_to_residue,
};
use alloc::vec;

// This is equivalent to `_nmod_mat_det_2x2` from `nmod_mat/det.c`, FLINT 3.6.0.
fn det_2x2<T: PrimitiveUnsigned>(a: T, b: T, c: T, d: T, m: T) -> T {
    a.mod_mul(d, m).mod_add(b.mod_neg(m).mod_mul(c, m), m)
}

// This is equivalent to `_nmod_mat_det_3x3` from `nmod_mat/det.c`, FLINT 3.6.0.
#[allow(clippy::too_many_arguments)]
fn det_3x3<T: PrimitiveUnsigned>(a: T, b: T, c: T, d: T, e: T, f: T, g: T, h: T, i: T, m: T) -> T {
    let s = det_2x2(e, f, h, i, m);
    let t = det_2x2(g, i, d, f, m);
    let u = det_2x2(d, e, g, h, m);
    a.mod_mul(s, m)
        .mod_add(b.mod_mul(t, m), m)
        .mod_add(c.mod_mul(u, m), m)
}

// This is equivalent to `_nmod_mat_det_4x4` from `nmod_mat/det.c`, FLINT 3.6.0.
fn det_4x4<T: PrimitiveUnsigned>(xs: &[T], m: T) -> T {
    let a = |i: usize, j: usize| xs[(i << 2) + j];
    let s = det_3x3(
        a(1, 1),
        a(1, 2),
        a(1, 3),
        a(2, 1),
        a(2, 2),
        a(2, 3),
        a(3, 1),
        a(3, 2),
        a(3, 3),
        m,
    );
    let t = det_3x3(
        a(1, 0),
        a(1, 2),
        a(1, 3),
        a(2, 0),
        a(2, 2),
        a(2, 3),
        a(3, 0),
        a(3, 2),
        a(3, 3),
        m,
    )
    .mod_neg(m);
    let u = det_3x3(
        a(1, 0),
        a(1, 1),
        a(1, 3),
        a(2, 0),
        a(2, 1),
        a(2, 3),
        a(3, 0),
        a(3, 1),
        a(3, 3),
        m,
    );
    let v = det_3x3(
        a(1, 0),
        a(1, 1),
        a(1, 2),
        a(2, 0),
        a(2, 1),
        a(2, 2),
        a(3, 0),
        a(3, 1),
        a(3, 2),
        m,
    )
    .mod_neg(m);
    a(0, 0)
        .mod_mul(s, m)
        .mod_add(a(0, 1).mod_mul(t, m), m)
        .mod_add(a(0, 2).mod_mul(u, m), m)
        .mod_add(a(0, 3).mod_mul(v, m), m)
}

// Sets `cp`, of length n + 1, to the coefficients of the characteristic polynomial of the n by n
// matrix with elements `xs`, from the constant coefficient up, with Berkowitz's division-free
// algorithm.
//
// This is equivalent to `_nmod_mat_charpoly_berkowitz` from `nmod_mat/charpoly.c`, FLINT 3.6.0.
crate_test_fn! {
charpoly_berkowitz<T: PrimitiveUnsigned>(cp: &mut [T], xs: &[T], n: usize, m: T) {
    let ring = UnsignedModBerkowitzRing::new(m, n);
    cp[..=n].copy_from_slice(&charpoly_berkowitz_over_ring(&ring, xs, n));
}}

// This is equivalent to `_nmod_mat_det` from `nmod_mat/det.c`, FLINT 3.6.0.
pub(crate) fn det_lu<T: PrimitiveUnsigned>(mut a: UnsignedMatrix<T>, m: T) -> T {
    let n = a.height;
    let (perm, rank) = lu_in_place(&mut a, m, true);
    let mut det = T::ZERO;
    if rank == n {
        det = T::ONE;
        for i in 0..n {
            det.mod_mul_assign(a.elements[i * n + i], m);
        }
    }
    if permutation_is_odd(&perm) {
        det.mod_neg_assign(m);
    }
    det
}

// Whether the permutation `perm` of `0..perm.len()` is odd.
//
// This is equivalent to `_perm_parity` from `perm/parity.c`, FLINT 3.6.0.
fn permutation_is_odd(perm: &[usize]) -> bool {
    let mut visited = vec![false; perm.len()];
    let mut odd = false;
    for start in 0..perm.len() {
        if visited[start] {
            continue;
        }
        // A cycle of length l is a product of l - 1 transpositions.
        let mut i = start;
        let mut length = 0;
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

// Returns `(g, s, t)`, where `g = sa - tb` is the GCD of `a` and `b` modulo `m` and `s` is a unit
// modulo `m`. `a` and `b` must be reduced modulo `m`.
//
// This is equivalent to `_nmod_xgcd_unit` from `nmod_mat/det_howell.c`, FLINT 3.6.0, starting from
// the cofactors of `extended_gcd` rather than those of `n_xgcd`; any starting cofactors lead to a
// unit.
fn xgcd_unit<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    a: U,
    b: U,
    m: U,
) -> (U, U, U) {
    let (g, s, t) = a.extended_gcd(b);
    let mut s = signed_to_residue(s, m);
    let mut t = signed_to_residue(t, m).mod_neg(m);
    let ag = a / g;
    let bg = b / g;
    while !s.coprime_with(m) {
        s.mod_add_assign(bg, m);
        t.mod_add_assign(ag, m);
    }
    (g, s, t)
}

// This is equivalent to `_nmod_mat_pivot` from `nmod_mat/det_howell.c`, FLINT 3.6.0, returning
// `None` if there is no pivot and `Some(swapped)` otherwise.
fn howell_pivot<T: PrimitiveUnsigned>(
    xs: &mut [T],
    n: usize,
    start_row: usize,
    col: usize,
) -> Option<bool> {
    if xs[start_row * n + col] != T::ZERO {
        return Some(false);
    }
    let j = (start_row + 1..n).find(|&j| xs[j * n + col] != T::ZERO)?;
    let (top, bottom) = xs.split_at_mut(j * n);
    top[start_row * n..(start_row + 1) * n].swap_with_slice(&mut bottom[..n]);
    Some(true)
}

// The determinant modulo `m`, which may be composite, of the n by n matrix with elements `xs`,
// computed by reducing it to upper triangular form with row swaps and Euclidean row operations.
//
// This is equivalent to `_nmod_mat_det_howell` from `nmod_mat/det_howell.c`, FLINT 3.6.0.
pub(crate) fn det_howell<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    xs: &mut [U],
    n: usize,
    m: U,
) -> U {
    if n == 0 {
        return if m == U::ONE { U::ZERO } else { U::ONE };
    }
    let mut det = U::ONE;
    let mut unit = U::ONE;
    let mut row = 0;
    let mut col = 0;
    while row < n && col < n {
        match howell_pivot(xs, n, row, col) {
            None => return U::ZERO,
            Some(true) => det.mod_neg_assign(m),
            Some(false) => {}
        }
        let mut i = row + 1;
        while i < n {
            let x = xs[i * n + col];
            if x == U::ZERO {
                i += 1;
                continue;
            }
            let (top, bottom) = xs.split_at_mut(i * n);
            let pivot_row = &mut top[row * n..(row + 1) * n];
            let other_row = &mut bottom[..n];
            if let Some(s) = is_divisible::<U, S>(x, pivot_row[col], m) {
                for (y, &p) in other_row[col..].iter_mut().zip(&pivot_row[col..]) {
                    y.mod_sub_assign(s.mod_mul(p, m), m);
                }
                i += 1;
            } else {
                let (_, s, t) = xgcd_unit::<U, S>(pivot_row[col], x, m);
                // Now g = s * x - t * y mod m.
                unit.mod_mul_assign(s, m);
                for (p, &y) in pivot_row[col..].iter_mut().zip(&other_row[col..]) {
                    *p = s.mod_mul(*p, m).mod_sub(t.mod_mul(y, m), m);
                }
                // Now the entry is divisible, so this row is processed again.
            }
        }
        det.mod_mul_assign(xs[row * n + col], m);
        row += 1;
        col += 1;
    }
    det.mod_mul(unit.mod_inverse(m).unwrap(), m)
}

// Whether `m` is prime. Moduli of more than 64 bits are treated as composite, since there is no
// primality test for them; the determinant of a matrix modulo such a modulus is computed with the
// method for composite moduli, which works for any modulus.
fn is_prime_modulus<T: PrimitiveUnsigned>(m: T) -> bool {
    if m.significant_bits() > u64::WIDTH {
        false
    } else {
        let m: u64 = m.wrapping_into();
        m.is_prime()
    }
}

// This is equivalent to `nmod_mat_det` from `nmod_mat/det.c`, FLINT 3.6.0.
fn mod_determinant_helper<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    a: UnsignedMatrix<U>,
    m: U,
) -> U {
    assert_reduced(&a, m);
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    let xs = &a.elements;
    match n {
        0 => {
            if m == U::ONE {
                U::ZERO
            } else {
                U::ONE
            }
        }
        1 => xs[0],
        2 => det_2x2(xs[0], xs[1], xs[2], xs[3], m),
        3 => det_3x3(
            xs[0], xs[1], xs[2], xs[3], xs[4], xs[5], xs[6], xs[7], xs[8], m,
        ),
        4 => det_4x4(xs, m),
        5..=8 => {
            let mut cp = [U::ZERO; 9];
            charpoly_berkowitz(&mut cp, xs, n, m);
            if n.odd() { cp[0].mod_neg(m) } else { cp[0] }
        }
        _ => {
            if is_prime_modulus(m) {
                det_lu(a, m)
            } else {
                let mut a = a;
                det_howell::<U, S>(&mut a.elements, n, m)
            }
        }
    }
}

macro_rules! impl_mod_determinant {
    ($u:ident, $s:ident) => {
        impl ModDeterminant<$u> for UnsignedMatrix<$u> {
            type Output = $u;

            /// Computes the determinant of a square [`UnsignedMatrix`] modulo $m$, taking it by
            /// value.
            ///
            /// The elements of the matrix must already be reduced modulo $m$, which may be
            /// composite. The determinant of an $n \times n$ matrix $A$ is
            ///
            /// $$
            /// f(A, m) = \left ( \sum_{\sigma \in S_n} \operatorname{sgn}(\sigma) \prod_{i=0}^{n-1}
            /// A_{i, \sigma(i)} \right ) \bmod m,
            /// $$
            ///
            /// so the determinant of the $0 \times 0$ matrix is $1 \bmod m$.
            ///
            /// Matrices up to $4 \times 4$ use explicit formulas, and matrices up to $8 \times 8$
            /// Berkowitz's division-free algorithm. Larger matrices use LU decomposition if $m$ is
            /// prime, and otherwise are reduced to upper triangular form with Euclidean row
            /// operations, which work for any modulus.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n^3 \log m)$
            ///
            /// $M(n) = O(n^2)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and
            /// $m$ is the modulus.
            ///
            /// # Panics
            /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix
            /// is greater than or equal to `m`.
            ///
            /// # Examples
            /// ```
            /// use core::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModDeterminant;
            /// use malachite_base::unsigned_matrix::UnsignedMatrix;
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
            /// assert_eq!(a.mod_determinant(5), 1);
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
            /// assert_eq!(a.mod_determinant(7), 5);
            ///
            /// // The modulus may be composite.
            /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 0, 3]").unwrap();
            /// assert_eq!(a.mod_determinant(6), 0);
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
            /// assert_eq!(a.mod_determinant(11), 8);
            /// ```
            ///
            /// This is equivalent to `nmod_mat_det` from `nmod_mat/det.c`, FLINT 3.6.0, except that
            /// it panics if an element is not reduced, and that for moduli of more than 64 bits,
            /// which only `u128` matrices can have, the method for composite moduli is used.
            #[inline]
            fn mod_determinant(self, m: $u) -> $u {
                mod_determinant_helper::<$u, $s>(self, m)
            }
        }

        impl ModDeterminant<$u> for &UnsignedMatrix<$u> {
            type Output = $u;

            /// Computes the determinant of a square [`UnsignedMatrix`] modulo $m$, taking it by
            /// reference.
            ///
            /// The elements of the matrix must already be reduced modulo $m$, which may be
            /// composite. The determinant of an $n \times n$ matrix $A$ is
            ///
            /// $$
            /// f(A, m) = \left ( \sum_{\sigma \in S_n} \operatorname{sgn}(\sigma) \prod_{i=0}^{n-1}
            /// A_{i, \sigma(i)} \right ) \bmod m,
            /// $$
            ///
            /// so the determinant of the $0 \times 0$ matrix is $1 \bmod m$.
            ///
            /// Matrices up to $4 \times 4$ use explicit formulas, and matrices up to $8 \times 8$
            /// Berkowitz's division-free algorithm. Larger matrices use LU decomposition if $m$ is
            /// prime, and otherwise are reduced to upper triangular form with Euclidean row
            /// operations, which work for any modulus.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n^3 \log m)$
            ///
            /// $M(n) = O(n^2)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and
            /// $m$ is the modulus.
            ///
            /// # Panics
            /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix
            /// is greater than or equal to `m`.
            ///
            /// # Examples
            /// ```
            /// use core::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModDeterminant;
            /// use malachite_base::unsigned_matrix::UnsignedMatrix;
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
            /// assert_eq!((&a).mod_determinant(5), 1);
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
            /// assert_eq!((&a).mod_determinant(7), 5);
            ///
            /// // The modulus may be composite.
            /// let a = UnsignedMatrix::<u8>::from_str("[2, 0; 0, 3]").unwrap();
            /// assert_eq!((&a).mod_determinant(6), 0);
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
            /// assert_eq!((&a).mod_determinant(11), 8);
            /// ```
            ///
            /// This is equivalent to `nmod_mat_det` from `nmod_mat/det.c`, FLINT 3.6.0, except that
            /// it panics if an element is not reduced, and that for moduli of more than 64 bits,
            /// which only `u128` matrices can have, the method for composite moduli is used.
            #[inline]
            fn mod_determinant(self, m: $u) -> $u {
                mod_determinant_helper::<$u, $s>(self.clone(), m)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_determinant);
