// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

impl Square for Rational {
    type Output = Rational;

    /// Squares a [`Rational`], taking it by value.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.square(), 0);
    /// assert_eq!(Rational::from_signeds(22, 7).square().to_string(), "484/49");
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).square().to_string(),
    ///     "484/49"
    /// );
    /// ```
    #[inline]
    fn square(mut self) -> Rational {
        self.square_assign();
        self
    }
}

impl Square for &Rational {
    type Output = Rational;

    /// Squares a [`Rational`], taking it by reference.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::ZERO).square(), 0);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7)).square().to_string(),
    ///     "484/49"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).square().to_string(),
    ///     "484/49"
    /// );
    /// ```
    #[inline]
    fn square(self) -> Rational {
        Rational {
            sign: true,
            numerator: (&self.numerator).square(),
            denominator: (&self.denominator).square(),
        }
    }
}

impl SquareAssign for Rational {
    /// Squares a [`Rational`] in place.
    ///
    /// $$
    /// x \gets x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ZERO;
    /// x.square_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.square_assign();
    /// assert_eq!(x.to_string(), "484/49");
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.square_assign();
    /// assert_eq!(x.to_string(), "484/49");
    /// ```
    fn square_assign(&mut self) {
        self.sign = true;
        self.numerator.square_assign();
        self.denominator.square_assign();
    }
}
