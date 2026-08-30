// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};

impl Conjugate for Integer {
    type Output = Self;

    /// Computes the complex conjugate of a [`Integer`], taking it by value. A real number is its
    /// own conjugate, so this is the identity.
    ///
    /// $$
    /// f(x) = \overline{x} = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Conjugate;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-123).conjugate(), Integer::from(-123));
    /// ```
    #[inline]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugate for &Integer {
    type Output = Integer;

    /// Computes the complex conjugate of a [`Integer`], taking it by reference. A real number is
    /// its own conjugate, so this just clones.
    ///
    /// $$
    /// f(x) = \overline{x} = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Conjugate;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-123)).conjugate(), Integer::from(-123));
    /// ```
    #[inline]
    fn conjugate(self) -> Integer {
        self.clone()
    }
}

impl ConjugateAssign for Integer {
    /// Replaces a [`Integer`] with its complex conjugate. A real number is its own conjugate, so
    /// this does nothing.
    ///
    /// $$
    /// x \gets \overline{x} = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ConjugateAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-123);
    /// x.conjugate_assign();
    /// assert_eq!(x, Integer::from(-123));
    /// ```
    #[inline]
    fn conjugate_assign(&mut self) {}
}
