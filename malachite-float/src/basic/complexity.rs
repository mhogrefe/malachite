// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::max;
use malachite_base::num::logic::traits::SignificantBits;

impl Float {
    /// Determines a [`Float`]'s complexity. The complexity is defined as follows:
    ///
    /// $$
    /// f(\text{NaN}) = f(\pm\infty) = f(\pm 0.0) = 1,
    /// $$
    ///
    /// and, if $x$ is finite and nonzero,
    ///
    /// $$
    /// f(x) = \max(|\lfloor \log_2 x\rfloor|, p),
    /// $$
    ///
    /// where $p$ is the precision of $x$.
    ///
    /// Informally, the complexity is proportional to the number of characters you would need to
    /// write the [`Float`] out without using exponents.
    ///
    /// See also the [`Float`] implementation of [`SignificantBits`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.complexity(), 1);
    /// assert_eq!(Float::ONE.complexity(), 1);
    /// assert_eq!(Float::one_prec(100).complexity(), 100);
    /// assert_eq!(Float::from(std::f64::consts::PI).complexity(), 53);
    /// assert_eq!(Float::power_of_2(100u64).complexity(), 100);
    /// assert_eq!(Float::power_of_2(-100i64).complexity(), 100);
    /// ```
    pub fn complexity(&self) -> u64 {
        match self {
            Float(Finite {
                exponent,
                precision,
                ..
            }) => max(
                u64::from(exponent.checked_sub(1).unwrap().unsigned_abs()),
                *precision,
            ),
            _ => 1,
        }
    }
}

impl<'a> SignificantBits for &'a Float {
    /// Returns the number of significant bits of a [`Float`]. This is defined as follows:
    ///
    /// $$
    /// f(\text{NaN}) = f(\pm\infty) = f(\pm 0.0) = 1,
    /// $$
    ///
    /// and, if $x$ is finite and nonzero,
    ///
    /// $$
    /// f(x) = p,
    /// $$
    ///
    /// where $p$ is the precision of $x$.
    ///
    /// See also the [`complexity`](Float::complexity) function.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.significant_bits(), 1);
    /// assert_eq!(Float::ONE.significant_bits(), 1);
    /// assert_eq!(Float::one_prec(100).significant_bits(), 100);
    /// assert_eq!(Float::from(std::f64::consts::PI).significant_bits(), 53);
    /// assert_eq!(Float::power_of_2(100u64).significant_bits(), 1);
    /// assert_eq!(Float::power_of_2(-100i64).significant_bits(), 1);
    /// ```
    fn significant_bits(self) -> u64 {
        match self {
            Float(Finite { precision, .. }) => *precision,
            _ => 1,
        }
    }
}
