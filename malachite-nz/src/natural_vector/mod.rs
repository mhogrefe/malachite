// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::integer_vector::arithmetic::sum_max_bits::vec_sum_max_bits;
use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;

/// Implementations of [`Index`](core::ops::Index) and [`IndexMut`](core::ops::IndexMut) for
/// [`NaturalVector`].
pub mod access;
/// Traits for arithmetic on [`NaturalVector`]s.
pub mod arithmetic;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`ShortlexNaturalVector`] and
/// [`ShortlexNaturalVectorRef`], comparing two vectors by dimension and then lexicographically.
pub mod comparison;
/// Functions for converting a [`NaturalVector`] to and from other types.
pub mod conversion;
/// Iterators that generate [`NaturalVector`]s without repetition.
pub mod exhaustive;
/// An implementation of [`Extend`], for appending the elements produced by an iterator to a
/// [`NaturalVector`].
pub mod extend;
/// Traits for logic and bit manipulation on [`NaturalVector`]s.
pub mod logic;
/// Iterators that generate [`NaturalVector`]s randomly.
#[cfg(feature = "random")]
pub mod random;
/// Implementations of [`SelectCoordinates`](malachite_base::vector::SelectCoordinates) and
/// [`SelectCoordinatesAssign`](malachite_base::vector::SelectCoordinatesAssign), for selecting
/// coordinates of a [`NaturalVector`] by index.
pub mod select_coordinates;

// The height of the 0-dimensional vector, which `height_ref` lends. A `Natural` owns a `Vec` when
// it is large, so it has a destructor, and a reference to a constant with a destructor cannot be
// promoted to `'static`; a `static` can be borrowed for as long as needed.
pub(crate) static ZERO: Natural = Natural::ZERO;

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
/// use malachite_base::vector::Vector;
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

impl Vector for NaturalVector {
    type Element = Natural;
    type ElementOutput<'a>
        = &'a Natural
    where
        Self: 'a;

    /// Converts a slice of [`Natural`]s to a [`NaturalVector`], cloning them.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_elements(&[Natural::ONE, Natural::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(NaturalVector::from_elements(&[]).to_string(), "()");
    /// ```
    #[inline]
    fn from_elements(xs: &[Natural]) -> Self {
        Self {
            elements: xs.to_vec(),
        }
    }

