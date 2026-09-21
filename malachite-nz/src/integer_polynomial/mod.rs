// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

/// Implementations of [`Ord`] and [`PartialOrd`] for [`IntegerPolynomial`], comparing two
/// polynomials by their behavior for large arguments.
pub mod comparison;
/// Functions for converting an [`IntegerPolynomial`] to and from other types.
pub mod conversion;
/// Iterators that generate [`IntegerPolynomial`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`IntegerPolynomial`]s randomly.
pub mod random;

// The zero `Integer`, as something a reference can be handed out to.
//
// A `Integer` owns a `Vec` when it is large, so it has a destructor, and a `&Integer::ZERO` written
// where a reference is returned would point at a temporary that does not outlive the call. A
// `static` is the same zero with a lifetime long enough to hand out.
static ZERO: Integer = Integer::ZERO;

/// A polynomial in one variable whose coefficients are [`Integer`]s.
///
/// The coefficients are held in ascending order, so that the coefficient of $x^i$ is the one at
/// index $i$, and the last is the leading one. Trailing zero coefficients are not held at all: the
/// zero polynomial has no coefficients, and every other polynomial's last coefficient is nonzero.
/// That is what makes a polynomial's representation unique, and so what lets [`Eq`] be derived.
///
/// The field is private, since not every [`Vec`] of [`Integer`]s is one:
/// [`from_coefficients_asc`](IntegerPolynomial::from_coefficients_asc) is how a [`Vec`] becomes
/// one.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeIntegerPolynomial", into = "SerdeIntegerPolynomial")
)]
pub struct IntegerPolynomial {
    coefficients: Vec<Integer>,
}

// As for a `NaturalPolynomial`: the coefficients are the polynomial, so the encoding is the list of
// them and nothing around it, and a list whose last coefficient is zero is rejected.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct SerdeIntegerPolynomial(pub(crate) Vec<Integer>);

/// The constant 0.
impl Zero for IntegerPolynomial {
    const ZERO: Self = Self {
        coefficients: Vec::new(),
    };
}

