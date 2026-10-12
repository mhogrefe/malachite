// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::comparison::traits::{OrdProducts, OrdProductsNeg};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

// Four factorwise comparisons decide a comparison of two four-factor products when they all point
// one way, equalities allowed.
fn screen_4(o_1: Ordering, o_2: Ordering, o_3: Ordering, o_4: Ordering) -> Option<Ordering> {
    let os = [o_1, o_2, o_3, o_4];
    if os.iter().all(|&o| o != Greater) {
        Some(if os.iter().all(|&o| o == Equal) {
            Equal
        } else {
            Less
        })
    } else if os.iter().all(|&o| o != Less) {
        Some(Greater)
    } else {
        None
    }
}

// Compares `nw * nx * dy * dz` with `ny * nz * dw * dx`, all positive, multiplying only as a last
// resort: first the bit lengths, whose sums bound each product to within a factor of 16; then three
// factorwise screens, each pairing the four factors of one side with the four of the other; and
// finally the four two-factor products, compared through `Natural::cmp_products`, whose own screens
// may still avoid the last two multiplications.
#[allow(clippy::too_many_arguments)]
fn cmp_products_magnitudes(
    nw: &Natural,
    dw: &Natural,
    nx: &Natural,
    dx: &Natural,
    ny: &Natural,
    dy: &Natural,
    nz: &Natural,
    dz: &Natural,
) -> Ordering {
    let left_bits = nw.significant_bits()
        + nx.significant_bits()
        + dy.significant_bits()
        + dz.significant_bits();
    let right_bits = ny.significant_bits()
        + nz.significant_bits()
        + dw.significant_bits()
        + dx.significant_bits();
    if right_bits + 4 <= left_bits {
        return Greater;
    } else if left_bits + 4 <= right_bits {
        return Less;
    }
    // |w| and |x| against 1, and 1 against |y| and |z|.
    if let Some(o) = screen_4(nw.cmp(dw), nx.cmp(dx), dy.cmp(ny), dz.cmp(nz)) {
        return o;
    }
    if let Some(o) = screen_4(nw.cmp(ny), nx.cmp(nz), dy.cmp(dw), dz.cmp(dx)) {
        return o;
    }
    if let Some(o) = screen_4(nw.cmp(nz), nx.cmp(ny), dy.cmp(dx), dz.cmp(dw)) {
        return o;
    }
    Natural::cmp_products(&(nw * nx), &(dy * dz), &(ny * nz), &(dw * dx))
}

// Compares `w * x` with `y * z`, or with `-(y * z)` if `negate_right`. The signs settle every case
// except when both products are nonzero with the same sign.
fn cmp_products_helper(
    w: &Rational,
    x: &Rational,
    y: &Rational,
    z: &Rational,
    negate_right: bool,
) -> Ordering {
    let left_zero = w.numerator == 0u32 || x.numerator == 0u32;
    let right_zero = y.numerator == 0u32 || z.numerator == 0u32;
    let left_positive = w.sign == x.sign;
    let right_positive = (y.sign == z.sign) != negate_right;
    let magnitudes = || {
        cmp_products_magnitudes(
            &w.numerator,
            &w.denominator,
            &x.numerator,
            &x.denominator,
            &y.numerator,
            &y.denominator,
            &z.numerator,
            &z.denominator,
        )
    };
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
            magnitudes()
        }
    } else if right_zero || right_positive {
        Less
    } else {
        magnitudes().reverse()
    }
}

impl OrdProducts for Rational {
    /// Compares the products of two pairs of [`Rational`]s, computing products only as a last
    /// resort and never reducing a product to lowest terms.
    ///
    /// $$
    /// f(w, x, y, z) = \operatorname{cmp}(wx, yz).
    /// $$
    ///
    /// The signs decide every case except when both products are nonzero with the same sign. Then,
    /// with $w = \pm n_w/d_w$ and so on, the magnitudes compare as $n_wn_xd_yd_z$ against
    /// $n_yn_zd_wd_x$. That comparison is decided, when possible, by the bit lengths of the eight
    /// factors, or by comparing the factors in pairs (such as each of $|w|$, $|x|$, $|y|$, and
    /// $|z|$ against 1); otherwise four two-factor products are formed and compared through
    /// [`Natural::cmp_products`](OrdProducts::cmp_products).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the largest number of significant
    /// bits of `w`, `x`, `y`, and `z`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::comparison::traits::OrdProducts;
    /// use malachite_q::Rational;
    ///
    /// let q = |s| Rational::from_str(s).unwrap();
    /// assert_eq!(
    ///     Rational::cmp_products(&q("3/7"), &q("7/3"), &q("1/2"), &q("2")),
    ///     Equal
    /// );
    /// assert_eq!(
    ///     Rational::cmp_products(&q("-3/7"), &q("7/3"), &q("1/2"), &q("3")),
    ///     Less
    /// );
    /// assert_eq!(
    ///     Rational::cmp_products(&q("2/3"), &q("1/4"), &q("3/5"), &q("1/5")),
    ///     Greater
    /// );
    /// ```
    ///
    /// This is equivalent to `compareProducts` from `AzRat/CompareProducts.lean`, Azurite.
    #[inline]
    fn cmp_products(w: &Self, x: &Self, y: &Self, z: &Self) -> Ordering {
        cmp_products_helper(w, x, y, z, false)
    }
}

impl OrdProductsNeg for Rational {
    /// Compares the product of two [`Rational`]s with the negative of the product of two others,
    /// computing products only as a last resort and never computing the negative; equivalently, the
    /// sign of $wx + yz$.
    ///
    /// $$
    /// f(w, x, y, z) = \operatorname{cmp}(wx, -yz).
    /// $$
    ///
    /// This is [`Rational::cmp_products`](OrdProducts::cmp_products) with the sign of $yz$ flipped
    /// as it is read from the signs of $y$ and $z$, so no value is allocated for it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the largest number of significant
    /// bits of `w`, `x`, `y`, and `z`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::comparison::traits::OrdProductsNeg;
    /// use malachite_q::Rational;
    ///
    /// let q = |s| Rational::from_str(s).unwrap();
    /// assert_eq!(
    ///     Rational::cmp_products_neg(&q("3/7"), &q("7/3"), &q("-1/2"), &q("2")),
    ///     Equal
    /// );
    /// assert_eq!(
    ///     Rational::cmp_products_neg(&q("-3/7"), &q("7/3"), &q("1/2"), &q("1")),
    ///     Less
    /// );
    /// assert_eq!(
    ///     Rational::cmp_products_neg(&q("3/7"), &q("7/3"), &q("1/2"), &q("3")),
    ///     Greater
    /// );
    /// ```
    #[inline]
    fn cmp_products_neg(w: &Self, x: &Self, y: &Self, z: &Self) -> Ordering {
        cmp_products_helper(w, x, y, z, true)
    }
}
