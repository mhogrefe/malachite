// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign};

impl Gcd<Self> for Integer {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s, taking both by value.
    ///
    /// The GCD is always non-negative, so it is a [`Natural`]. The GCD of 0 and $n$, for any $n$,
    /// is $|n|$; in particular, $\gcd(0, 0) = 0$.
    ///
    /// $$
    /// f(x, y) = \gcd(|x|, |y|).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-12).gcd(Integer::from(90)), 6);
    /// assert_eq!(Integer::from(3).gcd(Integer::from(-5)), 1);
    /// assert_eq!(Integer::from(-7).gcd(Integer::ZERO), 7);
    /// ```
    #[inline]
    fn gcd(self, other: Self) -> Natural {
        self.abs.gcd(other.abs)
    }
}

impl<'a> Gcd<&'a Self> for Integer {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s, taking the first by value
    /// and the second by reference.
    ///
    /// The GCD is always non-negative, so it is a [`Natural`]. The GCD of 0 and $n$, for any $n$,
    /// is $|n|$; in particular, $\gcd(0, 0) = 0$.
    ///
    /// $$
    /// f(x, y) = \gcd(|x|, |y|).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-12).gcd(&Integer::from(90)), 6);
    /// assert_eq!(Integer::from(3).gcd(&Integer::from(-5)), 1);
    /// assert_eq!(Integer::from(-7).gcd(&Integer::ZERO), 7);
    /// ```
    #[inline]
    fn gcd(self, other: &'a Self) -> Natural {
        self.abs.gcd(&other.abs)
    }
}

impl Gcd<Integer> for &Integer {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s, taking the first by
    /// reference and the second by value.
    ///
    /// The GCD is always non-negative, so it is a [`Natural`]. The GCD of 0 and $n$, for any $n$,
    /// is $|n|$; in particular, $\gcd(0, 0) = 0$.
    ///
    /// $$
    /// f(x, y) = \gcd(|x|, |y|).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-12)).gcd(Integer::from(90)), 6);
    /// assert_eq!((&Integer::from(3)).gcd(Integer::from(-5)), 1);
    /// assert_eq!((&Integer::from(-7)).gcd(Integer::ZERO), 7);
    /// ```
    #[inline]
    fn gcd(self, other: Integer) -> Natural {
        other.abs.gcd(&self.abs)
    }
}

impl Gcd<&Integer> for &Integer {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Integer`]s, taking both by reference.
    ///
    /// The GCD is always non-negative, so it is a [`Natural`]. The GCD of 0 and $n$, for any $n$,
    /// is $|n|$; in particular, $\gcd(0, 0) = 0$.
    ///
    /// $$
    /// f(x, y) = \gcd(|x|, |y|).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-12)).gcd(&Integer::from(90)), 6);
    /// assert_eq!((&Integer::from(3)).gcd(&Integer::from(-5)), 1);
    /// assert_eq!((&Integer::from(-7)).gcd(&Integer::ZERO), 7);
    /// ```
    #[inline]
    fn gcd(self, other: &Integer) -> Natural {
        (&self.abs).gcd(&other.abs)
    }
}

impl GcdAssign<Self> for Integer {
    /// Replaces an [`Integer`] with the GCD (greatest common divisor) of it and another
    /// [`Integer`], taking the [`Integer`] on the right-hand side by value.
    ///
    /// The GCD is always non-negative. The GCD of 0 and $n$, for any $n$, is $|n|$; in particular,
    /// $\gcd(0, 0) = 0$.
    ///
    /// $$
    /// x \gets \gcd(|x|, |y|).
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
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-12);
    /// x.gcd_assign(Integer::from(90));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Integer::from(-7);
    /// x.gcd_assign(Integer::ZERO);
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: Self) {
        self.sign = true;
        self.abs.gcd_assign(other.abs);
    }
}

impl<'a> GcdAssign<&'a Self> for Integer {
    /// Replaces an [`Integer`] with the GCD (greatest common divisor) of it and another
    /// [`Integer`], taking the [`Integer`] on the right-hand side by reference.
    ///
    /// The GCD is always non-negative. The GCD of 0 and $n$, for any $n$, is $|n|$; in particular,
    /// $\gcd(0, 0) = 0$.
    ///
    /// $$
    /// x \gets \gcd(|x|, |y|).
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
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-12);
    /// x.gcd_assign(&Integer::from(90));
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Integer::from(-7);
    /// x.gcd_assign(&Integer::ZERO);
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: &'a Self) {
        self.sign = true;
        self.abs.gcd_assign(&other.abs);
    }
}
