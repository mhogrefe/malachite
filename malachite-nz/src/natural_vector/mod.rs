// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::conversion::traits::ExactFrom;

/// Implementations of [`Ord`] and [`PartialOrd`] for [`ShortlexNaturalVector`] and
/// [`ShortlexNaturalVectorRef`], comparing two vectors by dimension and then lexicographically.
pub mod comparison;
/// Functions for converting a [`NaturalVector`] to and from other types.
pub mod conversion;
/// Iterators that generate [`NaturalVector`]s without repetition.
pub mod exhaustive;
/// Iterators that generate [`NaturalVector`]s randomly.
#[cfg(feature = "random")]
pub mod random;

/// A vector whose elements are [`Natural`]s.
///
/// Its dimension is the number of elements, and the 0-dimensional vector has none. The field is
/// public, since every [`Vec`] of [`Natural`]s is a valid vector: unlike a polynomial, a vector has
/// no normal form to maintain, and two vectors are equal exactly when their elements are.
///
/// A `NaturalVector` is serialized as the list of its elements, each serialized the way a
/// [`Natural`] is.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Two};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let v = NaturalVector {
///     elements: vec![Natural::ONE, Natural::TWO, Natural::from(3u32)],
/// };
/// assert_eq!(v.dimension(), 3);
/// ```
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NaturalVector {
    pub elements: Vec<Natural>,
}

impl NaturalVector {
    /// Returns the dimension of a [`NaturalVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector {
    ///     elements: vec![Natural::from(4u32), Natural::from(5u32)],
    /// };
    /// assert_eq!(v.dimension(), 2);
    /// assert_eq!(
    ///     NaturalVector {
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

// Implements `Named` for `NaturalVector`.
impl_named!(NaturalVector);

/// `ShortlexNaturalVector` is a wrapper around a [`NaturalVector`], taking the [`NaturalVector`] by
/// value.
///
/// [`NaturalVector`] does not implement [`Ord`], because no order on vectors is canonical: lex,
/// graded lex, and the others all have their uses, and the order most natural mathematically,
/// comparing vectors of the same dimension element by element, is only partial. Wrapping a
/// [`NaturalVector`] in a `ShortlexNaturalVector` provides one total order: vectors are compared
/// first by dimension and then, in case of a tie, lexicographically, by their elements from first
/// to last. Its equality agrees with [`NaturalVector`] equality.
///
/// Unlike lexicographic order on [`Vec`]s, this order puts every vector of a smaller dimension
/// before every vector of a larger one, so that $(5) < (0, 0)$. It is not a well-order, since the
/// vectors of any positive dimension, like the [`Natural`]s, have no largest element and so there
/// are infinitely many vectors below $(0, 0)$; but every vector has only finitely many vectors of
/// its own dimension below it.
///
/// `ShortlexNaturalVector` owns its value. This is useful in many cases, for example if you want to
/// use [`NaturalVector`]s as keys in a [`BTreeMap`](alloc::collections::BTreeMap). In other
/// situations, it is better to use [`ShortlexNaturalVectorRef`], which only has a reference to its
/// value.
// Serialized as its inner `NaturalVector`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexNaturalVector(pub NaturalVector);

/// `ShortlexNaturalVectorRef` is a wrapper around a [`NaturalVector`], taking the [`NaturalVector`]
/// by reference.
///
/// See the [`ShortlexNaturalVector`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortlexNaturalVectorRef<'a>(pub &'a NaturalVector);

impl ShortlexNaturalVector {
    /// Borrows a [`ShortlexNaturalVector`] as a [`ShortlexNaturalVectorRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_vector::{
    ///     NaturalVector, ShortlexNaturalVector, ShortlexNaturalVectorRef,
    /// };
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexNaturalVector(v.clone());
    /// assert_eq!(x.as_ref(), ShortlexNaturalVectorRef(&v));
    /// ```
    pub const fn as_ref(&self) -> ShortlexNaturalVectorRef<'_> {
        ShortlexNaturalVectorRef(&self.0)
    }
}

impl Deref for ShortlexNaturalVector {
    type Target = NaturalVector;

    /// Allows a [`ShortlexNaturalVector`] to dereference to a [`NaturalVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_vector::{NaturalVector, ShortlexNaturalVector};
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexNaturalVector(v.clone());
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &NaturalVector {
        &self.0
    }
}

impl Deref for ShortlexNaturalVectorRef<'_> {
    type Target = NaturalVector;

    /// Allows a [`ShortlexNaturalVectorRef`] to dereference to a [`NaturalVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_vector::{NaturalVector, ShortlexNaturalVectorRef};
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexNaturalVectorRef(&v);
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &NaturalVector {
        self.0
    }
}
