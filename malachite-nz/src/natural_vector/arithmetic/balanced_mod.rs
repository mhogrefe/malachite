// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::BalancedMod;

impl BalancedMod<Natural> for NaturalVector {
    type Output = IntegerVector;

    /// Reduces every element of a [`NaturalVector`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerVector`], taking the vector by value and the modulus
    /// by value.
    ///
    /// Each element $w_i$ of the result satisfies $-m/2 < w_i \leq m/2$ and $w_i \equiv v_i \bmod
    /// m$, which determine it uniquely, as with [`BalancedMod`] for [`Natural`]s. A remainder of
    /// exactly $m/2$ is positive. Remainders above $m/2$ become negative, which is why the result
    /// is an [`IntegerVector`]. The dimension is unchanged.
    ///
    /// Applied to a vector of residues modulo $m$, this gives their balanced representatives. An
    /// [`UnsignedVector`](malachite_base::unsigned_vector::UnsignedVector) of residues can be
    /// converted to a [`NaturalVector`] with [`From`] first.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let v = NaturalVector::from_str("(1, 27, 23, 5, 10)").unwrap();
    /// assert_eq!(
    ///     v.clone().balanced_mod(Natural::from(10u32)).to_string(),
    ///     "(1, -3, 3, 5, 0)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_set_nmod_vec` from `fmpz_vec/set_nmod_vec.c`, FLINT 3.6.0,
    /// for elements less than the modulus.
    #[inline]
    fn balanced_mod(self, m: Natural) -> IntegerVector {
        (&self).balanced_mod(&m)
    }
}

impl<'a> BalancedMod<&'a Natural> for NaturalVector {
    type Output = IntegerVector;

    /// Reduces every element of a [`NaturalVector`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerVector`], taking the vector by value and the modulus
    /// by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`NaturalVector`] that takes
    /// both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let v = NaturalVector::from_str("(1, 27, 23, 5, 10)").unwrap();
    /// assert_eq!(
    ///     v.clone().balanced_mod(&Natural::from(10u32)).to_string(),
    ///     "(1, -3, 3, 5, 0)"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(self, m: &'a Natural) -> IntegerVector {
        (&self).balanced_mod(m)
    }
}

impl BalancedMod<Natural> for &NaturalVector {
    type Output = IntegerVector;

    /// Reduces every element of a [`NaturalVector`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerVector`], taking the vector by reference and the
    /// modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`NaturalVector`] that takes
    /// both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let v = NaturalVector::from_str("(1, 27, 23, 5, 10)").unwrap();
    /// assert_eq!(
    ///     (&v).balanced_mod(Natural::from(10u32)).to_string(),
    ///     "(1, -3, 3, 5, 0)"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(self, m: Natural) -> IntegerVector {
        self.balanced_mod(&m)
    }
}

impl<'a> BalancedMod<&'a Natural> for &NaturalVector {
    type Output = IntegerVector;

    /// Reduces every element of a [`NaturalVector`] modulo a [`Natural`] to the representative
    /// closest to zero, returning an [`IntegerVector`], taking the vector by reference and the
    /// modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`NaturalVector`] that takes
    /// both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// vector's elements.
    ///
    /// # Panics
    /// Panics if `m` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus stays
    /// // positive.
    /// let v = NaturalVector::from_str("(1, 27, 23, 5, 10)").unwrap();
    /// assert_eq!(
    ///     (&v).balanced_mod(&Natural::from(10u32)).to_string(),
    ///     "(1, -3, 3, 5, 0)"
    /// );
    /// ```
    fn balanced_mod(self, m: &'a Natural) -> IntegerVector {
        assert_ne!(*m, 0u32, "division by zero");
        IntegerVector {
            elements: self.elements.iter().map(|x| x.balanced_mod(m)).collect(),
        }
    }
}
