// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ModSub, ModSubAssign};

impl ModSub<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. All three [`Natural`]s are taken by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32)
    ///         .mod_sub(Natural::from(3u32), Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32)
    ///         .mod_sub(Natural::from(9u32), Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value.
    #[inline]
    fn mod_sub(mut self, other: Natural, m: Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl ModSub<Natural, &Natural> for Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first two [`Natural`]s are taken by value and the third by
    /// reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32)
    ///         .mod_sub(Natural::from(3u32), &Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32)
    ///         .mod_sub(Natural::from(9u32), &Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value and `m` is taken by reference.
    #[inline]
    fn mod_sub(mut self, other: Natural, m: &Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl ModSub<&Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first and third [`Natural`]s are taken by value and the second by
    /// reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32)
    ///         .mod_sub(&Natural::from(3u32), Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32)
    ///         .mod_sub(&Natural::from(9u32), Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This isequivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value and `c` is taken by reference.
    #[inline]
    fn mod_sub(mut self, other: &Natural, m: Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl ModSub<&Natural, &Natural> for Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] is taken by value and the second and third by
    /// reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32)
    ///         .mod_sub(&Natural::from(3u32), &Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32)
    ///         .mod_sub(&Natural::from(9u32), &Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` is
    /// taken by value and `c` and `m` are taken by reference.
    #[inline]
    fn mod_sub(mut self, other: &Natural, m: &Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl ModSub<Natural, Natural> for &Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first [`Natural`] is taken by reference and the second and third by
    /// value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32))
    ///         .mod_sub(Natural::from(3u32), Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32))
    ///         .mod_sub(Natural::from(9u32), Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` is
    /// taken by reference and `c` and `m` are taken by value.
    fn mod_sub(self, other: Natural, m: Natural) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        if *self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl ModSub<Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first and third [`Natural`]s are taken by reference and the second
    /// by value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32))
    ///         .mod_sub(Natural::from(3u32), &Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32))
    ///         .mod_sub(Natural::from(9u32), &Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by reference and `c` is taken by value.
    fn mod_sub(self, other: Natural, m: &Natural) -> Natural {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        if *self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl ModSub<&Natural, Natural> for &Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. The first two [`Natural`]s are taken by reference and the third by
    /// value.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32))
    ///         .mod_sub(&Natural::from(3u32), Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32))
    ///         .mod_sub(&Natural::from(9u32), Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by reference and `m` is taken by value.
    fn mod_sub(self, other: &Natural, m: Natural) -> Natural {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        if self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl ModSub<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$. The inputs must be already
    /// reduced modulo $m$. All three [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32))
    ///         .mod_sub(&Natural::from(3u32), &Natural::from(5u32))
    ///         .to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32))
    ///         .mod_sub(&Natural::from(9u32), &Natural::from(10u32))
    ///         .to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by reference.
    fn mod_sub(self, other: &Natural, m: &Natural) -> Natural {
        assert!(self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        if self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl ModSubAssign<Natural, Natural> for Natural {
    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by value.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(Natural::from(9u32), Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b`, `c`,
    /// and `m` are taken by value and `a == b`.
    fn mod_sub_assign(&mut self, other: Natural, m: Natural) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        if *self >= other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}

impl ModSubAssign<Natural, &Natural> for Natural {
    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by value
    /// and the second by reference.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(Natural::from(9u32), &Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` and `c`
    /// are taken by value, `m` is taken by reference, and `a == b`.
    fn mod_sub_assign(&mut self, other: Natural, m: &Natural) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            other < *m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        if *self >= other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}

impl ModSubAssign<&Natural, Natural> for Natural {
    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. The first [`Natural`] on the right-hand side is taken by
    /// reference and the second by value.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(&Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(&Natural::from(9u32), Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` and `m`
    /// are taken by value, `c` is taken by reference, and `a == b`.
    fn mod_sub_assign(&mut self, other: &Natural, m: Natural) {
        assert!(*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(
            *other < m,
            "other must be reduced mod m, but {other} >= {m}"
        );
        if *self >= *other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}

impl ModSubAssign<&Natural, &Natural> for Natural {
    /// Subtracts two [`Natural`]s modulo a third [`Natural`] $m$, in place. The inputs must be
    /// already reduced modulo $m$. Both [`Natural`]s on the right-hand side are taken by reference.
    ///
    /// $x \gets z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(&Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(&Natural::from(9u32), &Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_subN` from `fmpz_mod/sub.c`, FLINT 2.7.1, where `b` is
    /// taken by value, `c` and `m` are taken by reference, and `a == b`.
    fn mod_sub_assign(&mut self, other: &Natural, m: &Natural) {
        assert!(&*self < m, "self must be reduced mod m, but {self} >= {m}");
        assert!(other < m, "other must be reduced mod m, but {other} >= {m}");
        if *self >= *other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}
