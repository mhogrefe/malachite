// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::DotSign;
use malachite_base::num::comparison::traits::OrdProductsNeg;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// What one pass over the terms finds out, for one sign: how many nonzero terms have it, and the
// index and size exponent of the largest of them. A term `(n_x n_y) / (d_x d_y)` with exponent `e =
// bits(n_x) + bits(n_y) - bits(d_x) - bits(d_y)` lies in `[2^(e - 2), 2^(e + 2))`.
#[derive(Clone, Copy, Default)]
struct Side {
    count: u64,
    max_exponent: i64,
    max_index: usize,
}

impl Side {
    const fn add(&mut self, index: usize, exponent: i64) {
        if self.count == 0 || exponent > self.max_exponent {
            self.max_exponent = exponent;
            self.max_index = index;
        }
        self.count += 1;
    }

    // An upper bound on the size exponent of the side's sum: each of the `count` terms is below
    // `2^(max_exponent + 2)`.
    fn sum_exponent_upper_bound(self) -> i64 {
        self.max_exponent + 2 + i64::exact_from(self.count.significant_bits())
    }
}

fn exponent(x: &Rational) -> i64 {
    i64::exact_from(x.numerator.significant_bits())
        - i64::exact_from(x.denominator.significant_bits())
}

// The sign of the dot product of `xs` and `ys`, which have the same length, if it can be found
// without writing the vectors over common denominators: two-dimensional vectors, and otherwise an
// empty side, one term on each side, or sizes far enough apart.
fn dot_sign_screens(xs: &[Rational], ys: &[Rational]) -> Option<Ordering> {
    assert_eq!(
        xs.len(),
        ys.len(),
        "the vectors must have the same dimension"
    );
    // In two dimensions the sign of `x_0 y_0 + x_1 y_1` is the comparison of `x_0 y_0` with `-x_1
    // y_1`, which `cmp_products_neg` makes with all of its screens.
    if let ([x_0, x_1], [y_0, y_1]) = (xs, ys) {
        return Some(Rational::cmp_products_neg(x_0, y_0, x_1, y_1));
    }
    let mut positive = Side::default();
    let mut negative = Side::default();
    for (i, (x, y)) in xs.iter().zip(ys).enumerate() {
        if x.numerator == 0u32 || y.numerator == 0u32 {
            continue;
        }
        let e = exponent(x) + exponent(y);
        if x.sign == y.sign {
            positive.add(i, e);
        } else {
            negative.add(i, e);
        }
    }
    match (positive.count, negative.count) {
        (0, 0) => return Some(Equal),
        (_, 0) => return Some(Greater),
        (0, _) => return Some(Less),
        (1, 1) => {
            let (p, n) = (positive.max_index, negative.max_index);
            return Some(Rational::cmp_products_neg(&xs[p], &ys[p], &xs[n], &ys[n]));
        }
        _ => {}
    }
    // A side's largest term is at least `2^(max_exponent - 2)`, so it alone outweighs a whole other
    // side whose sum is below that.
    if positive.sum_exponent_upper_bound() + 2 <= negative.max_exponent {
        Some(Less)
    } else if negative.sum_exponent_upper_bound() + 2 <= positive.max_exponent {
        Some(Greater)
    } else {
        None
    }
}

impl DotSign<Self> for RationalVector {
    /// Determines the sign of the dot product of two [`RationalVector`]s, taking the first by value
    /// and the second by value.
    ///
    /// The result is the ordering of the dot product against 0, found with as little arithmetic as
    /// possible. For two-dimensional vectors it is
    /// [`Rational::cmp_products_neg`](OrdProductsNeg::cmp_products_neg) on the two pairs of
    /// elements. Otherwise the terms are split by sign, and when one side is empty, when there is
    /// one term on each side (also compared through `cmp_products_neg`), or when the sizes of the
    /// largest terms on the two sides are far enough apart, no product is formed. Only then are the
    /// vectors written as integer numerators over common denominators, whose dot product has the
    /// same sign, as in [`Dot`](malachite_base::num::arithmetic::traits::Dot); its sign is found by
    /// [`IntegerVector::dot_sign`](DotSign::dot_sign).
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, 1/3, 1/6)").unwrap();
    /// let w = RationalVector::from_str("(1, 1, -4)").unwrap();
    /// assert_eq!(v.clone().dot_sign(w.clone()), Greater);
    /// let w = RationalVector::from_str("(1, 1, -5)").unwrap();
    /// assert_eq!(v.clone().dot_sign(w.clone()), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: Self) -> Ordering {
        dot_sign_screens(&self.elements, &other.elements).unwrap_or_else(|| {
            // The common denominators are positive, so the dot product of the numerators has the
            // sign of the dot product.
            self.into_numerators_and_denominator()
                .0
                .dot_sign(other.into_numerators_and_denominator().0)
        })
    }
}

