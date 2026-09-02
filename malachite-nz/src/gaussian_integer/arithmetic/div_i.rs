// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{DivI, DivIAssign, NegAssign};

impl DivI for GaussianInteger {
    type Output = Self;

    /// Divides a [`GaussianInteger`] by $i$, taking it by value. This is a clockwise quarter turn.
    ///
    /// $$
    /// f(a + bi) = (a + bi)/i = b - ai.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivI;
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::I.div_i().to_string(), "1");
    /// assert_eq!(
    ///     GaussianInteger::from_str("2-3i")
    ///         .unwrap()
    ///         .div_i()
    ///         .to_string(),
    ///     "-3-2i"
    /// );
    /// ```
    #[inline]
    fn div_i(mut self) -> Self {
        self.div_i_assign();
        self
    }
}

impl DivI for &GaussianInteger {
    type Output = GaussianInteger;

    /// Divides a [`GaussianInteger`] by $i$, taking it by reference. This is a clockwise quarter
    /// turn.
    ///
    /// $$
    /// f(a + bi) = (a + bi)/i = b - ai.
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
    /// use malachite_base::num::arithmetic::traits::DivI;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!((&x).div_i().to_string(), "-3-2i");
    /// ```
    #[inline]
    fn div_i(self) -> GaussianInteger {
        self.clone().div_i()
    }
}

impl DivIAssign for GaussianInteger {
    /// Divides a [`GaussianInteger`] by $i$ in place. This is a clockwise quarter turn.
    ///
    /// $$
    /// a + bi \\gets b - ai.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivIAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianInteger::from_str("2-3i").unwrap();
    /// x.div_i_assign();
    /// assert_eq!(x.to_string(), "-3-2i");
    /// ```
    fn div_i_assign(&mut self) {
        swap(&mut self.real, &mut self.imaginary);
        self.imaginary.neg_assign();
    }
}