impl IntegerPolynomial {
    // Returns true iff `self` is valid.
    //
    // To be valid, its last coefficient, if it has one at all, must be nonzero. All
    // `IntegerPolynomial`s must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.coefficients.last() != Some(&Integer::ZERO)
    }

    // Drops the trailing zero coefficients, which is what makes a `Vec` of coefficients the one
    // representation of its polynomial.
    fn trim(&mut self) {
        while self.coefficients.last() == Some(&Integer::ZERO) {
            self.coefficients.pop();
        }
    }

    /// The constant polynomial 1.
    ///
    /// This is a function rather than an associated constant, and
    /// [`One`](malachite_base::num::basic::traits::One) is not implemented, because a polynomial
    /// holds its coefficients in a [`Vec`] and a [`Vec`] with anything in it cannot be built at
    /// compile time. The zero polynomial has no coefficients, so
    /// [`ZERO`](malachite_base::num::basic::traits::Zero::ZERO) is a constant after all.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::one().to_string(), "1");
    /// assert_eq!(IntegerPolynomial::one().degree(), Some(0));
    /// ```
    pub fn one() -> Self {
        Self {
            coefficients: vec![Integer::ONE],
        }
    }

    /// The constant polynomial 2.
    ///
    /// This is a function rather than an associated constant, for the reason given by
    /// [`one`](Self::one).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::two().to_string(), "2");
    /// assert_eq!(IntegerPolynomial::two().degree(), Some(0));
    /// ```
    pub fn two() -> Self {
        Self {
            coefficients: vec![Integer::TWO],
        }
    }

    /// The constant polynomial -1.
    ///
    /// This is a function rather than an associated constant, for the reason given by
    /// [`one`](Self::one).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::negative_one().to_string(), "-1");
    /// assert_eq!(IntegerPolynomial::negative_one().degree(), Some(0));
    /// ```
    pub fn negative_one() -> Self {
        Self {
            coefficients: vec![Integer::NEGATIVE_ONE],
        }
    }

    /// Returns a reference to an [`IntegerPolynomial`]'s coefficients, in ascending order.
    ///
    /// The first is the constant term and the last is the leading coefficient, so the slice is what
    /// [`from_coefficients_asc`](Self::from_coefficients_asc) would take back. It holds no trailing
    /// zeros, and for the zero polynomial it is empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO.coefficients_asc().to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn coefficients_asc(&self) -> &[Integer] {
        &self.coefficients
    }

    /// Converts a [`Vec`] of [`Integer`]s to an [`IntegerPolynomial`].
    ///
    /// The coefficients are in ascending order, so that the first is the constant term. Trailing
    /// zeros are dropped, since a polynomial does not hold them; the [`Vec`] may therefore end with
    /// as many as it likes, and the empty [`Vec`] is the zero polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `coefficients.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_coefficients_asc(vec![
    ///     Integer::TWO,
    ///     Integer::from(3u32),
    ///     Integer::ONE,
    /// ]);
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// // The trailing zeros are not part of the polynomial.
    /// let q = IntegerPolynomial::from_coefficients_asc(vec![
    ///     Integer::TWO,
    ///     Integer::from(3u32),
    ///     Integer::ONE,
    ///     Integer::ZERO,
    ///     Integer::ZERO,
    /// ]);
    /// assert_eq!(q.to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_coefficients_asc(vec![]).to_string(),
    ///     "0"
    /// );
    /// ```
    pub fn from_coefficients_asc(coefficients: Vec<Integer>) -> Self {
        let mut p = Self { coefficients };
        p.trim();
        p
    }

    /// Converts an [`IntegerPolynomial`] to a [`Vec`] of [`Integer`]s, in ascending order.
    ///
    /// The first is the constant term and the last is the leading coefficient, so the [`Vec`] is
    /// what [`from_coefficients_asc`](Self::from_coefficients_asc) would take back. It holds no
    /// trailing zeros, and for the zero polynomial it is empty.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.into_coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     IntegerPolynomial::ZERO
    ///         .into_coefficients_asc()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn into_coefficients_asc(self) -> Vec<Integer> {
        self.coefficients
    }

    /// Returns the degree of an [`IntegerPolynomial`].
    ///
    /// The zero polynomial has no degree, and gives `None`. Every other polynomial's degree is the
    /// index of its leading coefficient, so that a nonzero constant has degree 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::ZERO.degree(), None);
    /// assert_eq!(IntegerPolynomial::from_str("5").unwrap().degree(), Some(0));
    /// assert_eq!(IntegerPolynomial::from_str("x").unwrap().degree(), Some(1));
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2+3*x+2").unwrap().degree(),
    ///     Some(2)
    /// );
    /// ```
    #[inline]
    pub fn degree(&self) -> Option<u64> {
        self.coefficients.len().checked_sub(1).map(u64::exact_from)
    }

    /// Returns a reference to one of an [`IntegerPolynomial`]'s coefficients.
    ///
    /// The index is the power of the variable the coefficient belongs to, so that index 0 gives the
    /// constant term. An index past the degree gives zero, which is the coefficient a polynomial
    /// has there.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(*p.coefficient(0), 2);
    /// assert_eq!(*p.coefficient(1), 3);
    /// assert_eq!(*p.coefficient(2), 1);
    /// assert_eq!(*p.coefficient(100), 0);
    /// ```
    #[inline]
    pub fn coefficient(&self, index: u64) -> &Integer {
        usize::try_from(index)
            .ok()
            .and_then(|i| self.coefficients.get(i))
            .unwrap_or(&ZERO)
    }

    /// Returns a reference to an [`IntegerPolynomial`]'s leading coefficient.
    ///
    /// This is the coefficient of the highest power of the variable that the polynomial has one
    /// for. The zero polynomial has no such power, and gives zero, which is what every one of its
    /// coefficients is.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("7*x^2+3*x+2").unwrap();
    /// assert_eq!(*p.leading_coefficient(), 7);
    /// assert_eq!(*IntegerPolynomial::ZERO.leading_coefficient(), 0);
    /// ```
    #[inline]
    pub fn leading_coefficient(&self) -> &Integer {
        self.coefficients.last().unwrap_or(&ZERO)
    }

    /// Mutates one of an [`IntegerPolynomial`]'s coefficients using a provided closure, and then
    /// returns whatever the closure returns.
    ///
    /// The index is the power of the variable the coefficient belongs to. An index past the degree
    /// is not an error: the polynomial grows to reach it, and the closure is handed the zero that
    /// was there all along.
    ///
    /// After the closure executes, this function drops whatever trailing zero coefficients the
    /// polynomial has acquired, so that a coefficient set to zero, or a growth that came to
    /// nothing, leaves no trace.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `index`, and $m$ is the cost of the
    /// closure.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    ///
    /// let ret = p.mutate_coefficient(1, |c| {
    ///     *c += Integer::ONE;
    ///     true
    /// });
    /// assert_eq!(p.to_string(), "x^2+4*x+2");
    /// assert_eq!(ret, true);
    ///
    /// // The polynomial grows to reach a coefficient it did not have.
    /// p.mutate_coefficient(5, |c| *c += Integer::ONE);
    /// assert_eq!(p.to_string(), "x^5+x^2+4*x+2");
    ///
    /// // Clearing the leading coefficient lowers the degree.
    /// p.mutate_coefficient(5, |c| *c = Integer::ZERO);
    /// assert_eq!(p.to_string(), "x^2+4*x+2");
    /// ```
    pub fn mutate_coefficient<F: FnOnce(&mut Integer) -> T, T>(&mut self, index: u64, f: F) -> T {
        let index = usize::exact_from(index);
        if index >= self.coefficients.len() {
            self.coefficients.resize(index + 1, Integer::ZERO);
        }
        let out = f(&mut self.coefficients[index]);
        self.trim();
        out
    }
}

