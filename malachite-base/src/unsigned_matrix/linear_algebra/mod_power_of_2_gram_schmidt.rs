// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::linear_algebra::traits::ModPowerOf2GramSchmidt;
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::linear_algebra::gram_schmidt_over_ring::{
    ModPowerOf2WordRing, gram_schmidt_in_ring,
};

fn mod_power_of_2_gram_schmidt_helper<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> Option<UnsignedMatrix<T>> {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^pow, but {a} has an element >= 2^{pow}"
    );
    Some(UnsignedMatrix {
        elements: gram_schmidt_in_ring(
            &ModPowerOf2WordRing::new(pow),
            &a.elements,
            a.height,
            a.width,
        )?,
        height: a.height,
        width: a.width,
    })
}

impl<T: PrimitiveUnsigned> ModPowerOf2GramSchmidt for UnsignedMatrix<T> {
    type Output = Self;

    /// Applies the Gram-Schmidt process to the columns of an [`UnsignedMatrix`] modulo $2^k$,
    /// taking it by value, without normalizing them; or returns `None` if a squared length that it
    /// needs to divide by is even.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. Column $i$ of the result
    /// $B$ is column $i$ of the matrix $A$ minus its projections onto the earlier columns of $B$:
    /// $$
    /// b_i \equiv a_i - \sum_{j < i} \frac{\langle a_i, b_j \rangle}{\langle b_j, b_j \rangle}b_j
    /// \pmod{2^k},
    /// $$
    /// where the terms with $b_j = 0$ are left out. The columns of the result are orthogonal modulo
    /// $2^k$, but not normalized. A column that depends on the earlier ones becomes 0, and the
    /// later columns skip it, so the result has the shape of the matrix.
    ///
    /// A squared length must be odd to be divided by. A nonzero column can even be orthogonal to
    /// itself, as $(2, 2)$ is modulo 8, and then nothing can be projected onto it; the result is
    /// `None` in both cases. The squared length of the last column is never needed. When $k$ is 0,
    /// every matrix is 0, and so is the result.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of the matrix is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2GramSchmidt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_gram_schmidt(3).unwrap().to_string(),
    ///     "[1, 6; 2, 5]"
    /// );
    ///
    /// // (2, 2) is orthogonal to itself modulo 8
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 1; 2, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_gram_schmidt(3), None);
    ///
    /// // (1, 1) has squared length 2, which is even
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_gram_schmidt(3), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a power of 2; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_power_of_2_gram_schmidt(self, pow: u64) -> Option<Self> {
        mod_power_of_2_gram_schmidt_helper(&self, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2GramSchmidt for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Applies the Gram-Schmidt process to the columns of an [`UnsignedMatrix`] modulo $2^k$,
    /// taking it by reference, without normalizing them; or returns `None` if a squared length that
    /// it needs to divide by is even.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. Column $i$ of the result
    /// $B$ is column $i$ of the matrix $A$ minus its projections onto the earlier columns of $B$:
    /// $$
    /// b_i \equiv a_i - \sum_{j < i} \frac{\langle a_i, b_j \rangle}{\langle b_j, b_j \rangle}b_j
    /// \pmod{2^k},
    /// $$
    /// where the terms with $b_j = 0$ are left out. The columns of the result are orthogonal modulo
    /// $2^k$, but not normalized. A column that depends on the earlier ones becomes 0, and the
    /// later columns skip it, so the result has the shape of the matrix.
    ///
    /// A squared length must be odd to be divided by. A nonzero column can even be orthogonal to
    /// itself, as $(2, 2)$ is modulo 8, and then nothing can be projected onto it; the result is
    /// `None` in both cases. The squared length of the last column is never needed. When $k$ is 0,
    /// every matrix is 0, and so is the result.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, and $w$ is its
    /// width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of the matrix is greater than
    /// or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2GramSchmidt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_gram_schmidt(3).unwrap().to_string(),
    ///     "[1, 6; 2, 5]"
    /// );
    ///
    /// // (2, 2) is orthogonal to itself modulo 8
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 1; 2, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_gram_schmidt(3), None);
    ///
    /// // (1, 1) has squared length 2, which is even
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_gram_schmidt(3), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a power of 2; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_power_of_2_gram_schmidt(self, pow: u64) -> Option<UnsignedMatrix<T>> {
        mod_power_of_2_gram_schmidt_helper(self, pow)
    }
}
