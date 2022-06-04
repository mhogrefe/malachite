use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd, Reciprocal};
use malachite_base::num::basic::traits::Zero;
use std::ops::{Div, DivAssign};
use Rational;

impl Div<Rational> for Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking both by value.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::TWO / Rational::TWO, 1);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) / Rational::from_signeds(99, 100)).to_string(),
    ///     "200/63"
    /// );
    /// ```
    fn div(self, other: Rational) -> Rational {
        if other == 0u32 {
            panic!("division by zero");
        } else if self == 0u32 {
            return Rational::ZERO;
        } else if self == 1u32 {
            return other.reciprocal();
        } else if other == 1u32 {
            return self;
        }
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        Rational {
            sign: self.sign == other.sign,
            numerator: (self.numerator).div_exact(&g_1) * (other.denominator).div_exact(&g_2),
            denominator: (other.numerator).div_exact(g_1) * (self.denominator).div_exact(g_2),
        }
    }
}

impl<'a> Div<&'a Rational> for Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::TWO / &Rational::TWO, 1);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) / &Rational::from_signeds(99, 100)).to_string(),
    ///     "200/63"
    /// );
    /// ```
    #[inline]
    fn div(self, other: &'a Rational) -> Rational {
        if self == 0u32 {
            Rational::ZERO
        } else {
            (other / self).reciprocal()
        }
    }
}

impl<'a> Div<Rational> for &'a Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking the first by reference and the second
    /// by value.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::TWO / Rational::TWO, 1);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) / Rational::from_signeds(99, 100)).to_string(),
    ///     "200/63"
    /// );
    /// ```
    fn div(self, other: Rational) -> Rational {
        if other == 0u32 {
            panic!("division by zero");
        } else if *self == 0u32 {
            return Rational::ZERO;
        } else if *self == 1u32 {
            return other.reciprocal();
        } else if other == 1u32 {
            return self.clone();
        }
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        Rational {
            sign: self.sign == other.sign,
            numerator: (&self.numerator).div_exact(&g_1) * (other.denominator).div_exact(&g_2),
            denominator: (other.numerator).div_exact(g_1) * (&self.denominator).div_exact(g_2),
        }
    }
}

impl<'a, 'b> Div<&'a Rational> for &'b Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking both by reference.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::TWO / &Rational::TWO, 1);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) / &Rational::from_signeds(99, 100)).to_string(),
    ///     "200/63"
    /// );
    /// ```
    fn div(self, other: &'a Rational) -> Rational {
        if *other == 0u32 {
            panic!("division by zero");
        } else if *self == 0u32 {
            return Rational::ZERO;
        } else if *self == 1u32 {
            return other.reciprocal();
        } else if *other == 1u32 {
            return self.clone();
        }
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        Rational {
            sign: self.sign == other.sign,
            numerator: (&self.numerator).div_exact(&g_1) * (&other.denominator).div_exact(&g_2),
            denominator: (&other.numerator).div_exact(g_1) * (&self.denominator).div_exact(g_2),
        }
    }
}

impl DivAssign<Rational> for Rational {
    /// Divides a [`Rational`] by a [`Rational`] in place, taking the [`Rational`] on the right-hand
    /// side by value.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::TWO;
    /// x /= Rational::TWO;
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x /= Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "200/63");
    /// ```
    fn div_assign(&mut self, other: Rational) {
        if other == 0u32 {
            panic!("division by zero");
        } else if *self == 0u32 || other == 1u32 {
            return;
        } else if *self == 1u32 {
            *self = other.reciprocal();
            return;
        }
        self.sign = self.sign == other.sign;
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&g_1);
        self.denominator.div_exact_assign(&g_2);
        self.numerator *= (other.denominator).div_exact(g_2);
        self.denominator *= (other.numerator).div_exact(g_1);
    }
}

impl<'a> DivAssign<&'a Rational> for Rational {
    /// Divides a [`Rational`] by a [`Rational`] in place, taking the [`Rational`] on the right-hand
    /// side by reference.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::TWO;
    /// x /= &Rational::TWO;
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x /= &Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "200/63");
    /// ```
    fn div_assign(&mut self, other: &'a Rational) {
        if *other == 0u32 {
            panic!("division by zero");
        } else if *self == 0u32 || *other == 1u32 {
            return;
        } else if *self == 1u32 {
            *self = other.reciprocal();
            return;
        }
        self.sign = self.sign == other.sign;
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        self.numerator.div_exact_assign(&g_1);
        self.denominator.div_exact_assign(&g_2);
        self.numerator *= (&other.denominator).div_exact(g_2);
        self.denominator *= (&other.numerator).div_exact(g_1);
    }
}
