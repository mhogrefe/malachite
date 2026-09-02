// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{DivI, DivIAssign, NegAssign};

impl DivI for GaussianRational {
    type Output = Self;

    /// Divides a [`GaussianRational`] by $i$, taking it by value. This is a clockwise quarter turn.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianRational::I.div_i().to_string(), "1");
    /// assert_eq!(
    ///     GaussianRational::from_str("1/2-2i/3")
    ///         .unwrap()
    ///         .div_i()
    ///         .to_string(),
    ///     "-2/3-i/2"
    /// );
    /// ```
    #[inline]
    fn div_i(mut self) -> Self {
        self.div_i_assign();
        self
    }
}

impl DivI for &GaussianRational {
    type Output = GaussianRational;

    /// Divides a [`GaussianRational`] by $i$, taking it by reference. This is a clockwise quarter
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2-2i/3").unwrap();
    /// assert_eq!((&x).div_i().to_string(), "-2/3-i/2");
    /// ```
    #[inline]
    fn div_i(self) -> GaussianRational {
        self.clone().div_i()
    }
}

impl DivIAssign for GaussianRational {
    /// Divides a [`GaussianRational`] by $i$ in place. This is a clockwise quarter turn.
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("1/2-2i/3").unwrap();
    /// x.div_i_assign();
    /// assert_eq!(x.to_string(), "-2/3-i/2");
    /// ```
    fn div_i_assign(&mut self) {
        swap(&mut self.real, &mut self.imaginary);
        self.imaginary.neg_assign();
    }
}
