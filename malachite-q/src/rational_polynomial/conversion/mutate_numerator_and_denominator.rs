// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::mem::take;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

impl RationalPolynomial {
    /// Mutates the numerator of a [`RationalPolynomial`] using a provided closure, and then returns
    /// whatever the closure returns.
    ///
    /// After the closure executes, this function reduces the [`RationalPolynomial`], dividing out
    /// whatever the numerator's coefficients now share with the denominator.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator after the closure executes, not counting the
    /// closure itself.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // 1/2*x+1/3 is (3*x+2)/6.
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// let ret = p.mutate_numerator(|n| {
    ///     *n = IntegerPolynomial::from_str("3*x").unwrap();
    ///     true
    /// });
    /// assert_eq!(p.to_string(), "1/2*x");
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_numerator<F: FnOnce(&mut IntegerPolynomial) -> T, T>(&mut self, f: F) -> T {
        let out = f(&mut self.numerator);
        *self = Self::canonicalize(take(&mut self.numerator), take(&mut self.denominator));
        out
    }

    /// Mutates the denominator of a [`RationalPolynomial`] using a provided closure, and then
    /// returns whatever the closure returns.
    ///
    /// After the closure executes, this function reduces the [`RationalPolynomial`], dividing out
    /// whatever the numerator's coefficients now share with the denominator.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator after the closure executes, not counting the
    /// closure itself.
    ///
    /// # Panics
    /// Panics if the closure sets the denominator to zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// // 1/2*x+1/3 is (3*x+2)/6.
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// let ret = p.mutate_denominator(|d| {
    ///     *d = Natural::from(3u32);
    ///     true
    /// });
    /// assert_eq!(p.to_string(), "x+2/3");
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_denominator<F: FnOnce(&mut Natural) -> T, T>(&mut self, f: F) -> T {
        let out = f(&mut self.denominator);
        assert_ne!(self.denominator, 0u32, "the denominator may not be zero");
        *self = Self::canonicalize(take(&mut self.numerator), take(&mut self.denominator));
        out
    }

    /// Mutates the numerator and denominator of a [`RationalPolynomial`] using a provided closure,
    /// and then returns whatever the closure returns.
    ///
    /// After the closure executes, this function reduces the [`RationalPolynomial`], dividing out
    /// whatever the numerator's coefficients now share with the denominator.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator after the closure executes, not counting the
    /// closure itself.
    ///
    /// # Panics
    /// Panics if the closure sets the denominator to zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// let ret = p.mutate_numerator_and_denominator(|n, d| {
    ///     *n = IntegerPolynomial::from_str("6*x+4").unwrap();
    ///     *d = Natural::from(4u32);
    ///     true
    /// });
    /// // (6*x+4)/4 is reduced to (3*x+2)/2.
    /// assert_eq!(p.to_string(), "3/2*x+1");
    /// assert_eq!(ret, true);
    /// ```
    pub fn mutate_numerator_and_denominator<
        F: FnOnce(&mut IntegerPolynomial, &mut Natural) -> T,
        T,
    >(
        &mut self,
        f: F,
    ) -> T {
        let out = f(&mut self.numerator, &mut self.denominator);
        assert_ne!(self.denominator, 0u32, "the denominator may not be zero");
        *self = Self::canonicalize(take(&mut self.numerator), take(&mut self.denominator));
        out
    }
}
