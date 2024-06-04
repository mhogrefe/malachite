// Copyright © 2024 Mikhail Hogrefe
//
// CheckedDiv implementation by Park Joon-Kyu.
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1994-1996, 2000, 2001, 2015, 2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::{
    CheckedDiv, DivExact, DivExactAssign, Gcd, Reciprocal,
};
use malachite_base::num::basic::traits::Zero;

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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
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
        if *other == 0u32 {
            panic!("division by zero");
        } else if self == 0u32 {
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
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

impl CheckedDiv<Rational> for Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking both by value. Returns `None` when
    /// the second [`Rational`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::TWO.checked_div(Rational::TWO).unwrap(), 1);
    /// assert_eq!(Rational::TWO.checked_div(Rational::ZERO), None);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7).checked_div(Rational::from_signeds(99, 100)))
    ///         .unwrap()
    ///         .to_string(),
    ///     "200/63"
    /// );
    /// ```
    fn checked_div(self, other: Rational) -> Option<Rational> {
        if other == 0u32 {
            return None;
        } else if self == 0u32 {
            return Some(Rational::ZERO);
        } else if self == 1u32 {
            return Some(other.reciprocal());
        } else if other == 1u32 {
            return Some(self);
        }
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        Some(Rational {
            sign: self.sign == other.sign,
            numerator: (self.numerator).div_exact(&g_1) * (other.denominator).div_exact(&g_2),
            denominator: (other.numerator).div_exact(g_1) * (self.denominator).div_exact(g_2),
        })
    }
}

impl<'a> CheckedDiv<&'a Rational> for Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking the first by value and the second by
    /// reference. Returns `None` when the second [`Rational`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::TWO.checked_div(&Rational::TWO).unwrap(), 1);
    /// assert_eq!(Rational::TWO.checked_div(&Rational::ZERO), None);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7).checked_div(&Rational::from_signeds(99, 100)))
    ///         .unwrap()
    ///         .to_string(),
    ///     "200/63"
    /// );
    /// ```
    #[inline]
    fn checked_div(self, other: &'a Rational) -> Option<Rational> {
        if other == &0u32 {
            None
        } else if self == 0u32 {
            Some(Rational::ZERO)
        } else {
            (other.checked_div(self)).map(Rational::reciprocal)
        }
    }
}

impl<'a> CheckedDiv<Rational> for &'a Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking the first by reference and the second
    /// by value. Returns `None` when the second [`Rational`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::TWO).checked_div(Rational::TWO).unwrap(), 1);
    /// assert_eq!((&Rational::TWO).checked_div(Rational::ZERO), None);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .checked_div(Rational::from_signeds(99, 100))
    ///         .unwrap()
    ///         .to_string(),
    ///     "200/63"
    /// );
    /// ```
    fn checked_div(self, other: Rational) -> Option<Rational> {
        if other == 0u32 {
            return None;
        } else if *self == 0u32 {
            return Some(Rational::ZERO);
        } else if *self == 1u32 {
            return Some(other.reciprocal());
        } else if other == 1u32 {
            return Some(self.clone());
        }
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        Some(Rational {
            sign: self.sign == other.sign,
            numerator: (&self.numerator).div_exact(&g_1) * (other.denominator).div_exact(&g_2),
            denominator: (other.numerator).div_exact(g_1) * (&self.denominator).div_exact(g_2),
        })
    }
}

impl<'a, 'b> CheckedDiv<&'a Rational> for &'b Rational {
    type Output = Rational;

    /// Divides a [`Rational`] by another [`Rational`], taking both by reference. Returns `None`
    /// when the second [`Rational`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::TWO).checked_div(&Rational::TWO).unwrap(), 1);
    /// assert_eq!((&Rational::TWO).checked_div(&Rational::ZERO), None);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .checked_div(&Rational::from_signeds(99, 100))
    ///         .unwrap()
    ///         .to_string(),
    ///     "200/63"
    /// );
    /// ```
    fn checked_div(self, other: &'a Rational) -> Option<Rational> {
        if *other == 0u32 {
            return None;
        } else if *self == 0u32 {
            return Some(Rational::ZERO);
        } else if *self == 1u32 {
            return Some(other.reciprocal());
        } else if *other == 1u32 {
            return Some(self.clone());
        }
        let g_1 = (&self.numerator).gcd(&other.numerator);
        let g_2 = (&other.denominator).gcd(&self.denominator);
        Some(Rational {
            sign: self.sign == other.sign,
            numerator: (&self.numerator).div_exact(&g_1) * (&other.denominator).div_exact(&g_2),
            denominator: (&other.numerator).div_exact(g_1) * (&self.denominator).div_exact(g_2),
        })
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if the second [`Rational`] is zero.
    ///
    /// # Examples
    /// ```
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
