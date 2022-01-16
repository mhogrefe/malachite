use malachite_base::num::arithmetic::traits::{Ceiling, CeilingAssign, DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::mem::swap;
use Rational;

impl Ceiling for Rational {
    type Output = Integer;

    /// Finds the ceiling of a `Rational`, taking the `Rational` by value.
    ///
    /// $$
    /// f(x) = \lceil x \rceil.
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
            Integer::from(
                self.numerator
                    .div_round(self.denominator, RoundingMode::Ceiling),
            )
        } else {
            Integer::from_sign_and_abs(false, self.numerator / self.denominator)
        }
    }
}

impl<'a> Ceiling for &'a Rational {
    type Output = Integer;

    /// Finds the ceiling of a `Rational`, taking the `Rational` by reference.
    ///
    /// $$
    /// f(x) = \lceil x \rceil.
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
            Integer::from((&self.numerator).div_round(&self.denominator, RoundingMode::Ceiling))
        } else {
            Integer::from_sign_and_abs(false, &self.numerator / &self.denominator)
        }
    }
}

impl CeilingAssign for Rational {
    /// Replaces a `Rational` with its ceiling.
    ///
    /// $$
    /// x \gets \lceil x \rceil.
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
            self.numerator.div_round_assign(d, RoundingMode::Ceiling);
        } else {
            self.numerator /= d;
            if !self.sign && self.numerator == 0 {
                self.sign = true;
            }
        }
    }
}
