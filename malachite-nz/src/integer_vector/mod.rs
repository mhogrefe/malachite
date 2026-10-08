// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;

/// Implementations of [`Index`](core::ops::Index) and [`IndexMut`](core::ops::IndexMut) for
/// [`IntegerVector`].
pub mod access;
/// Traits for arithmetic on [`IntegerVector`]s.
pub mod arithmetic;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`ShortlexIntegerVector`] and
/// [`ShortlexIntegerVectorRef`], comparing two vectors by dimension and then lexicographically.
pub mod comparison;
/// Functions for converting an [`IntegerVector`] to and from other types.
pub mod conversion;
/// Iterators that generate [`IntegerVector`]s without repetition.
pub mod exhaustive;
/// Iterators that generate [`IntegerVector`]s randomly.
#[cfg(feature = "random")]
pub mod random;

/// A vector whose elements are [`Integer`]s.
///
/// Its dimension is the number of elements, and the 0-dimensional vector has none. The field is
/// public, since every [`Vec`] of [`Integer`]s is a valid vector: unlike a polynomial, a vector has
/// no normal form to maintain, and two vectors are equal exactly when their elements are.
///
/// An `IntegerVector` is serialized as the list of its elements, each serialized the way a
/// [`Integer`] is.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Two};
/// use malachite_base::vector::Vector;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_vector::IntegerVector;
///
/// let v = IntegerVector {
///     elements: vec![Integer::ONE, Integer::TWO, Integer::from(3u32)],
/// };
/// assert_eq!(v.dimension(), 3);
/// ```
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct IntegerVector {
    pub elements: Vec<Integer>,
}

impl Vector for IntegerVector {
    type Element = Integer;
    type ElementOutput<'a>
        = &'a Integer
    where
        Self: 'a;

    /// Converts a slice of [`Integer`]s to an [`IntegerVector`], cloning them.
    ///
    /// The vector's dimension is the length of the slice. Every slice is a valid vector, so this
    /// cannot fail; the empty slice gives the 0-dimensional vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_elements(&[Integer::ONE, Integer::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(IntegerVector::from_elements(&[]).to_string(), "()");
    /// ```
    #[inline]
    fn from_elements(xs: &[Integer]) -> Self {
        Self {
            elements: xs.to_vec(),
        }
    }

    /// Converts a [`Vec`] of [`Integer`]s to an [`IntegerVector`], taking ownership of it.
    ///
    /// The vector's dimension is the length of the [`Vec`]. Every [`Vec`] is a valid vector, so
    /// this cannot fail, and nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_owned_elements(vec![Integer::ONE, Integer::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_owned_elements(Vec::new()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    fn from_owned_elements(xs: Vec<Integer>) -> Self {
        Self { elements: xs }
    }

    /// Returns an [`IntegerVector`]'s elements as a [`Vec`], cloning them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.to_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .to_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn to_elements(&self) -> Vec<Integer> {
        self.elements.clone()
    }

    /// Returns an [`IntegerVector`]'s elements as a [`Vec`], taking ownership of the
    /// [`IntegerVector`].
    ///
    /// Nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.into_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .into_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn into_elements(self) -> Vec<Integer> {
        self.elements
    }

    /// Returns a reference to an [`IntegerVector`]'s elements, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.elements_ref().to_debug_string(), "[1, 2, 3]");
    ///
    /// // A slice of the elements can be taken directly.
    /// let tail = &v.elements_ref()[1..];
    /// assert_eq!(tail.to_debug_string(), "[2, 3]");
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .elements_ref()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn elements_ref(&self) -> &[Integer] {
        &self.elements
    }

    /// Returns the dimension of an [`IntegerVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let v = IntegerVector {
    ///     elements: vec![Integer::from(4u32), Integer::from(5u32)],
    /// };
    /// assert_eq!(v.dimension(), 2);
    /// assert_eq!(
    ///     IntegerVector {
    ///         elements: Vec::new()
    ///     }
    ///     .dimension(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn dimension(&self) -> u64 {
        u64::exact_from(self.elements.len())
    }

    /// Returns the pivot of an [`IntegerVector`]: its first nonzero element.
    ///
    /// This is the element that leads the vector when it is a row of a matrix in echelon form. It
    /// returns a reference to it, or `None` if every element is zero, which includes the
    /// 0-dimensional vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0, -3, 0, 5)").unwrap().pivot(),
    ///     Some(&Integer::from(-3))
    /// );
    /// assert_eq!(IntegerVector::from_str("(0, 0)").unwrap().pivot(), None);
    /// assert_eq!(IntegerVector::from_str("()").unwrap().pivot(), None);
    /// ```
    #[inline]
    fn pivot(&self) -> Option<&Integer> {
        self.elements.iter().find(|x| **x != 0u32)
    }

    /// Returns the index of the pivot of an [`IntegerVector`]: the position of its first nonzero
    /// element.
    ///
    /// Indices start at 0, as they do for [`Index`](core::ops::Index). Returns `None` if every
    /// element is zero, which includes the 0-dimensional vector. When it returns `Some(i)`,
    /// [`pivot`](Self::pivot) is the element at `i`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0, -3, 0, 5)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     Some(2)
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0)").unwrap().pivot_index(),
    ///     None
    /// );
    /// assert_eq!(IntegerVector::from_str("()").unwrap().pivot_index(), None);
    /// ```
    #[inline]
    fn pivot_index(&self) -> Option<u64> {
        self.elements
            .iter()
            .position(|x| *x != 0u32)
            .map(u64::exact_from)
    }
}

