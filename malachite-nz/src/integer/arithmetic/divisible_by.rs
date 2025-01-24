// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::DivisibleBy;

impl DivisibleBy<Integer> for Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. Both [`Integer`]s are taken by value.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by(Integer::ZERO), true);
    /// assert_eq!(Integer::from(-100).divisible_by(Integer::from(-3)), false);
    /// assert_eq!(Integer::from(102).divisible_by(Integer::from(-3)), true);
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000000")
    ///         .unwrap()
    ///         .divisible_by(Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: Integer) -> bool {
        self.abs.divisible_by(other.abs)
    }
}

impl DivisibleBy<&Integer> for Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. The first [`Integer`] is taken by value and the
    /// second by reference.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by(&Integer::ZERO), true);
    /// assert_eq!(Integer::from(-100).divisible_by(&Integer::from(-3)), false);
    /// assert_eq!(Integer::from(102).divisible_by(&Integer::from(-3)), true);
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000000")
    ///         .unwrap()
    ///         .divisible_by(&Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: &Integer) -> bool {
        self.abs.divisible_by(&other.abs)
    }
}

impl DivisibleBy<Integer> for &Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. The first [`Integer`] is taken by reference and the
    /// second by value.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO).divisible_by(Integer::ZERO), true);
    /// assert_eq!(
    ///     (&Integer::from(-100)).divisible_by(Integer::from(-3)),
    ///     false
    /// );
    /// assert_eq!((&Integer::from(102)).divisible_by(Integer::from(-3)), true);
    /// assert_eq!(
    ///     (&Integer::from_str("-1000000000000000000000000").unwrap())
    ///         .divisible_by(Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: Integer) -> bool {
        (&self.abs).divisible_by(other.abs)
    }
}

impl DivisibleBy<&Integer> for &Integer {
    /// Returns whether an [`Integer`] is divisible by another [`Integer`]; in other words, whether
    /// the first is a multiple of the second. Both [`Integer`]s are taken by reference.
    ///
    /// This means that zero is divisible by any [`Integer`], including zero; but a nonzero
    /// [`Integer`] is never divisible by zero.
    ///
    /// It's more efficient to use this function than to compute the remainder and check whether
    /// it's zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivisibleBy;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO).divisible_by(&Integer::ZERO), true);
    /// assert_eq!(
    ///     (&Integer::from(-100)).divisible_by(&Integer::from(-3)),
    ///     false
    /// );
    /// assert_eq!((&Integer::from(102)).divisible_by(&Integer::from(-3)), true);
    /// assert_eq!(
    ///     (&Integer::from_str("-1000000000000000000000000").unwrap())
    ///         .divisible_by(&Integer::from_str("1000000000000").unwrap()),
    ///     true
    /// );
    /// ```
    fn divisible_by(self, other: &Integer) -> bool {
        (&self.abs).divisible_by(&other.abs)
    }
}
