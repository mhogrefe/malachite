// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign, NegAssign};

impl Conjugate for GaussianInteger {
    type Output = Self;

    /// Computes the complex conjugate of a [`GaussianInteger`], taking it by value. The sign of the
    /// imaginary part is flipped.
    ///
    /// $$
    /// f(x) = \overline{x} = \Re(x) - \Im(x) i.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Conjugate;
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::I.conjugate().to_string(), "-i");
    /// assert_eq!(
    ///     GaussianInteger::from_str("2-3i")
    ///         .unwrap()
    ///         .conjugate()
    ///         .to_string(),
    ///     "2+3i"
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("-123")
    ///         .unwrap()
    ///         .conjugate()
    ///         .to_string(),
    ///     "-123"
    /// );
    /// ```
    fn conjugate(mut self) -> Self {
        self.conjugate_assign();
        self
    }
}

impl Conjugate for &GaussianInteger {
    type Output = GaussianInteger;

    /// Computes the complex conjugate of a [`GaussianInteger`], taking it by reference. The sign of
    /// the imaginary part is flipped.
    ///
    /// $$
    /// f(x) = \overline{x} = \Re(x) - \Im(x) i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Conjugate;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!((&x).conjugate().to_string(), "2+3i");
    /// ```
    fn conjugate(self) -> GaussianInteger {
        GaussianInteger {
            real: self.real.clone(),
            imaginary: -&self.imaginary,
        }
    }
}

impl ConjugateAssign for GaussianInteger {
    /// Replaces a [`GaussianInteger`] with its complex conjugate. The sign of the imaginary part is
    /// flipped.
    ///
    /// $$
    /// x \gets \overline{x} = \Re(x) - \Im(x) i.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ConjugateAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianInteger::from_str("2-3i").unwrap();
    /// x.conjugate_assign();
    /// assert_eq!(x.to_string(), "2+3i");
    /// ```
    #[inline]
    fn conjugate_assign(&mut self) {
        self.imaginary.neg_assign();
    }
}
