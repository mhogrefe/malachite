// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};

impl DivExact<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value. The first [`Integer`]
    /// must be exactly divisible by the second. If it isn't, this function may panic or return a
    /// meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -123 * 456 = -56088
    /// assert_eq!(Integer::from(-56088).div_exact(Integer::from(456)), -123);
    ///
    /// // -123456789000 * -987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     Integer::from_str("121932631112635269000000")
    ///         .unwrap()
    ///         .div_exact(Integer::from_str("-987654321000").unwrap()),
    ///     -123456789000i64
    /// );
    /// ```
    #[inline]
    fn div_exact(mut self, other: Integer) -> Integer {
        self.div_exact_assign(other);
        self
    }
}

impl<'a> DivExact<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference. The first [`Integer`] must be exactly divisible by the second. If it isn't, this
    /// function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self / &other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(&other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round(&other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -123 * 456 = -56088
    /// assert_eq!(Integer::from(-56088).div_exact(&Integer::from(456)), -123);
    ///
    /// // -123456789000 * -987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     Integer::from_str("121932631112635269000000")
    ///         .unwrap()
    ///         .div_exact(&Integer::from_str("-987654321000").unwrap()),
    ///     -123456789000i64
    /// );
    /// ```
    #[inline]
    fn div_exact(mut self, other: &'a Integer) -> Integer {
        self.div_exact_assign(other);
        self
    }
}

impl<'a> DivExact<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value. The first [`Integer`] must be exactly divisible by the second. If it isn't, this
    /// function may panic or return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `&self / other` instead. If you're
    /// unsure and you want to know, use `self.div_mod(other)` and check whether the remainder is
    /// zero. If you want a function that panics if the division is not exact, use
    /// `(&self).div_round(other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -123 * 456 = -56088
    /// assert_eq!((&Integer::from(-56088)).div_exact(Integer::from(456)), -123);
    ///
    /// // -123456789000 * -987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     (&Integer::from_str("121932631112635269000000").unwrap())
    ///         .div_exact(Integer::from_str("-987654321000").unwrap()),
    ///     -123456789000i64
    /// );
    /// ```
    fn div_exact(self, other: Integer) -> Integer {
        let q_abs = (&self.abs).div_exact(other.abs);
        Integer {
            sign: self.sign == other.sign || q_abs == 0,
            abs: q_abs,
        }
    }
}

impl<'a, 'b> DivExact<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference. The first
    /// [`Integer`] must be exactly divisible by the second. If it isn't, this function may panic or
    /// return a meaningless result.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `&self / &other` instead. If
    /// you're unsure and you want to know, use `(&self).div_mod(&other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `(&self).div_round(&other, Exact)`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -123 * 456 = -56088
    /// assert_eq!(
    ///     (&Integer::from(-56088)).div_exact(&Integer::from(456)),
    ///     -123
    /// );
    ///
    /// // -123456789000 * -987654321000 = 121932631112635269000000
    /// assert_eq!(
    ///     (&Integer::from_str("121932631112635269000000").unwrap())
    ///         .div_exact(&Integer::from_str("-987654321000").unwrap()),
    ///     -123456789000i64
    /// );
    /// ```
    fn div_exact(self, other: &'b Integer) -> Integer {
        let q_abs = (&self.abs).div_exact(&other.abs);
        Integer {
            sign: self.sign == other.sign || q_abs == 0,
            abs: q_abs,
        }
    }
}

impl DivExactAssign<Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value. The first [`Integer`] must be exactly divisible by the second. If
    /// it isn't, this function may panic or return a meaningless result.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self /= other` instead. If you're
    /// unsure and you want to know, use `self.div_assign_mod(other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round_assign(other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -123 * 456 = -56088
    /// let mut x = Integer::from(-56088);
    /// x.div_exact_assign(Integer::from(456));
    /// assert_eq!(x, -123);
    ///
    /// // -123456789000 * -987654321000 = 121932631112635269000000
    /// let mut x = Integer::from_str("121932631112635269000000").unwrap();
    /// x.div_exact_assign(Integer::from_str("-987654321000").unwrap());
    /// assert_eq!(x, -123456789000i64);
    /// ```
    fn div_exact_assign(&mut self, other: Integer) {
        self.abs.div_exact_assign(other.abs);
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}

impl<'a> DivExactAssign<&'a Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference. The first [`Integer`] must be exactly divisible by the second.
    /// If it isn't, this function may panic or return a meaningless result.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
    /// $$
    ///
    /// If you are unsure whether the division will be exact, use `self /= &other` instead. If
    /// you're unsure and you want to know, use `self.div_assign_mod(&other)` and check whether the
    /// remainder is zero. If you want a function that panics if the division is not exact, use
    /// `self.div_round_assign(&other, Exact)`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero. May panic if `self` is not divisible by `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // -123 * 456 = -56088
    /// let mut x = Integer::from(-56088);
    /// x.div_exact_assign(&Integer::from(456));
    /// assert_eq!(x, -123);
    ///
    /// // -123456789000 * -987654321000 = 121932631112635269000000
    /// let mut x = Integer::from_str("121932631112635269000000").unwrap();
    /// x.div_exact_assign(&Integer::from_str("-987654321000").unwrap());
    /// assert_eq!(x, -123456789000i64);
    /// ```
    fn div_exact_assign(&mut self, other: &'a Integer) {
        self.abs.div_exact_assign(&other.abs);
        self.sign = self.sign == other.sign || self.abs == 0;
    }
}
