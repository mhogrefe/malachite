// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{Reciprocal, ReciprocalAssign};

impl Reciprocal for Rational {
    type Output = Rational;

    /// Reciprocates a [`Rational`], taking it by value.
    ///
    /// $$
    /// f(x) = 1/x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Reciprocal;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).reciprocal().to_string(),
    ///     "7/22"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(7, 22).reciprocal().to_string(),
    ///     "22/7"
    /// );
    /// ```
    #[inline]
    fn reciprocal(mut self) -> Rational {
        self.reciprocal_assign();
        self
    }
}

impl Reciprocal for &Rational {
    type Output = Rational;

    /// Reciprocates a [`Rational`], taking it by reference.
    ///
    /// $$
    /// f(x) = 1/x.
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
    /// use malachite_base::num::arithmetic::traits::Reciprocal;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7)).reciprocal().to_string(),
    ///     "7/22"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(7, 22)).reciprocal().to_string(),
    ///     "22/7"
    /// );
    /// ```
    fn reciprocal(self) -> Rational {
        assert_ne!(self.numerator, 0, "Cannot take reciprocal of zero");
        Rational {
            sign: self.sign,
            numerator: self.denominator.clone(),
            denominator: self.numerator.clone(),
        }
    }
}

impl ReciprocalAssign for Rational {
    /// Reciprocates a [`Rational`] in place.
    ///
    /// $$
    /// x \gets 1/x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.reciprocal_assign();
    /// assert_eq!(x.to_string(), "7/22");
    ///
    /// let mut x = Rational::from_signeds(7, 22);
    /// x.reciprocal_assign();
    /// assert_eq!(x.to_string(), "22/7");
    /// ```
    fn reciprocal_assign(&mut self) {
        assert_ne!(self.numerator, 0, "Cannot take reciprocal of zero");
        swap(&mut self.numerator, &mut self.denominator);
    }
}
