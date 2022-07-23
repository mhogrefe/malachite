use crate::Rational;
use malachite_base::num::arithmetic::traits::{DivExactAssign, Gcd};
use malachite_nz::natural::Natural;

impl Rational {
    /// Mutates the numerator of a [`Rational`] using a provided closure, and then returns
    /// whatever the closure returns.
    ///
    /// After the closure executes, this function reduces the [`Rational`].
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let mut q = Rational::from_signeds(22, 7);
    /// let ret = q.mutate_numerator(|x| {
    ///     *x -= Natural::ONE;
    ///     true
    /// });
    /// assert_eq!(q, 3);
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_numerator<F: FnOnce(&mut Natural) -> T, T>(&mut self, f: F) -> T {
        let out = f(&mut self.numerator);
        let gcd = (&self.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&gcd);
        self.denominator.div_exact_assign(gcd);
        if !self.sign && self.numerator == 0 {
            self.sign = true;
        }
        out
    }

    /// Mutates the denominator of a [`Rational`] using a provided closure.
    ///
    /// After the closure executes, this function reduces the [`Rational`].
    ///
    /// # Panics
    /// Panics if the closure sets the denominator to zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let mut q = Rational::from_signeds(22, 7);
    /// let ret = q.mutate_denominator(|x| {
    ///     *x -= Natural::ONE;
    ///     true
    /// });
    /// assert_eq!(q.to_string(), "11/3");
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_denominator<F: FnOnce(&mut Natural) -> T, T>(&mut self, f: F) -> T {
        let out = f(&mut self.denominator);
        assert_ne!(self.denominator, 0);
        let gcd = (&self.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&gcd);
        self.denominator.div_exact_assign(gcd);
        out
    }

    /// Mutates the numerator and denominator of a [`Rational`] using a provided closure.
    ///
    /// After the closure executes, this function reduces the [`Rational`].
    ///
    /// # Panics
    /// Panics if the closure sets the denominator to zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::Rational;
    ///
    /// let mut q = Rational::from_signeds(22, 7);
    /// let ret = q.mutate_numerator_and_denominator(|x, y| {
    ///     *x -= Natural::ONE;
    ///     *y -= Natural::ONE;
    ///     true
    /// });
    /// assert_eq!(q.to_string(), "7/2");
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_numerator_and_denominator<F: FnOnce(&mut Natural, &mut Natural) -> T, T>(
        &mut self,
        f: F,
    ) -> T {
        let out = f(&mut self.numerator, &mut self.denominator);
        assert_ne!(self.denominator, 0);
        let gcd = (&self.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&gcd);
        self.denominator.div_exact_assign(gcd);
        if !self.sign && self.numerator == 0 {
            self.sign = true;
        }
        out
    }
}
