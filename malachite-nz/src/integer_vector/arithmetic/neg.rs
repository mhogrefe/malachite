// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;

impl Neg for IntegerVector {
    type Output = Self;

    /// Negates an [`IntegerVector`], taking it by value.
    ///
    /// Every element is negated, so the dimension is unchanged.
    ///
    /// $$
    /// f(v) = -v.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -3, 2)").unwrap();
    /// assert_eq!((-v).to_string(), "(-1, 3, -2)");
    /// assert_eq!((-IntegerVector::from_str("()").unwrap()).to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_neg` from `fmpz_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }
}

impl Neg for &IntegerVector {
    type Output = IntegerVector;

    /// Negates an [`IntegerVector`], taking it by reference.
    ///
    /// Every element is negated, so the dimension is unchanged.
    ///
    /// $$
    /// f(v) = -v.
    /// $$
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, -3, 2)").unwrap();
    /// assert_eq!((-&v).to_string(), "(-1, 3, -2)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_neg` from `fmpz_vec/neg.c`, FLINT 3.6.0.
    #[inline]
    fn neg(self) -> IntegerVector {
        IntegerVector {
            elements: self.elements.iter().map(|x| -x).collect(),
        }
    }
}

impl NegAssign for IntegerVector {
    /// Negates an [`IntegerVector`] in place.
    ///
    /// $$
    /// v \gets -v.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -3, 2)").unwrap();
    /// v.neg_assign();
    /// assert_eq!(v.to_string(), "(-1, 3, -2)");
    /// ```
    #[inline]
    fn neg_assign(&mut self) {
        for x in &mut self.elements {
            x.neg_assign();
        }
    }
}
