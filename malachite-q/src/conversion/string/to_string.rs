// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::fmt::{Debug, Display, Formatter, Result, Write};

impl Display for Rational {
    /// Converts a [`Rational`] to a [`String`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.to_string(), "0");
    /// assert_eq!(Rational::from(123).to_string(), "123");
    /// assert_eq!(Rational::from_str("22/7").unwrap().to_string(), "22/7");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        let result = Display::fmt(&self.numerator, f);
        if self.denominator == 1 {
            result
        } else {
            f.write_char('/')?;
            Display::fmt(&self.denominator, f)
        }
    }
}

impl Debug for Rational {
    /// Converts a [`Rational`] to a [`String`].
    ///
    /// This is the same implementation as for [`Display`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.to_debug_string(), "0");
    /// assert_eq!(Rational::from(123).to_debug_string(), "123");
    /// assert_eq!(Rational::from_signeds(22, 7).to_debug_string(), "22/7");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
