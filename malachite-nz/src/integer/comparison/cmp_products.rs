// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::{OrdProducts, OrdProductsNeg};

// Compares `a * b` with `c * d`, or with `-(c * d)` if `negate_right`. The signs settle every case
// except when both products are nonzero with the same sign, where the absolute values are compared
// by `Natural::cmp_products`.
fn cmp_products_helper(
    a: &Integer,
    b: &Integer,
    c: &Integer,
    d: &Integer,
    negate_right: bool,
) -> Ordering {
    let left_zero = a.abs == 0u32 || b.abs == 0u32;
    let right_zero = c.abs == 0u32 || d.abs == 0u32;
    let left_positive = a.sign == b.sign;
    let right_positive = (c.sign == d.sign) != negate_right;
    if left_zero {
        if right_zero {
            Equal
        } else if right_positive {
            Less
        } else {
            Greater
        }
    } else if left_positive {
        if right_zero || !right_positive {
            Greater
        } else {
            Natural::cmp_products(&a.abs, &b.abs, &c.abs, &d.abs)
        }
    } else if right_zero || right_positive {
        Less
    } else {
        Natural::cmp_products(&c.abs, &d.abs, &a.abs, &b.abs)
    }
}

impl OrdProducts for Integer {
    /// Compares the products of two pairs of [`Integer`]s, computing the products only as a last
    /// resort.
    ///
    /// $$
    /// f(a, b, c, d) = \operatorname{cmp}(ab, cd).
    /// $$
    ///
    /// The signs decide every case except when both products are nonzero and have the same sign,
    /// where the absolute values are compared by
    /// [`Natural::cmp_products`](OrdProducts::cmp_products), which multiplies only as a last
    /// resort.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the largest number of significant
    /// bits of `a`, `b`, `c`, and `d`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::comparison::traits::OrdProducts;
    /// use malachite_nz::integer::Integer;
    ///
    /// let z = |x: i32| Integer::from(x);
    /// assert_eq!(Integer::cmp_products(&z(-3), &z(7), &z(-7), &z(3)), Equal);
    /// assert_eq!(Integer::cmp_products(&z(-3), &z(7), &z(-4), &z(5)), Less);
    /// assert_eq!(Integer::cmp_products(&z(3), &z(7), &z(-4), &z(5)), Greater);
    /// ```
    ///
    /// This is equivalent to `compareProducts` from `AzInt/CompareProducts.lean`, Azurite.
    #[inline]
    fn cmp_products(a: &Self, b: &Self, c: &Self, d: &Self) -> Ordering {
        cmp_products_helper(a, b, c, d, false)
    }
}

impl OrdProductsNeg for Integer {
    /// Compares the product of two [`Integer`]s with the negative of the product of two others,
    /// computing the products only as a last resort and never computing the negative; equivalently,
    /// the sign of $ab + cd$.
    ///
    /// $$
    /// f(a, b, c, d) = \operatorname{cmp}(ab, -cd).
    /// $$
    ///
    /// The negative is handled by flipping the sign of $cd$, which is read from the signs of $c$
    /// and $d$, so no value is allocated for it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the largest number of significant
    /// bits of `a`, `b`, `c`, and `d`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::comparison::traits::OrdProductsNeg;
    /// use malachite_nz::integer::Integer;
    ///
    /// let z = |x: i32| Integer::from(x);
    /// assert_eq!(
    ///     Integer::cmp_products_neg(&z(-3), &z(7), &z(7), &z(3)),
    ///     Equal
    /// );
    /// assert_eq!(Integer::cmp_products_neg(&z(-3), &z(7), &z(4), &z(5)), Less);
    /// assert_eq!(
    ///     Integer::cmp_products_neg(&z(3), &z(7), &z(4), &z(5)),
    ///     Greater
    /// );
    /// ```
    #[inline]
    fn cmp_products_neg(a: &Self, b: &Self, c: &Self, d: &Self) -> Ordering {
        cmp_products_helper(a, b, c, d, true)
    }
}
