// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};

impl Conjugate for Float {
    type Output = Self;

    /// Computes the complex conjugate of a [`Float`], taking it by value. A real number is its own
    /// conjugate, so this is the identity; even `NaN` is returned unchanged.
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
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(
    ///     ComparableFloat(Float::from(-1.5).conjugate()),
    ///     ComparableFloat(Float::from(-1.5))
    /// );
    /// ```
    #[inline]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugate for &Float {
    type Output = Float;

    /// Computes the complex conjugate of a [`Float`], taking it by reference. A real number is its
    /// own conjugate, so this just clones; even `NaN` is returned unchanged.
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
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(
    ///     ComparableFloat((&Float::from(-1.5)).conjugate()),
    ///     ComparableFloat(Float::from(-1.5))
    /// );
    /// ```
    #[inline]
    fn conjugate(self) -> Float {
        self.clone()
    }
}

impl ConjugateAssign for Float {
    /// Replaces a [`Float`] with its complex conjugate. A real number is its own conjugate, so this
    /// does nothing.
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
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// let mut x = Float::from(-1.5);
    /// x.conjugate_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::from(-1.5)));
    /// ```
    #[inline]
    fn conjugate_assign(&mut self) {}
}
