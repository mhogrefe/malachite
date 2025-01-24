// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};

impl Abs for Rational {
    type Output = Rational;

    /// Takes the absolute value of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.abs(), 0);
    /// assert_eq!(Rational::from_signeds(22, 7).abs().to_string(), "22/7");
    /// assert_eq!(Rational::from_signeds(-22, 7).abs().to_string(), "22/7");
    /// ```
    fn abs(mut self) -> Rational {
        self.sign = true;
        self
    }
}

impl Abs for &Rational {
    type Output = Rational;

    /// Takes the absolute value of a [`Rational`], taking the [`Rational`] by reference.
    ///
    /// $$
    /// f(x) = |x|.
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
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::ZERO).abs(), 0);
    /// assert_eq!((&Rational::from_signeds(22, 7)).abs().to_string(), "22/7");
    /// assert_eq!((&Rational::from_signeds(-22, 7)).abs().to_string(), "22/7");
    /// ```
    fn abs(self) -> Rational {
        Rational {
            sign: true,
            numerator: self.numerator.clone(),
            denominator: self.denominator.clone(),
        }
    }
}

impl AbsAssign for Rational {
    /// Replaces a [`Rational`] with its absolute value.
    ///
    /// $$
    /// x \gets |x|.
    /// $$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ZERO;
    /// x.abs_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.abs_assign();
    /// assert_eq!(x.to_string(), "22/7");
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.abs_assign();
    /// assert_eq!(x.to_string(), "22/7");
    /// ```
    fn abs_assign(&mut self) {
        self.sign = true;
    }
}
