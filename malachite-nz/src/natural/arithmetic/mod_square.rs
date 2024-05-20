// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign, ModSquare, ModSquareAssign};
use malachite_base::num::basic::traits::Two;

impl ModSquare<Natural> for Natural {
    type Output = Natural;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by value.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(2u32).mod_square(Natural::from(10u32)), 4);
    /// assert_eq!(Natural::from(100u32).mod_square(Natural::from(497u32)), 60);
    /// ```
    fn mod_square(self, m: Natural) -> Natural {
        (&self).mod_pow(&Natural::TWO, &m)
    }
}

impl<'a> ModSquare<&'a Natural> for Natural {
    type Output = Natural;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by value and the second by reference.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(2u32).mod_square(&Natural::from(10u32)), 4);
    /// assert_eq!(Natural::from(100u32).mod_square(&Natural::from(497u32)), 60);
    /// ```
    fn mod_square(self, m: &'a Natural) -> Natural {
        (&self).mod_pow(&Natural::TWO, m)
    }
}

impl<'a> ModSquare<Natural> for &'a Natural {
    type Output = Natural;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by reference and the second by value.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(2u32)).mod_square(Natural::from(10u32)), 4);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_square(Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    fn mod_square(self, m: Natural) -> Natural {
        self.mod_pow(&Natural::TWO, &m)
    }
}

impl<'a, 'b> ModSquare<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Squares a [`Natural`] modulo another [`Natural`] $m$. The input must be already reduced
    /// modulo $m$. Both [`Natural`]s are taken by reference.
    ///
    /// $f(x, m) = y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(2u32)).mod_square(&Natural::from(10u32)), 4);
    /// assert_eq!(
    ///     (&Natural::from(100u32)).mod_square(&Natural::from(497u32)),
    ///     60
    /// );
    /// ```
    fn mod_square(self, m: &'b Natural) -> Natural {
        self.mod_pow(&Natural::TWO, m)
    }
}

impl ModSquareAssign<Natural> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$, in place. The input must be already
    /// reduced modulo $m$. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_assign(Natural::from(10u32));
    /// assert_eq!(x, 4);
    ///
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_assign(Natural::from(497u32));
    /// assert_eq!(x, 60);
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: Natural) {
        self.mod_pow_assign(&Natural::TWO, &m);
    }
}

impl<'a> ModSquareAssign<&'a Natural> for Natural {
    /// Squares a [`Natural`] modulo another [`Natural`] $m$, in place. The input must be already
    /// reduced modulo $m$. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// $x \gets y$, where $x, y < m$ and $x^2 \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_assign(&Natural::from(10u32));
    /// assert_eq!(x, 4);
    ///
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_assign(&Natural::from(497u32));
    /// assert_eq!(x, 60);
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: &'a Natural) {
        self.mod_pow_assign(&Natural::TWO, m);
    }
}
