// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ModNeg, ModNegAssign};
use malachite_base::num::basic::traits::Zero;

impl ModNeg<Natural> for Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by value.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $-x \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModNeg, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_neg(Natural::from(5u32)), 0);
    /// assert_eq!(Natural::from(7u32).mod_neg(Natural::from(10u32)), 3);
    /// assert_eq!(
    ///     Natural::from(7u32).mod_neg(Natural::from(10u32).pow(12)),
    ///     999999999993u64
    /// );
    /// ```
    #[inline]
    fn mod_neg(mut self, m: Natural) -> Natural {
        self.mod_neg_assign(&m);
        self
    }
}

impl<'a> ModNeg<&'a Natural> for Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by value and the second by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModNeg, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_neg(&Natural::from(5u32)), 0);
    /// assert_eq!(Natural::from(7u32).mod_neg(&Natural::from(10u32)), 3);
    /// assert_eq!(
    ///     Natural::from(7u32).mod_neg(&Natural::from(10u32).pow(12)),
    ///     999999999993u64
    /// );
    /// ```
    #[inline]
    fn mod_neg(mut self, m: &'a Natural) -> Natural {
        self.mod_neg_assign(m);
        self
    }
}

impl<'a> ModNeg<Natural> for &'a Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by reference and the second by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModNeg, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_neg(Natural::from(5u32)), 0);
    /// assert_eq!((&Natural::from(7u32)).mod_neg(Natural::from(10u32)), 3);
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_neg(Natural::from(10u32).pow(12)),
    ///     999999999993u64
    /// );
    /// ```
    fn mod_neg(self, m: Natural) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        if *self == 0 {
            Natural::ZERO
        } else {
            m - self
        }
    }
}

impl<'a, 'b> ModNeg<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Negates a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModNeg, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).mod_neg(&Natural::from(5u32)), 0);
    /// assert_eq!((&Natural::from(7u32)).mod_neg(&Natural::from(10u32)), 3);
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_neg(&Natural::from(10u32).pow(12)),
    ///     999999999993u64
    /// );
    /// ```
    fn mod_neg(self, m: &'b Natural) -> Natural {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        if *self == 0 {
            Natural::ZERO
        } else {
            m - self
        }
    }
}

impl ModNegAssign<Natural> for Natural {
    /// Negates a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// $x \gets y$, where $x, y < m$ and $-x \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModNegAssign, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_neg_assign(Natural::from(5u32));
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(Natural::from(10u32));
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(Natural::from(10u32).pow(12));
    /// assert_eq!(n, 999999999993u64);
    /// ```
    #[inline]
    fn mod_neg_assign(&mut self, m: Natural) {
        self.mod_neg_assign(&m);
    }
}

impl<'a> ModNegAssign<&'a Natural> for Natural {
    /// Negates a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// $x \gets y$, where $x, y < m$ and $-x \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModNegAssign, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_neg_assign(&Natural::from(5u32));
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(&Natural::from(10u32));
    /// assert_eq!(n, 3);
    ///
    /// let mut n = Natural::from(7u32);
    /// n.mod_neg_assign(&Natural::from(10u32).pow(12));
    /// assert_eq!(n, 999999999993u64);
    /// ```
    fn mod_neg_assign(&mut self, m: &'a Natural) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        if *self != 0 {
            assert!(!self.sub_right_assign_no_panic(m));
        }
    }
}