// Implements `Named` for `IntegerVector`.
impl_named!(IntegerVector);

/// `ShortlexIntegerVector` is a wrapper around an [`IntegerVector`], taking the [`IntegerVector`]
/// by value.
///
/// [`IntegerVector`] does not implement [`Ord`], because no order on vectors is canonical: lex,
/// graded lex, and the others all have their uses, and the order most natural mathematically,
/// comparing vectors of the same dimension element by element, is only partial. Wrapping a
/// [`IntegerVector`] in an `ShortlexIntegerVector` provides one total order: vectors are compared
/// first by dimension and then, in case of a tie, lexicographically, by their elements from first
/// to last. Its equality agrees with [`IntegerVector`] equality.
///
/// Unlike lexicographic order on [`Vec`]s, this order puts every vector of a smaller dimension
/// before every vector of a larger one, so that $(5) < (0, 0)$. It is not a well-order, since the
/// [`Integer`]s are not: $(0) > (-1) > (-2) > \ldots$ descends forever.
///
/// `ShortlexIntegerVector` owns its value. This is useful in many cases, for example if you want to
/// use [`IntegerVector`]s as keys in a [`BTreeMap`](alloc::collections::BTreeMap). In other
/// situations, it is better to use [`ShortlexIntegerVectorRef`], which only has a reference to its
/// value.
// Serialized as its inner `IntegerVector`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexIntegerVector(pub IntegerVector);

/// `ShortlexIntegerVectorRef` is a wrapper around an [`IntegerVector`], taking the
/// [`IntegerVector`] by reference.
///
/// See the [`ShortlexIntegerVector`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortlexIntegerVectorRef<'a>(pub &'a IntegerVector);

impl ShortlexIntegerVector {
    /// Borrows an [`ShortlexIntegerVector`] as a [`ShortlexIntegerVectorRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::{
    ///     IntegerVector, ShortlexIntegerVector, ShortlexIntegerVectorRef,
    /// };
    ///
    /// let v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexIntegerVector(v.clone());
    /// assert_eq!(x.as_ref(), ShortlexIntegerVectorRef(&v));
    /// ```
    pub const fn as_ref(&self) -> ShortlexIntegerVectorRef<'_> {
        ShortlexIntegerVectorRef(&self.0)
    }
}

impl Deref for ShortlexIntegerVector {
    type Target = IntegerVector;

    /// Allows an [`ShortlexIntegerVector`] to dereference to an [`IntegerVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::{IntegerVector, ShortlexIntegerVector};
    ///
    /// let v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexIntegerVector(v.clone());
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &IntegerVector {
        &self.0
    }
}

impl Deref for ShortlexIntegerVectorRef<'_> {
    type Target = IntegerVector;

    /// Allows a [`ShortlexIntegerVectorRef`] to dereference to an [`IntegerVector`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::{IntegerVector, ShortlexIntegerVectorRef};
    ///
    /// let v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    /// let x = ShortlexIntegerVectorRef(&v);
    /// assert_eq!(*x, v);
    /// ```
    fn deref(&self) -> &IntegerVector {
        self.0
    }
}
