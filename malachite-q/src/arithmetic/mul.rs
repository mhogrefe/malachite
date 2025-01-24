// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1994-1996, 2000-2002 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use core::iter::Product;
use core::ops::{Mul, MulAssign};
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd};
use malachite_base::num::basic::traits::{One, Zero};

impl Mul<Rational> for Rational {
    type Output = Rational;

    /// Multiplies two [`Rational`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = xy.
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

impl Mul<&Rational> for Rational {
    type Output = Rational;

    /// Multiplies two [`Rational`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = xy.
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
    fn mul(self, other: &Rational) -> Rational {
        other * self
    }
}

impl Mul<Rational> for &Rational {
    type Output = Rational;

    /// Multiplies two [`Rational`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = xy.
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

impl Mul<&Rational> for &Rational {
    type Output = Rational;

    /// Multiplies two [`Rational`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = xy.
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
    /// use malachite_base::num::basic::traits::{OneHalf, Two};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::ONE_HALF * &Rational::TWO, 1);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) * &Rational::from_signeds(99, 100)).to_string(),
    ///     "1089/350"
    /// );
    /// ```
    fn mul(self, other: &Rational) -> Rational {
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
    /// Multiplies a [`Rational`] by a [`Rational`] in place, taking the [`Rational`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets xy.
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

impl MulAssign<&Rational> for Rational {
    /// Multiplies a [`Rational`] by a [`Rational`] in place, taking the [`Rational`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
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
    fn mul_assign(&mut self, other: &Rational) {
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

impl Product for Rational {
    /// Multiplies together all the [`Rational`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Rational::sum(xs.map(Rational::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_q::Rational;
    /// use std::iter::Product;
    ///
    /// assert_eq!(
    ///     Rational::product(
    ///         vec_from_str::<Rational>("[1, 2/3, 3/4, 4/5, 5/6, 6/7, 7/8, 8/9, 9/10]")
    ///             .unwrap()
    ///             .into_iter()
    ///     )
    ///     .to_string(),
    ///     "1/5"
    /// );
    /// ```
    fn product<I>(xs: I) -> Rational
    where
        I: Iterator<Item = Rational>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate() {
            if x == 0 {
                return Rational::ZERO;
            }
            let mut p = x;
            for _ in 0..(i + 1).trailing_zeros() {
                p *= stack.pop().unwrap();
            }
            stack.push(p);
        }
        let mut p = Rational::ONE;
        for x in stack.into_iter().rev() {
            p *= x;
        }
        p
    }
}

impl<'a> Product<&'a Rational> for Rational {
    /// Multiplies together all the [`Rational`]s in an iterator of [`Rational`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Rational::sum(xs.map(Rational::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_q::Rational;
    /// use std::iter::Product;
    ///
    /// assert_eq!(
    ///     Rational::product(
    ///         vec_from_str::<Rational>("[1, 2/3, 3/4, 4/5, 5/6, 6/7, 7/8, 8/9, 9/10]")
    ///             .unwrap()
    ///             .iter()
    ///     )
    ///     .to_string(),
    ///     "1/5"
    /// );
    /// ```
    fn product<I>(xs: I) -> Rational
    where
        I: Iterator<Item = &'a Rational>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate() {
            if *x == 0 {
                return Rational::ZERO;
            }
            let mut p = x.clone();
            for _ in 0..(i + 1).trailing_zeros() {
                p *= stack.pop().unwrap();
            }
            stack.push(p);
        }
        let mut p = Rational::ONE;
        for x in stack.into_iter().rev() {
            p *= x;
        }
        p
    }
}
