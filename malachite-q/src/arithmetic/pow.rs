// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{
    Parity, Pow, PowAssign, Reciprocal, ReciprocalAssign,
};

impl Pow<u64> for Rational {
    type Output = Rational;

    /// Raises a [`Rational`] to a power, taking the [`Rational`] by value.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).pow(3u64).to_string(),
    ///     "10648/343"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).pow(3u64).to_string(),
    ///     "-10648/343"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).pow(4u64).to_string(),
    ///     "234256/2401"
    /// );
    /// ```
    #[inline]
    fn pow(mut self, exp: u64) -> Rational {
        self.pow_assign(exp);
        self
    }
}

impl Pow<u64> for &Rational {
    type Output = Rational;

    /// Raises a [`Rational`] to a power, taking the [`Rational`] by reference.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7)).pow(3u64).to_string(),
    ///     "10648/343"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).pow(3u64).to_string(),
    ///     "-10648/343"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).pow(4u64).to_string(),
    ///     "234256/2401"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: u64) -> Rational {
        Rational {
            sign: self.sign || exp.even(),
            numerator: (&self.numerator).pow(exp),
            denominator: (&self.denominator).pow(exp),
        }
    }
}

impl PowAssign<u64> for Rational {
    /// Raises a [`Rational`] to a power in place.
    ///
    /// $x \gets x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.pow_assign(3u64);
    /// assert_eq!(x.to_string(), "10648/343");
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.pow_assign(3u64);
    /// assert_eq!(x.to_string(), "-10648/343");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.pow_assign(4u64);
    /// assert_eq!(x.to_string(), "234256/2401");
    /// ```
    fn pow_assign(&mut self, exp: u64) {
        self.sign |= exp.even();
        self.numerator.pow_assign(exp);
        self.denominator.pow_assign(exp);
    }
}

impl Pow<i64> for Rational {
    type Output = Rational;

    /// Raises a [`Rational`] to a power, taking the [`Rational`] by value.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp.abs()`.
    ///
    /// # Panics
    /// Panics if `self` is zero and `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).pow(3i64).to_string(),
    ///     "10648/343"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).pow(3i64).to_string(),
    ///     "-10648/343"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).pow(4i64).to_string(),
    ///     "234256/2401"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).pow(-3i64).to_string(),
    ///     "343/10648"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).pow(-3i64).to_string(),
    ///     "-343/10648"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).pow(-4i64).to_string(),
    ///     "2401/234256"
    /// );
    /// ```
    #[inline]
    fn pow(mut self, exp: i64) -> Rational {
        self.pow_assign(exp);
        self
    }
}

impl Pow<i64> for &Rational {
    type Output = Rational;

    /// Raises a [`Rational`] to a power, taking the [`Rational`] by reference.
    ///
    /// $f(x, n) = x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp.abs()`.
    ///
    /// # Panics
    /// Panics if `self` is zero and `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7)).pow(3i64).to_string(),
    ///     "10648/343"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).pow(3i64).to_string(),
    ///     "-10648/343"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).pow(4i64).to_string(),
    ///     "234256/2401"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7)).pow(-3i64).to_string(),
    ///     "343/10648"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).pow(-3i64).to_string(),
    ///     "-343/10648"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-22, 7)).pow(-4i64).to_string(),
    ///     "2401/234256"
    /// );
    /// ```
    #[inline]
    fn pow(self, exp: i64) -> Rational {
        let abs_exp = exp.unsigned_abs();
        if exp >= 0 {
            self.pow(abs_exp)
        } else {
            self.pow(abs_exp).reciprocal()
        }
    }
}

impl PowAssign<i64> for Rational {
    /// Raises a [`Rational`] to a power in place.
    ///
    /// $x \gets x^n$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `exp.abs()`.
    ///
    /// # Panics
    /// Panics if `self` is zero and `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.pow_assign(3i64);
    /// assert_eq!(x.to_string(), "10648/343");
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.pow_assign(3i64);
    /// assert_eq!(x.to_string(), "-10648/343");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.pow_assign(4i64);
    /// assert_eq!(x.to_string(), "234256/2401");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.pow_assign(-3i64);
    /// assert_eq!(x.to_string(), "343/10648");
    ///
    /// let mut x = Rational::from_signeds(-22, 7);
    /// x.pow_assign(-3i64);
    /// assert_eq!(x.to_string(), "-343/10648");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.pow_assign(-4i64);
    /// assert_eq!(x.to_string(), "2401/234256");
    /// ```
    fn pow_assign(&mut self, exp: i64) {
        let abs_exp = exp.unsigned_abs();
        if exp >= 0 {
            self.pow_assign(abs_exp);
        } else {
            self.pow_assign(abs_exp);
            self.reciprocal_assign();
        }
    }
}
