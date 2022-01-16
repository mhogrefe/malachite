use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign, Floor, FloorAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::mem::swap;
use Rational;

impl Floor for Rational {
    type Output = Integer;

    /// Finds the floor of a `Rational`, taking the `Rational` by value.
    ///
    /// $$
    /// f(x) = \lfloor x \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Floor;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.floor(), 0);
    /// assert_eq!(Rational::from_signeds(22, 7).floor(), 3);
    /// assert_eq!(Rational::from_signeds(-22, 7).floor(), -4);
    /// ```
    fn floor(self) -> Integer {
        if self.sign {
            Integer::from(self.numerator / self.denominator)
        } else {
            Integer::from_sign_and_abs(
                false,
                self.numerator
                    .div_round(self.denominator, RoundingMode::Ceiling),
            )
        }
    }
}

impl<'a> Floor for &'a Rational {
    type Output = Integer;

    /// Finds the floor of a `Rational`, taking the `Rational` by reference.
    ///
    /// $$
    /// f(x) = \lfloor x \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Floor;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Rational::ZERO).floor(), 0);
    /// assert_eq!((&Rational::from_signeds(22, 7)).floor(), 3);
    /// assert_eq!((&Rational::from_signeds(-22, 7)).floor(), -4);
    /// ```
    fn floor(self) -> Integer {
        if self.sign {
            Integer::from(&self.numerator / &self.denominator)
        } else {
            Integer::from_sign_and_abs(
                false,
                (&self.numerator).div_round(&self.denominator, RoundingMode::Ceiling),
            )
        }
    }
}

impl FloorAssign for Rational {
    /// Replaces a `Rational` with its floor.
    ///
    /// $$
    /// x \gets \lfloor x \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// let mut x = Rational::ZERO;
    /// x.floor_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.floor_assign();
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.floor_assign();
    /// assert_eq!(x, -4);
    /// ```
    fn floor_assign(&mut self) {
        let mut d = Natural::ONE;
        swap(&mut self.denominator, &mut d);
        if self.sign {
            self.numerator /= d;
        } else {
            self.numerator.div_round_assign(d, RoundingMode::Ceiling);
            if !self.sign && self.numerator == 0 {
                self.sign = true;
            }
        }
    }
}
