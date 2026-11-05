// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::{Natural, TWICE_WIDTH};
use crate::natural_matrix::NaturalMatrix;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::ModPowerOf2GramSchmidt;
use malachite_base::num::modular::traits::{
    ModPowerOf2, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2SubAssign,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::linear_algebra::gram_schmidt_over_ring::{
    GramSchmidtRing, gram_schmidt_in_ring,
};

// The integers modulo $2^k$, for a $k$ greater than the width of two limbs. Dot products are summed
// exactly and reduced once.
pub(crate) struct NaturalModPowerOf2Ring {
    pub(crate) pow: u64,
}

impl GramSchmidtRing for NaturalModPowerOf2Ring {
    type Element = Natural;

    #[inline]
    fn is_zero(&self, x: &Natural) -> bool {
        *x == 0u32
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_mul(y, self.pow)
    }

    #[inline]
    fn inverse(&self, x: &Natural) -> Option<Natural> {
        x.mod_power_of_2_inverse(self.pow)
    }

    fn dot(&self, xs: &[Natural], ys: &[Natural]) -> Natural {
        let mut sum = Natural::ZERO;
        for (x, y) in xs.iter().zip(ys) {
            sum.add_mul_assign(x, y);
        }
        sum.mod_power_of_2(self.pow)
    }

    fn sub_mul_assign(&self, xs: &mut [Natural], c: &Natural, ys: &[Natural]) {
        for (x, y) in xs.iter_mut().zip(ys) {
            x.mod_power_of_2_sub_assign(c.mod_power_of_2_mul(y, self.pow), self.pow);
        }
    }
}

// The Gram-Schmidt process applied to the columns of `a` modulo 2^pow: in single limbs when 2^pow
// fits in one, in double limbs when it fits in two, and otherwise in `Natural`s.
fn mod_power_of_2_gram_schmidt_helper(a: &NaturalMatrix, pow: u64) -> Option<NaturalMatrix> {
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    if pow <= Limb::WIDTH {
        let elements: Vec<Limb> = a.elements.iter().map(Limb::exact_from).collect();
        Some(NaturalMatrix::from(
            UnsignedMatrix::from_owned_elements(a.height(), a.width(), elements)
                .mod_power_of_2_gram_schmidt(pow)?,
        ))
    } else if pow <= TWICE_WIDTH {
        let elements: Vec<DoubleLimb> = a.elements.iter().map(DoubleLimb::exact_from).collect();
        Some(NaturalMatrix::from(
            UnsignedMatrix::from_owned_elements(a.height(), a.width(), elements)
                .mod_power_of_2_gram_schmidt(pow)?,
        ))
    } else {
        Some(NaturalMatrix {
            elements: gram_schmidt_in_ring(
                &NaturalModPowerOf2Ring { pow },
                &a.elements,
                a.height,
                a.width,
            )?,
            height: a.height,
            width: a.width,
        })
    }
}

impl ModPowerOf2GramSchmidt for NaturalMatrix {
    type Output = Self;

    /// Applies the Gram-Schmidt process to the columns of a [`NaturalMatrix`] modulo $2^k$, taking
    /// it by value, without normalizing them; or returns `None` if a squared length that it needs
    /// to divide by is even.
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
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(hw^2 k \log k \log\log k)$
    ///
    /// $M(h, w, k) = O(hwk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2GramSchmidt;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_gram_schmidt(3).unwrap().to_string(),
    ///     "[1, 6; 2, 5]"
    /// );
    ///
    /// // (2, 2) is orthogonal to itself modulo 8
    /// let a = NaturalMatrix::from_str("[2, 1; 2, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_gram_schmidt(3), None);
    ///
    /// // (1, 1) has squared length 2, which is even
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
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

impl ModPowerOf2GramSchmidt for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Applies the Gram-Schmidt process to the columns of a [`NaturalMatrix`] modulo $2^k$, taking
    /// it by reference, without normalizing them; or returns `None` if a squared length that it
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
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(h, w, k) = O(hw^2 k \log k \log\log k)$
    ///
    /// $M(h, w, k) = O(hwk)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2GramSchmidt;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 0; 2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_gram_schmidt(3).unwrap().to_string(),
    ///     "[1, 6; 2, 5]"
    /// );
    ///
    /// // (2, 2) is orthogonal to itself modulo 8
    /// let a = NaturalMatrix::from_str("[2, 1; 2, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_gram_schmidt(3), None);
    ///
    /// // (1, 1) has squared length 2, which is even
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_gram_schmidt(3), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a power of 2; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_power_of_2_gram_schmidt(self, pow: u64) -> Option<NaturalMatrix> {
        mod_power_of_2_gram_schmidt_helper(self, pow)
    }
}
