use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd};
use malachite_base::num::basic::traits::Zero;
use std::ops::{Mul, MulAssign};
use Rational;

impl Mul<Rational> for Rational {
    type Output = Rational;

    /// Multiplies a `Rational` by a `Rational`, taking both `Rational`s by value.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::{OneHalf, Two};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF * Rational::TWO, 1);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) * Rational::from_signeds(99, 100)).to_string(),
    ///     "1089/350"
    /// );
    /// ```
    fn mul(self, other: Rational) -> Rational {
        if self == 0u32 || other == 0u32 {
            return Rational::ZERO;
        } else if self == 1u32 {
            return other;
        } else if other == 1u32 {
            return self;
        }
        let g_1 = (&self.numerator).gcd(&other.denominator);
        let g_2 = (&other.numerator).gcd(&self.denominator);
        Rational {
            sign: self.sign == other.sign,
            numerator: (self.numerator).div_exact(&g_1) * (other.numerator).div_exact(&g_2),
            denominator: (other.denominator).div_exact(g_1) * (self.denominator).div_exact(g_2),
        }
    }
}

impl<'a> Mul<&'a Rational> for Rational {
    type Output = Rational;

    /// Multiplies a `Rational` by a `Rational`, taking the first `Rational`s by value and the
    /// second by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::{OneHalf, Two};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF * &Rational::TWO, 1);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) * &Rational::from_signeds(99, 100)).to_string(),
    ///     "1089/350"
    /// );
    /// ```
    #[inline]
    fn mul(self, other: &'a Rational) -> Rational {
        other * self
    }
}

impl<'a> Mul<Rational> for &'a Rational {
    type Output = Rational;

    /// Multiplies a `Rational` by a `Rational`, taking the first `Rational` by value and the
    /// second by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::{OneHalf, Two};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::ONE_HALF * Rational::TWO, 1);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) * Rational::from_signeds(99, 100)).to_string(),
    ///     "1089/350"
    /// );
    /// ```
    fn mul(self, other: Rational) -> Rational {
        if *self == 0u32 || other == 0u32 {
            return Rational::ZERO;
        } else if *self == 1u32 {
            return other;
        } else if other == 1u32 {
            return self.clone();
        }
        let g_1 = (&self.numerator).gcd(&other.denominator);
        let g_2 = (&other.numerator).gcd(&self.denominator);
        Rational {
            sign: self.sign == other.sign,
            numerator: (&self.numerator).div_exact(&g_1) * (other.numerator).div_exact(&g_2),
            denominator: (other.denominator).div_exact(g_1) * (&self.denominator).div_exact(g_2),
        }
    }
}

impl<'a, 'b> Mul<&'a Rational> for &'b Rational {
    type Output = Rational;

    /// Multiplies a `Rational` by a `Rational`, taking both `Rational`s by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::{OneHalf, Two};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::ONE_HALF * &Rational::TWO, 1);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) * &Rational::from_signeds(99, 100)).to_string(),
    ///     "1089/350"
    /// );
    /// ```
    fn mul(self, other: &'a Rational) -> Rational {
        if *self == 0u32 || *other == 0u32 {
            return Rational::ZERO;
        } else if *self == 1u32 {
            return other.clone();
        } else if *other == 1u32 {
            return self.clone();
        }
        let g_1 = (&self.numerator).gcd(&other.denominator);
        let g_2 = (&other.numerator).gcd(&self.denominator);
        Rational {
            sign: self.sign == other.sign,
            numerator: (&self.numerator).div_exact(&g_1) * (&other.numerator).div_exact(&g_2),
            denominator: (&other.denominator).div_exact(g_1) * (&self.denominator).div_exact(g_2),
        }
    }
}

impl MulAssign<Rational> for Rational {
    /// Multiplies a `Rational` by a `Rational` in place, taking the `Rational` on the right-hand
    /// side by value.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::{OneHalf, Two};
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x *= Rational::TWO;
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x *= Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "1089/350");
    /// ```
    fn mul_assign(&mut self, other: Rational) {
        if *self == 0u32 || other == 1u32 {
            return;
        } else if other == 0u32 {
            *self = Rational::ZERO;
            return;
        } else if *self == 1u32 {
            *self = other;
            return;
        }
        self.sign = self.sign == other.sign;
        let g_1 = (&self.numerator).gcd(&other.denominator);
        let g_2 = (&other.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&g_1);
        self.denominator.div_exact_assign(&g_2);
        self.numerator *= (other.numerator).div_exact(g_2);
        self.denominator *= (other.denominator).div_exact(g_1);
    }
}

impl<'a> MulAssign<&'a Rational> for Rational {
    /// Multiplies a `Rational` by a `Rational` in place, taking the `Rational` on the right-hand
    /// side by reference.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::{OneHalf, Two};
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x *= &Rational::TWO;
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x *= &Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "1089/350");
    /// ```
    fn mul_assign(&mut self, other: &'a Rational) {
        if *self == 0u32 || *other == 1u32 {
            return;
        } else if *other == 0u32 {
            *self = Rational::ZERO;
            return;
        } else if *self == 1u32 {
            *self = other.clone();
            return;
        }
        self.sign = self.sign == other.sign;
        let g_1 = (&self.numerator).gcd(&other.denominator);
        let g_2 = (&other.numerator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&g_1);
        self.denominator.div_exact_assign(&g_2);
        self.numerator *= (&other.numerator).div_exact(g_2);
        self.denominator *= (&other.denominator).div_exact(g_1);
    }
}
