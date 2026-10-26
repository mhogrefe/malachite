// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModPermanent, ModPowerOf2Permanent};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::unsigned_matrix::arithmetic::permanent_over_ring::{
    ModRing, mod_permanent_by_parts, permanent_structured,
};

// Returns the permanent modulo m of the square matrix `a`, whose elements are reduced modulo m, by
// `mod_permanent_by_parts`.
fn mod_permanent_helper<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> T {
    assert_reduced(a, m);
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    let n_u64 = u64::exact_from(n);
    mod_permanent_by_parts(
        a.elements.clone(),
        n,
        &m,
        |xs, pow| {
            UnsignedMatrix::from_owned_elements(n_u64, n_u64, xs).mod_power_of_2_permanent(pow)
        },
        |xs, q| permanent_structured(&ModRing::new(*q), xs, n),
    )
}

impl<T: PrimitiveUnsigned> ModPermanent<T> for UnsignedMatrix<T> {
    type Output = T;

    /// Computes the permanent of a square [`UnsignedMatrix`] modulo $m$, taking it by value. The
    /// elements must already be reduced modulo $m$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1, reduced modulo $m$.
    ///
    /// Writing $m = 2^k q$ with $q$ odd, the permanent is computed modulo $q$ and modulo $2^k$, and
    /// the two are combined by the Chinese remainder theorem. Modulo $2^k$ it is computed as by
    /// [`mod_power_of_2_permanent`](crate::num::arithmetic::traits::ModPowerOf2Permanent). Modulo
    /// $q$, the matrix's structure is used first, in the same way: a zero row or column gives 0, a
    /// row or column with a single nonzero element contributes that element times the permanent of
    /// the remaining minor, a row or column with two nonzero elements is merged into a smaller
    /// matrix with the same permanent, and independent blocks of nonzero elements contribute the
    /// product of their permanents. What remains is computed by cofactor expansion when it is small
    /// and otherwise by Glynn's formula, a sum over $2^{n-1}$ sign vectors, which divides by
    /// $2^{n-1}$ and so needs 2 to be invertible. A remaining matrix of dimension greater than 63
    /// is expanded along its sparsest row or column.
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(2^n n)$ for a dense matrix of dimension at most 63
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPermanent;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.clone().mod_permanent(7), 1);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.clone().mod_permanent(7), 3);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str(
    ///     "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
    /// )
    /// .unwrap();
    /// // An odd modulus
    /// assert_eq!(a.clone().mod_permanent(251), 88);
    /// // A power of 2 times an odd number
    /// assert_eq!(a.clone().mod_permanent(240), 237);
    /// ```
    #[inline]
    fn mod_permanent(self, m: T) -> T {
        mod_permanent_helper(&self, m)
    }
}

impl<T: PrimitiveUnsigned> ModPermanent<T> for &UnsignedMatrix<T> {
    type Output = T;

    /// Computes the permanent of a square [`UnsignedMatrix`] modulo $m$, taking it by reference.
    /// The elements must already be reduced modulo $m$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1, reduced modulo $m$.
    ///
    /// Writing $m = 2^k q$ with $q$ odd, the permanent is computed modulo $q$ and modulo $2^k$, and
    /// the two are combined by the Chinese remainder theorem. Modulo $2^k$ it is computed as by
    /// [`mod_power_of_2_permanent`](crate::num::arithmetic::traits::ModPowerOf2Permanent). Modulo
    /// $q$, the matrix's structure is used first, in the same way: a zero row or column gives 0, a
    /// row or column with a single nonzero element contributes that element times the permanent of
    /// the remaining minor, a row or column with two nonzero elements is merged into a smaller
    /// matrix with the same permanent, and independent blocks of nonzero elements contribute the
    /// product of their permanents. What remains is computed by cofactor expansion when it is small
    /// and otherwise by Glynn's formula, a sum over $2^{n-1}$ sign vectors, which divides by
    /// $2^{n-1}$ and so needs 2 to be invertible. A remaining matrix of dimension greater than 63
    /// is expanded along its sparsest row or column.
    ///
    /// $$
    /// f(A, m) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(2^n n)$ for a dense matrix of dimension at most 63
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element is greater than or
    /// equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPermanent;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).mod_permanent(7), 1);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_permanent(7), 3);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str(
    ///     "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
    /// )
    /// .unwrap();
    /// // An odd modulus
    /// assert_eq!((&a).mod_permanent(251), 88);
    /// // A power of 2 times an odd number
    /// assert_eq!((&a).mod_permanent(240), 237);
    /// ```
    #[inline]
    fn mod_permanent(self, m: T) -> T {
        mod_permanent_helper(self, m)
    }
}
