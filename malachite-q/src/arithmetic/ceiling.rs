// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{Ceiling, CeilingAssign, DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl Ceiling for Rational {
    type Output = Integer;

    /// Finds the ceiling of a [`Rational`], taking the [`Rational`] by value.
    ///
    /// $$
    /// f(x) = \lceil x \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ceiling;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.ceiling(), 0);
    /// assert_eq!(Rational::from_signeds(22, 7).ceiling(), 4);
    /// assert_eq!(Rational::from_signeds(-22, 7).ceiling(), -3);
    /// ```
    fn ceiling(self) -> Integer {
        if self.sign {
            Integer::from(self.numerator.div_round(self.denominator, Ceiling).0)
        } else {
            Integer::from_sign_and_abs(false, self.numerator / self.denominator)
        }
    }
}

impl<'a> Ceiling for &'a Rational {
    type Output = Integer;

    /// Finds the ceiling of a [`Rational`], taking the [`Rational`] by reference.
    ///
    /// $$
    /// f(x) = \lceil x \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Ceiling;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::ZERO).ceiling(), 0);
    /// assert_eq!((&Rational::from_signeds(22, 7)).ceiling(), 4);
    /// assert_eq!((&Rational::from_signeds(-22, 7)).ceiling(), -3);
    /// ```
    fn ceiling(self) -> Integer {
        if self.sign {
            Integer::from((&self.numerator).div_round(&self.denominator, Ceiling).0)
        } else {
            Integer::from_sign_and_abs(false, &self.numerator / &self.denominator)
        }
    }
}

impl CeilingAssign for Rational {
    /// Replaces a [`Rational`] with its ceiling.
    ///
    /// $$
    /// x \gets \lceil x \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ZERO;
    /// x.ceiling_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.ceiling_assign();
    /// assert_eq!(x, 4);
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.ceiling_assign();
    /// assert_eq!(x, -3);
    /// ```
    fn ceiling_assign(&mut self) {
        let mut d = Natural::ONE;
        swap(&mut self.denominator, &mut d);
        if self.sign {
            self.numerator.div_round_assign(d, Ceiling);
        } else {
            self.numerator /= d;
            if !self.sign && self.numerator == 0 {
                self.sign = true;
            }
        }
    }
}
