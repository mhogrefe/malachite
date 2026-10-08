// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::conversion::traits::ExactFrom;

/// Implementations of [`Index`](core::ops::Index) and [`IndexMut`](core::ops::IndexMut) for
/// [`RationalVector`].
pub mod access;
/// Traits for arithmetic on [`RationalVector`]s.
pub mod arithmetic;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`ShortlexRationalVector`] and
/// [`ShortlexRationalVectorRef`], comparing two vectors by dimension and then lexicographically.
pub mod comparison;
/// Functions for converting a [`RationalVector`] to and from other types.
pub mod conversion;
/// Iterators that generate [`RationalVector`]s without repetition.
pub mod exhaustive;
/// Functions for finding the pivot of a [`RationalVector`], its first nonzero element, and its
/// index.
pub mod pivot;
/// Iterators that generate [`RationalVector`]s randomly.
#[cfg(feature = "random")]
pub mod random;

/// A vector whose elements are [`Rational`]s.
///
/// Its dimension is the number of elements, and the 0-dimensional vector has none. The field is
/// public, since every [`Vec`] of [`Rational`]s is a valid vector: unlike a polynomial, a vector
/// has no normal form to maintain, and two vectors are equal exactly when their elements are.
///
/// The elements are held entrywise, each as a [`Rational`] of its own. This is the representation
/// FLINT uses for its `fmpq_vec` and `fmpq_mat`, rather than the single common denominator of a
/// [`RationalPolynomial`](crate::rational_polynomial::RationalPolynomial): a vector's elements are
/// read and written one at a time, and a common denominator would make every element as large as
/// the least common multiple of all their denominators.
/// [`to_numerators_and_denominator`](RationalVector::to_numerators_and_denominator) gives the
/// common-denominator form when it is wanted.
///
/// A `RationalVector` is serialized as the list of its elements, each serialized the way a
/// [`Rational`] is.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Two};
/// use malachite_q::Rational;
/// use malachite_q::rational_vector::RationalVector;
///
/// let v = RationalVector {
///     elements: vec![Rational::ONE, Rational::TWO, Rational::from(3u32)],
/// };
/// assert_eq!(v.dimension(), 3);
/// ```
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RationalVector {
    pub elements: Vec<Rational>,
}

impl RationalVector {
    /// Returns the dimension of a [`RationalVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector {
    ///     elements: vec![Rational::from(4u32), Rational::from(5u32)],
    /// };
    /// assert_eq!(v.dimension(), 2);
    /// assert_eq!(
    ///     RationalVector {
    ///         elements: Vec::new()
    ///     }
    ///     .dimension(),
    ///     0
    /// );
    /// ```
    #[inline]
    pub fn dimension(&self) -> u64 {
        u64::exact_from(self.elements.len())
    }
}

// Implements `Named` for `RationalVector`.
impl_named!(RationalVector);

/// `ShortlexRationalVector` is a wrapper around a [`RationalVector`], taking the [`RationalVector`]
/// by value.
///
/// [`RationalVector`] does not implement [`Ord`], because no order on vectors is canonical: lex,
/// graded lex, and the others all have their uses, and the order most natural mathematically,
/// comparing vectors of the same dimension element by element, is only partial. Wrapping a
/// [`RationalVector`] in a `ShortlexRationalVector` provides one total order: vectors are compared
/// first by dimension and then, in case of a tie, lexicographically, by their elements from first
/// to last. Its equality agrees with [`RationalVector`] equality.
///
/// Unlike lexicographic order on [`Vec`]s, this order puts every vector of a smaller dimension
/// before every vector of a larger one, so that $(5) < (0, 0)$. It is not a well-order, since the
/// [`Rational`]s are not: $(0) > (-1) > (-2) > \ldots$ descends forever.
///
/// `ShortlexRationalVector` owns its value. This is useful in many cases, for example if you want
/// to use [`RationalVector`]s as keys in a [`BTreeMap`](alloc::collections::BTreeMap). In other
/// situations, it is better to use [`ShortlexRationalVectorRef`], which only has a reference to its
/// value.
// Serialized as its inner `RationalVector`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexRationalVector(pub RationalVector);

/// `ShortlexRationalVectorRef` is a wrapper around a [`RationalVector`], taking the
/// [`RationalVector`] by reference.
///
/// See the [`ShortlexRationalVector`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortlexRationalVectorRef<'a>(pub &'a RationalVector);

impl ShortlexRationalVector {
    /// Borrows a [`ShortlexRationalVector`] as a [`ShortlexRationalVectorRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::{
    ///     RationalVector, ShortlexRationalVector, ShortlexRationalVectorRef,
    /// };
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexRationalVector(v.clone());
    /// assert_eq!(x.as_ref(), ShortlexRationalVectorRef(&v));
    /// ```
    pub const fn as_ref(&self) -> ShortlexRationalVectorRef<'_> {
        ShortlexRationalVectorRef(&self.0)
    }
}

impl Deref for ShortlexRationalVector {
    type Target = RationalVector;

    /// Allows a [`ShortlexRationalVector`] to dereference to a [`RationalVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::{RationalVector, ShortlexRationalVector};
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexRationalVector(v.clone());
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &RationalVector {
        &self.0
    }
}

impl Deref for ShortlexRationalVectorRef<'_> {
    type Target = RationalVector;

    /// Allows a [`ShortlexRationalVectorRef`] to dereference to a [`RationalVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_vector::{RationalVector, ShortlexRationalVectorRef};
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexRationalVectorRef(&v);
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &RationalVector {
        self.0
    }
}
