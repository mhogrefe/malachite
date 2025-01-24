// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1994-1997, 2000, 2001, 2004, 2005 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use core::iter::Sum;
use core::ops::{Add, AddAssign};
use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, Gcd, GcdAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;

impl Add<Rational> for Rational {
    type Output = Rational;

    /// Adds two [`Rational`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF + Rational::ONE_HALF, 1);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) + Rational::from_signeds(99, 100)).to_string(),
    ///     "2893/700"
    /// );
    /// ```
    fn add(self, other: Rational) -> Rational {
        if self == 0u32 {
            return other;
        } else if other == 0u32 {
            return self;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let sum_n = Integer::from_sign_and_abs(self.sign, self.numerator * &other.denominator)
                + Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            let sum_d = self.denominator * other.denominator;
            Rational {
                sign: sum_n >= 0,
                numerator: sum_n.unsigned_abs(),
                denominator: sum_d,
            }
        } else {
            let reduced_self_d = (self.denominator).div_exact(&gcd);
            let sum_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    self.numerator * (&other.denominator).div_exact(&gcd),
                ) + Integer::from_sign_and_abs(other.sign, other.numerator * &reduced_self_d);
            gcd.gcd_assign(sum_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: sum_n >= 0,
                    numerator: sum_n.unsigned_abs(),
                    denominator: other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: sum_n >= 0,
                    numerator: sum_n.unsigned_abs().div_exact(&gcd),
                    denominator: (other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl Add<&Rational> for Rational {
    type Output = Rational;

    /// Adds two [`Rational`]s, taking both by the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE_HALF + &Rational::ONE_HALF, 1);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) + &Rational::from_signeds(99, 100)).to_string(),
    ///     "2893/700"
    /// );
    /// ```
    #[inline]
    fn add(self, other: &Rational) -> Rational {
        other + self
    }
}

impl Add<Rational> for &Rational {
    type Output = Rational;

    /// Adds two [`Rational`]s, taking the first by reference and the second by value
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::ONE_HALF + Rational::ONE_HALF, 1);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) + Rational::from_signeds(99, 100)).to_string(),
    ///     "2893/700"
    /// );
    /// ```
    fn add(self, other: Rational) -> Rational {
        if *self == 0u32 {
            return other;
        } else if other == 0u32 {
            return self.clone();
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let sum_n = Integer::from_sign_and_abs(self.sign, &self.numerator * &other.denominator)
                + Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            let sum_d = &self.denominator * other.denominator;
            Rational {
                sign: sum_n >= 0,
                numerator: sum_n.unsigned_abs(),
                denominator: sum_d,
            }
        } else {
            let reduced_self_d = (&self.denominator).div_exact(&gcd);
            let sum_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    &self.numerator * (&other.denominator).div_exact(&gcd),
                ) + Integer::from_sign_and_abs(other.sign, other.numerator * &reduced_self_d);
            gcd.gcd_assign(sum_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: sum_n >= 0,
                    numerator: sum_n.unsigned_abs(),
                    denominator: other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: sum_n >= 0,
                    numerator: sum_n.unsigned_abs().div_exact(&gcd),
                    denominator: (other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl Add<&Rational> for &Rational {
    type Output = Rational;

    /// Adds two [`Rational`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(&Rational::ONE_HALF + &Rational::ONE_HALF, 1);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) + &Rational::from_signeds(99, 100)).to_string(),
    ///     "2893/700"
    /// );
    /// ```
    fn add(self, other: &Rational) -> Rational {
        if *self == 0u32 {
            return other.clone();
        } else if *other == 0u32 {
            return self.clone();
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let sum_n = Integer::from_sign_and_abs(self.sign, &self.numerator * &other.denominator)
                + Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            let sum_d = &self.denominator * &other.denominator;
            Rational {
                sign: sum_n >= 0,
                numerator: sum_n.unsigned_abs(),
                denominator: sum_d,
            }
        } else {
            let reduced_self_d = (&self.denominator).div_exact(&gcd);
            let sum_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    &self.numerator * (&other.denominator).div_exact(&gcd),
                ) + Integer::from_sign_and_abs(other.sign, &other.numerator * &reduced_self_d);
            gcd.gcd_assign(sum_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: sum_n >= 0,
                    numerator: sum_n.unsigned_abs(),
                    denominator: &other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: sum_n >= 0,
                    numerator: sum_n.unsigned_abs().div_exact(&gcd),
                    denominator: (&other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl AddAssign<Rational> for Rational {
    /// Adds a [`Rational`] to a [`Rational`] in place, taking the [`Rational`] on the right-hand
    /// side by value.
    ///
    /// $$
    /// x \gets x + y.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x += Rational::ONE_HALF;
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x += Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "2893/700");
    /// ```
    fn add_assign(&mut self, other: Rational) {
        if *self == 0u32 {
            *self = other;
            return;
        } else if other == 0u32 {
            return;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            self.numerator *= &other.denominator;
            let sum_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                + Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            self.sign = sum_n >= 0;
            self.numerator = sum_n.unsigned_abs();
            self.denominator *= other.denominator;
        } else {
            self.denominator.div_exact_assign(&gcd);
            self.numerator *= (&other.denominator).div_exact(&gcd);
            let sum_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                + Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            gcd.gcd_assign(sum_n.unsigned_abs_ref());
            self.sign = sum_n >= 0;
            if gcd == 1u32 {
                self.numerator = sum_n.unsigned_abs();
                self.denominator *= other.denominator;
            } else {
                self.numerator = sum_n.unsigned_abs().div_exact(&gcd);
                self.denominator *= (other.denominator).div_exact(gcd);
            }
        }
    }
}

impl AddAssign<&Rational> for Rational {
    /// Adds a [`Rational`] to a [`Rational`] in place, taking the [`Rational`] on the right-hand
    /// side by reference.
    ///
    /// $$
    /// x \gets x + y.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x += &Rational::ONE_HALF;
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x += &Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "2893/700");
    /// ```
    fn add_assign(&mut self, other: &Rational) {
        if *self == 0u32 {
            self.clone_from(other);
            return;
        } else if *other == 0u32 {
            return;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            self.numerator *= &other.denominator;
            let sum_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                + Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            self.sign = sum_n >= 0;
            self.numerator = sum_n.unsigned_abs();
            self.denominator *= &other.denominator;
        } else {
            self.denominator.div_exact_assign(&gcd);
            self.numerator *= (&other.denominator).div_exact(&gcd);
            let sum_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                + Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            gcd.gcd_assign(sum_n.unsigned_abs_ref());
            self.sign = sum_n >= 0;
            if gcd == 1u32 {
                self.numerator = sum_n.unsigned_abs();
                self.denominator *= &other.denominator;
            } else {
                self.numerator = sum_n.unsigned_abs().div_exact(&gcd);
                self.denominator *= (&other.denominator).div_exact(gcd);
            }
        }
    }
}

impl Sum for Rational {
    /// Adds up all the [`Rational`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
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
    /// use std::iter::Sum;
    ///
    /// assert_eq!(
    ///     Rational::sum(
    ///         vec_from_str::<Rational>("[0, 1, 2/3, 3/4, 4/5, 5/6, 6/7, 7/8, 8/9, 9/10]")
    ///             .unwrap()
    ///             .into_iter()
    ///     )
    ///     .to_string(),
    ///     "19079/2520"
    /// );
    /// ```
    fn sum<I>(xs: I) -> Rational
    where
        I: Iterator<Item = Rational>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate() {
            let mut s = x;
            for _ in 0..(i + 1).trailing_zeros() {
                s += stack.pop().unwrap();
            }
            stack.push(s);
        }
        let mut s = Rational::ZERO;
        for x in stack.into_iter().rev() {
            s += x;
        }
        s
    }
}

impl<'a> Sum<&'a Rational> for Rational {
    /// Adds up all the [`Rational`]s in an iterator of [`Rational`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
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
    /// use std::iter::Sum;
    ///
    /// assert_eq!(
    ///     Rational::sum(
    ///         vec_from_str::<Rational>("[0, 1, 2/3, 3/4, 4/5, 5/6, 6/7, 7/8, 8/9, 9/10]")
    ///             .unwrap()
    ///             .iter()
    ///     )
    ///     .to_string(),
    ///     "19079/2520"
    /// );
    /// ```
    fn sum<I>(xs: I) -> Rational
    where
        I: Iterator<Item = &'a Rational>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate() {
            let mut s = x.clone();
            for _ in 0..(i + 1).trailing_zeros() {
                s += stack.pop().unwrap();
            }
            stack.push(s);
        }
        let mut s = Rational::ZERO;
        for x in stack.into_iter().rev() {
            s += x;
        }
        s
    }
}