    /// Converts a [`Vec`] of [`Natural`]s to a [`NaturalVector`], taking ownership of it.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_owned_elements(vec![Natural::ONE, Natural::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_owned_elements(Vec::new()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    fn from_owned_elements(xs: Vec<Natural>) -> Self {
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(NaturalVector::zero(3).to_string(), "(0, 0, 0)");
    /// assert_eq!(NaturalVector::zero(0).to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `fmpz_vec_init` from `fmpz_vec/vec.c`, FLINT 3.6.0, and to
    /// `_fmpz_vec_init` from `fmpz_vec.h`, with elements that are never negative.
    #[inline]
    fn zero(dimension: u64) -> Self {
        Self {
            elements: vec![Natural::ZERO; usize::exact_from(dimension)],
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::standard_basis_vector(3, 1).to_string(),
    ///     "(0, 1, 0)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::standard_basis_vector(1, 0).to_string(),
    ///     "(1)"
    /// );
    /// ```
    fn standard_basis_vector(dimension: u64, index: u64) -> Self {
        assert!(
            index < dimension,
            "the index {index} is not less than the dimension {dimension}"
        );
        let mut v = Self::zero(dimension);
        v.elements[usize::exact_from(index)] = Natural::ONE;
        v
    }

    /// Appends an element to the end of a [`NaturalVector`], increasing its dimension by 1.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    /// v.push(Natural::from(3u32));
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    ///
    /// let mut v = NaturalVector::zero(0);
    /// v.push(Natural::from(5u32));
    /// assert_eq!(v.to_string(), "(5)");
    /// ```
    ///
    /// This is equivalent to `fmpz_vec_append` from `fmpz_vec/vec.c`, FLINT 3.6.0, with an element
    /// that is never negative.
    #[inline]
    fn push(&mut self, x: Natural) {
        self.elements.push(x);
    }

    /// Sets the dimension of a [`NaturalVector`], removing elements from the end if the new
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.set_dimension(5);
    /// assert_eq!(v.to_string(), "(1, 2, 3, 0, 0)");
    /// v.set_dimension(2);
    /// assert_eq!(v.to_string(), "(1, 2)");
    /// v.set_dimension(0);
    /// assert_eq!(v.to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `fmpz_vec_set_length` from `fmpz_vec/vec.c`, FLINT 3.6.0, with
    /// elements that are never negative.
    #[inline]
    fn set_dimension(&mut self, dimension: u64) {
        self.elements
            .resize(usize::exact_from(dimension), Natural::ZERO);
    }

    /// Sets every element of a [`NaturalVector`] to zero, keeping its dimension.
    ///
    /// Afterwards the vector equals [`zero`](Vector::zero) of the same dimension.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// v.set_zero();
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    ///
    /// let mut v = NaturalVector::from_str("()").unwrap();
    /// v.set_zero();
    /// assert_eq!(v.to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_zero` from `fmpz_vec/zero.c`, FLINT 3.6.0.
    #[inline]
    fn set_zero(&mut self) {
        self.elements.fill(Natural::ZERO);
    }

    /// Returns a [`NaturalVector`]'s elements as a [`Vec`], cloning them.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.to_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .to_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn to_elements(&self) -> Vec<Natural> {
        self.elements.clone()
    }

    /// Returns a [`NaturalVector`]'s elements as a [`Vec`], taking ownership of the
    /// [`NaturalVector`].
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.into_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .into_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn into_elements(self) -> Vec<Natural> {
        self.elements
    }

    /// Returns a reference to a [`NaturalVector`]'s elements, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.elements_ref().to_debug_string(), "[1, 2, 3]");
    ///
    /// // A slice of the elements can be taken directly.
    /// let tail = &v.elements_ref()[1..];
    /// assert_eq!(tail.to_debug_string(), "[2, 3]");
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .elements_ref()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn elements_ref(&self) -> &[Natural] {
        &self.elements
    }

    /// Returns the dimension of a [`NaturalVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::vector::Vector;
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
    fn dimension(&self) -> u64 {
        u64::exact_from(self.elements.len())
    }

    /// Returns the pivot of a [`NaturalVector`]: its first nonzero element.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(0, 0, 3, 0, 5)").unwrap().pivot(),
    ///     Some(&Natural::from(3u32))
    /// );
    /// assert_eq!(NaturalVector::from_str("(0, 0)").unwrap().pivot(), None);
    /// assert_eq!(NaturalVector::from_str("()").unwrap().pivot(), None);
    /// ```
    #[inline]
    fn pivot(&self) -> Option<&Natural> {
        self.elements.iter().find(|x| **x != 0u32)
    }

    /// Returns the index of the pivot of a [`NaturalVector`]: the position of its first nonzero
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(0, 0, 3, 0, 5)")
    ///         .unwrap()
    ///         .pivot_index(),
    ///     Some(2)
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_str("(0, 0)").unwrap().pivot_index(),
    ///     None
    /// );
    /// assert_eq!(NaturalVector::from_str("()").unwrap().pivot_index(), None);
    /// ```
    #[inline]
    fn pivot_index(&self) -> Option<u64> {
        self.elements
            .iter()
            .position(|x| *x != 0u32)
            .map(u64::exact_from)
    }

    /// Returns the index of the largest element of a [`NaturalVector`]: the first one, when several
    /// are tied.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 3, 3, 2)")
    ///         .unwrap()
    ///         .height_index(),
    ///     Some(1)
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_str("(5, 5)").unwrap().height_index(),
    ///     Some(0)
    /// );
    /// assert_eq!(NaturalVector::from_str("()").unwrap().height_index(), None);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_height_index` from `fmpz_vec/height_index.c`, FLINT 3.6.0,
    /// except that it returns `None` for the 0-dimensional vector, where FLINT requires a nonempty
    /// vector.
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

    /// Returns the number of significant bits of the height of a [`NaturalVector`], together with
    /// whether any of its elements is negative.
    ///
    /// Since no element is negative, the flag is always `false`. The top limbs of the elements with
    /// the most limbs are or-ed together, so no element's bit length is computed separately.
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
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 5, 2)").unwrap().max_bits(),
    ///     (3, false)
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_str("()").unwrap().max_bits(),
    ///     (0, false)
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_max_bits` from `fmpz_vec/max_bits.c`, FLINT 3.6.0, which
    /// returns the two results combined, as a count that is negated when some element is negative.
    #[inline]
    fn max_bits(&self) -> (u64, bool) {
        vec_max_bits(&self.elements)
    }

    /// Returns the number of significant bits of the sum of the absolute values of the elements of
    /// a [`NaturalVector`] (its $\ell^1$ norm), together with the number of significant bits of its
    /// height.
    ///
    /// While every element fits in one limb, the elements are added in two limbs, so nothing is
    /// allocated.
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
    /// use malachite_base::vector::Vector;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 5, 2)").unwrap().sum_max_bits(),
    ///     (4, 3)
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_str("()").unwrap().sum_max_bits(),
    ///     (0, 0)
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_sum_max_bits` from `fmpz_vec/sum_max_bits.c`, FLINT 3.6.0.
    #[inline]
    fn sum_max_bits(&self) -> (u64, u64) {
        vec_sum_max_bits(&self.elements)
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
/// before every vector of a larger one, so that $(5) < (0, 0)$. It is a well-order: any nonempty
/// set of vectors has a least element, the lexicographically least of those of the smallest
/// dimension present, which exists because lexicographic order on the vectors of a fixed dimension
/// is a well-order.
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