impl DotSign<&Self> for RationalVector {
    /// Determines the sign of the dot product of two [`RationalVector`]s, taking the first by value
    /// and the second by reference.
    ///
    /// The result is the ordering of the dot product against 0, found with as little arithmetic as
    /// possible. For two-dimensional vectors it is
    /// [`Rational::cmp_products_neg`](OrdProductsNeg::cmp_products_neg) on the two pairs of
    /// elements. Otherwise the terms are split by sign, and when one side is empty, when there is
    /// one term on each side (also compared through `cmp_products_neg`), or when the sizes of the
    /// largest terms on the two sides are far enough apart, no product is formed. Only then are the
    /// vectors written as integer numerators over common denominators, whose dot product has the
    /// same sign, as in [`Dot`](malachite_base::num::arithmetic::traits::Dot); its sign is found by
    /// [`IntegerVector::dot_sign`](DotSign::dot_sign).
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, 1/3, 1/6)").unwrap();
    /// let w = RationalVector::from_str("(1, 1, -4)").unwrap();
    /// assert_eq!(v.clone().dot_sign(&w), Greater);
    /// let w = RationalVector::from_str("(1, 1, -5)").unwrap();
    /// assert_eq!(v.clone().dot_sign(&w), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: &Self) -> Ordering {
        dot_sign_screens(&self.elements, &other.elements).unwrap_or_else(|| {
            // The common denominators are positive, so the dot product of the numerators has the
            // sign of the dot product.
            self.into_numerators_and_denominator()
                .0
                .dot_sign(other.to_numerators_and_denominator().0)
        })
    }
}

impl DotSign<RationalVector> for &RationalVector {
    /// Determines the sign of the dot product of two [`RationalVector`]s, taking the first by
    /// reference and the second by value.
    ///
    /// The result is the ordering of the dot product against 0, found with as little arithmetic as
    /// possible. For two-dimensional vectors it is
    /// [`Rational::cmp_products_neg`](OrdProductsNeg::cmp_products_neg) on the two pairs of
    /// elements. Otherwise the terms are split by sign, and when one side is empty, when there is
    /// one term on each side (also compared through `cmp_products_neg`), or when the sizes of the
    /// largest terms on the two sides are far enough apart, no product is formed. Only then are the
    /// vectors written as integer numerators over common denominators, whose dot product has the
    /// same sign, as in [`Dot`](malachite_base::num::arithmetic::traits::Dot); its sign is found by
    /// [`IntegerVector::dot_sign`](DotSign::dot_sign).
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, 1/3, 1/6)").unwrap();
    /// let w = RationalVector::from_str("(1, 1, -4)").unwrap();
    /// assert_eq!((&v).dot_sign(w.clone()), Greater);
    /// let w = RationalVector::from_str("(1, 1, -5)").unwrap();
    /// assert_eq!((&v).dot_sign(w.clone()), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: RationalVector) -> Ordering {
        dot_sign_screens(&self.elements, &other.elements).unwrap_or_else(|| {
            // The common denominators are positive, so the dot product of the numerators has the
            // sign of the dot product.
            self.to_numerators_and_denominator()
                .0
                .dot_sign(other.into_numerators_and_denominator().0)
        })
    }
}

impl DotSign<&RationalVector> for &RationalVector {
    /// Determines the sign of the dot product of two [`RationalVector`]s, taking the first by
    /// reference and the second by reference.
    ///
    /// The result is the ordering of the dot product against 0, found with as little arithmetic as
    /// possible. For two-dimensional vectors it is
    /// [`Rational::cmp_products_neg`](OrdProductsNeg::cmp_products_neg) on the two pairs of
    /// elements. Otherwise the terms are split by sign, and when one side is empty, when there is
    /// one term on each side (also compared through `cmp_products_neg`), or when the sizes of the
    /// largest terms on the two sides are far enough apart, no product is formed. Only then are the
    /// vectors written as integer numerators over common denominators, whose dot product has the
    /// same sign, as in [`Dot`](malachite_base::num::arithmetic::traits::Dot); its sign is found by
    /// [`IntegerVector::dot_sign`](DotSign::dot_sign).
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of the numerators the elements take over each
    /// vector's common denominator.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, 1/3, 1/6)").unwrap();
    /// let w = RationalVector::from_str("(1, 1, -4)").unwrap();
    /// assert_eq!((&v).dot_sign(&w), Greater);
    /// let w = RationalVector::from_str("(1, 1, -5)").unwrap();
    /// assert_eq!((&v).dot_sign(&w), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: &RationalVector) -> Ordering {
        dot_sign_screens(&self.elements, &other.elements).unwrap_or_else(|| {
            // The common denominators are positive, so the dot product of the numerators has the
            // sign of the dot product.
            self.to_numerators_and_denominator()
                .0
                .dot_sign(other.to_numerators_and_denominator().0)
        })
    }
}
