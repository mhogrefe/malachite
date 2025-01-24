// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use alloc::vec::Vec;
use core::iter::Product;
use core::ops::{Mul, MulAssign};
use malachite_base::num::basic::traits::{One, Zero};

impl Mul<Integer> for Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ONE * Integer::from(123), 123);
    /// assert_eq!(Integer::from(123) * Integer::ZERO, 0);
    /// assert_eq!(Integer::from(123) * Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (Integer::from(-123456789000i64) * Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(mut self, other: Integer) -> Integer {
        self *= other;
        self
    }
}

impl Mul<&Integer> for Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ONE * &Integer::from(123), 123);
    /// assert_eq!(Integer::from(123) * &Integer::ZERO, 0);
    /// assert_eq!(Integer::from(123) * &Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (Integer::from(-123456789000i64) * &Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(mut self, other: &Integer) -> Integer {
        self *= other;
        self
    }
}

impl Mul<Integer> for &Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::ONE * Integer::from(123), 123);
    /// assert_eq!(&Integer::from(123) * Integer::ZERO, 0);
    /// assert_eq!(&Integer::from(123) * Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (&Integer::from(-123456789000i64) * Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(self, mut other: Integer) -> Integer {
        other *= self;
        other
    }
}

impl Mul<&Integer> for &Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::ONE * &Integer::from(123), 123);
    /// assert_eq!(&Integer::from(123) * &Integer::ZERO, 0);
    /// assert_eq!(&Integer::from(123) * &Integer::from(-456), -56088);
    /// assert_eq!(
    ///     (&Integer::from(-123456789000i64) * &Integer::from(-987654321000i64)).to_string(),
    ///     "121932631112635269000000"
    /// );
    /// ```
    fn mul(self, other: &Integer) -> Integer {
        let product_abs = &self.abs * &other.abs;
        Integer {
            sign: self.sign == other.sign || product_abs == 0,
            abs: product_abs,
        }
    }
}

impl MulAssign<Integer> for Integer {
    /// Multiplies an [`Integer`] by an [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::NEGATIVE_ONE;
    /// x *= Integer::from(1000);
    /// x *= Integer::from(2000);
    /// x *= Integer::from(3000);
    /// x *= Integer::from(4000);
    /// assert_eq!(x, -24000000000000i64);
    /// ```
    fn mul_assign(&mut self, other: Integer) {
        self.abs *= other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}

impl MulAssign<&Integer> for Integer {
    /// Multiplies an [`Integer`] by an [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::NEGATIVE_ONE;
    /// x *= &Integer::from(1000);
    /// x *= &Integer::from(2000);
    /// x *= &Integer::from(3000);
    /// x *= &Integer::from(4000);
    /// assert_eq!(x, -24000000000000i64);
    /// ```
    fn mul_assign(&mut self, other: &Integer) {
        self.abs *= &other.abs;
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}

impl Product for Integer {
    /// Multiplies together all the [`Integer`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Integer::sum(xs.map(Integer::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::product(
    ///         vec_from_str::<Integer>("[2, -3, 5, 7]")
    ///             .unwrap()
    ///             .into_iter()
    ///     ),
    ///     -210
    /// );
    /// ```
    fn product<I>(xs: I) -> Integer
    where
        I: Iterator<Item = Integer>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate().map(|(i, x)| (i + 1, x)) {
            if x == 0 {
                return Integer::ZERO;
            }
            let mut p = x;
            for _ in 0..i.trailing_zeros() {
                p *= stack.pop().unwrap();
            }
            stack.push(p);
        }
        let mut p = Integer::ONE;
        for x in stack.into_iter().rev() {
            p *= x;
        }
        p
    }
}

impl<'a> Product<&'a Integer> for Integer {
    /// Multiplies together all the [`Integer`]s in an iterator of [`Integer`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Integer::sum(xs.map(Integer::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::product(vec_from_str::<Integer>("[2, -3, 5, 7]").unwrap().iter()),
    ///     -210
    /// );
    /// ```
    fn product<I>(xs: I) -> Integer
    where
        I: Iterator<Item = &'a Integer>,
    {
        let mut stack = Vec::new();
        for (i, x) in xs.enumerate().map(|(i, x)| (i + 1, x)) {
            if *x == 0 {
                return Integer::ZERO;
            }
            let mut p = x.clone();
            for _ in 0..i.trailing_zeros() {
                p *= stack.pop().unwrap();
            }
            stack.push(p);
        }
        let mut p = Integer::ONE;
        for x in stack.into_iter().rev() {
            p *= x;
        }
        p
    }
}
