// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;

impl Neg for RationalVector {
    type Output = Self;

    /// Negates a [`RationalVector`], taking it by value.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -3, 2/3)").unwrap();
    /// assert_eq!((-v).to_string(), "(-1/2, 3, -2/3)");
    /// assert_eq!((-RationalVector::from_str("()").unwrap()).to_string(), "()");
    /// ```
    #[inline]
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }
}

impl Neg for &RationalVector {
    type Output = RationalVector;

    /// Negates a [`RationalVector`], taking it by reference.
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
    /// elements' numerators and denominators.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -3, 2/3)").unwrap();
    /// assert_eq!((-&v).to_string(), "(-1/2, 3, -2/3)");
    /// ```
    #[inline]
    fn neg(self) -> RationalVector {
        RationalVector {
            elements: self.elements.iter().map(|x| -x).collect(),
        }
    }
}

impl NegAssign for RationalVector {
    /// Negates a [`RationalVector`] in place.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -3, 2/3)").unwrap();
    /// v.neg_assign();
    /// assert_eq!(v.to_string(), "(-1/2, 3, -2/3)");
    /// ```
    #[inline]
    fn neg_assign(&mut self) {
        for x in &mut self.elements {
            x.neg_assign();
        }
    }
}
