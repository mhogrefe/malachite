// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};

impl Conjugate for Natural {
    type Output = Self;

    /// Computes the complex conjugate of a [`Natural`], taking it by value. A real number is its
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).conjugate(), Natural::from(123u32));
    /// ```
    #[inline]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugate for &Natural {
    type Output = Natural;

    /// Computes the complex conjugate of a [`Natural`], taking it by reference. A real number is
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(123u32)).conjugate(), Natural::from(123u32));
    /// ```
    #[inline]
    fn conjugate(self) -> Natural {
        self.clone()
    }
}

impl ConjugateAssign for Natural {
    /// Replaces a [`Natural`] with its complex conjugate. A real number is its own conjugate, so
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
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(123u32);
    /// x.conjugate_assign();
    /// assert_eq!(x, Natural::from(123u32));
    /// ```
    #[inline]
    fn conjugate_assign(&mut self) {}
}
