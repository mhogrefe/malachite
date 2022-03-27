use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use Rational;

impl Square for Rational {
    type Output = Rational;

    /// Squares a `Rational`, taking it by value.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.square(), 0);
    /// assert_eq!(Rational::from_signeds(22, 7).square().to_string(), "484/49");
    /// assert_eq!(Rational::from_signeds(-22, 7).square().to_string(), "484/49");
    /// ```
    #[inline]
    fn square(mut self) -> Rational {
        self.square_assign();
        self
    }
}

impl<'a> Square for &'a Rational {
    type Output = Rational;

    /// Squares a `Rational`, taking it by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::ZERO).square(), 0);
    /// assert_eq!((&Rational::from_signeds(22, 7)).square().to_string(), "484/49");
    /// assert_eq!((&Rational::from_signeds(-22, 7)).square().to_string(), "484/49");
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
    /// Squares a `Rational` in place.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
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
