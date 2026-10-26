// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2Determinant, ModPowerOf2IsReduced, ModPowerOf2Permanent,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::permanent_over_ring::{
    ModPowerOf2Ring, permanent_structured,
};

// Returns the permanent modulo 2^pow of the square matrix `a`, whose elements are reduced modulo
// 2^pow. Modulo 1 it is 0, and modulo 2 it is the determinant, since -1 is 1; otherwise it is found
// by `permanent_structured`.
fn permanent_helper<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) -> T {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most the width of the element type, {}, but it is {pow}",
        T::WIDTH
    );
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    match pow {
        0 => T::ZERO,
        1 => a.mod_power_of_2_determinant(1),
        _ => permanent_structured(&ModPowerOf2Ring::new(pow), a.elements.clone(), a.height),
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Permanent for UnsignedMatrix<T> {
    type Output = T;

    /// Computes the permanent of a square [`UnsignedMatrix`] modulo $2^k$, taking it by value. The
    /// elements must already be reduced modulo $2^k$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1, reduced modulo $2^k$.
    ///
    /// Modulo 2 the permanent equals the determinant, since $-1 \equiv 1$, so it is computed in
    /// polynomial time. Otherwise the matrix's structure is used first, as for the permanent of an
    /// `IntegerMatrix`: a zero row or column gives 0, a row or column with a single nonzero element
    /// contributes that element times the permanent of the remaining minor, a row or column with
    /// two nonzero elements is merged into a smaller matrix with the same permanent, and
    /// independent blocks of nonzero elements contribute the product of their permanents. What
    /// remains is computed by cofactor expansion when it is small and otherwise by Glynn's formula,
    /// a sum over $2^{n-1}$ sign vectors that equals $2^{n-1}$ times the permanent: the sum is
    /// computed modulo $2^{k+n-1}$ and its low $n - 1$ bits are shifted out. If $2^{k+n-1}$ does
    /// not fit in a `T`, Ryser's formula, a sum over the $2^n$ sets of rows, is used instead. A
    /// remaining matrix of dimension greater than 63 is expanded along its sparsest row or column.
    ///
    /// $$
    /// f(A, k) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod 2^k.
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
    /// Panics if the matrix is not square, if `pow` is greater than `T::WIDTH`, or if any element
    /// is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Permanent;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.clone().mod_power_of_2_permanent(3), 2);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str(
    ///     "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
    /// )
    /// .unwrap();
    /// assert_eq!(a.clone().mod_power_of_2_permanent(8), 189);
    ///
    /// // Modulo 2, the permanent is the determinant.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1, 0; 0, 1, 1; 1, 0, 1]").unwrap();
    /// assert_eq!(a.clone().mod_power_of_2_permanent(1), 0);
    /// ```
    #[inline]
    fn mod_power_of_2_permanent(self, pow: u64) -> T {
        permanent_helper(&self, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Permanent for &UnsignedMatrix<T> {
    type Output = T;

    /// Computes the permanent of a square [`UnsignedMatrix`] modulo $2^k$, taking it by reference.
    /// The elements must already be reduced modulo $2^k$.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1, reduced modulo $2^k$.
    ///
    /// Modulo 2 the permanent equals the determinant, since $-1 \equiv 1$, so it is computed in
    /// polynomial time. Otherwise the matrix's structure is used first, as for the permanent of an
    /// `IntegerMatrix`: a zero row or column gives 0, a row or column with a single nonzero element
    /// contributes that element times the permanent of the remaining minor, a row or column with
    /// two nonzero elements is merged into a smaller matrix with the same permanent, and
    /// independent blocks of nonzero elements contribute the product of their permanents. What
    /// remains is computed by cofactor expansion when it is small and otherwise by Glynn's formula,
    /// a sum over $2^{n-1}$ sign vectors that equals $2^{n-1}$ times the permanent: the sum is
    /// computed modulo $2^{k+n-1}$ and its low $n - 1$ bits are shifted out. If $2^{k+n-1}$ does
    /// not fit in a `T`, Ryser's formula, a sum over the $2^n$ sets of rows, is used instead. A
    /// remaining matrix of dimension greater than 63 is expanded along its sparsest row or column.
    ///
    /// $$
    /// f(A, k) = \left ( \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)} \right )
    ///     \bmod 2^k.
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
    /// Panics if the matrix is not square, if `pow` is greater than `T::WIDTH`, or if any element
    /// is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Permanent;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_permanent(3), 2);
    ///
    /// let a = UnsignedMatrix::<u8>::from_str(
    ///     "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
    /// )
    /// .unwrap();
    /// assert_eq!((&a).mod_power_of_2_permanent(8), 189);
    ///
    /// // Modulo 2, the permanent is the determinant.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1, 0; 0, 1, 1; 1, 0, 1]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_permanent(1), 0);
    /// ```
    #[inline]
    fn mod_power_of_2_permanent(self, pow: u64) -> T {
        permanent_helper(self, pow)
    }
}
