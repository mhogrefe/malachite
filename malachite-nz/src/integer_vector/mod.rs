// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::integer_vector::arithmetic::sum_max_bits::vec_sum_max_bits;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{One, Zero};
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
/// An implementation of [`Extend`], for appending the elements produced by an iterator to an
/// [`IntegerVector`].
pub mod extend;
/// Traits for logic and bit manipulation on [`IntegerVector`]s.
pub mod logic;
/// Iterators that generate [`IntegerVector`]s randomly.
#[cfg(feature = "random")]
pub mod random;
/// Implementations of [`SelectCoordinates`](malachite_base::vector::SelectCoordinates) and
/// [`SelectCoordinatesAssign`](malachite_base::vector::SelectCoordinatesAssign), for selecting
/// coordinates of an [`IntegerVector`] by index.
pub mod select_coordinates;

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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(IntegerVector::zero(3).to_string(), "(0, 0, 0)");
    /// assert_eq!(IntegerVector::zero(0).to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `fmpz_vec_init` from `fmpz_vec/vec.c`, FLINT 3.6.0, and to
    /// `_fmpz_vec_init` from `fmpz_vec.h`.
    #[inline]
    fn zero(dimension: u64) -> Self {
        Self {
            elements: vec![Integer::ZERO; usize::exact_from(dimension)],
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::standard_basis_vector(3, 1).to_string(),
    ///     "(0, 1, 0)"
    /// );
    /// assert_eq!(
    ///     IntegerVector::standard_basis_vector(1, 0).to_string(),
    ///     "(1)"
    /// );
    /// ```
    fn standard_basis_vector(dimension: u64, index: u64) -> Self {
        assert!(
            index < dimension,
            "the index {index} is not less than the dimension {dimension}"
        );
        let mut v = Self::zero(dimension);
        v.elements[usize::exact_from(index)] = Integer::ONE;
        v
    }

    /// Appends an element to the end of an [`IntegerVector`], increasing its dimension by 1.
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, 2)").unwrap();
    /// v.push(Integer::from(-3));
    /// assert_eq!(v.to_string(), "(1, 2, -3)");
    ///
    /// let mut v = IntegerVector::zero(0);
    /// v.push(Integer::from(5));
    /// assert_eq!(v.to_string(), "(5)");
    /// ```
    ///
    /// This is equivalent to `fmpz_vec_append` from `fmpz_vec/vec.c`, FLINT 3.6.0.
    #[inline]
    fn push(&mut self, x: Integer) {
        self.elements.push(x);
    }

    /// Sets the dimension of an [`IntegerVector`], removing elements from the end if the new
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    /// v.set_dimension(5);
    /// assert_eq!(v.to_string(), "(1, 2, 3, 0, 0)");
    /// v.set_dimension(2);
    /// assert_eq!(v.to_string(), "(1, 2)");
    /// v.set_dimension(0);
    /// assert_eq!(v.to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `fmpz_vec_set_length` from `fmpz_vec/vec.c`, FLINT 3.6.0.
    #[inline]
    fn set_dimension(&mut self, dimension: u64) {
        self.elements
            .resize(usize::exact_from(dimension), Integer::ZERO);
    }

    /// Sets every element of an [`IntegerVector`] to zero, keeping its dimension.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, -2, 3)").unwrap();
    /// v.set_zero();
    /// assert_eq!(v.to_string(), "(0, 0, 0)");
    ///
    /// let mut v = IntegerVector::from_str("()").unwrap();
    /// v.set_zero();
    /// assert_eq!(v.to_string(), "()");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_zero` from `fmpz_vec/zero.c`, FLINT 3.6.0.
    #[inline]
    fn set_zero(&mut self) {
        self.elements.fill(Integer::ZERO);
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

    /// Returns the index of an element of largest absolute value of an [`IntegerVector`]: the first
    /// one, when several are tied.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, -3, 3, 2)")
    ///         .unwrap()
    ///         .height_index(),
    ///     Some(1)
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(-5, 5)").unwrap().height_index(),
    ///     Some(0)
    /// );
    /// assert_eq!(IntegerVector::from_str("()").unwrap().height_index(), None);
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
            .max_by(|(_, x), (_, y)| x.unsigned_abs_ref().cmp(y.unsigned_abs_ref()))
            .map(|(i, _)| u64::exact_from(i))
    }

    /// Determines whether every element of an [`IntegerVector`] is zero.
    ///
    /// The 0-dimensional vector, which has no elements, counts as zero.
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
    /// assert!(IntegerVector::from_str("(0, 0)").unwrap().is_zero());
    /// assert!(IntegerVector::from_str("()").unwrap().is_zero());
    /// assert!(!IntegerVector::from_str("(0, 1, 0)").unwrap().is_zero());
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_is_zero` from `fmpz_vec/is_zero.c`, FLINT 3.6.0.
    #[inline]
    fn is_zero(&self) -> bool {
        self.elements.iter().all(|x| *x == 0u32)
    }

    /// Determines whether an [`IntegerVector`] is a standard basis vector, returning the index of
    /// its 1 if it is.
    ///
    /// A standard basis vector has a single element equal to 1 and every other element 0. Returns
    /// `None` for every other vector, including the zero vectors and the 0-dimensional vector.
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
    ///     IntegerVector::from_str("(0, 1, 0)")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     Some(1)
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, -1, 0)")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     None
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, 0, 1)")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     None
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("(0, 0)")
    ///         .unwrap()
    ///         .standard_basis_index(),
    ///     None
    /// );
    /// ```
    fn standard_basis_index(&self) -> Option<u64> {
        let index = self.pivot_index()?;
        let i = usize::exact_from(index);
        if self.elements[i] == 1u32 && self.elements[i + 1..].iter().all(|x| *x == 0u32) {
            Some(index)
        } else {
            None
        }
    }

    /// Returns the number of significant bits of the height of an [`IntegerVector`], together with
    /// whether any of its elements is negative.
    ///
    /// The top limbs of the elements with the most limbs are or-ed together, so no element's bit
    /// length is computed separately.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, -5, 2)").unwrap().max_bits(),
    ///     (3, true)
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("()").unwrap().max_bits(),
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
    /// an [`IntegerVector`] (its $\ell^1$ norm), together with the number of significant bits of
    /// its height.
    ///
    /// While every element fits in one limb, the absolute values are added in two limbs, so nothing
    /// is allocated.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, -5, 2)")
    ///         .unwrap()
    ///         .sum_max_bits(),
    ///     (4, 3)
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("()").unwrap().sum_max_bits(),
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
