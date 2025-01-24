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
use core::ops::{Sub, SubAssign};
use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, Gcd, GcdAssign, NegAssign, UnsignedAbs,
};
use malachite_nz::integer::Integer;

impl Sub<Rational> for Rational {
    type Output = Rational;

    /// Subtracts a [`Rational`] by another [`Rational`], taking both by value.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// assert_eq!(Rational::ONE_HALF - Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) - Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    fn sub(self, other: Rational) -> Rational {
        if self == 0u32 {
            return -other;
        } else if other == 0u32 {
            return self;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let diff_n = Integer::from_sign_and_abs(self.sign, self.numerator * &other.denominator)
                - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            let diff_d = self.denominator * other.denominator;
            Rational {
                sign: diff_n >= 0,
                numerator: diff_n.unsigned_abs(),
                denominator: diff_d,
            }
        } else {
            let reduced_self_d = (self.denominator).div_exact(&gcd);
            let diff_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    self.numerator * (&other.denominator).div_exact(&gcd),
                ) - Integer::from_sign_and_abs(other.sign, other.numerator * &reduced_self_d);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs(),
                    denominator: other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs().div_exact(&gcd),
                    denominator: (other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl Sub<&Rational> for Rational {
    type Output = Rational;

    /// Subtracts a [`Rational`] by another [`Rational`], taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// assert_eq!(Rational::ONE_HALF - &Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (Rational::from_signeds(22, 7) - &Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    #[inline]
    fn sub(self, other: &Rational) -> Rational {
        -(other - self)
    }
}

impl Sub<Rational> for &Rational {
    type Output = Rational;

    /// Subtracts a [`Rational`] by another [`Rational`], taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// assert_eq!(&Rational::ONE_HALF - Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) - Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    fn sub(self, other: Rational) -> Rational {
        if *self == 0u32 {
            return -other;
        } else if other == 0u32 {
            return self.clone();
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let diff_n =
                Integer::from_sign_and_abs(self.sign, &self.numerator * &other.denominator)
                    - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            let diff_d = &self.denominator * other.denominator;
            Rational {
                sign: diff_n >= 0,
                numerator: diff_n.unsigned_abs(),
                denominator: diff_d,
            }
        } else {
            let reduced_self_d = (&self.denominator).div_exact(&gcd);
            let diff_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    &self.numerator * (&other.denominator).div_exact(&gcd),
                ) - Integer::from_sign_and_abs(other.sign, other.numerator * &reduced_self_d);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs(),
                    denominator: other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs().div_exact(&gcd),
                    denominator: (other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl Sub<&Rational> for &Rational {
    type Output = Rational;

    /// Subtracts a [`Rational`] by another [`Rational`], taking both by reference.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// assert_eq!(&Rational::ONE_HALF - &Rational::ONE_HALF, 0);
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7) - &Rational::from_signeds(99, 100)).to_string(),
    ///     "1507/700"
    /// );
    /// ```
    fn sub(self, other: &Rational) -> Rational {
        if *self == 0u32 {
            return -other.clone();
        } else if *other == 0u32 {
            return self.clone();
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            let diff_n =
                Integer::from_sign_and_abs(self.sign, &self.numerator * &other.denominator)
                    - Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            let diff_d = &self.denominator * &other.denominator;
            Rational {
                sign: diff_n >= 0,
                numerator: diff_n.unsigned_abs(),
                denominator: diff_d,
            }
        } else {
            let reduced_self_d = (&self.denominator).div_exact(&gcd);
            let diff_n =
                Integer::from_sign_and_abs(
                    self.sign,
                    &self.numerator * (&other.denominator).div_exact(&gcd),
                ) - Integer::from_sign_and_abs(other.sign, &other.numerator * &reduced_self_d);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            if gcd == 1u32 {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs(),
                    denominator: &other.denominator * reduced_self_d,
                }
            } else {
                Rational {
                    sign: diff_n >= 0,
                    numerator: diff_n.unsigned_abs().div_exact(&gcd),
                    denominator: (&other.denominator).div_exact(gcd) * reduced_self_d,
                }
            }
        }
    }
}

impl SubAssign<Rational> for Rational {
    /// Subtracts a [`Rational`] by another [`Rational`] in place, taking the [`Rational`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets x - y.
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
    /// x -= Rational::ONE_HALF;
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x -= Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "1507/700");
    /// ```
    fn sub_assign(&mut self, other: Rational) {
        if *self == 0u32 {
            *self = -other;
            return;
        } else if other == 0u32 {
            return;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            self.numerator *= &other.denominator;
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            self.sign = diff_n >= 0;
            self.numerator = diff_n.unsigned_abs();
            self.denominator *= other.denominator;
        } else {
            self.denominator.div_exact_assign(&gcd);
            self.numerator *= (&other.denominator).div_exact(&gcd);
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, other.numerator * &self.denominator);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            self.sign = diff_n >= 0;
            if gcd == 1u32 {
                self.numerator = diff_n.unsigned_abs();
                self.denominator *= other.denominator;
            } else {
                self.numerator = diff_n.unsigned_abs().div_exact(&gcd);
                self.denominator *= (other.denominator).div_exact(gcd);
            }
        }
    }
}

impl SubAssign<&Rational> for Rational {
    /// Subtracts a [`Rational`] by another [`Rational`] in place, taking the [`Rational`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets x - y.
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
    /// x -= &Rational::ONE_HALF;
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x -= &Rational::from_signeds(99, 100);
    /// assert_eq!(x.to_string(), "1507/700");
    /// ```
    fn sub_assign(&mut self, other: &Rational) {
        if *self == 0u32 {
            self.clone_from(other);
            self.neg_assign();
            return;
        } else if *other == 0u32 {
            return;
        }
        let mut gcd = (&self.denominator).gcd(&other.denominator);
        if gcd == 1u32 {
            self.numerator *= &other.denominator;
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            self.sign = diff_n >= 0;
            self.numerator = diff_n.unsigned_abs();
            self.denominator *= &other.denominator;
        } else {
            self.denominator.div_exact_assign(&gcd);
            self.numerator *= (&other.denominator).div_exact(&gcd);
            let diff_n = Integer::from_sign_and_abs_ref(self.sign, &self.numerator)
                - Integer::from_sign_and_abs(other.sign, &other.numerator * &self.denominator);
            gcd.gcd_assign(diff_n.unsigned_abs_ref());
            self.sign = diff_n >= 0;
            if gcd == 1u32 {
                self.numerator = diff_n.unsigned_abs();
                self.denominator *= &other.denominator;
            } else {
                self.numerator = diff_n.unsigned_abs().div_exact(&gcd);
                self.denominator *= (&other.denominator).div_exact(gcd);
            }
        }
    }
}
