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
use malachite_base::num::comparison::traits::{OrdProduct, OrdProductNeg};

// Compares `a * b` with the integer whose sign is `c_sign` (`true` for non-negative) and whose
// absolute value is `c_abs`; `c_sign` is ignored when `c_abs` is zero. The signs settle every case
// except when `a * b` and `c` are nonzero with the same sign, where the absolute values are
// compared by `Natural::cmp_product`.
fn cmp_product_helper(a: &Integer, b: &Integer, c_sign: bool, c_abs: &Natural) -> Ordering {
    let c_zero = *c_abs == 0u32;
    if a.abs == 0u32 || b.abs == 0u32 {
        if c_zero {
            Equal
        } else if c_sign {
            Less
        } else {
            Greater
        }
    } else if a.sign == b.sign {
        if c_zero || !c_sign {
            Greater
        } else {
            Natural::cmp_product(&a.abs, &b.abs, c_abs)
        }
    } else if c_zero || c_sign {
        Less
    } else {
        Natural::cmp_product(&a.abs, &b.abs, c_abs).reverse()
    }
}

impl OrdProduct for Integer {
    /// Compares the product of two [`Integer`]s with a third, computing the product only as a last
    /// resort.
    ///
    /// $$
    /// f(a, b, c) = \operatorname{cmp}(ab, c).
    /// $$
    ///
    /// The signs decide every case except when $ab$ and $c$ are nonzero and have the same sign,
    /// where the absolute values are compared by [`Natural::cmp_product`](OrdProduct::cmp_product),
    /// which multiplies only as a last resort.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.significant_bits(),
    /// b.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::comparison::traits::OrdProduct;
    /// use malachite_nz::integer::Integer;
    ///
    /// let z = |x: i32| Integer::from(x);
    /// assert_eq!(Integer::cmp_product(&z(-3), &z(-7), &z(21)), Equal);
    /// assert_eq!(Integer::cmp_product(&z(-3), &z(7), &z(-20)), Less);
    /// assert_eq!(Integer::cmp_product(&z(3), &z(7), &z(-21)), Greater);
    /// ```
    ///
    /// This is equivalent to `compareProduct` from `AzInt/CompareProduct.lean`, Azurite.
    #[inline]
    fn cmp_product(a: &Self, b: &Self, c: &Self) -> Ordering {
        cmp_product_helper(a, b, c.sign, &c.abs)
    }
}

impl OrdProductNeg for Integer {
    /// Compares the product of two [`Integer`]s with the negative of a third, computing the product
    /// only as a last resort and never computing the negative; equivalently, the sign of $ab + c$.
    ///
    /// $$
    /// f(a, b, c) = \operatorname{cmp}(ab, -c).
    /// $$
    ///
    /// The negative is handled by flipping the sign of $c$ as it is read, so no value is allocated
    /// for it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.significant_bits(),
    /// b.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::comparison::traits::OrdProductNeg;
    /// use malachite_nz::integer::Integer;
    ///
    /// let z = |x: i32| Integer::from(x);
    /// assert_eq!(Integer::cmp_product_neg(&z(-3), &z(-7), &z(-21)), Equal);
    /// assert_eq!(Integer::cmp_product_neg(&z(-3), &z(7), &z(20)), Less);
    /// assert_eq!(Integer::cmp_product_neg(&z(3), &z(7), &z(21)), Greater);
    /// ```
    #[inline]
    fn cmp_product_neg(a: &Self, b: &Self, c: &Self) -> Ordering {
        cmp_product_helper(a, b, !c.sign, &c.abs)
    }
}