impl_named!(IntegerPolynomial);

/// `ShortlexIntegerPolynomial` is a wrapper around an [`IntegerPolynomial`], taking the
/// [`IntegerPolynomial`] by value.
///
/// [`IntegerPolynomial`] is ordered by how its polynomials behave for large arguments, which is the
/// order that respects their arithmetic. Sometimes a different order is wanted: one that puts the
/// smaller polynomials first, whatever their signs, so that a list of them is enumerated from the
/// simplest upward. Wrapping an [`IntegerPolynomial`] in a `ShortlexIntegerPolynomial` provides
/// one: polynomials are compared first by degree and then, in case of a tie, by their coefficients
/// from highest to lowest. This is a total order whose equality agrees with [`IntegerPolynomial`]
/// equality; it is FLINT's order for polynomials, the one `fmpq_poly_cmp` implements.
///
/// The difference from the [`Ord`] implementation on [`IntegerPolynomial`] is what happens when the
/// degrees differ. There, a polynomial of higher degree dominates, so it is the greater one only if
/// its leading coefficient is positive, and $-x^3 < x^2$. Here, degree decides outright, so $-x^3 >
/// x^2$.
///
/// Neither order is a well-order. Ordering by degree first does not make one: $x > x - 1 > x - 2 >
/// \ldots$ all have degree 1, so the chain descends forever under either order. No order that
/// restricts to the usual order on the constant polynomials can be a well-order, since the
/// [`Integer`]s are not well-ordered.
///
/// `ShortlexIntegerPolynomial` owns its value. This is useful in many cases, for example if you
/// want to use [`IntegerPolynomial`]s as keys in a map. In other situations, it is better to use
/// [`ShortlexIntegerPolynomialRef`], which only has a reference to its value.
// Serialized as its inner `IntegerPolynomial`, since the wrapper adds no data of its own.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ShortlexIntegerPolynomial(pub IntegerPolynomial);

/// `ShortlexIntegerPolynomialRef` is a wrapper around an [`IntegerPolynomial`], taking the
/// [`IntegerPolynomial`] by reference.
///
/// See the [`ShortlexIntegerPolynomial`] documentation for details.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortlexIntegerPolynomialRef<'a>(pub &'a IntegerPolynomial);

impl ShortlexIntegerPolynomial {
    /// Borrows a [`ShortlexIntegerPolynomial`] as a [`ShortlexIntegerPolynomialRef`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::{
    ///     IntegerPolynomial, ShortlexIntegerPolynomial, ShortlexIntegerPolynomialRef,
    /// };
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// let x = ShortlexIntegerPolynomial(p.clone());
    /// assert_eq!(x.as_ref(), ShortlexIntegerPolynomialRef(&p));
    /// ```
    pub const fn as_ref(&self) -> ShortlexIntegerPolynomialRef<'_> {
        ShortlexIntegerPolynomialRef(&self.0)
    }
}

impl Deref for ShortlexIntegerPolynomial {
    type Target = IntegerPolynomial;

    /// Allows a [`ShortlexIntegerPolynomial`] to dereference to an [`IntegerPolynomial`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::{IntegerPolynomial, ShortlexIntegerPolynomial};
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// let x = ShortlexIntegerPolynomial(p.clone());
    /// assert_eq!(*x, p);
    /// ```
    fn deref(&self) -> &IntegerPolynomial {
        &self.0
    }
}

impl Deref for ShortlexIntegerPolynomialRef<'_> {
    type Target = IntegerPolynomial;

    /// Allows a [`ShortlexIntegerPolynomialRef`] to dereference to an [`IntegerPolynomial`].
    ///
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::{IntegerPolynomial, ShortlexIntegerPolynomialRef};
    ///
    /// let p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// let x = ShortlexIntegerPolynomialRef(&p);
    /// assert_eq!(*x, p);
    /// ```
    fn deref(&self) -> &IntegerPolynomial {
        self.0
    }
}
