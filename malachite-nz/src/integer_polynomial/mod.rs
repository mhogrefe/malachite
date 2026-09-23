// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::conversion::string::from_string::from_string_with;
use crate::integer_polynomial::conversion::string::to_string::Language;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::vars::{Var, VarScheme};

/// Traits for arithmetic on [`IntegerPolynomial`]s.
pub mod arithmetic;
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
pub(crate) static ZERO: Integer = Integer::ZERO;

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
    /// use malachite_base::polynomial::Polynomial;
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
}

impl Polynomial for IntegerPolynomial {
    type Coefficient = Integer;
    type CoefficientOutput<'a>
        = &'a Integer
    where
        Self: 'a;

    /// The constant polynomial 1.
    ///
    /// This is a function rather than an associated constant, and [`One`] is not implemented,
    /// because a polynomial holds its coefficients in a [`Vec`] and a [`Vec`] with anything in it
    /// cannot be built at compile time. The zero polynomial has no coefficients, so
    /// [`ZERO`](malachite_base::num::basic::traits::Zero::ZERO) is a constant after all.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::one().to_string(), "1");
    /// assert_eq!(IntegerPolynomial::one().degree(), Some(0));
    /// ```
    fn one() -> Self {
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
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::two().to_string(), "2");
    /// assert_eq!(IntegerPolynomial::two().degree(), Some(0));
    /// ```
    fn two() -> Self {
        Self {
            coefficients: vec![Integer::TWO],
        }
    }

