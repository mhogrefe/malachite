// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::integer_vector::arithmetic::dot_general::vec_dot_general;
use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{DotSign, Sign};
use malachite_base::num::comparison::traits::{OrdProductNeg, OrdProducts, OrdProductsNeg};
use malachite_base::num::logic::traits::SignificantBits;

// What one pass over the terms finds out, for one sign: how many nonzero terms have it, and the
// index and bit length (the sum of its factors' bit lengths) of the largest of them.
#[derive(Clone, Copy, Default)]
struct Side {
    count: u64,
    max_bits: u64,
    max_index: usize,
}

impl Side {
    const fn add(&mut self, index: usize, bits: u64) {
        if self.count == 0 || bits > self.max_bits {
            self.max_bits = bits;
            self.max_index = index;
        }
        self.count += 1;
    }

    // An upper bound on the bit length of the side's sum: each of the `count` terms is below
    // `2^max_bits`.
    fn sum_bits_upper_bound(self) -> u64 {
        self.max_bits + self.count.significant_bits()
    }
}

// The sign of the dot product of `xs` and `ys`, which have the same length. Two-dimensional vectors
// go straight to `Integer::cmp_products_neg`. Otherwise the terms are split by sign in one pass; an
// empty side, a single term on each side, or bit lengths far enough apart decide without any
// product, as does a largest term that outweighs all the others. Then, if the largest product is at
// least twice as wide as any other, the dot product of the rest is computed and the largest term is
// weighed against it by `Integer::cmp_product_neg`; otherwise the whole dot product is computed.
fn dot_sign(xs: &[Integer], ys: &[Integer]) -> Ordering {
    assert_eq!(
        xs.len(),
        ys.len(),
        "the vectors must have the same dimension"
    );
    // In two dimensions the sign of `x_0 y_0 + x_1 y_1` is the comparison of `x_0 y_0` with `-x_1
    // y_1`, which `cmp_products_neg` makes with all of its screens.
    if let ([x_0, x_1], [y_0, y_1]) = (xs, ys) {
        return Integer::cmp_products_neg(x_0, y_0, x_1, y_1);
    }
    let mut positive = Side::default();
    let mut negative = Side::default();
    // The bit lengths of the largest and second-largest terms of either sign, and the index of the
    // largest.
    let mut top_bits = 0;
    let mut top_index = 0;
    let mut second_bits = 0;
    for (i, (x, y)) in xs.iter().zip(ys).enumerate() {
        if x.abs == 0u32 || y.abs == 0u32 {
            continue;
        }
        let bits = x.abs.significant_bits() + y.abs.significant_bits();
        if x.sign == y.sign {
            positive.add(i, bits);
        } else {
            negative.add(i, bits);
        }
        if bits > top_bits {
            second_bits = top_bits;
            top_bits = bits;
            top_index = i;
        } else if bits > second_bits {
            second_bits = bits;
        }
    }
    match (positive.count, negative.count) {
        (0, 0) => return Equal,
        (_, 0) => return Greater,
        (0, _) => return Less,
        (1, 1) => {
            let (p, n) = (positive.max_index, negative.max_index);
            return Natural::cmp_products(&xs[p].abs, &ys[p].abs, &xs[n].abs, &ys[n].abs);
        }
        _ => {}
    }
    // A side's largest term is at least `2^(max_bits - 2)`, so it alone outweighs a whole other
    // side whose sum is below `2^(max_bits - 2)`.
    if positive.sum_bits_upper_bound() + 2 <= negative.max_bits {
        return Less;
    } else if negative.sum_bits_upper_bound() + 2 <= positive.max_bits {
        return Greater;
    }
    // The largest term is at least `2^(top_bits - 2)`, and the other terms, each below
    // `2^second_bits`, sum to less than `2^(second_bits + bits(count - 1))`; so if the largest term
    // is far enough above the others, its sign is the dot product's.
    let k = top_index;
    let others_bits_upper_bound =
        second_bits + (positive.count + negative.count - 1).significant_bits();
    if others_bits_upper_bound + 2 <= top_bits {
        return if xs[k].sign == ys[k].sign {
            Greater
        } else {
            Less
        };
    }
    // Setting the largest term aside pays only when its product is a large share of the work, at
    // least twice as wide as any other; then it is weighed against the rest by `cmp_product_neg`,
    // which forms it only if its screens cannot decide. Otherwise the whole dot product is computed
    // in one call.
    if top_bits >= second_bits << 1 {
        let left = vec_dot_general(None, false, &xs[..k], &ys[..k], false);
        let rest = vec_dot_general(Some(&left), false, &xs[k + 1..], &ys[k + 1..], false);
        Integer::cmp_product_neg(&xs[k], &ys[k], &rest)
    } else {
        vec_dot_general(None, false, xs, ys, false).sign()
    }
}

