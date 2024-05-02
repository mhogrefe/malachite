// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::NotAssign;

impl Neg for Rational {
    type Output = Rational;

    /// Negates a [`Rational`], taking it by value.
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
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(-Rational::ZERO, 0);
    /// assert_eq!((-Rational::from_signeds(22, 7)).to_string(), "-22/7");
    /// assert_eq!((-Rational::from_signeds(-22, 7)).to_string(), "22/7");
    /// ```
    fn neg(mut self) -> Rational {
        if self.numerator != 0 {
            self.sign.not_assign();
        }
        self
    }
}

impl<'a> Neg for &'a Rational {
    type Output = Rational;

    /// Negates a [`Rational`], taking it by reference.
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
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(-&Rational::ZERO, 0);
    /// assert_eq!((-&Rational::from_signeds(22, 7)).to_string(), "-22/7");
    /// assert_eq!((-&Rational::from_signeds(-22, 7)).to_string(), "22/7");
    /// ```
    fn neg(self) -> Rational {
        if self.numerator == 0 {
            Rational::ZERO
        } else {
            Rational {
                sign: !self.sign,
                numerator: self.numerator.clone(),
                denominator: self.denominator.clone(),
            }
        }
    }
}

impl NegAssign for Rational {
    /// Negates a [`Rational`] in place.
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
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ZERO;
    /// x.neg_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "-22/7");
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.neg_assign();
    /// assert_eq!(x.to_string(), "22/7");
    /// ```
    fn neg_assign(&mut self) {
        if self.numerator != 0 {
            self.sign.not_assign();
        }
    }
}
