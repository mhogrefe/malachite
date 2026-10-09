// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::HeightRef;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;

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
/// An implementation of [`Extend`], for appending the elements produced by an iterator to a
/// [`RationalVector`].
pub mod extend;
/// Iterators that generate [`RationalVector`]s randomly.
#[cfg(feature = "random")]
pub mod random;
/// Implementations of [`SelectCoordinates`](malachite_base::vector::SelectCoordinates) and
/// [`SelectCoordinatesAssign`](malachite_base::vector::SelectCoordinatesAssign), for selecting
/// coordinates of a [`RationalVector`] by index.
pub mod select_coordinates;

// The height of the 0-dimensional vector, which `height_ref` lends. A `Natural` owns a `Vec` when
// it is large, so it has a destructor, and a reference to a constant with a destructor cannot be
// promoted to `'static`; a `static` can be borrowed for as long as needed.
pub(crate) static ZERO: Natural = Natural::ZERO;

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
/// use malachite_base::vector::Vector;
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

impl Vector for RationalVector {
    type Element = Rational;
    type ElementOutput<'a>
        = &'a Rational
    where
        Self: 'a;

    /// Converts a slice of [`Rational`]s to a [`RationalVector`], cloning them.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_elements(&[Rational::ONE, Rational::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(RationalVector::from_elements(&[]).to_string(), "()");
    /// ```
    #[inline]
    fn from_elements(xs: &[Rational]) -> Self {
        Self {
            elements: xs.to_vec(),
        }
    }

    /// Converts a [`Vec`] of [`Rational`]s to a [`RationalVector`], taking ownership of it.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_owned_elements(vec![Rational::ONE, Rational::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(
    ///     RationalVector::from_owned_elements(Vec::new()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    fn from_owned_elements(xs: Vec<Rational>) -> Self {
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
    /// use malachite_base::vector::Vector;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(RationalVector::zero(3).to_string(), "(0, 0, 0)");
    /// assert_eq!(RationalVector::zero(0).to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `_fmpq_vec_init` from `fmpq_vec/init.c`, FLINT 3.6.0.
    #[inline]
    fn zero(dimension: u64) -> Self {
        Self {
            elements: vec![Rational::ZERO; usize::exact_from(dimension)],
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
    /// use malachite_base::vector::Vector;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::standard_basis_vector(3, 1).to_string(),
    ///     "(0, 1, 0)"
    /// );
    /// assert_eq!(
    ///     RationalVector::standard_basis_vector(1, 0).to_string(),
    ///     "(1)"
    /// );
    /// ```
    fn standard_basis_vector(dimension: u64, index: u64) -> Self {
        assert!(
            index < dimension,
            "the index {index} is not less than the dimension {dimension}"
        );
        let mut v = Self::zero(dimension);
        v.elements[usize::exact_from(index)] = Rational::ONE;
        v
    }

    /// Appends an element to the end of a [`RationalVector`], increasing its dimension by 1.
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
    /// use malachite_base::vector::Vector;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1, 2)").unwrap();
    /// v.push(Rational::from_unsigneds(1u32, 3));
    /// assert_eq!(v.to_string(), "(1, 2, 1/3)");
    ///
    /// let mut v = RationalVector::zero(0);
    /// v.push(Rational::from(5));
    /// assert_eq!(v.to_string(), "(5)");
    /// ```
    #[inline]
    fn push(&mut self, x: Rational) {
        self.elements.push(x);
    }

    /// Sets the dimension of a [`RationalVector`], removing elements from the end if the new
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
    /// use malachite_base::vector::Vector;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// v.set_dimension(5);
    /// assert_eq!(v.to_string(), "(1, 2, 3, 0, 0)");
    /// v.set_dimension(2);
    /// assert_eq!(v.to_string(), "(1, 2)");
    /// v.set_dimension(0);
    /// assert_eq!(v.to_string(), "()");
    /// ```
    #[inline]
    fn set_dimension(&mut self, dimension: u64) {
        self.elements
            .resize(usize::exact_from(dimension), Rational::ZERO);
    }

    /// Returns a [`RationalVector`]'s elements as a [`Vec`], cloning them.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.to_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .to_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn to_elements(&self) -> Vec<Rational> {
        self.elements.clone()
    }

    /// Returns a [`RationalVector`]'s elements as a [`Vec`], taking ownership of the
    /// [`RationalVector`].
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.into_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .into_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn into_elements(self) -> Vec<Rational> {
        self.elements
    }

    /// Returns a reference to a [`RationalVector`]'s elements, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::Vector;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.elements_ref().to_debug_string(), "[1, 2, 3]");
    ///
    /// // A slice of the elements can be taken directly.
    /// let tail = &v.elements_ref()[1..];
    /// assert_eq!(tail.to_debug_string(), "[2, 3]");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .elements_ref()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn elements_ref(&self) -> &[Rational] {
        &self.elements
    }

    /// Returns the dimension of a [`RationalVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vector::Vector;
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
    fn dimension(&self) -> u64 {
        u64::exact_from(self.elements.len())
    }

    /// Returns the pivot of a [`RationalVector`]: its first nonzero element.
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(0, 0, -3/4, 0, 5)")
    ///         .unwrap()
    ///         .pivot(),
    ///     Some(&Rational::from_signeds(-3, 4))
    /// );
    /// assert_eq!(RationalVector::from_str("(0, 0)").unwrap().pivot(), None);
    /// assert_eq!(RationalVector::from_str("()").unwrap().pivot(), None);
    /// ```
    #[inline]
    fn pivot(&self) -> Option<&Rational> {
        self.elements.iter().find(|x| **x != 0u32)
    }

    /// Returns the index of the pivot of a [`RationalVector`]: the position of its first nonzero
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(0, 0, -3/4, 0, 5)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     Some(2)
    /// );
    /// assert_eq!(
    ///     RationalVector::from_str("(0, 0)").unwrap().pivot_index(),
    ///     None
    /// );
    /// assert_eq!(RationalVector::from_str("()").unwrap().pivot_index(), None);
    /// ```
    #[inline]
    fn pivot_index(&self) -> Option<u64> {
        self.elements
            .iter()
            .position(|x| *x != 0u32)
            .map(u64::exact_from)
    }

    /// Returns the index of an element of largest height of a [`RationalVector`]: the first one,
    /// when several are tied.
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
    /// use malachite_base::vector::Vector;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -3, 1/4)")
    ///         .unwrap()
    ///         .height_index(),
    ///     Some(2)
    /// );
    /// assert_eq!(
    ///     RationalVector::from_str("(3, -1/3)")
    ///         .unwrap()
    ///         .height_index(),
    ///     Some(0)
    /// );
    /// assert_eq!(RationalVector::from_str("()").unwrap().height_index(), None);
    /// ```
    #[inline]
    fn height_index(&self) -> Option<u64> {
        // `max_by` returns the last of several equal maxima, so iterating in reverse gives the
        // first.
        self.elements
            .iter()
            .enumerate()
            .rev()
            .max_by(|(_, x), (_, y)| x.height_ref().cmp(y.height_ref()))
            .map(|(i, _)| u64::exact_from(i))
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
