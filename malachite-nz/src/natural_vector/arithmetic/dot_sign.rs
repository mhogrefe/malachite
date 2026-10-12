// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::DotSign;

// `Greater` if some coordinate is nonzero in both vectors, which have the same dimension, and
// `Equal` otherwise.
fn dot_sign(v: &NaturalVector, w: &NaturalVector) -> Ordering {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
    if v.elements
        .iter()
        .zip(&w.elements)
        .any(|(x, y)| *x != 0u32 && *y != 0u32)
    {
        Greater
    } else {
        Equal
    }
}

impl DotSign<Self> for NaturalVector {
    /// Determines the sign of the dot product of two [`NaturalVector`]s, taking the first by value
    /// and the second by value.
    ///
    /// Every term $v_iw_i$ is non-negative, so the dot product is positive exactly when some
    /// coordinate is nonzero in both vectors, and zero otherwise; no product or sum is computed.
    /// The result is the ordering of the dot product against 0: `Greater` or `Equal`.
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 0, 3)").unwrap();
    /// let w = NaturalVector::from_str("(0, 5, 7)").unwrap();
    /// assert_eq!(v.clone().dot_sign(w.clone()), Greater);
    /// let w = NaturalVector::from_str("(0, 5, 0)").unwrap();
    /// assert_eq!(v.clone().dot_sign(w.clone()), Equal);
    /// ```
    ///
    /// This is equivalent to `dotSign` from `AzVector/DotSign.lean`, Azurite.
    #[inline]
    fn dot_sign(self, other: Self) -> Ordering {
        dot_sign(&self, &other)
    }
}

impl DotSign<&Self> for NaturalVector {
    /// Determines the sign of the dot product of two [`NaturalVector`]s, taking the first by value
    /// and the second by reference.
    ///
    /// Every term $v_iw_i$ is non-negative, so the dot product is positive exactly when some
    /// coordinate is nonzero in both vectors, and zero otherwise; no product or sum is computed.
    /// The result is the ordering of the dot product against 0: `Greater` or `Equal`.
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 0, 3)").unwrap();
    /// let w = NaturalVector::from_str("(0, 5, 7)").unwrap();
    /// assert_eq!(v.clone().dot_sign(&w), Greater);
    /// let w = NaturalVector::from_str("(0, 5, 0)").unwrap();
    /// assert_eq!(v.clone().dot_sign(&w), Equal);
    /// ```
    ///
    /// This is equivalent to `dotSign` from `AzVector/DotSign.lean`, Azurite.
    #[inline]
    fn dot_sign(self, other: &Self) -> Ordering {
        dot_sign(&self, other)
    }
}

impl DotSign<NaturalVector> for &NaturalVector {
    /// Determines the sign of the dot product of two [`NaturalVector`]s, taking the first by
    /// reference and the second by value.
    ///
    /// Every term $v_iw_i$ is non-negative, so the dot product is positive exactly when some
    /// coordinate is nonzero in both vectors, and zero otherwise; no product or sum is computed.
    /// The result is the ordering of the dot product against 0: `Greater` or `Equal`.
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 0, 3)").unwrap();
    /// let w = NaturalVector::from_str("(0, 5, 7)").unwrap();
    /// assert_eq!((&v).dot_sign(w.clone()), Greater);
    /// let w = NaturalVector::from_str("(0, 5, 0)").unwrap();
    /// assert_eq!((&v).dot_sign(w.clone()), Equal);
    /// ```
    ///
    /// This is equivalent to `dotSign` from `AzVector/DotSign.lean`, Azurite.
    #[inline]
    fn dot_sign(self, other: NaturalVector) -> Ordering {
        dot_sign(self, &other)
    }
}

impl DotSign<&NaturalVector> for &NaturalVector {
    /// Determines the sign of the dot product of two [`NaturalVector`]s, taking the first by
    /// reference and the second by reference.
    ///
    /// Every term $v_iw_i$ is non-negative, so the dot product is positive exactly when some
    /// coordinate is nonzero in both vectors, and zero otherwise; no product or sum is computed.
    /// The result is the ordering of the dot product against 0: `Greater` or `Equal`.
    ///
    /// $$
    /// f(v, w) = \operatorname{sgn}\left ( \sum_{i=0}^{n-1} v_iw_i \right ),
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DotSign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 0, 3)").unwrap();
    /// let w = NaturalVector::from_str("(0, 5, 7)").unwrap();
    /// assert_eq!((&v).dot_sign(&w), Greater);
    /// let w = NaturalVector::from_str("(0, 5, 0)").unwrap();
    /// assert_eq!((&v).dot_sign(&w), Equal);
    /// ```
    ///
    /// This is equivalent to `dotSign` from `AzVector/DotSign.lean`, Azurite.
    #[inline]
    fn dot_sign(self, other: &NaturalVector) -> Ordering {
        dot_sign(self, other)
    }
}
