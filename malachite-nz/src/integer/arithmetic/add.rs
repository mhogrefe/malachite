// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::iter::Sum;
use core::mem::swap;
use core::ops::{Add, AddAssign};
use malachite_base::num::basic::traits::Zero;

impl Add<Self> for Integer {
    type Output = Self;

    /// Adds two [`Integer`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO + Integer::from(123), 123);
    /// assert_eq!(Integer::from(-123) + Integer::ZERO, -123);
    /// assert_eq!(Integer::from(-123) + Integer::from(456), 333);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) + (Integer::from(10u32).pow(12) << 1),
    ///     1000000000000u64
    /// );
    /// ```
    fn add(mut self, mut other: Self) -> Self {
        if self.abs.limb_count() >= other.abs.limb_count() {
            self += other;
            self
        } else {
            other += self;
            other
        }
    }
}

impl Add<&Self> for Integer {
    type Output = Self;

    /// Adds two [`Integer`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO + &Integer::from(123), 123);
    /// assert_eq!(Integer::from(-123) + &Integer::ZERO, -123);
    /// assert_eq!(Integer::from(-123) + &Integer::from(456), 333);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) + &(Integer::from(10u32).pow(12) << 1),
    ///     1000000000000u64
    /// );
    /// ```
    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}

impl Add<Integer> for &Integer {
    type Output = Integer;

    /// Adds two [`Integer`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::ZERO + Integer::from(123), 123);
    /// assert_eq!(&Integer::from(-123) + Integer::ZERO, -123);
    /// assert_eq!(&Integer::from(-123) + Integer::from(456), 333);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) + (Integer::from(10u32).pow(12) << 1),
    ///     1000000000000u64
    /// );
    /// ```
    #[inline]
    fn add(self, mut other: Integer) -> Integer {
        other += self;
        other
    }
}

impl Add<&Integer> for &Integer {
    type Output = Integer;

    /// Adds two [`Integer`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::ZERO + &Integer::from(123), 123);
    /// assert_eq!(&Integer::from(-123) + &Integer::ZERO, -123);
    /// assert_eq!(&Integer::from(-123) + &Integer::from(456), 333);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) + &(Integer::from(10u32).pow(12) << 1),
    ///     1000000000000u64
    /// );
    /// ```
    fn add(self, other: &Integer) -> Integer {
        match (self, other) {
            (x, y) if core::ptr::eq(x, y) => x << 1,
            (&integer_zero!(), y) => y.clone(),
            (x, &integer_zero!()) => x.clone(),
            // e.g. 10 + 5 or -10 + -5; sign of result is sign of self
            (
                &Integer {
                    sign: sx,
                    abs: ref ax,
                },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (sy && *ay != 0) => Integer {
                sign: sx,
                abs: ax + ay,
            },
            // e.g. 10 + -5, -10 + 5, or 5 + -5; sign of result is sign of self
            (
                &Integer {
                    sign: sx,
                    abs: ref ax,
                },
                Integer { abs: ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => Integer {
                sign: sx,
                abs: ax - ay,
            },
            // e.g. 5 + -10, -5 + 10, or -5 + 5; sign of result is sign of other
            (
                Integer { abs: ax, .. },
                &Integer {
                    sign: sy,
                    abs: ref ay,
                },
            ) => Integer {
                sign: sy,
                abs: ay - ax,
            },
        }
    }
}

impl AddAssign<Self> for Integer {
    /// Adds an [`Integer`] to an [`Integer`] in place, taking the [`Integer`] on the right-hand
    /// side by value.
    ///
    /// $$
    /// x \gets x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x += -Integer::from(10u32).pow(12);
    /// x += Integer::from(10u32).pow(12) * Integer::from(2u32);
    /// x += -Integer::from(10u32).pow(12) * Integer::from(3u32);
    /// x += Integer::from(10u32).pow(12) * Integer::from(4u32);
    /// assert_eq!(x, 2000000000000u64);
    /// ```
    fn add_assign(&mut self, mut other: Self) {
        match (&mut *self, &other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), _) => {
                *self = other;
            }
            // e.g. 10 += 5 or -10 += -5; sign of self is unchanged
            (
                &mut Self {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Self {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (sy && *ay != 0) => *ax += ay,
            // e.g. 10 += -5, -10 += 5, or 5 += -5; sign of self is unchanged
            (
                &mut Self {
                    sign: sx,
                    abs: ref mut ax,
                },
                Self { abs: ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            // e.g. 5 += -10, -5 += 10, or -5 += 5; sign of self is flipped
            _ => {
                swap(self, &mut other);
                self.abs -= other.abs;
            }
        };
    }
}

impl AddAssign<&Self> for Integer {
    /// Adds an [`Integer`] to an [`Integer`] in place, taking the [`Integer`] on the right-hand
    /// side by reference.
    ///
    /// $$
    /// x \gets x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x += &(-Integer::from(10u32).pow(12));
    /// x += &(Integer::from(10u32).pow(12) * Integer::from(2u32));
    /// x += &(-Integer::from(10u32).pow(12) * Integer::from(3u32));
    /// x += &(Integer::from(10u32).pow(12) * Integer::from(4u32));
    /// assert_eq!(x, 2000000000000u64);
    /// ```
    fn add_assign(&mut self, other: &Self) {
        match (&mut *self, other) {
            (_, &integer_zero!()) => {}
            (&mut integer_zero!(), _) => {
                *self = other.clone();
            }
            // e.g. 10 += 5 or -10 += -5; sign of self is unchanged
            (
                &mut Self {
                    sign: sx,
                    abs: ref mut ax,
                },
                &Self {
                    sign: sy,
                    abs: ref ay,
                },
            ) if sx == (sy && *ay != 0) => *ax += ay,
            // e.g. 10 += -5, -10 += 5, or 5 += -5; sign of self is unchanged
            (
                &mut Self {
                    sign: sx,
                    abs: ref mut ax,
                },
                Self { abs: ay, .. },
            ) if sx && *ax == *ay || *ax > *ay => *ax -= ay,
            // e.g. 5 += -10, -5 += 10, or -5 += 5; sign of self is flipped
            (
                &mut Self {
                    sign: ref mut sx,
                    abs: ref mut ax,
                },
                &Self {
                    sign: sy,
                    abs: ref ay,
                },
            ) => {
                *sx = sy;
                ax.sub_right_assign_no_panic(ay);
            }
        };
    }
}

impl Sum for Integer {
    /// Adds up all the [`Integer`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Integer::sum(xs.map(Integer::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::sum(
    ///         vec_from_str::<Integer>("[2, -3, 5, 7]")
    ///             .unwrap()
    ///             .into_iter()
    ///     ),
    ///     11
    /// );
    /// ```
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut s = Self::ZERO;
        for x in xs {
            s += x;
        }
        s
    }
}

impl<'a> Sum<&'a Self> for Integer {
    /// Adds up all the [`Integer`]s in an iterator of [`Integer`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Integer::sum(xs.map(Integer::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::sum(vec_from_str::<Integer>("[2, -3, 5, 7]").unwrap().iter()),
    ///     11
    /// );
    /// ```
    fn sum<I>(xs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let mut s = Self::ZERO;
        for x in xs {
            s += x;
        }
        s
    }
}
