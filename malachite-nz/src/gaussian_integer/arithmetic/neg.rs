// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;

impl Neg for GaussianInteger {
    type Output = Self;

    /// Negates a [`GaussianInteger`], taking it by value. Both the real and imaginary parts are
    /// negated.
    ///
    /// $$
    /// f(x) = -x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, Zero};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((-GaussianInteger::ZERO).to_string(), "0");
    /// assert_eq!((-GaussianInteger::I).to_string(), "-i");
    /// assert_eq!(
    ///     (-GaussianInteger::from_str("2-3i").unwrap()).to_string(),
    ///     "-2+3i"
    /// );
    /// ```
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }
}

impl Neg for &GaussianInteger {
    type Output = GaussianInteger;

    /// Negates a [`GaussianInteger`], taking it by reference. Both the real and imaginary parts are
    /// negated.
    ///
    /// $$
    /// f(x) = -x.
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
    /// use malachite_base::num::basic::traits::{I, Zero};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((-&GaussianInteger::ZERO).to_string(), "0");
    /// assert_eq!((-&GaussianInteger::I).to_string(), "-i");
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!((-&x).to_string(), "-2+3i");
    /// ```
    fn neg(self) -> GaussianInteger {
        GaussianInteger {
            real: -&self.real,
            imaginary: -&self.imaginary,
        }
    }
}

impl NegAssign for GaussianInteger {
    /// Negates a [`GaussianInteger`] in place. Both the real and imaginary parts are negated.
    ///
    /// $$
    /// x \gets -x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianInteger::from_str("2-3i").unwrap();
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "-2+3i");
    /// ```
    fn neg_assign(&mut self) {
        self.real.neg_assign();
        self.imaginary.neg_assign();
    }
}
