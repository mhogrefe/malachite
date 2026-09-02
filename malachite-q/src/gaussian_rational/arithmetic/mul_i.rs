// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{MulI, MulIAssign, NegAssign};

impl MulI for GaussianRational {
    type Output = Self;

    /// Multiplies a [`GaussianRational`] by $i$, taking it by value. This is a counterclockwise
    /// quarter turn.
    ///
    /// $$
    /// f(a + bi) = (a + bi)i = -b + ai.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulI;
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianRational::I.mul_i().to_string(), "-1");
    /// assert_eq!(
    ///     GaussianRational::from_str("1/2-2i/3")
    ///         .unwrap()
    ///         .mul_i()
    ///         .to_string(),
    ///     "2/3+i/2"
    /// );
    /// ```
    #[inline]
    fn mul_i(mut self) -> Self {
        self.mul_i_assign();
        self
    }
}

impl MulI for &GaussianRational {
    type Output = GaussianRational;

    /// Multiplies a [`GaussianRational`] by $i$, taking it by reference. This is a counterclockwise
    /// quarter turn.
    ///
    /// $$
    /// f(a + bi) = (a + bi)i = -b + ai.
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
    /// use malachite_base::num::arithmetic::traits::MulI;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2-2i/3").unwrap();
    /// assert_eq!((&x).mul_i().to_string(), "2/3+i/2");
    /// ```
    #[inline]
    fn mul_i(self) -> GaussianRational {
        self.clone().mul_i()
    }
}

impl MulIAssign for GaussianRational {
    /// Multiplies a [`GaussianRational`] by $i$ in place. This is a counterclockwise quarter turn.
    ///
    /// $$
    /// a + bi \\gets -b + ai.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulIAssign;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("1/2-2i/3").unwrap();
    /// x.mul_i_assign();
    /// assert_eq!(x.to_string(), "2/3+i/2");
    /// ```
    fn mul_i_assign(&mut self) {
        swap(&mut self.real, &mut self.imaginary);
        self.real.neg_assign();
    }
}
