// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd, Lcm, LcmAssign};
use malachite_base::num::basic::traits::Zero;

impl Lcm<Natural> for Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).lcm(Natural::from(5u32)), 15);
    /// assert_eq!(Natural::from(12u32).lcm(Natural::from(90u32)), 180);
    /// ```
    fn lcm(mut self, other: Natural) -> Natural {
        self.lcm_assign(other);
        self
    }
}

impl<'a> Lcm<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two [`Natural`]s, taking the first by value and
    /// the second by reference.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).lcm(&Natural::from(5u32)), 15);
    /// assert_eq!(Natural::from(12u32).lcm(&Natural::from(90u32)), 180);
    /// ```
    #[inline]
    fn lcm(mut self, other: &'a Natural) -> Natural {
        self.lcm_assign(other);
        self
    }
}

impl Lcm<Natural> for &Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two [`Natural`]s, taking the first by reference
    /// and the second by value.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).lcm(Natural::from(5u32)), 15);
    /// assert_eq!((&Natural::from(12u32)).lcm(Natural::from(90u32)), 180);
    /// ```
    #[inline]
    fn lcm(self, mut other: Natural) -> Natural {
        other.lcm_assign(self);
        other
    }
}

impl Lcm<&Natural> for &Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).lcm(&Natural::from(5u32)), 15);
    /// assert_eq!((&Natural::from(12u32)).lcm(&Natural::from(90u32)), 180);
    /// ```
    #[inline]
    fn lcm(self, other: &Natural) -> Natural {
        if *self == 0 || *other == 0 {
            return Natural::ZERO;
        }
        let gcd = self.gcd(other);
        // Division is slower than multiplication, so we choose the arguments to div_exact to be as
        // small as possible. This also allows the special case of lcm(x, y) when x is a multiple of
        // y to be quickly reduced to x.
        if self >= other {
            self * other.div_exact(gcd)
        } else {
            other * self.div_exact(gcd)
        }
    }
}

impl LcmAssign<Natural> for Natural {
    /// Replaces a [`Natural`] by its LCM (least common multiple) with another [`Natural`], taking
    /// the [`Natural`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets \operatorname{lcm}(x, y).
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
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.lcm_assign(Natural::from(5u32));
    /// assert_eq!(x, 15);
    ///
    /// let mut x = Natural::from(12u32);
    /// x.lcm_assign(Natural::from(90u32));
    /// assert_eq!(x, 180);
    /// ```
    #[inline]
    fn lcm_assign(&mut self, mut other: Natural) {
        if *self == 0 {
            return;
        } else if other == 0 {
            *self = Natural::ZERO;
            return;
        }
        let gcd = (&*self).gcd(&other);
        if *self >= other {
            other.div_exact_assign(gcd);
        } else {
            self.div_exact_assign(gcd);
        }
        *self *= other;
    }
}

impl<'a> LcmAssign<&'a Natural> for Natural {
    /// Replaces a [`Natural`] by its LCM (least common multiple) with another [`Natural`], taking
    /// the [`Natural`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets \operatorname{lcm}(x, y).
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
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.lcm_assign(&Natural::from(5u32));
    /// assert_eq!(x, 15);
    ///
    /// let mut x = Natural::from(12u32);
    /// x.lcm_assign(&Natural::from(90u32));
    /// assert_eq!(x, 180);
    /// ```
    #[inline]
    fn lcm_assign(&mut self, other: &'a Natural) {
        if *self == 0 {
            return;
        } else if *other == 0 {
            *self = Natural::ZERO;
            return;
        }
        self.div_exact_assign((&*self).gcd(other));
        *self *= other;
    }
}
