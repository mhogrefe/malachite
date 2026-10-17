// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModAssign};

impl BalancedMod<Integer> for IntegerMatrix {
    type Output = Self;

    /// Reduces every element of an [`IntegerMatrix`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the matrix by value and the modulus by value.
    ///
    /// Each element $B_{ij}$ of the result satisfies $-|m|/2 < B_{ij} \leq |m|/2$ and $B_{ij}
    /// \equiv A_{ij} \bmod m$, which determine it uniquely, as with [`BalancedMod`] for
    /// [`Integer`]s. A remainder of exactly $|m|/2$ is positive, and only the magnitude of $m$
    /// matters. The height and width are unchanged, and elements that reduce to zero stay.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let a = IntegerMatrix::from_str("[1, 27, -23, -5]").unwrap();
    /// assert_eq!(
    ///     a.clone().balanced_mod(Integer::from(10)).to_string(),
    ///     "[1, -3, -3, 5]"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let a = IntegerMatrix::from_str("[10, 7]").unwrap();
    /// assert_eq!(
    ///     a.clone().balanced_mod(Integer::from(-10)).to_string(),
    ///     "[0, -3]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_smod_fmpz` from `fmpz_vec/scalar_smod.c`, FLINT
    /// 3.6.0, for a positive modulus.
    #[inline]
    fn balanced_mod(mut self, m: Integer) -> Self {
        self.balanced_mod_assign(m);
        self
    }
}

impl<'a> BalancedMod<&'a Integer> for IntegerMatrix {
    type Output = Self;

    /// Reduces every element of an [`IntegerMatrix`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the matrix by value and the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerMatrix`] that takes
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let a = IntegerMatrix::from_str("[1, 27, -23, -5]").unwrap();
    /// assert_eq!(
    ///     a.clone().balanced_mod(&Integer::from(10)).to_string(),
    ///     "[1, -3, -3, 5]"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let a = IntegerMatrix::from_str("[10, 7]").unwrap();
    /// assert_eq!(
    ///     a.clone().balanced_mod(&Integer::from(-10)).to_string(),
    ///     "[0, -3]"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(mut self, m: &'a Integer) -> Self {
        self.balanced_mod_assign(m);
        self
    }
}

impl BalancedMod<Integer> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Reduces every element of an [`IntegerMatrix`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the matrix by reference and the modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerMatrix`] that takes
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let a = IntegerMatrix::from_str("[1, 27, -23, -5]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_mod(Integer::from(10)).to_string(),
    ///     "[1, -3, -3, 5]"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let a = IntegerMatrix::from_str("[10, 7]").unwrap();
    /// assert_eq!((&a).balanced_mod(Integer::from(-10)).to_string(), "[0, -3]");
    /// ```
    #[inline]
    fn balanced_mod(self, m: Integer) -> IntegerMatrix {
        self.balanced_mod(&m)
    }
}

impl<'a> BalancedMod<&'a Integer> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Reduces every element of an [`IntegerMatrix`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the matrix by reference and the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerMatrix`] that takes
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let a = IntegerMatrix::from_str("[1, 27, -23, -5]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_mod(&Integer::from(10)).to_string(),
    ///     "[1, -3, -3, 5]"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let a = IntegerMatrix::from_str("[10, 7]").unwrap();
    /// assert_eq!(
    ///     (&a).balanced_mod(&Integer::from(-10)).to_string(),
    ///     "[0, -3]"
    /// );
    /// ```
    fn balanced_mod(self, m: &'a Integer) -> IntegerMatrix {
        assert_ne!(*m, 0u32, "division by zero");
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements.iter().map(|x| x.balanced_mod(m)).collect(),
        )
    }
}

impl BalancedModAssign<Integer> for IntegerMatrix {
    /// Reduces every element of an [`IntegerMatrix`] modulo an [`Integer`] to the representative
    /// closest to zero, in place, taking the modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerMatrix`] that takes
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
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 27, -23, -5]").unwrap();
    /// a.balanced_mod_assign(Integer::from(10));
    /// assert_eq!(a.to_string(), "[1, -3, -3, 5]");
    ///
    /// let mut a = IntegerMatrix::from_str("[10, 7]").unwrap();
    /// a.balanced_mod_assign(Integer::from(-10));
    /// assert_eq!(a.to_string(), "[0, -3]");
    /// ```
    #[inline]
    fn balanced_mod_assign(&mut self, m: Integer) {
        self.balanced_mod_assign(&m);
    }
}

impl<'a> BalancedModAssign<&'a Integer> for IntegerMatrix {
    /// Reduces every element of an [`IntegerMatrix`] modulo an [`Integer`] to the representative
    /// closest to zero, in place, taking the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerMatrix`] that takes
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
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 27, -23, -5]").unwrap();
    /// a.balanced_mod_assign(&Integer::from(10));
    /// assert_eq!(a.to_string(), "[1, -3, -3, 5]");
    ///
    /// let mut a = IntegerMatrix::from_str("[10, 7]").unwrap();
    /// a.balanced_mod_assign(&Integer::from(-10));
    /// assert_eq!(a.to_string(), "[0, -3]");
    /// ```
    fn balanced_mod_assign(&mut self, m: &'a Integer) {
        assert_ne!(*m, 0u32, "division by zero");
        for x in &mut self.elements {
            x.balanced_mod_assign(m);
        }
    }
}
