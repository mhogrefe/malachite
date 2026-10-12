// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::{OrdProduct, OrdProductNeg, OrdProducts};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

// Three factorwise comparisons decide a comparison of two three-factor products when they all point
// one way, equalities allowed.
fn screen_3(o_1: Ordering, o_2: Ordering, o_3: Ordering) -> Option<Ordering> {
    if o_1 != Greater && o_2 != Greater && o_3 != Greater {
        Some(if o_1 == Equal && o_2 == Equal && o_3 == Equal {
            Equal
        } else {
            Less
        })
    } else if o_1 != Less && o_2 != Less && o_3 != Less {
        Some(Greater)
    } else {
        None
    }
}

// Compares `nx * ny * dz` with `nz * dx * dy`, all positive, multiplying only as a last resort:
// first the bit lengths, whose sums bound each product to within a factor of 8; then three
// factorwise screens, each pairing the three factors of one side with the three of the other; and
// finally the products `nx * ny` and `dx * dy`, compared through `Natural::cmp_products`, whose own
// screens may still avoid the last two multiplications.
fn cmp_product_magnitudes(
    nx: &Natural,
    dx: &Natural,
    ny: &Natural,
    dy: &Natural,
    nz: &Natural,
    dz: &Natural,
) -> Ordering {
    let left_bits = nx.significant_bits() + ny.significant_bits() + dz.significant_bits();
    let right_bits = nz.significant_bits() + dx.significant_bits() + dy.significant_bits();
    if right_bits + 3 <= left_bits {
        return Greater;
    } else if left_bits + 3 <= right_bits {
        return Less;
    }
    // |x| against 1, |y| against 1, and |z| against 1.
    if let Some(o) = screen_3(nx.cmp(dx), ny.cmp(dy), dz.cmp(nz)) {
        return o;
    }
    if let Some(o) = screen_3(nx.cmp(nz), ny.cmp(dy), dz.cmp(dx)) {
        return o;
    }
    if let Some(o) = screen_3(nx.cmp(dx), ny.cmp(nz), dz.cmp(dy)) {
        return o;
    }
    Natural::cmp_products(&(nx * ny), dz, &(dx * dy), nz)
}

// Compares `x * y` with the rational whose sign is `z_sign` (`true` for non-negative) and whose
// absolute value is `z_num / z_den`; `z_sign` is ignored when `z_num` is zero. The signs settle
// every case except when `x * y` and `z` are nonzero with the same sign.
fn cmp_product_helper(
    x: &Rational,
    y: &Rational,
    z_sign: bool,
    z_num: &Natural,
    z_den: &Natural,
) -> Ordering {
    let z_zero = *z_num == 0u32;
    if x.numerator == 0u32 || y.numerator == 0u32 {
        if z_zero {
            Equal
        } else if z_sign {
            Less
        } else {
            Greater
        }
    } else if x.sign == y.sign {
        if z_zero || !z_sign {
            Greater
        } else {
            cmp_product_magnitudes(
                &x.numerator,
                &x.denominator,
                &y.numerator,
                &y.denominator,
                z_num,
                z_den,
            )
        }
    } else if z_zero || z_sign {
        Less
    } else {
        cmp_product_magnitudes(
            &x.numerator,
            &x.denominator,
            &y.numerator,
            &y.denominator,
            z_num,
            z_den,
        )
        .reverse()
    }
}

impl OrdProduct for Rational {
    /// Compares the product of two [`Rational`]s with a third, computing products only as a last
    /// resort and never reducing a product to lowest terms.
    ///
    /// $$
    /// f(x, y, z) = \operatorname{cmp}(xy, z).
    /// $$
    ///
    /// The signs decide every case except when $xy$ and $z$ are nonzero with the same sign. Then,
    /// with $x = \pm n_x/d_x$, $y = \pm n_y/d_y$, and $z = \pm n_z/d_z$, the magnitudes compare as
    /// $n_xn_yd_z$ against $n_zd_xd_y$. That comparison is decided, when possible, by the bit
    /// lengths of the six factors, or by comparing the factors in pairs (such as each of $|x|$,
    /// $|y|$, and $|z|$ against 1); otherwise $n_xn_y$ and $d_xd_y$ are formed and compared through
    /// [`Natural::cmp_products`](OrdProducts::cmp_products).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::comparison::traits::OrdProduct;
    /// use malachite_q::Rational;
    ///
    /// let q = |s| Rational::from_str(s).unwrap();
    /// assert_eq!(Rational::cmp_product(&q("3/7"), &q("7/3"), &q("1")), Equal);
    /// assert_eq!(
    ///     Rational::cmp_product(&q("-3/7"), &q("7/3"), &q("-1/2")),
    ///     Less
    /// );
    /// assert_eq!(
    ///     Rational::cmp_product(&q("11/10"), &q("9/10"), &q("49/50")),
    ///     Greater
    /// );
    /// ```
    ///
    /// This is equivalent to `compareProduct` from `AzRat/CompareProduct.lean`, Azurite.
    #[inline]
    fn cmp_product(x: &Self, y: &Self, z: &Self) -> Ordering {
        cmp_product_helper(x, y, z.sign, &z.numerator, &z.denominator)
    }
}

impl OrdProductNeg for Rational {
    /// Compares the product of two [`Rational`]s with the negative of a third, computing products
    /// only as a last resort and never computing the negative; equivalently, the sign of $xy + z$.
    ///
    /// $$
    /// f(x, y, z) = \operatorname{cmp}(xy, -z).
    /// $$
    ///
    /// This is [`Rational::cmp_product`](OrdProduct::cmp_product) with the sign of $z$ flipped as
    /// it is read, so no value is allocated for $-z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.significant_bits(), z.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::comparison::traits::OrdProductNeg;
    /// use malachite_q::Rational;
    ///
    /// let q = |s| Rational::from_str(s).unwrap();
    /// assert_eq!(
    ///     Rational::cmp_product_neg(&q("3/7"), &q("7/3"), &q("-1")),
    ///     Equal
    /// );
    /// assert_eq!(
    ///     Rational::cmp_product_neg(&q("-3/7"), &q("7/3"), &q("1/2")),
    ///     Less
    /// );
    /// assert_eq!(
    ///     Rational::cmp_product_neg(&q("3/7"), &q("7/3"), &q("5")),
    ///     Greater
    /// );
    /// ```
    #[inline]
    fn cmp_product_neg(x: &Self, y: &Self, z: &Self) -> Ordering {
        cmp_product_helper(x, y, !z.sign, &z.numerator, &z.denominator)
    }
}
