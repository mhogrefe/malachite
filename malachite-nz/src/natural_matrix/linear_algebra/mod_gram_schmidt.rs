// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::{Natural, TWICE_WIDTH};
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::linear_algebra::mod_lu_decomposition::assert_reduced;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{ModInverse, ModMul, ModSubAssign};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::linear_algebra::gram_schmidt_over_ring::{
    GramSchmidtRing, gram_schmidt_in_ring,
};

// The integers modulo a positive `m` that does not fit in two limbs. Dot products are summed
// exactly and reduced once.
pub(crate) struct NaturalModRing<'a> {
    pub(crate) m: &'a Natural,
}

impl GramSchmidtRing for NaturalModRing<'_> {
    type Element = Natural;

    #[inline]
    fn is_zero(&self, x: &Natural) -> bool {
        *x == 0u32
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_mul(y, self.m)
    }

    #[inline]
    fn inverse(&self, x: &Natural) -> Option<Natural> {
        x.mod_inverse(self.m)
    }

    fn dot(&self, xs: &[Natural], ys: &[Natural]) -> Natural {
        let mut sum = Natural::ZERO;
        for (x, y) in xs.iter().zip(ys) {
            sum.add_mul_assign(x, y);
        }
        sum % self.m
    }

    fn sub_mul_assign(&self, xs: &mut [Natural], c: &Natural, ys: &[Natural]) {
        for (x, y) in xs.iter_mut().zip(ys) {
            x.mod_sub_assign(c.mod_mul(y, self.m), self.m);
        }
    }
}

// The Gram-Schmidt process applied to the columns of `a` modulo `m`: in single limbs when `m` fits
// in one, in double limbs when it fits in two, and otherwise in `Natural`s.
fn mod_gram_schmidt_helper(a: &NaturalMatrix, m: &Natural) -> Option<NaturalMatrix> {
    assert_reduced(a, m);
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        let elements: Vec<Limb> = a.elements.iter().map(Limb::exact_from).collect();
        Some(NaturalMatrix::from(
            UnsignedMatrix::from_owned_elements(a.height(), a.width(), elements)
                .mod_gram_schmidt(Limb::exact_from(m))?,
        ))
    } else if bits <= TWICE_WIDTH {
        let elements: Vec<DoubleLimb> = a.elements.iter().map(DoubleLimb::exact_from).collect();
        Some(NaturalMatrix::from(
            UnsignedMatrix::from_owned_elements(a.height(), a.width(), elements)
                .mod_gram_schmidt(DoubleLimb::exact_from(m))?,
        ))
    } else {
        Some(NaturalMatrix {
            elements: gram_schmidt_in_ring(&NaturalModRing { m }, &a.elements, a.height, a.width)?,
            height: a.height,
            width: a.width,
        })
    }
}

impl ModGramSchmidt<Natural> for NaturalMatrix {
    type Output = Self;

    /// Applies the Gram-Schmidt process to the columns of a [`NaturalMatrix`] modulo $m$, taking
    /// the matrix by value and the modulus by value, without normalizing them; or returns `None` if
    /// a squared length that it needs to divide by is not invertible modulo $m$.
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
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw^2 b \log b \log\log b + w b (\log b)^2 \log\log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_gram_schmidt(Natural::from(5u32)).unwrap().to_string(),
    ///     "[1, 3; 1, 2]"
    /// );
    ///
    /// // (1, 1) is orthogonal to itself modulo 2
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_gram_schmidt(Natural::TWO), None);
    ///
    /// // (1, 1) has squared length 2, which is not invertible modulo 6
    /// let a = NaturalMatrix::from_str("[1, 0; 1, 1]").unwrap();
    /// assert_eq!(a.mod_gram_schmidt(Natural::from(6u32)), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a number; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_gram_schmidt(self, m: Natural) -> Option<Self> {
        mod_gram_schmidt_helper(&self, &m)
    }
}

impl ModGramSchmidt<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Applies the Gram-Schmidt process to the columns of a [`NaturalMatrix`] modulo $m$, taking
    /// the matrix by value and the modulus by reference, without normalizing them; or returns
    /// `None` if a squared length that it needs to divide by is not invertible modulo $m$.
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
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw^2 b \log b \log\log b + w b (\log b)^2 \log\log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_gram_schmidt(&Natural::from(5u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[1, 3; 1, 2]"
    /// );
    ///
    /// // (1, 1) is orthogonal to itself modulo 2
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.mod_gram_schmidt(&Natural::TWO), None);
    ///
    /// // (1, 1) has squared length 2, which is not invertible modulo 6
    /// let a = NaturalMatrix::from_str("[1, 0; 1, 1]").unwrap();
    /// assert_eq!(a.mod_gram_schmidt(&Natural::from(6u32)), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a number; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_gram_schmidt(self, m: &Natural) -> Option<Self> {
        mod_gram_schmidt_helper(&self, m)
    }
}

impl ModGramSchmidt<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Applies the Gram-Schmidt process to the columns of a [`NaturalMatrix`] modulo $m$, taking
    /// the matrix by reference and the modulus by value, without normalizing them; or returns
    /// `None` if a squared length that it needs to divide by is not invertible modulo $m$.
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
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw^2 b \log b \log\log b + w b (\log b)^2 \log\log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_gram_schmidt(Natural::from(5u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[1, 3; 1, 2]"
    /// );
    ///
    /// // (1, 1) is orthogonal to itself modulo 2
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).mod_gram_schmidt(Natural::TWO), None);
    ///
    /// // (1, 1) has squared length 2, which is not invertible modulo 6
    /// let a = NaturalMatrix::from_str("[1, 0; 1, 1]").unwrap();
    /// assert_eq!((&a).mod_gram_schmidt(Natural::from(6u32)), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a number; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_gram_schmidt(self, m: Natural) -> Option<NaturalMatrix> {
        mod_gram_schmidt_helper(self, &m)
    }
}

impl ModGramSchmidt<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Applies the Gram-Schmidt process to the columns of a [`NaturalMatrix`] modulo $m$, taking
    /// the matrix by reference and the modulus by reference, without normalizing them; or returns
    /// `None` if a squared length that it needs to divide by is not invertible modulo $m$.
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
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedMatrix`].
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw^2 b \log b \log\log b + w b (\log b)^2 \log\log b)$
    ///
    /// $M(h, w, b) = O(hwb)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::num::linear_algebra::traits::ModGramSchmidt;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_gram_schmidt(&Natural::from(5u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[1, 3; 1, 2]"
    /// );
    ///
    /// // (1, 1) is orthogonal to itself modulo 2
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).mod_gram_schmidt(&Natural::TWO), None);
    ///
    /// // (1, 1) has squared length 2, which is not invertible modulo 6
    /// let a = NaturalMatrix::from_str("[1, 0; 1, 1]").unwrap();
    /// assert_eq!((&a).mod_gram_schmidt(&Natural::from(6u32)), None);
    /// ```
    ///
    /// FLINT has no Gram-Schmidt process modulo a number; this follows `fmpq_mat_gso` from
    /// `fmpq_mat/gso.c`, FLINT 3.6.0, which works over the rationals.
    #[inline]
    fn mod_gram_schmidt(self, m: &Natural) -> Option<NaturalMatrix> {
        mod_gram_schmidt_helper(self, m)
    }
}
