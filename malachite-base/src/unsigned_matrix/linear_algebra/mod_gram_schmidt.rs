// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::linear_algebra::traits::ModGramSchmidt;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::linear_algebra::gram_schmidt_over_ring::{
    ModWordRing, gram_schmidt_in_ring,
};
use crate::unsigned_matrix::linear_algebra::mod_lu_decomposition::assert_reduced;

fn mod_gram_schmidt_helper<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> Option<UnsignedMatrix<T>> {
    assert_reduced(a, m);
    Some(UnsignedMatrix {
        elements: gram_schmidt_in_ring(
            &ModWordRing::new(m, a.height.max(1)),
            &a.elements,
            a.height,
            a.width,
        )?,
        height: a.height,
        width: a.width,
    })
}

impl<T: PrimitiveUnsigned> ModGramSchmidt<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Applies the Gram-Schmidt process to the columns of an [`UnsignedMatrix`] modulo $m$, taking
    /// it by value, without normalizing them; or returns `None` if a squared length that it needs
    /// to divide by is not invertible modulo $m$.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. Column $i$ of the result $B$
    /// is column $i$ of the matrix $A$ minus its projections onto the earlier columns of $B$:
    /// $$
    /// b_i \equiv a_i - \sum_{j < i} \frac{\langle a_i, b_j \rangle}{\langle b_j, b_j \rangle}b_j
    /// \pmod{m},
    /// $$
    /// where the terms with $b_j = 0$ are left out. The columns of the result are orthogonal modulo
    /// $m$, but not normalized. A column that depends on the earlier ones becomes 0, and the later
    /// columns skip it, so the result has the shape of the matrix.
    ///
    /// A nonzero column can be orthogonal to itself, as $(1, 1)$ is modulo 2, and then nothing can
    /// be projected onto it, so the result is `None`. It is also `None` when a squared length is
    /// nonzero but not invertible, which happens only for composite $m$. Each divisor is checked
    /// directly, so $m$ need not be prime. The squared length of the last column is never needed.
    /// When $m$ is 1, every matrix is 0, and so is the result.
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
    /// Panics if `m` is 0, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_gram_schmidt(5).unwrap().to_string(), "[1, 3; 1, 2]");
    ///
    /// // (1, 1) is orthogonal to itself modulo 2
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_gram_schmidt(2), None);
    ///
    /// // (1, 1) has squared length 2, which is not invertible modulo 6
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 0; 1, 1]").unwrap();
    /// assert_eq!(a.mod_gram_schmidt(6), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a number; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_gram_schmidt(self, m: T) -> Option<Self> {
        mod_gram_schmidt_helper(&self, m)
    }
}

impl<T: PrimitiveUnsigned> ModGramSchmidt<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Applies the Gram-Schmidt process to the columns of an [`UnsignedMatrix`] modulo $m$, taking
    /// it by reference, without normalizing them; or returns `None` if a squared length that it
    /// needs to divide by is not invertible modulo $m$.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. Column $i$ of the result $B$
    /// is column $i$ of the matrix $A$ minus its projections onto the earlier columns of $B$:
    /// $$
    /// b_i \equiv a_i - \sum_{j < i} \frac{\langle a_i, b_j \rangle}{\langle b_j, b_j \rangle}b_j
    /// \pmod{m},
    /// $$
    /// where the terms with $b_j = 0$ are left out. The columns of the result are orthogonal modulo
    /// $m$, but not normalized. A column that depends on the earlier ones becomes 0, and the later
    /// columns skip it, so the result has the shape of the matrix.
    ///
    /// A nonzero column can be orthogonal to itself, as $(1, 1)$ is modulo 2, and then nothing can
    /// be projected onto it, so the result is `None`. It is also `None` when a squared length is
    /// nonzero but not invertible, which happens only for composite $m$. Each divisor is checked
    /// directly, so $m$ need not be prime. The squared length of the last column is never needed.
    /// When $m$ is 1, every matrix is 0, and so is the result.
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
    /// Panics if `m` is 0, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_gram_schmidt(5).unwrap().to_string(),
    ///     "[1, 3; 1, 2]"
    /// );
    ///
    /// // (1, 1) is orthogonal to itself modulo 2
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).mod_gram_schmidt(2), None);
    ///
    /// // (1, 1) has squared length 2, which is not invertible modulo 6
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 0; 1, 1]").unwrap();
    /// assert_eq!((&a).mod_gram_schmidt(6), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a number; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_gram_schmidt(self, m: T) -> Option<UnsignedMatrix<T>> {
        mod_gram_schmidt_helper(self, m)
    }
}