    /// The polynomial $x$, of degree 1 with leading coefficient 1 and constant term 0.
    ///
    /// This is a function rather than an associated constant, for the reason given by
    /// [`one`](Self::one).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::x().to_string(), "x");
    /// assert_eq!(IntegerPolynomial::x().degree(), Some(1));
    /// ```
    fn x() -> Self {
        Self {
            coefficients: vec![Integer::ZERO, Integer::ONE],
        }
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
    /// use malachite_base::polynomial::Polynomial;
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
    fn from_coefficients_asc(coefficients: Vec<Integer>) -> Self {
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
    /// use malachite_base::polynomial::Polynomial;
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
    fn into_coefficients_asc(self) -> Vec<Integer> {
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
    /// use malachite_base::polynomial::Polynomial;
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
    fn degree(&self) -> Option<u64> {
        self.coefficients.len().checked_sub(1).map(u64::exact_from)
    }

    /// Returns the length of a [`IntegerPolynomial`]: the number of coefficients it holds.
    ///
    /// A polynomial holds no trailing zeros, so its length is one more than its
    /// [`degree`](Self::degree), and the zero polynomial, which has no degree, has length 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::ZERO.len(), 0);
    /// assert_eq!(IntegerPolynomial::from_str("-5").unwrap().len(), 1);
    /// assert_eq!(IntegerPolynomial::from_str("x").unwrap().len(), 2);
    /// assert_eq!(IntegerPolynomial::from_str("x^2-3*x+2").unwrap().len(), 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_length` from `fmpz_poly.h`, FLINT 3.6.0.
    #[inline]
    fn len(&self) -> u64 {
        u64::exact_from(self.coefficients.len())
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
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(*p.coefficient(0), 2);
    /// assert_eq!(*p.coefficient(1), 3);
    /// assert_eq!(*p.coefficient(2), 1);
    /// assert_eq!(*p.coefficient(100), 0);
    /// ```
    #[inline]
    fn coefficient(&self, index: u64) -> &Integer {
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
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("7*x^2+3*x+2").unwrap();
    /// assert_eq!(*p.leading_coefficient(), 7);
    /// assert_eq!(*IntegerPolynomial::ZERO.leading_coefficient(), 0);
    /// ```
    #[inline]
    fn leading_coefficient(&self) -> &Integer {
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
    /// # Panics
    /// Panics if `index` does not fit in a [`usize`], which cannot happen on a target with 64-bit
    /// pointers, or if growing to reach `index` would exceed the maximum length of a [`Vec`].
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::polynomial::Polynomial;
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
    fn mutate_coefficient<F: FnOnce(&mut Integer) -> T, T>(&mut self, index: u64, f: F) -> T {
        let index = usize::exact_from(index);
        if index >= self.coefficients.len() {
            self.coefficients.resize(index + 1, Integer::ZERO);
        }
        let out = f(&mut self.coefficients[index]);
        self.trim();
        out
    }

    /// Sets the coefficients of a [`IntegerPolynomial`] of $x^i$ for $i$ in `start..end` to zero.
    ///
    /// Indices past the degree are allowed; the coefficients there are zero already. Zeroing the
    /// leading coefficient lowers the degree, to that of the highest nonzero coefficient that
    /// remains.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `start > end`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p;
    /// p = IntegerPolynomial::from_str("5*x^4-4*x^3+3*x^2-2*x+1").unwrap();
    /// p.zero_coefficients(1, 3);
    /// assert_eq!(p.to_string(), "5*x^4-4*x^3+1");
    /// p = IntegerPolynomial::from_str("5*x^4-4*x^3+3*x^2-2*x+1").unwrap();
    /// p.zero_coefficients(2, 10);
    /// assert_eq!(p.to_string(), "-2*x+1");
    /// p = IntegerPolynomial::from_str("5*x^4-4*x^3+3*x^2-2*x+1").unwrap();
    /// p.zero_coefficients(5, 10);
    /// assert_eq!(p.to_string(), "5*x^4-4*x^3+3*x^2-2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_zero_coeffs` from `fmpz_poly/zero_coeffs.c`, FLINT 3.6.0.
    fn zero_coefficients(&mut self, start: u64, end: u64) {
        assert!(start <= end);
        let len = self.coefficients.len();
        let Ok(start) = usize::try_from(start) else {
            return;
        };
        if start >= len {
            return;
        }
        let end = usize::try_from(end).map_or(len, |end| end.min(len));
        if end == len {
            // The range reaches the leading coefficient, so the zeros it leaves are trailing.
            self.coefficients.truncate(start);
            self.trim();
        } else {
            self.coefficients[start..end].fill(Integer::ZERO);
        }
    }

    /// Reverses the coefficients of a [`IntegerPolynomial`], considered as having length `len`,
    /// taking the polynomial by reference.
    ///
    /// The polynomial is first truncated, or padded with zeros, to exactly `len` coefficients, and
    /// those are then reversed, so that the result's coefficient of $x^i$ is the polynomial's
    /// coefficient of $x^{\mathrm{len} - 1 - i}$:
    ///
    /// $$
    /// f(p, n) = x^{n-1} \left( p \bmod x^n \right)\!\left(\frac{1}{x}\right).
    /// $$
    ///
    /// A polynomial holds no trailing zeros, so the result may have fewer than `len` coefficients:
    /// it does whenever the polynomial's constant term is zero.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n + m)$
    ///
    /// $M(n, m) = O(n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $m$ is the total number of
    /// bits of the coefficients.
    ///
    /// # Panics
    /// Panics if `len` exceeds `self.len()` and does not fit in a [`usize`], which cannot happen on
    /// a target with 64-bit pointers.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-2*x+3")
    ///         .unwrap()
    ///         .reverse(3)
    ///         .to_string(),
    ///     "3*x^2-2*x+1"
    /// );
    /// // Padding to length 5 adds low zeros.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-2*x+3")
    ///         .unwrap()
    ///         .reverse(5)
    ///         .to_string(),
    ///     "3*x^4-2*x^3+x^2"
    /// );
    /// // Truncating to length 2 drops x^2 first.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-2*x+3")
    ///         .unwrap()
    ///         .reverse(2)
    ///         .to_string(),
    ///     "3*x-2"
    /// );
    /// // A zero constant term becomes a trailing zero, and is dropped.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-2*x")
    ///         .unwrap()
    ///         .reverse(3)
    ///         .to_string(),
    ///     "-2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_reverse` from `fmpz_poly/reverse.c`, FLINT 3.6.0.
    fn reverse(&self, len: u64) -> Self {
        let kept = usize::try_from(len).map_or(self.coefficients.len(), |len| {
            len.min(self.coefficients.len())
        });
        if kept == 0 {
            return Self::ZERO;
        }
        // The coefficients past the kept ones become the result's low zeros.
        let mut coefficients = vec![Integer::ZERO; usize::exact_from(len) - kept];
        coefficients.extend(self.coefficients[..kept].iter().rev().cloned());
        Self::from_coefficients_asc(coefficients)
    }

    /// Reverses the coefficients of a [`IntegerPolynomial`], considered as having length `len`, in
    /// place.
    ///
    /// See [`reverse`](Self::reverse) for what the result is.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the larger of `len` and
    /// `self.len()`.
    ///
    /// # Panics
    /// Panics if `len` exceeds `self.len()` and does not fit in a [`usize`], which cannot happen on
    /// a target with 64-bit pointers.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p;
    /// p = IntegerPolynomial::from_str("x^2-2*x+3").unwrap();
    /// p.reverse_assign(3);
    /// assert_eq!(p.to_string(), "3*x^2-2*x+1");
    /// // Padding to length 5 adds low zeros.
    /// p = IntegerPolynomial::from_str("x^2-2*x+3").unwrap();
    /// p.reverse_assign(5);
    /// assert_eq!(p.to_string(), "3*x^4-2*x^3+x^2");
    /// // Truncating to length 2 drops x^2 first.
    /// p = IntegerPolynomial::from_str("x^2-2*x+3").unwrap();
    /// p.reverse_assign(2);
    /// assert_eq!(p.to_string(), "3*x-2");
    /// // A zero constant term becomes a trailing zero, and is dropped.
    /// p = IntegerPolynomial::from_str("x^2-2*x").unwrap();
    /// p.reverse_assign(3);
    /// assert_eq!(p.to_string(), "-2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_reverse` from `fmpz_poly/reverse.c`, FLINT 3.6.0.
    fn reverse_assign(&mut self, len: u64) {
        let kept = usize::try_from(len).map_or(self.coefficients.len(), |len| {
            len.min(self.coefficients.len())
        });
        if kept == 0 {
            *self = Self::ZERO;
            return;
        }
        self.coefficients.truncate(kept);
        self.coefficients.reverse();
        // Pad at the high end and rotate the padding down, so that it becomes the low zeros.
        let len = usize::exact_from(len);
        self.coefficients.resize(len, Integer::ZERO);
        self.coefficients.rotate_right(len - kept);
        // The polynomial's low zeros, if any, are now at the top.
        self.trim();
    }

    /// Converts an [`IntegerPolynomial`] to a [`String`], naming its variable with any
    /// [`VarScheme`].
    ///
    /// The syntax is the one [`Display`](core::fmt::Display) writes, which that implementation
    /// describes; the only difference is that the variable is whichever one is handed in rather
    /// than `x`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the
    /// coefficients.
    ///
    /// # Panics
    /// Panics if `var`'s index is not less than its scheme's [`capacity`](VarScheme::capacity).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    /// use malachite_base::vars::list::ListVars;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.to_string_with(GreekVars.var(0)), "α^2+3*α+2");
    /// assert_eq!(p.to_string_with(IndexedVars.var(7)), "x₇^2+3*x₇+2");
    ///
    /// let vars = ListVars::new(["t"]);
    /// assert_eq!(p.to_string_with(vars.var(0)), "t^2+3*t+2");
    /// ```
    fn to_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String {
        let mut s = String::new();
        // Writing to a `String` cannot fail, so the result is the string itself.
        self.write_with_var(var, Language::Plain, &mut s).unwrap();
        s
    }

    /// Converts an [`IntegerPolynomial`] to a LaTeX math-mode fragment, naming its variable with
    /// any [`VarScheme`].
    ///
    /// The fragment is the one [`ToLatex`](malachite_base::strings::latex::ToLatex) writes, which
    /// that implementation describes; the only difference is that the variable is whichever one is
    /// handed in rather than `x`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the
    /// coefficients.
    ///
    /// # Panics
    /// Panics if `var`'s index is not less than its scheme's [`capacity`](VarScheme::capacity).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(
    ///     p.to_latex_string_with(GreekVars.var(0)),
    ///     r"\alpha^2+3\alpha+2"
    /// );
    /// assert_eq!(p.to_latex_string_with(IndexedVars.var(7)), "x_7^2+3x_7+2");
    /// ```
    ///
    /// The polynomial is `x^2+3*x+2` in each row; only its variable differs.
    ///
    /// | variable | fragment             | renders as           |
    /// |----------|----------------------|----------------------|
    /// | `α`      | `\alpha^2+3\alpha+2` | $\alpha^2+3\alpha+2$ |
    /// | `x₇`     | `x_7^2+3x_7+2`       | $x_7^2+3x_7+2$       |
    fn to_latex_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String {
        let mut s = String::new();
        // Writing to a `String` cannot fail, so the result is the string itself.
        self.write_with_var(var, Language::Latex, &mut s).unwrap();
        s
    }

    /// Converts an [`IntegerPolynomial`] to a Typst math-mode fragment, naming its variable with
    /// any [`VarScheme`].
    ///
    /// The fragment is the one [`ToTypst`](malachite_base::strings::typst::ToTypst) writes, which
    /// that implementation describes; the only difference is that the variable is whichever one is
    /// handed in rather than `x`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the
    /// coefficients.
    ///
    /// # Panics
    /// Panics if `var`'s index is not less than its scheme's [`capacity`](VarScheme::capacity).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.to_typst_string_with(GreekVars.var(0)), "α^2+3α+2");
    /// assert_eq!(p.to_typst_string_with(IndexedVars.var(7)), "x_7^2+3x_7+2");
    /// ```
    ///
    /// The polynomial is `x^2+3*x+2` in each row; only its variable differs.
    ///
    /// | variable | fragment       |
    /// |----------|----------------|
    /// | `α`      | `α^2+3α+2`     |
    /// | `x₇`     | `x_7^2+3x_7+2` |
    fn to_typst_string_with<S: VarScheme + ?Sized>(&self, var: Var<'_, S>) -> String {
        let mut s = String::new();
        // Writing to a `String` cannot fail, so the result is the string itself.
        self.write_with_var(var, Language::Typst, &mut s).unwrap();
        s
    }

    /// Converts a string to an [`IntegerPolynomial`], with its variable named by any [`VarScheme`].
    ///
    /// The syntax is the one [`FromStr`](core::str::FromStr) reads, which that implementation
    /// describes; the only difference is that the variable is whichever one is handed in rather
    /// than `x`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::list::ListVars;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_string_with(GreekVars.var(0), "α^2+3*α+2").unwrap();
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// let vars = ListVars::new(["t"]);
    /// assert_eq!(
    ///     IntegerPolynomial::from_string_with(vars.var(0), "t^2+1")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+1"
    /// );
    ///
    /// // The variable must be the one that was asked for.
    /// assert!(IntegerPolynomial::from_string_with(GreekVars.var(0), "β^2").is_none());
    /// ```
    #[inline]
    fn from_string_with<S: VarScheme + ?Sized>(var: Var<'_, S>, s: &str) -> Option<Self> {
        from_string_with(var, s)
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
