// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::NotAssign;

impl Neg for Integer {
    type Output = Integer;

    /// Negates an [`Integer`], taking it by value.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(-Integer::ZERO, 0);
    /// assert_eq!(-Integer::from(123), -123);
    /// assert_eq!(-Integer::from(-123), 123);
    /// ```
    fn neg(mut self) -> Integer {
        if self.abs != 0 {
            self.sign.not_assign();
        }
        self
    }
}

impl<'a> Neg for &'a Integer {
    type Output = Integer;

    /// Negates an [`Integer`], taking it by reference.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(-&Integer::ZERO, 0);
    /// assert_eq!(-&Integer::from(123), -123);
    /// assert_eq!(-&Integer::from(-123), 123);
    /// ```
    fn neg(self) -> Integer {
        if self.abs == 0 {
            Integer::ZERO
        } else {
            Integer {
                sign: !self.sign,
                abs: self.abs.clone(),
            }
        }
    }
}

impl NegAssign for Integer {
    /// Negates an [`Integer`] in place.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.neg_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(123);
    /// x.neg_assign();
    /// assert_eq!(x, -123);
    ///
    /// let mut x = Integer::from(-123);
    /// x.neg_assign();
    /// assert_eq!(x, 123);
    /// ```
    fn neg_assign(&mut self) {
        if self.abs != 0 {
            self.sign.not_assign();
        }
    }
}
