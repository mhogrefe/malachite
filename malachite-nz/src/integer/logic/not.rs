// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::ops::Not;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::NotAssign;

impl Not for Integer {
    type Output = Integer;

    /// Returns the bitwise negation of an [`Integer`], taking it by value.
    ///
    /// $$
    /// f(n) = -n - 1.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(!Integer::ZERO, -1);
    /// assert_eq!(!Integer::from(123), -124);
    /// assert_eq!(!Integer::from(-123), 122);
    /// ```
    #[inline]
    fn not(mut self) -> Integer {
        self.not_assign();
        self
    }
}

impl<'a> Not for &'a Integer {
    type Output = Integer;

    /// Returns the bitwise negation of an [`Integer`], taking it by reference.
    ///
    /// $$
    /// f(n) = -n - 1.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(!&Integer::ZERO, -1);
    /// assert_eq!(!&Integer::from(123), -124);
    /// assert_eq!(!&Integer::from(-123), 122);
    /// ```
    fn not(self) -> Integer {
        match *self {
            Integer {
                sign: true,
                ref abs,
            } => Integer {
                sign: false,
                abs: abs.add_limb_ref(1),
            },
            Integer {
                sign: false,
                ref abs,
            } => Integer {
                sign: true,
                abs: abs.sub_limb_ref(1),
            },
        }
    }
}

impl NotAssign for Integer {
    /// Replaces an [`Integer`] with its bitwise negation.
    ///
    /// $$
    /// n \gets -n - 1.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::NotAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.not_assign();
    /// assert_eq!(x, -1);
    ///
    /// let mut x = Integer::from(123);
    /// x.not_assign();
    /// assert_eq!(x, -124);
    ///
    /// let mut x = Integer::from(-123);
    /// x.not_assign();
    /// assert_eq!(x, 122);
    /// ```
    fn not_assign(&mut self) {
        if self.sign {
            self.sign = false;
            self.abs += Natural::ONE;
        } else {
            self.sign = true;
            self.abs -= Natural::ONE;
        }
    }
}
