// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
#[cfg(doc)]
use crate::num::linear_algebra::traits::ModPowerOf2Determinant;
use crate::num::linear_algebra::traits::{ModPowerOf2Adjugate, ModPowerOf2Cofactor};
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::linear_algebra::mod_power_of_2_minor::check_mod_power_of_2;

impl<T: PrimitiveUnsigned> ModPowerOf2Adjugate for UnsignedMatrix<T> {
    type Output = Self;

    /// Computes the adjugate of a square [`UnsignedMatrix`] modulo $2^k$, taking the matrix by
    /// value.
    ///
    /// The adjugate is the transpose of the matrix of cofactors modulo $2^k$:
    ///
    /// $$
    /// f(A)_{i, j} = C_{j, i} \bmod 2^k,
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv (\det A) I
    /// \pmod{2^k}$. Each element is found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of
    /// [`mod_power_of_2_determinant`](ModPowerOf2Determinant::mod_power_of_2_determinant) for a
    /// square matrix of the given height with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any element of the matrix is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2Adjugate;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_adjugate(3).to_string(),
    ///     "[5, 6, 5; 6, 4, 6; 5, 6, 5]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_adjugate(self, pow: u64) -> Self {
        (&self).mod_power_of_2_adjugate(pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Adjugate for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Computes the adjugate of a square [`UnsignedMatrix`] modulo $2^k$, taking the matrix by
    /// reference.
    ///
    /// The adjugate is the transpose of the matrix of cofactors modulo $2^k$:
    ///
    /// $$
    /// f(A)_{i, j} = C_{j, i} \bmod 2^k,
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv (\det A) I
    /// \pmod{2^k}$. Each element is found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of
    /// [`mod_power_of_2_determinant`](ModPowerOf2Determinant::mod_power_of_2_determinant) for a
    /// square matrix of the given height with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any element of the matrix is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2Adjugate;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_adjugate(3).to_string(),
    ///     "[5, 6, 5; 6, 4, 6; 5, 6, 5]"
    /// );
    /// ```
    fn mod_power_of_2_adjugate(self, pow: u64) -> UnsignedMatrix<T> {
        check_mod_power_of_2(self, pow);
        let n = self.height();
        UnsignedMatrix::from_owned_elements(
            n,
            n,
            (0..n)
                .flat_map(|i| (0..n).map(move |j| self.mod_power_of_2_cofactor(j, i, pow)))
                .collect(),
        )
    }
}
