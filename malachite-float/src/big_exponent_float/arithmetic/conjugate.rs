// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};

impl Conjugate for BigExponentFloat {
    type Output = Self;

    /// Computes the complex conjugate of a [`BigExponentFloat`], taking it by value. A real number
    /// is its own conjugate, so this is the identity; even `NaN` is returned unchanged.
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
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(-1.5).conjugate(),
    ///     BigExponentFloat::from(-1.5)
    /// );
    /// ```
    #[inline]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugate for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Computes the complex conjugate of a [`BigExponentFloat`], taking it by reference. A real
    /// number is its own conjugate, so this just clones; even `NaN` is returned unchanged.
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
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     (&BigExponentFloat::from(-1.5)).conjugate(),
    ///     BigExponentFloat::from(-1.5)
    /// );
    /// ```
    #[inline]
    fn conjugate(self) -> BigExponentFloat {
        self.clone()
    }
}

impl ConjugateAssign for BigExponentFloat {
    /// Replaces a [`BigExponentFloat`] with its complex conjugate. A real number is its own
    /// conjugate, so this does nothing.
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
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(-1.5);
    /// x.conjugate_assign();
    /// assert_eq!(x, BigExponentFloat::from(-1.5));
    /// ```
    #[inline]
    fn conjugate_assign(&mut self) {}
}
