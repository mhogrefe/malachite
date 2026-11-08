// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::linear_algebra::mod_minor::check_mod;
use malachite_base::matrix::Matrix;
#[cfg(doc)]
use malachite_base::num::linear_algebra::traits::ModDeterminant;
use malachite_base::num::linear_algebra::traits::{ModAdjugate, ModCofactor};

impl ModAdjugate<Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the adjugate of a square [`NaturalMatrix`] modulo $m$, taking the matrix by value
    /// and the modulus by value.
    ///
    /// The adjugate is the transpose of the matrix of cofactors modulo $m$:
    ///
    /// $$
    /// f(A, m)_{i, j} = C_{j, i} \bmod m,
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv (\det A) I \pmod{m}$.
    /// Each element is found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of [`mod_determinant`](ModDeterminant::mod_determinant) for a
    /// square matrix of the given height with the same elements.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAdjugate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_adjugate(Natural::from(7u32)).to_string(),
    ///     "[3, 5, 4; 4, 6, 6; 4, 6, 4]"
    /// );
    /// ```
    #[inline]
    fn mod_adjugate(self, m: Natural) -> Self {
        (&self).mod_adjugate(m)
    }
}

impl ModAdjugate<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the adjugate of a square [`NaturalMatrix`] modulo $m$, taking the matrix by value
    /// and the modulus by reference.
    ///
    /// The adjugate is the transpose of the matrix of cofactors modulo $m$:
    ///
    /// $$
    /// f(A, m)_{i, j} = C_{j, i} \bmod m,
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv (\det A) I \pmod{m}$.
    /// Each element is found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of [`mod_determinant`](ModDeterminant::mod_determinant) for a
    /// square matrix of the given height with the same elements.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAdjugate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_adjugate(&Natural::from(7u32)).to_string(),
    ///     "[3, 5, 4; 4, 6, 6; 4, 6, 4]"
    /// );
    /// ```
    #[inline]
    fn mod_adjugate(self, m: &Natural) -> Self {
        (&self).mod_adjugate(m)
    }
}

impl ModAdjugate<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the adjugate of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by value.
    ///
    /// The adjugate is the transpose of the matrix of cofactors modulo $m$:
    ///
    /// $$
    /// f(A, m)_{i, j} = C_{j, i} \bmod m,
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv (\det A) I \pmod{m}$.
    /// Each element is found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of [`mod_determinant`](ModDeterminant::mod_determinant) for a
    /// square matrix of the given height with the same elements.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAdjugate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_adjugate(Natural::from(7u32)).to_string(),
    ///     "[3, 5, 4; 4, 6, 6; 4, 6, 4]"
    /// );
    /// ```
    #[inline]
    fn mod_adjugate(self, m: Natural) -> NaturalMatrix {
        self.mod_adjugate(&m)
    }
}

impl ModAdjugate<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the adjugate of a square [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by reference.
    ///
    /// The adjugate is the transpose of the matrix of cofactors modulo $m$:
    ///
    /// $$
    /// f(A, m)_{i, j} = C_{j, i} \bmod m,
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) \equiv \operatorname{adj}(A) A \equiv (\det A) I \pmod{m}$.
    /// Each element is found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of [`mod_determinant`](ModDeterminant::mod_determinant) for a
    /// square matrix of the given height with the same elements.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any element of the matrix is
    /// greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModAdjugate;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_adjugate(&Natural::from(7u32)).to_string(),
    ///     "[3, 5, 4; 4, 6, 6; 4, 6, 4]"
    /// );
    /// ```
    fn mod_adjugate(self, m: &Natural) -> NaturalMatrix {
        check_mod(self, m);
        let n = self.height();
        NaturalMatrix::from_owned_elements(
            n,
            n,
            (0..n)
                .flat_map(|i| (0..n).map(move |j| self.mod_cofactor(j, i, m)))
                .collect(),
        )
    }
}
