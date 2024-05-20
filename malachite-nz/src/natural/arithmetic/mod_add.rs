// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2019 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign};

impl ModAdd<Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. All three [`Natural`]s are taken by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(Natural::from(3u32), Natural::from(5u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(Natural::from(5u32), Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value.
    #[inline]
    fn mod_add(mut self, other: Natural, m: Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a> ModAdd<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. The first two [`Natural`]s are taken by value and the third by reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(Natural::from(3u32), &Natural::from(5u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(Natural::from(5u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value and `m` is taken by reference.
    #[inline]
    fn mod_add(mut self, other: Natural, m: &'a Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a> ModAdd<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. The first and third [`Natural`]s are taken by value and the second by reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(&Natural::from(3u32), Natural::from(5u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(&Natural::from(5u32), Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value and `c` is taken by reference.
    #[inline]
    fn mod_add(mut self, other: &'a Natural, m: Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a, 'b> ModAdd<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by value and the second and third by reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(&Natural::from(3u32), &Natural::from(5u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(&Natural::from(5u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` is
    /// taken by value and `c` and `m` are taken by reference.
    #[inline]
    fn mod_add(mut self, other: &'a Natural, m: &'b Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a> ModAdd<Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. The first [`Natural`] is taken by reference and the second and third by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO)
    ///         .mod_add(Natural::from(3u32), Natural::from(5u32))
    ///         .to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32))
    ///         .mod_add(Natural::from(5u32), Natural::from(10u32))
    ///         .to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` is
    /// taken by reference and `c` and `m` are taken by value.
    #[inline]
    fn mod_add(self, mut other: Natural, m: Natural) -> Natural {
        other.mod_add_assign(self, m);
        other
    }
}

impl<'a, 'b> ModAdd<Natural, &'b Natural> for &'a Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. The first and third [`Natural`]s are taken by reference and the second by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_add(Natural::from(3u32), &Natural::from(5u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_add(Natural::from(5u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by reference and `c` is taken by value.
    #[inline]
    fn mod_add(self, mut other: Natural, m: &'b Natural) -> Natural {
        other.mod_add_assign(self, m);
        other
    }
}

impl<'a, 'b> ModAdd<&'b Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. The first two [`Natural`]s are taken by reference and the third by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_add(&Natural::from(3u32), Natural::from(5u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_add(&Natural::from(5u32), Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by reference and `m` is taken by value.
    fn mod_add(self, other: &'b Natural, m: Natural) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        let sum = self + other;
        if sum < m {
            sum
        } else {
            sum - m
        }
    }
}

impl<'a, 'b, 'c> ModAdd<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already reduced
    /// modulo $m$. All three [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_add(&Natural::from(3u32), &Natural::from(5u32)),
    ///     3
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_add(&Natural::from(5u32), &Natural::from(10u32)),
    ///     2
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by reference.
    fn mod_add(self, other: &'b Natural, m: &'c Natural) -> Natural {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        let sum = self + other;
        if sum < *m {
            sum
        } else {
            sum - m
        }
    }
}

impl ModAddAssign<Natural, Natural> for Natural {
    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be already
    /// reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by value.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(Natural::from(5u32), Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value and `a == b`.
    fn mod_add_assign(&mut self, other: Natural, m: Natural) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        *self += other;
        if *self >= m {
            *self -= m;
        }
    }
}

impl<'a> ModAddAssign<Natural, &'a Natural> for Natural {
    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by value and the
    /// second by reference.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(Natural::from(5u32), &Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value, `m` is taken by reference, and `a == b`.
    fn mod_add_assign(&mut self, other: Natural, m: &'a Natural) {
        assert!(*self < *m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        *self += other;
        if *self >= *m {
            *self -= m;
        }
    }
}

impl<'a> ModAddAssign<&'a Natural, Natural> for Natural {
    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by reference and
    /// the second by value.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(&Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(&Natural::from(5u32), Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value, `c` is taken by reference, and `a == b`.
    fn mod_add_assign(&mut self, other: &'a Natural, m: Natural) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        *self += other;
        if *self >= m {
            *self -= m;
        }
    }
}

impl<'a, 'b> ModAddAssign<&'a Natural, &'b Natural> for Natural {
    /// Adds two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be already
    /// reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by reference.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(&Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(&Natural::from(5u32), &Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_addN` from `fmpz_mod/add.c`, FLINT 2.7.1, where `b` is
    /// taken by value, `c` and `m` are taken by reference, and `a == b`.
    fn mod_add_assign(&mut self, other: &'a Natural, m: &'b Natural) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        *self += other;
        if *self >= *m {
            *self -= m;
        }
    }
}
