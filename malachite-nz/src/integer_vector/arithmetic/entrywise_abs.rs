// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, EntrywiseAbs, EntrywiseAbsAssign, EntrywiseUnsignedAbs, UnsignedAbs,
};

impl EntrywiseAbs for IntegerVector {
    type Output = Self;

    /// Replaces every element of an [`IntegerVector`] by its absolute value, taking the vector by
    /// value.
    ///
    /// The dimension is unchanged. This is not [`Abs`], which for a vector would be its Euclidean
    /// length. For the result as a [`NaturalVector`], use [`EntrywiseUnsignedAbs`].
    ///
    /// $$
    /// f(v) = (|v_0|, |v_1|, \ldots, |v_{n-1}|).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseAbs;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 0, -3)").unwrap();
    /// assert_eq!(v.clone().entrywise_abs().to_string(), "(1, 2, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_abs` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn entrywise_abs(mut self) -> Self {
        self.entrywise_abs_assign();
        self
    }
}

impl EntrywiseAbs for &IntegerVector {
    type Output = IntegerVector;

    /// Replaces every element of an [`IntegerVector`] by its absolute value, taking the vector by
    /// reference.
    ///
    /// See the documentation for the [`EntrywiseAbs`] implementation on [`IntegerVector`] that
    /// takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseAbs;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 0, -3)").unwrap();
    /// assert_eq!((&v).entrywise_abs().to_string(), "(1, 2, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_abs` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn entrywise_abs(self) -> IntegerVector {
        IntegerVector {
            elements: self.elements.iter().map(Abs::abs).collect(),
        }
    }
}

impl EntrywiseAbsAssign for IntegerVector {
    /// Replaces every element of an [`IntegerVector`] by its absolute value, in place.
    ///
    /// See the documentation for the [`EntrywiseAbs`] implementation on [`IntegerVector`] that
    /// takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseAbsAssign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 0, -3)").unwrap();
    /// v.entrywise_abs_assign();
    /// assert_eq!(v.to_string(), "(1, 2, 0, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_scalar_abs` from `fmpz_vec/scalar.c`, FLINT 3.6.0, with
    /// `vec1` equal to `vec2`.
    fn entrywise_abs_assign(&mut self) {
        for x in &mut self.elements {
            x.abs_assign();
        }
    }
}

impl EntrywiseUnsignedAbs for IntegerVector {
    type Output = NaturalVector;

    /// Replaces every element of an [`IntegerVector`] by its absolute value, returning the result
    /// as a [`NaturalVector`] and taking the vector by value.
    ///
    /// Every absolute value is non-negative, so the result fits in a [`NaturalVector`], as
    /// [`UnsignedAbs`] gives a [`Natural`](crate::natural::Natural) for a single [`Integer`]. The
    /// dimension is unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseUnsignedAbs;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 0, -3)").unwrap();
    /// assert_eq!(
    ///     v.clone().entrywise_unsigned_abs().to_string(),
    ///     "(1, 2, 0, 3)"
    /// );
    /// ```
    fn entrywise_unsigned_abs(self) -> NaturalVector {
        NaturalVector {
            elements: self
                .elements
                .into_iter()
                .map(Integer::unsigned_abs)
                .collect(),
        }
    }
}

impl EntrywiseUnsignedAbs for &IntegerVector {
    type Output = NaturalVector;

    /// Replaces every element of an [`IntegerVector`] by its absolute value, returning the result
    /// as a [`NaturalVector`] and taking the vector by reference.
    ///
    /// See the documentation for the [`EntrywiseUnsignedAbs`] implementation on [`IntegerVector`]
    /// that takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseUnsignedAbs;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -2, 0, -3)").unwrap();
    /// assert_eq!((&v).entrywise_unsigned_abs().to_string(), "(1, 2, 0, 3)");
    /// ```
    fn entrywise_unsigned_abs(self) -> NaturalVector {
        NaturalVector {
            elements: self
                .elements
                .iter()
                .map(UnsignedAbs::unsigned_abs)
                .collect(),
        }
    }
}
