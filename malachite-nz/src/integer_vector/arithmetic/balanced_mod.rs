// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModAssign};

impl BalancedMod<Integer> for IntegerVector {
    type Output = Self;

    /// Reduces every element of an [`IntegerVector`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the vector by value and the modulus by value.
    ///
    /// Each element $w_i$ of the result satisfies $-|m|/2 < w_i \leq |m|/2$ and $w_i \equiv v_i
    /// \bmod m$, which determine it uniquely, as with [`BalancedMod`] for [`Integer`]s. A remainder
    /// of exactly $|m|/2$ is positive, and only the magnitude of $m$ matters. The dimension is
    /// unchanged: unlike a polynomial, a vector keeps the elements that reduce to zero.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let v = IntegerVector::from_str("(1, 27, -23, -5)").unwrap();
    /// assert_eq!(
    ///     v.clone().balanced_mod(Integer::from(10)).to_string(),
    ///     "(1, -3, -3, 5)"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let v = IntegerVector::from_str("(10, 7)").unwrap();
    /// assert_eq!(
    ///     v.clone().balanced_mod(Integer::from(-10)).to_string(),
    ///     "(0, -3)"
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

impl<'a> BalancedMod<&'a Integer> for IntegerVector {
    type Output = Self;

    /// Reduces every element of an [`IntegerVector`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the vector by value and the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerVector`] that takes
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let v = IntegerVector::from_str("(1, 27, -23, -5)").unwrap();
    /// assert_eq!(
    ///     v.clone().balanced_mod(&Integer::from(10)).to_string(),
    ///     "(1, -3, -3, 5)"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let v = IntegerVector::from_str("(10, 7)").unwrap();
    /// assert_eq!(
    ///     v.clone().balanced_mod(&Integer::from(-10)).to_string(),
    ///     "(0, -3)"
    /// );
    /// ```
    #[inline]
    fn balanced_mod(mut self, m: &'a Integer) -> Self {
        self.balanced_mod_assign(m);
        self
    }
}

impl BalancedMod<Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Reduces every element of an [`IntegerVector`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the vector by reference and the modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerVector`] that takes
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let v = IntegerVector::from_str("(1, 27, -23, -5)").unwrap();
    /// assert_eq!(
    ///     (&v).balanced_mod(Integer::from(10)).to_string(),
    ///     "(1, -3, -3, 5)"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let v = IntegerVector::from_str("(10, 7)").unwrap();
    /// assert_eq!((&v).balanced_mod(Integer::from(-10)).to_string(), "(0, -3)");
    /// ```
    #[inline]
    fn balanced_mod(self, m: Integer) -> IntegerVector {
        self.balanced_mod(&m)
    }
}

impl<'a> BalancedMod<&'a Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Reduces every element of an [`IntegerVector`] modulo an [`Integer`] to the representative
    /// closest to zero, taking the vector by reference and the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerVector`] that takes
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// // Each element goes to the representative closest to zero, and half the modulus is
    /// // positive.
    /// let v = IntegerVector::from_str("(1, 27, -23, -5)").unwrap();
    /// assert_eq!(
    ///     (&v).balanced_mod(&Integer::from(10)).to_string(),
    ///     "(1, -3, -3, 5)"
    /// );
    ///
    /// // Only the modulus's magnitude matters, and an element that reduces to zero stays.
    /// let v = IntegerVector::from_str("(10, 7)").unwrap();
    /// assert_eq!(
    ///     (&v).balanced_mod(&Integer::from(-10)).to_string(),
    ///     "(0, -3)"
    /// );
    /// ```
    fn balanced_mod(self, m: &'a Integer) -> IntegerVector {
        assert_ne!(*m, 0u32, "division by zero");
        IntegerVector {
            elements: self.elements.iter().map(|x| x.balanced_mod(m)).collect(),
        }
    }
}

impl BalancedModAssign<Integer> for IntegerVector {
    /// Reduces every element of an [`IntegerVector`] modulo an [`Integer`] to the representative
    /// closest to zero, in place, taking the modulus by value.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerVector`] that takes
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
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, 27, -23, -5)").unwrap();
    /// v.balanced_mod_assign(Integer::from(10));
    /// assert_eq!(v.to_string(), "(1, -3, -3, 5)");
    ///
    /// let mut v = IntegerVector::from_str("(10, 7)").unwrap();
    /// v.balanced_mod_assign(Integer::from(-10));
    /// assert_eq!(v.to_string(), "(0, -3)");
    /// ```
    #[inline]
    fn balanced_mod_assign(&mut self, m: Integer) {
        self.balanced_mod_assign(&m);
    }
}

impl<'a> BalancedModAssign<&'a Integer> for IntegerVector {
    /// Reduces every element of an [`IntegerVector`] modulo an [`Integer`] to the representative
    /// closest to zero, in place, taking the modulus by reference.
    ///
    /// See the documentation for the [`BalancedMod`] implementation on [`IntegerVector`] that takes
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
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, 27, -23, -5)").unwrap();
    /// v.balanced_mod_assign(&Integer::from(10));
    /// assert_eq!(v.to_string(), "(1, -3, -3, 5)");
    ///
    /// let mut v = IntegerVector::from_str("(10, 7)").unwrap();
    /// v.balanced_mod_assign(&Integer::from(-10));
    /// assert_eq!(v.to_string(), "(0, -3)");
    /// ```
    fn balanced_mod_assign(&mut self, m: &'a Integer) {
        assert_ne!(*m, 0u32, "division by zero");
        for x in &mut self.elements {
            x.balanced_mod_assign(m);
        }
    }
}