impl DotSign<Self> for IntegerVector {
    /// Determines the sign of the dot product of two [`IntegerVector`]s, taking the first by value
    /// and the second by value.
    ///
    /// The result is the ordering of the dot product against 0. It is found with as few
    /// multiplications as possible. For two-dimensional vectors it is
    /// [`Integer::cmp_products_neg`](malachite_base::num::comparison::traits::OrdProductsNeg) on
    /// the two pairs of elements. Otherwise the terms are split by sign, and when one side is
    /// empty, when there is one term on each side (compared through
    /// [`Natural::cmp_products`](malachite_base::num::comparison::traits::OrdProducts)), or when
    /// the bit lengths of the largest terms on the two sides are far enough apart, no product is
    /// formed at all; nor is one when the largest term alone outweighs all the others. Otherwise,
    /// if the largest term's product is at least twice as wide as any other, the dot product of the
    /// other terms is computed and the largest term is weighed against it by
    /// [`Integer::cmp_product_neg`](malachite_base::num::comparison::traits::OrdProductNeg), which
    /// forms that product only if it must; if not, the whole dot product is computed.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(3, 7)").unwrap();
    /// let w = IntegerVector::from_str("(8, -3)").unwrap();
    /// assert_eq!(v.clone().dot_sign(w.clone()), Greater);
    /// let w = IntegerVector::from_str("(7, -3)").unwrap();
    /// assert_eq!(v.clone().dot_sign(w.clone()), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: Self) -> Ordering {
        dot_sign(&self.elements, &other.elements)
    }
}

impl DotSign<&Self> for IntegerVector {
    /// Determines the sign of the dot product of two [`IntegerVector`]s, taking the first by value
    /// and the second by reference.
    ///
    /// The result is the ordering of the dot product against 0. It is found with as few
    /// multiplications as possible. For two-dimensional vectors it is
    /// [`Integer::cmp_products_neg`](malachite_base::num::comparison::traits::OrdProductsNeg) on
    /// the two pairs of elements. Otherwise the terms are split by sign, and when one side is
    /// empty, when there is one term on each side (compared through
    /// [`Natural::cmp_products`](malachite_base::num::comparison::traits::OrdProducts)), or when
    /// the bit lengths of the largest terms on the two sides are far enough apart, no product is
    /// formed at all; nor is one when the largest term alone outweighs all the others. Otherwise,
    /// if the largest term's product is at least twice as wide as any other, the dot product of the
    /// other terms is computed and the largest term is weighed against it by
    /// [`Integer::cmp_product_neg`](malachite_base::num::comparison::traits::OrdProductNeg), which
    /// forms that product only if it must; if not, the whole dot product is computed.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(3, 7)").unwrap();
    /// let w = IntegerVector::from_str("(8, -3)").unwrap();
    /// assert_eq!(v.clone().dot_sign(&w), Greater);
    /// let w = IntegerVector::from_str("(7, -3)").unwrap();
    /// assert_eq!(v.clone().dot_sign(&w), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: &Self) -> Ordering {
        dot_sign(&self.elements, &other.elements)
    }
}

impl DotSign<IntegerVector> for &IntegerVector {
    /// Determines the sign of the dot product of two [`IntegerVector`]s, taking the first by
    /// reference and the second by value.
    ///
    /// The result is the ordering of the dot product against 0. It is found with as few
    /// multiplications as possible. For two-dimensional vectors it is
    /// [`Integer::cmp_products_neg`](malachite_base::num::comparison::traits::OrdProductsNeg) on
    /// the two pairs of elements. Otherwise the terms are split by sign, and when one side is
    /// empty, when there is one term on each side (compared through
    /// [`Natural::cmp_products`](malachite_base::num::comparison::traits::OrdProducts)), or when
    /// the bit lengths of the largest terms on the two sides are far enough apart, no product is
    /// formed at all; nor is one when the largest term alone outweighs all the others. Otherwise,
    /// if the largest term's product is at least twice as wide as any other, the dot product of the
    /// other terms is computed and the largest term is weighed against it by
    /// [`Integer::cmp_product_neg`](malachite_base::num::comparison::traits::OrdProductNeg), which
    /// forms that product only if it must; if not, the whole dot product is computed.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(3, 7)").unwrap();
    /// let w = IntegerVector::from_str("(8, -3)").unwrap();
    /// assert_eq!((&v).dot_sign(w.clone()), Greater);
    /// let w = IntegerVector::from_str("(7, -3)").unwrap();
    /// assert_eq!((&v).dot_sign(w.clone()), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: IntegerVector) -> Ordering {
        dot_sign(&self.elements, &other.elements)
    }
}

impl DotSign<&IntegerVector> for &IntegerVector {
    /// Determines the sign of the dot product of two [`IntegerVector`]s, taking the first by
    /// reference and the second by reference.
    ///
    /// The result is the ordering of the dot product against 0. It is found with as few
    /// multiplications as possible. For two-dimensional vectors it is
    /// [`Integer::cmp_products_neg`](malachite_base::num::comparison::traits::OrdProductsNeg) on
    /// the two pairs of elements. Otherwise the terms are split by sign, and when one side is
    /// empty, when there is one term on each side (compared through
    /// [`Natural::cmp_products`](malachite_base::num::comparison::traits::OrdProducts)), or when
    /// the bit lengths of the largest terms on the two sides are far enough apart, no product is
    /// formed at all; nor is one when the largest term alone outweighs all the others. Otherwise,
    /// if the largest term's product is at least twice as wide as any other, the dot product of the
    /// other terms is computed and the largest term is weighed against it by
    /// [`Integer::cmp_product_neg`](malachite_base::num::comparison::traits::OrdProductNeg), which
    /// forms that product only if it must; if not, the whole dot product is computed.
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
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(3, 7)").unwrap();
    /// let w = IntegerVector::from_str("(8, -3)").unwrap();
    /// assert_eq!((&v).dot_sign(&w), Greater);
    /// let w = IntegerVector::from_str("(7, -3)").unwrap();
    /// assert_eq!((&v).dot_sign(&w), Equal);
    /// ```
    #[inline]
    fn dot_sign(self, other: &IntegerVector) -> Ordering {
        dot_sign(&self.elements, &other.elements)
    }
}
