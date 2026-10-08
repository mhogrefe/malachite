// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::named::Named;
use crate::num::conversion::traits::ExactFrom;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::Deref;

/// Implementations of [`Index`](core::ops::Index) and [`IndexMut`](core::ops::IndexMut) for
/// [`Vector`].
pub mod access;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`ShortlexVector`] and
/// [`ShortlexVectorRef`], comparing two vectors by dimension and then lexicographically.
pub mod comparison;
/// Functions for converting a [`Vector`] to and from other types.
pub mod conversion;
/// Iterators that generate [`Vector`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`Vector`]s randomly.
pub mod random;

/// A vector whose elements are `T`s.
///
/// Its dimension is the number of elements, and the 0-dimensional vector has none. The field is
/// public, since every [`Vec`] of `T`s is a valid vector: unlike a polynomial, a vector has no
/// normal form to maintain, and two vectors are equal exactly when their elements are.
///
/// The elements are held entrywise, each as a `T` of its own. For rational elements this is the
/// representation FLINT uses for its `fmpq_vec` and `fmpq_mat`, rather than the single common
/// denominator of a rational polynomial: a vector's elements are read and written one at a time,
/// and a common denominator would make every element as large as the least common multiple of all
/// their denominators. Operations that benefit from a common denominator clear the denominators
/// themselves.
///
/// Operations whose algorithm depends on the element type are implemented for every `Vector<T>`
/// at once, delegating to a trait that each element type implements in its own crate.
///
/// A `Vector` is serialized as the list of its elements, each serialized the way a `T` is.
///
/// # Examples
/// ```
/// use malachite_base::vector::Vector;
///
/// let v = Vector {
///     elements: vec![1u32, 2, 3],
/// };
/// assert_eq!(v.dimension(), 3);
/// assert_eq!(v.to_string(), "(1, 2, 3)");
/// ```
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Vector<T> {
    pub elements: Vec<T>,
}

impl<T> Vector<T> {
    /// Returns the dimension of a [`Vector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     Vector {
    ///         elements: vec![4u32, 5]
    ///     }
    ///     .dimension(),
    ///     2
    /// );
    /// assert_eq!(
    ///     Vector::<u32> {
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

impl<T> Named for Vector<T> {
    /// The name of this type: `Vector`, whatever the element type, since a `const` cannot be
    /// built from the element type's name.
    const NAME: &'static str = "Vector";
}

/// `ShortlexVector` is a wrapper around a [`Vector`], taking the [`Vector`] by value.
///
/// [`Vector`] does not implement [`Ord`], because no order on vectors is canonical: lex, graded
/// lex, and the others all have their uses, and the order most natural mathematically, comparing
/// vectors of the same dimension element by element, is only partial. Wrapping a [`Vector`] in a
/// `ShortlexVector` provides one total order: vectors are compared first by dimension and then,
/// in case of a tie, lexicographically, by their elements from first to last. Its equality agrees
/// with [`Vector`] equality.
///
/// Unlike lexicographic order on [`Vec`]s, this order puts every vector of a smaller dimension
/// before every vector of a larger one, so that $(5) < (0, 0)$. It is a well-order exactly when
/// the elements are well-ordered: for unsigned elements, any nonempty set of vectors has a least
/// element, the lexicographically least of those of the smallest dimension present; for signed or
/// rational elements, $(0) > (-1) > (-2) > \ldots$ descends forever.
///
/// `ShortlexVector` owns its value. This is useful in many cases, for example if you want to use
/// [`Vector`]s as keys in a [`BTreeMap`](alloc::collections::BTreeMap). In other situations, it
/// is better to use [`ShortlexVectorRef`], which only has a reference to its value.
// Serialized as its inner `Vector`, since the wrapper adds no data of its own.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexVector<T>(pub Vector<T>);

/// `ShortlexVectorRef` is a wrapper around a [`Vector`], taking the [`Vector`] by reference.
///
/// See the [`ShortlexVector`] documentation for details.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ShortlexVectorRef<'a, T>(pub &'a Vector<T>);

impl<T> ShortlexVector<T> {
    /// Borrows a [`ShortlexVector`] as a [`ShortlexVectorRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::{ShortlexVector, ShortlexVectorRef, Vector};
    ///
    /// let v = Vector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexVector(v.clone());
    /// assert_eq!(x.as_ref(), ShortlexVectorRef(&v));
    /// ```
    pub const fn as_ref(&self) -> ShortlexVectorRef<'_, T> {
        ShortlexVectorRef(&self.0)
    }
}

impl<T> Deref for ShortlexVector<T> {
    type Target = Vector<T>;

    /// Allows a [`ShortlexVector`] to dereference to a [`Vector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::{ShortlexVector, Vector};
    ///
    /// let v = Vector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexVector(v.clone());
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &Vector<T> {
        &self.0
    }
}

impl<T> Deref for ShortlexVectorRef<'_, T> {
    type Target = Vector<T>;

    /// Allows a [`ShortlexVectorRef`] to dereference to a [`Vector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::{ShortlexVectorRef, Vector};
    ///
    /// let v = Vector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexVectorRef(&v);
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &Vector<T> {
        self.0
    }
}

impl<T: Display> Debug for ShortlexVector<T> {
    /// Writes a [`ShortlexVector`] as `ShortlexVector(` followed by the vector and `)`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::{ShortlexVector, Vector};
    ///
    /// let v = Vector::<u32>::from_str("(1, 2)").unwrap();
    /// assert_eq!(
    ///     ShortlexVector(v).to_debug_string(),
    ///     "ShortlexVector((1, 2))"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("ShortlexVector").field(&self.0).finish()
    }
}

impl<T: Display> Debug for ShortlexVectorRef<'_, T> {
    /// Writes a [`ShortlexVectorRef`] as `ShortlexVectorRef(` followed by the vector and `)`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::{ShortlexVectorRef, Vector};
    ///
    /// let v = Vector::<u32>::from_str("(1, 2)").unwrap();
    /// assert_eq!(
    ///     ShortlexVectorRef(&v).to_debug_string(),
    ///     "ShortlexVectorRef((1, 2))"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("ShortlexVectorRef").field(self.0).finish()
    }
}
