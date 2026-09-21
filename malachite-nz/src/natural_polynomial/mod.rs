// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::named::Named;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

/// Traits for arithmetic on [`NaturalPolynomial`]s.
pub mod arithmetic;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`NaturalPolynomial`], comparing two
/// polynomials by their behavior for large arguments.
pub mod comparison;
/// Functions for converting a [`NaturalPolynomial`] to and from other types.
pub mod conversion;
/// Iterators that generate [`NaturalPolynomial`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`NaturalPolynomial`]s randomly.
pub mod random;

// The zero `Natural`, as something a reference can be handed out to.
//
// A `Natural` owns a `Vec` when it is large, so it has a destructor, and a `&Natural::ZERO` written
// where a reference is returned would point at a temporary that does not outlive the call. A
// `static` is the same zero with a lifetime long enough to hand out.
pub(crate) static ZERO: Natural = Natural::ZERO;

/// A polynomial in one variable whose coefficients are [`Natural`]s.
///
/// The coefficients are held in ascending order, so that the coefficient of $x^i$ is the one at
/// index $i$, and the last is the leading one. Trailing zero coefficients are not held at all: the
/// zero polynomial has no coefficients, and every other polynomial's last coefficient is nonzero.
/// That is what makes a polynomial's representation unique, and so what lets [`Eq`] be derived.
///
/// The field is private, since not every [`Vec`] of [`Natural`]s is one:
/// [`from_coefficients_asc`](NaturalPolynomial::from_coefficients_asc) is how a [`Vec`] becomes
/// one.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeNaturalPolynomial", into = "SerdeNaturalPolynomial")
)]
pub struct NaturalPolynomial {
    coefficients: Vec<Natural>,
}

// A `NaturalPolynomial` is its coefficients, so this is what is serialized: the list of them, in
// the order they are held in, each one serialized the way a `Natural` is. The wrapper is
// transparent, so the encoding is the list itself and nothing around it.
//
// Deserializing goes through `TryFrom`, which rejects a list whose last coefficient is zero: such a
// list is not a `NaturalPolynomial`'s coefficients, and accepting it would build one that two equal
// polynomials could disagree with.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct SerdeNaturalPolynomial(pub(crate) Vec<Natural>);

/// The constant 0.
impl Zero for NaturalPolynomial {
    const ZERO: Self = Self {
        coefficients: Vec::new(),
    };
}

impl NaturalPolynomial {
    // Returns true iff `self` is valid.
    //
    // To be valid, its last coefficient, if it has one at all, must be nonzero. All
    // `NaturalPolynomial`s must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.coefficients.last() != Some(&Natural::ZERO)
    }

    // Drops the trailing zero coefficients, which is what makes a `Vec` of coefficients the one
    // representation of its polynomial.
    fn trim(&mut self) {
        while self.coefficients.last() == Some(&Natural::ZERO) {
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(NaturalPolynomial::one().to_string(), "1");
    /// assert_eq!(NaturalPolynomial::one().degree(), Some(0));
    /// ```
    pub fn one() -> Self {
        Self {
            coefficients: vec![Natural::ONE],
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(NaturalPolynomial::two().to_string(), "2");
    /// assert_eq!(NaturalPolynomial::two().degree(), Some(0));
    /// ```
    pub fn two() -> Self {
        Self {
            coefficients: vec![Natural::TWO],
        }
    }

    /// Returns a reference to a [`NaturalPolynomial`]'s coefficients, in ascending order.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO.coefficients_asc().to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn coefficients_asc(&self) -> &[Natural] {
        &self.coefficients
    }

    /// Converts a [`Vec`] of [`Natural`]s to a [`NaturalPolynomial`].
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
    /// use malachite_base::num::basic::traits::{One, Two, Zero};
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_coefficients_asc(vec![
    ///     Natural::TWO,
    ///     Natural::from(3u32),
    ///     Natural::ONE,
    /// ]);
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// // The trailing zeros are not part of the polynomial.
    /// let q = NaturalPolynomial::from_coefficients_asc(vec![
    ///     Natural::TWO,
    ///     Natural::from(3u32),
    ///     Natural::ONE,
    ///     Natural::ZERO,
    ///     Natural::ZERO,
    /// ]);
    /// assert_eq!(q.to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_coefficients_asc(vec![]).to_string(),
    ///     "0"
    /// );
    /// ```
    pub fn from_coefficients_asc(coefficients: Vec<Natural>) -> Self {
        let mut p = Self { coefficients };
        p.trim();
        p
    }

    /// Converts a [`NaturalPolynomial`] to a [`Vec`] of [`Natural`]s, in ascending order.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.into_coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     NaturalPolynomial::ZERO
    ///         .into_coefficients_asc()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn into_coefficients_asc(self) -> Vec<Natural> {
        self.coefficients
    }

    /// Returns the degree of a [`NaturalPolynomial`].
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(NaturalPolynomial::ZERO.degree(), None);
    /// assert_eq!(NaturalPolynomial::from_str("5").unwrap().degree(), Some(0));
    /// assert_eq!(NaturalPolynomial::from_str("x").unwrap().degree(), Some(1));
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+3*x+2").unwrap().degree(),
    ///     Some(2)
    /// );
    /// ```
    #[inline]
    pub fn degree(&self) -> Option<u64> {
        self.coefficients.len().checked_sub(1).map(u64::exact_from)
    }

    /// Returns a reference to one of a [`NaturalPolynomial`]'s coefficients.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(*p.coefficient(0), 2);
    /// assert_eq!(*p.coefficient(1), 3);
    /// assert_eq!(*p.coefficient(2), 1);
    /// assert_eq!(*p.coefficient(100), 0);
    /// ```
    #[inline]
    pub fn coefficient(&self, index: u64) -> &Natural {
        usize::try_from(index)
            .ok()
            .and_then(|i| self.coefficients.get(i))
            .unwrap_or(&ZERO)
    }

    /// Returns a reference to a [`NaturalPolynomial`]'s leading coefficient.
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
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("7*x^2+3*x+2").unwrap();
    /// assert_eq!(*p.leading_coefficient(), 7);
    /// assert_eq!(*NaturalPolynomial::ZERO.leading_coefficient(), 0);
    /// ```
    #[inline]
    pub fn leading_coefficient(&self) -> &Natural {
        self.coefficients.last().unwrap_or(&ZERO)
    }

    /// Mutates one of a [`NaturalPolynomial`]'s coefficients using a provided closure, and then
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    ///
    /// let ret = p.mutate_coefficient(1, |c| {
    ///     *c += Natural::ONE;
    ///     true
    /// });
    /// assert_eq!(p.to_string(), "x^2+4*x+2");
    /// assert_eq!(ret, true);
    ///
    /// // The polynomial grows to reach a coefficient it did not have.
    /// p.mutate_coefficient(5, |c| *c += Natural::ONE);
    /// assert_eq!(p.to_string(), "x^5+x^2+4*x+2");
    ///
    /// // Clearing the leading coefficient lowers the degree.
    /// p.mutate_coefficient(5, |c| *c = Natural::ZERO);
    /// assert_eq!(p.to_string(), "x^2+4*x+2");
    /// ```
    pub fn mutate_coefficient<F: FnOnce(&mut Natural) -> T, T>(&mut self, index: u64, f: F) -> T {
        let index = usize::exact_from(index);
        if index >= self.coefficients.len() {
            self.coefficients.resize(index + 1, Natural::ZERO);
        }
        let out = f(&mut self.coefficients[index]);
        self.trim();
        out
    }
}

impl_named!(NaturalPolynomial);
