// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, MulAddMul, MulAddMulAssign};

impl MulAddMul<Self, Self, Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking all four by value.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         Integer::from(3),
    ///         Integer::from(4),
    ///         Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<Self, Self, &Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking $x$, $y$ and $z$ by value and $w$ by
    /// reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         Integer::from(3),
    ///         Integer::from(4),
    ///         &Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: Self, w: &Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<Self, &Self, Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking $x$, $y$ and $w$ by value and $z$ by
    /// reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         Integer::from(3),
    ///         &Integer::from(4),
    ///         Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<Self, &Self, &Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking $x$ and $y$ by value and $z$ and $w$
    /// by reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         Integer::from(3),
    ///         &Integer::from(4),
    ///         &Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: Self, z: &Self, w: &Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<&Self, Self, Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking $x$, $z$ and $w$ by value and $y$ by
    /// reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         &Integer::from(3),
    ///         Integer::from(4),
    ///         Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<&Self, Self, &Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking $x$ and $z$ by value and $y$ and $w$
    /// by reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         &Integer::from(3),
    ///         Integer::from(4),
    ///         &Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: Self, w: &Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<&Self, &Self, Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking $x$ and $w$ by value and $y$ and $z$
    /// by reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         &Integer::from(3),
    ///         &Integer::from(4),
    ///         Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<&Self, &Self, &Self> for Integer {
    type Output = Self;

    /// Adds the products of two pairs of [`Integer`]s, taking $x$ by value and $y$, $z$ and $w$ by
    /// reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(-10).mul_add_mul(
    ///         &Integer::from(3),
    ///         &Integer::from(4),
    ///         &Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Self, z: &Self, w: &Self) -> Self {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMul<&Integer, &Integer, &Integer> for &Integer {
    type Output = Integer;

    /// Adds the products of two pairs of [`Integer`]s, taking all four by reference.
    ///
    /// $f(x, y, z, w) = xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-10)).mul_add_mul(
    ///         &Integer::from(3),
    ///         &Integer::from(4),
    ///         &Integer::from(5)
    ///     ),
    ///     -10
    /// );
    /// ```
    #[inline]
    fn mul_add_mul(self, y: &Integer, z: &Integer, w: &Integer) -> Integer {
        (self * y).add_mul(z, w)
    }
}

impl MulAddMulAssign<Self, Self, Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking all four by value.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(Integer::from(3), Integer::from(4), Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}

impl MulAddMulAssign<Self, Self, &Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking $x$, $y$ and $z$ by value
    /// and $w$ by reference.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(Integer::from(3), Integer::from(4), &Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: Self, w: &Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}

impl MulAddMulAssign<Self, &Self, Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking $x$, $y$ and $w$ by value
    /// and $z$ by reference.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(Integer::from(3), &Integer::from(4), Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}

impl MulAddMulAssign<Self, &Self, &Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking $x$ and $y$ by value and
    /// $z$ and $w$ by reference.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(Integer::from(3), &Integer::from(4), &Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: Self, z: &Self, w: &Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}

impl MulAddMulAssign<&Self, Self, Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking $x$, $z$ and $w$ by value
    /// and $y$ by reference.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(&Integer::from(3), Integer::from(4), Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}

impl MulAddMulAssign<&Self, Self, &Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking $x$ and $z$ by value and
    /// $y$ and $w$ by reference.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(&Integer::from(3), Integer::from(4), &Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: Self, w: &Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}

impl MulAddMulAssign<&Self, &Self, Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking $x$ and $w$ by value and
    /// $y$ and $z$ by reference.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(&Integer::from(3), &Integer::from(4), Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}

impl MulAddMulAssign<&Self, &Self, &Self> for Integer {
    /// Adds the products of two pairs of [`Integer`]s, in place, taking $x$ by value and $y$, $z$
    /// and $w$ by reference.
    ///
    /// $x \gets xy + zw$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{MulAddMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-10);
    /// x.mul_add_mul_assign(&Integer::from(3), &Integer::from(4), &Integer::from(5));
    /// assert_eq!(x, -10);
    /// ```
    #[inline]
    fn mul_add_mul_assign(&mut self, y: &Self, z: &Self, w: &Self) {
        *self *= y;
        self.add_mul_assign(z, w);
    }
}
