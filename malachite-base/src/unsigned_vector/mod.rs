// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::named::Named;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use core::ops::Deref;

/// Implementations of [`Index`](core::ops::Index) and [`IndexMut`](core::ops::IndexMut) for
/// [`UnsignedVector`].
pub mod access;
/// Traits for arithmetic on [`UnsignedVector`]s.
pub mod arithmetic;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`ShortlexUnsignedVector`] and
/// [`ShortlexUnsignedVectorRef`], comparing two vectors by dimension and then lexicographically.
pub mod comparison;
/// Functions for converting an [`UnsignedVector`] to and from other types.
pub mod conversion;
/// Iterators that generate [`UnsignedVector`]s without repetition.
pub mod exhaustive;
/// Functions for finding the pivot of an [`UnsignedVector`], its first nonzero element, and its
/// index.
pub mod pivot;
#[cfg(feature = "random")]
/// Iterators that generate [`UnsignedVector`]s randomly.
pub mod random;

/// A vector whose elements are primitive unsigned integers.
///
/// Its dimension is the number of elements, and the 0-dimensional vector has none. The field is
/// public, since every [`Vec`] of `T`s is a valid vector: unlike a polynomial, a vector has no
/// normal form to maintain, and two vectors are equal exactly when their elements are.
///
/// An `UnsignedVector` is serialized as the list of its elements.
///
/// # Examples
/// ```
/// use malachite_base::unsigned_vector::UnsignedVector;
///
/// let v = UnsignedVector {
///     elements: vec![1u32, 2, 3],
/// };
/// assert_eq!(v.dimension(), 3);
/// assert_eq!(v.to_string(), "(1, 2, 3)");
/// ```
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct UnsignedVector<T: PrimitiveUnsigned> {
    pub elements: Vec<T>,
}

impl<T: PrimitiveUnsigned> UnsignedVector<T> {
    /// Returns the dimension of an [`UnsignedVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// assert_eq!(
    ///     UnsignedVector {
    ///         elements: vec![4u32, 5]
    ///     }
    ///     .dimension(),
    ///     2
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32> {
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

macro_rules! impl_named_unsigned_vector {
    ($t:ident, $name:expr) => {
        impl Named for UnsignedVector<$t> {
            /// The name of this type, with its element type spelled out.
            const NAME: &'static str = $name;
        }
    };
}
impl_named_unsigned_vector!(u8, "UnsignedVector<u8>");
impl_named_unsigned_vector!(u16, "UnsignedVector<u16>");
impl_named_unsigned_vector!(u32, "UnsignedVector<u32>");
impl_named_unsigned_vector!(u64, "UnsignedVector<u64>");
impl_named_unsigned_vector!(u128, "UnsignedVector<u128>");
impl_named_unsigned_vector!(usize, "UnsignedVector<usize>");

/// `ShortlexUnsignedVector` is a wrapper around an [`UnsignedVector`], taking the
/// [`UnsignedVector`] by value.
///
/// [`UnsignedVector`] does not implement [`Ord`], because no order on vectors is canonical: lex,
/// graded lex, and the others all have their uses, and the order most natural mathematically,
/// comparing vectors of the same dimension element by element, is only partial. Wrapping an
/// [`UnsignedVector`] in a `ShortlexUnsignedVector` provides one total order: vectors are compared
/// first by dimension and then, in case of a tie, lexicographically, by their elements from first
/// to last. Its equality agrees with [`UnsignedVector`] equality.
///
/// Unlike lexicographic order on [`Vec`]s, this order puts every vector of a smaller dimension
/// before every vector of a larger one, so that $(5) < (0, 0)$. It is a well-order: any nonempty
/// set of vectors has a least element, the lexicographically least of those of the smallest
/// dimension present.
///
/// `ShortlexUnsignedVector` owns its value. This is useful in many cases, for example if you want
/// to use [`UnsignedVector`]s as keys in a [`BTreeMap`](alloc::collections::BTreeMap). In other
/// situations, it is better to use [`ShortlexUnsignedVectorRef`], which only has a reference to its
/// value.
// Serialized as its inner `UnsignedVector`, since the wrapper adds no data of its own.
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexUnsignedVector<T: PrimitiveUnsigned>(pub UnsignedVector<T>);

/// `ShortlexUnsignedVectorRef` is a wrapper around an [`UnsignedVector`], taking the
/// [`UnsignedVector`] by reference.
///
/// See the [`ShortlexUnsignedVector`] documentation for details.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ShortlexUnsignedVectorRef<'a, T: PrimitiveUnsigned>(pub &'a UnsignedVector<T>);

impl<T: PrimitiveUnsigned> ShortlexUnsignedVector<T> {
    /// Borrows a [`ShortlexUnsignedVector`] as a [`ShortlexUnsignedVectorRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::{
    ///     ShortlexUnsignedVector, ShortlexUnsignedVectorRef, UnsignedVector,
    /// };
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexUnsignedVector(v.clone());
    /// assert_eq!(x.as_ref(), ShortlexUnsignedVectorRef(&v));
    /// ```
    pub const fn as_ref(&self) -> ShortlexUnsignedVectorRef<'_, T> {
        ShortlexUnsignedVectorRef(&self.0)
    }
}

impl<T: PrimitiveUnsigned> Deref for ShortlexUnsignedVector<T> {
    type Target = UnsignedVector<T>;

    /// Allows a [`ShortlexUnsignedVector`] to dereference to an [`UnsignedVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::{ShortlexUnsignedVector, UnsignedVector};
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexUnsignedVector(v.clone());
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &UnsignedVector<T> {
        &self.0
    }
}

impl<T: PrimitiveUnsigned> Deref for ShortlexUnsignedVectorRef<'_, T> {
    type Target = UnsignedVector<T>;

    /// Allows a [`ShortlexUnsignedVectorRef`] to dereference to an [`UnsignedVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::{ShortlexUnsignedVectorRef, UnsignedVector};
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexUnsignedVectorRef(&v);
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &UnsignedVector<T> {
        self.0
    }
}

impl<T: PrimitiveUnsigned> Debug for ShortlexUnsignedVector<T> {
    /// Writes a [`ShortlexUnsignedVector`] as `ShortlexUnsignedVector(` followed by the vector and
    /// `)`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_vector::{ShortlexUnsignedVector, UnsignedVector};
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2)").unwrap();
    /// assert_eq!(
    ///     ShortlexUnsignedVector(v).to_debug_string(),
    ///     "ShortlexUnsignedVector((1, 2))"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("ShortlexUnsignedVector")
            .field(&self.0)
            .finish()
    }
}

impl<T: PrimitiveUnsigned> Debug for ShortlexUnsignedVectorRef<'_, T> {
    /// Writes a [`ShortlexUnsignedVectorRef`] as `ShortlexUnsignedVectorRef(` followed by the
    /// vector and `)`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_vector::{ShortlexUnsignedVectorRef, UnsignedVector};
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2)").unwrap();
    /// assert_eq!(
    ///     ShortlexUnsignedVectorRef(&v).to_debug_string(),
    ///     "ShortlexUnsignedVectorRef((1, 2))"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("ShortlexUnsignedVectorRef")
            .field(self.0)
            .finish()
    }
}
