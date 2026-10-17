// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::BalancedMod;

impl BalancedMod<Natural> for NaturalMatrix {
    type Output = IntegerMatrix;

    /// Reduces every element of a [`NaturalMatrix`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerMatrix`], taking the matrix by value and the modulus
    /// by value.
    ///
    /// Each element $B_{ij}$ of the result satisfies $-m/2 < B_{ij} \leq m/2$ and $B_{ij} \equiv
    /// A_{ij} \bmod m$, which determine it uniquely, as with [`BalancedMod`] for [`Natural`]s. A
    /// remainder of exactly $m/2$ is positive. Remainders above $m/2$ become negative, which is why
    /// the result is an [`IntegerMatrix`]. The height and width are unchanged.
    ///
    /// Applied to a matrix of residues modulo $m$, this gives their balanced representatives. An
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix) of residues can be
    /// converted to a [`NaturalMatrix`] with [`From`] first.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// matrix's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let a = NaturalMatrix::from_str("[1, 27, 23, 5, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().balanced_mod(Natural::from(10u32)).to_string(),
    ///     "[1, -3, 3, 5, 0]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_set_nmod_vec` from `fmpz_vec/set_nmod_vec.c`, FLINT 3.6.0,
    /// for elements less than the modulus.
    #[inline]
    fn balanced_mod(self, m: Natural) -> IntegerMatrix {
        (&self).balanced_mod(&m)
    }
}

impl<'a> BalancedMod<&'a Natural> for NaturalMatrix {
    type Output = IntegerMatrix;

    /// Reduces every element of a [`NaturalMatrix`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerMatrix`], taking the matrix by value and the modulus
    /// by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`NaturalMatrix`] that takes
    /// both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// matrix's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let a = NaturalMatrix::from_str("[1, 27, 23, 5, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().balanced_mod(&Natural::from(10u32)).to_string(),
    ///     "[1, -3, 3, 5, 0]"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(self, m: &'a Natural) -> IntegerMatrix {
        (&self).balanced_mod(m)
    }
}

impl BalancedMod<Natural> for &NaturalMatrix {
    type Output = IntegerMatrix;

    /// Reduces every element of a [`NaturalMatrix`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerMatrix`], taking the matrix by reference and the
    /// modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`NaturalMatrix`] that takes
    /// both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// matrix's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let a = NaturalMatrix::from_str("[1, 27, 23, 5, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_mod(Natural::from(10u32)).to_string(),
    ///     "[1, -3, 3, 5, 0]"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(self, m: Natural) -> IntegerMatrix {
        self.balanced_mod(&m)
    }
}

impl<'a> BalancedMod<&'a Natural> for &NaturalMatrix {
    type Output = IntegerMatrix;

    /// Reduces every element of a [`NaturalMatrix`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerMatrix`], taking the matrix by reference and the
    /// modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`NaturalMatrix`] that takes
    /// both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// matrix's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let a = NaturalMatrix::from_str("[1, 27, 23, 5, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_mod(&Natural::from(10u32)).to_string(),
    ///     "[1, -3, 3, 5, 0]"
    /// );
    /// ```
    fn balanced_mod(self, m: &'a Natural) -> IntegerMatrix {
        assert_ne!(*m, 0u32, "division by zero");
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements.iter().map(|x| x.balanced_mod(m)).collect(),
        )
    }
}
