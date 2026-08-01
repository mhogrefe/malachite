// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign, SubMul, SubMulAssign};

impl MulSubMul<Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking all
    /// four by value.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<Self, Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking $x$,
    /// $y$ and $z$ by value and $w$ by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: Self, w: &Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<Self, &Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking $x$,
    /// $y$ and $w$ by value and $z$ by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking $x$
    /// and $y$ by value and $z$ and $w$ by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: Self, z: &Self, w: &Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<&Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking $x$,
    /// $z$ and $w$ by value and $y$ by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<&Self, Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking $x$
    /// and $z$ by value and $y$ and $w$ by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: Self, w: &Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<&Self, &Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking $x$
    /// and $w$ by value and $y$ and $z$ by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<&Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking $x$ by
    /// value and $y$, $z$ and $w$ by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Self, z: &Self, w: &Self) -> Self {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMul<&Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, taking all
    /// four by reference.
    ///
    /// $f(x, y, z, w) = xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// ```
    #[inline]
    fn mul_sub_mul(self, y: &Natural, z: &Natural, w: &Natural) -> Natural {
        (self * y).sub_mul(z, w)
    }
}

impl MulSubMulAssign<Self, Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking all four by value.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(Natural::from(3u32), Natural::from(4u32), Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}

impl MulSubMulAssign<Self, Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking $x$, $y$ and $z$ by value and $w$ by reference.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(Natural::from(3u32), Natural::from(4u32), &Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: Self, w: &Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}

impl MulSubMulAssign<Self, &Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking $x$, $y$ and $w$ by value and $z$ by reference.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(Natural::from(3u32), &Natural::from(4u32), Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}

impl MulSubMulAssign<Self, &Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking $x$ and $y$ by value and $z$ and $w$ by reference.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(Natural::from(3u32), &Natural::from(4u32), &Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: &Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}

impl MulSubMulAssign<&Self, Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking $x$, $z$ and $w$ by value and $y$ by reference.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(&Natural::from(3u32), Natural::from(4u32), Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}

impl MulSubMulAssign<&Self, Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking $x$ and $z$ by value and $y$ and $w$ by reference.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(&Natural::from(3u32), Natural::from(4u32), &Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: &Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}

impl MulSubMulAssign<&Self, &Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking $x$ and $w$ by value and $y$ and $z$ by reference.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32), Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}

impl MulSubMulAssign<&Self, &Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// taking $x$ by value and $y$, $z$ and $w$ by reference.
    ///
    /// $x \gets xy - zw$.
    ///
    /// # Panics
    /// Panics if $zw > xy$.
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
    /// use malachite_base::num::arithmetic::traits::{MulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mul_sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32), &Natural::from(5u32));
    /// assert_eq!(x, 10);
    /// ```
    #[inline]
    fn mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: &Self) {
        *self *= y;
        self.sub_mul_assign(z, w);
    }
}
