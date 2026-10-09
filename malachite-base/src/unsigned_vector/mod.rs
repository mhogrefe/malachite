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
use crate::num::logic::traits::SignificantBits;
use crate::vector::Vector;
use alloc::vec;
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
/// An implementation of [`Extend`], for appending the elements produced by an iterator to an
/// [`UnsignedVector`].
pub mod extend;
/// Traits for logic and bit manipulation on [`UnsignedVector`]s.
pub mod logic;
#[cfg(feature = "random")]
/// Iterators that generate [`UnsignedVector`]s randomly.
pub mod random;
/// Implementations of [`SelectCoordinates`](crate::vector::SelectCoordinates) and
/// [`SelectCoordinatesAssign`](crate::vector::SelectCoordinatesAssign), for selecting coordinates
/// of an [`UnsignedVector`] by index.
pub mod select_coordinates;

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
/// use malachite_base::vector::Vector;
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

impl<T: PrimitiveUnsigned> Vector for UnsignedVector<T> {
    type Element = T;
    type ElementOutput<'a>
        = T
    where
        Self: 'a;

    /// Converts a slice to an [`UnsignedVector`], cloning the elements.
    ///
    /// The vector's dimension is the length of the slice. Every slice is a valid vector, so this
    /// cannot fail; the empty slice gives the 0-dimensional vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::from_elements(&[1u32, 2]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(UnsignedVector::<u32>::from_elements(&[]).to_string(), "()");
    /// ```
    #[inline]
    fn from_elements(xs: &[T]) -> Self {
        Self {
            elements: xs.to_vec(),
        }
    }

    /// Converts a [`Vec`] to an [`UnsignedVector`], taking ownership of it.
    ///
    /// The vector's dimension is the length of the [`Vec`]. Every [`Vec`] is a valid vector, so
    /// this cannot fail, and nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::from_owned_elements(vec![1u32, 2]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_owned_elements(Vec::new()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    fn from_owned_elements(xs: Vec<T>) -> Self {
        Self { elements: xs }
    }

    /// Returns the zero vector of a given dimension: a vector of `dimension` zeros.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `dimension`.
    ///
    /// # Panics
    /// Panics if `dimension` is greater than [`usize::MAX`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(UnsignedVector::<u8>::zero(3).to_string(), "(0, 0, 0)");
    /// assert_eq!(UnsignedVector::<u8>::zero(0).to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_init` from `nmod_vec.h`, FLINT 3.6.0, followed by
    /// `_nmod_vec_zero`, since `_nmod_vec_init` leaves the elements uninitialized.
    #[inline]
    fn zero(dimension: u64) -> Self {
        Self {
            elements: vec![T::ZERO; usize::exact_from(dimension)],
        }
    }

    /// Returns the standard basis vector $e_i$ of a given dimension: the vector whose element at
    /// `index` is 1 and whose other elements are 0.
    ///
    /// Indices start at 0, as they do for [`Index`](core::ops::Index).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `dimension`.
    ///
    /// # Panics
    /// Panics if `index` is greater than or equal to `dimension`, or if `dimension` is greater than
    /// [`usize::MAX`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::standard_basis_vector(3, 1).to_string(),
    ///     "(0, 1, 0)"
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u8>::standard_basis_vector(1, 0).to_string(),
    ///     "(1)"
    /// );
    /// ```
    fn standard_basis_vector(dimension: u64, index: u64) -> Self {
        assert!(
            index < dimension,
            "the index {index} is not less than the dimension {dimension}"
        );
        let mut v = Self::zero(dimension);
        v.elements[usize::exact_from(index)] = T::ONE;
        v
    }

    /// Appends an element to the end of an [`UnsignedVector`], increasing its dimension by 1.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`. The capacity
    /// grows geometrically, so pushing $n$ elements onto an empty vector takes $O(n)$ time in
    /// total.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    /// v.push(3);
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    ///
    /// let mut v = UnsignedVector::<u8>::zero(0);
    /// v.push(5);
    /// assert_eq!(v.to_string(), "(5)");
    /// ```
    #[inline]
    fn push(&mut self, x: T) {
        self.elements.push(x);
    }

    /// Sets the dimension of an [`UnsignedVector`], removing elements from the end if the new
    /// dimension is smaller, and appending zeros if it is larger.
    ///
    /// Reducing the dimension keeps the first `dimension` coordinates: it is the projection onto
    /// them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the larger of `self.dimension()` and
    /// `dimension`.
    ///
    /// # Panics
    /// Panics if `dimension` is greater than [`usize::MAX`].
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// v.set_dimension(5);
    /// assert_eq!(v.to_string(), "(1, 2, 3, 0, 0)");
    /// v.set_dimension(2);
    /// assert_eq!(v.to_string(), "(1, 2)");
    /// v.set_dimension(0);
    /// assert_eq!(v.to_string(), "()");
    /// ```
    #[inline]
    fn set_dimension(&mut self, dimension: u64) {
        self.elements.resize(usize::exact_from(dimension), T::ZERO);
    }

    /// Returns an [`UnsignedVector`]'s elements as a [`Vec`], cloning them.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.to_elements(), [1, 2, 3]);
    /// assert!(
    ///     UnsignedVector::<u32>::from_str("()")
    ///         .unwrap()
    ///         .to_elements()
    ///         .is_empty()
    /// );
    /// ```
    #[inline]
    fn to_elements(&self) -> Vec<T> {
        self.elements.clone()
    }

    /// Returns an [`UnsignedVector`]'s elements as a [`Vec`], taking ownership of the
    /// [`UnsignedVector`].
    ///
    /// Nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.into_elements(), [1, 2, 3]);
    /// assert!(
    ///     UnsignedVector::<u32>::from_str("()")
    ///         .unwrap()
    ///         .into_elements()
    ///         .is_empty()
    /// );
    /// ```
    #[inline]
    fn into_elements(self) -> Vec<T> {
        self.elements
    }

    /// Returns a reference to an [`UnsignedVector`]'s elements, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// let v = UnsignedVector::<u32>::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.elements_ref(), [1, 2, 3]);
    /// // A slice of the elements can be taken directly.
    /// assert_eq!(&v.elements_ref()[1..], [2, 3]);
    /// assert!(
    ///     UnsignedVector::<u32>::from_str("()")
    ///         .unwrap()
    ///         .elements_ref()
    ///         .is_empty()
    /// );
    /// ```
    #[inline]
    fn elements_ref(&self) -> &[T] {
        &self.elements
    }

    /// Returns the dimension of an [`UnsignedVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
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
    fn dimension(&self) -> u64 {
        u64::exact_from(self.elements.len())
    }

    /// Returns the pivot of an [`UnsignedVector`]: its first nonzero element.
    ///
    /// This is the element that leads the vector when it is a row of a matrix in echelon form. It
    /// returns a copy of it, or `None` if every element is zero, which includes the 0-dimensional
    /// vector.
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
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0, 3, 0, 5)")
    ///         .unwrap()
    ///         .pivot(),
    ///     Some(3)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0)").unwrap().pivot(),
    ///     None
    /// );
    /// assert_eq!(UnsignedVector::<u32>::from_str("()").unwrap().pivot(), None);
    /// ```
    #[inline]
    fn pivot(&self) -> Option<T> {
        self.elements.iter().find(|&&x| x != T::ZERO).copied()
    }

    /// Returns the index of the pivot of an [`UnsignedVector`]: the position of its first nonzero
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
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0, 3, 0, 5)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     Some(2)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("(0, 0)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     None
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u32>::from_str("()").unwrap().pivot_index(),
    ///     None
    /// );
    /// ```
    #[inline]
    fn pivot_index(&self) -> Option<u64> {
        self.elements
            .iter()
            .position(|&x| x != T::ZERO)
            .map(u64::exact_from)
    }

    /// Returns the index of the largest element of an [`UnsignedVector`]: the first one, when
    /// several are tied.
    ///
    /// Indices start at 0, as they do for [`Index`](core::ops::Index). Returns `None` for the
    /// 0-dimensional vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 3, 3, 2)")
    ///         .unwrap()
    ///         .height_index(),
    ///     Some(1)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(5, 5)")
    ///         .unwrap()
    ///         .height_index(),
    ///     Some(0)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("()").unwrap().height_index(),
    ///     None
    /// );
    /// ```
    #[inline]
    fn height_index(&self) -> Option<u64> {
        // `max_by` returns the last of several equal maxima, so iterating in reverse gives the
        // first.
        self.elements
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(i, _)| u64::exact_from(i))
    }

    /// Returns the number of significant bits of the height of an [`UnsignedVector`], together with
    /// whether any of its elements is negative.
    ///
    /// Since no element is negative, the flag is always `false`. The bit count is the bit length of
    /// the bitwise or of the elements.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(1, 5, 2)")
    ///         .unwrap()
    ///         .max_bits(),
    ///     (3, false)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("()").unwrap().max_bits(),
    ///     (0, false)
    /// );
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_max_bits` from `nmod_vec/max_bits.c`, FLINT 3.6.0, together
    /// with the flag.
    #[inline]
    fn max_bits(&self) -> (u64, bool) {
        let mut or = T::ZERO;
        for &x in &self.elements {
            or |= x;
        }
        (or.significant_bits(), false)
    }

    /// Returns the number of significant bits of the sum of the absolute values of the elements of
    /// an [`UnsignedVector`] (its $\ell^1$ norm), together with the number of significant bits of
    /// its height.
    ///
    /// The sum may not fit in a `T`, so the carries out of the top bit are counted; no wider type
    /// is needed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_base::vector::Vector;
    ///
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("(255, 255)")
    ///         .unwrap()
    ///         .sum_max_bits(),
    ///     (9, 8)
    /// );
    /// assert_eq!(
    ///     UnsignedVector::<u8>::from_str("()").unwrap().sum_max_bits(),
    ///     (0, 0)
    /// );
    /// ```
    fn sum_max_bits(&self) -> (u64, u64) {
        let mut sum = T::ZERO;
        let mut carries = 0u64;
        let mut or = T::ZERO;
        for &x in &self.elements {
            let overflow;
            (sum, overflow) = sum.overflowing_add(x);
            if overflow {
                carries += 1;
            }
            or |= x;
        }
        // The sum is `carries` times 2^W plus `sum`.
        let sum_bits = if carries == 0 {
            sum.significant_bits()
        } else {
            T::WIDTH + carries.significant_bits()
        };
        (sum_bits, or.significant_bits())
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
