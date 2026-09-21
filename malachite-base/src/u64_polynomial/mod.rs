// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::named::Named;
use crate::num::basic::traits::Zero;
use crate::num::conversion::traits::ExactFrom;
use alloc::vec;
use alloc::vec::Vec;

/// Functions for converting a [`U64Polynomial`] to and from other types.
pub mod conversion;
/// Iterators that generate [`U64Polynomial`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`U64Polynomial`]s randomly.
pub mod random;

/// A polynomial in one variable whose coefficients are [`u64`]s.
///
/// The coefficients are held in ascending order, so that the coefficient of $x^i$ is the one at
/// index $i$, and the last is the leading one. Trailing zero coefficients are not held at all: the
/// zero polynomial has no coefficients, and every other polynomial's last coefficient is nonzero.
/// That is what makes a polynomial's representation unique, and so what lets [`Eq`] be derived.
///
/// The field is private, since not every [`Vec`] of [`u64`]s is one:
/// [`from_coefficients_asc`](U64Polynomial::from_coefficients_asc) is how a [`Vec`] becomes one.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct U64Polynomial {
    coefficients: Vec<u64>,
}

/// The constant 0.
impl Zero for U64Polynomial {
    const ZERO: Self = Self {
        coefficients: Vec::new(),
    };
}

impl U64Polynomial {
    // Returns true iff `self` is valid.
    //
    // To be valid, its last coefficient, if it has one at all, must be nonzero. All
    // `U64Polynomial`s must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.coefficients.last() != Some(&0)
    }

    // Drops the trailing zero coefficients, which is what makes a `Vec` of coefficients the one
    // representation of its polynomial.
    fn trim(&mut self) {
        while self.coefficients.last() == Some(&0) {
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// assert_eq!(U64Polynomial::one().to_string(), "1");
    /// assert_eq!(U64Polynomial::one().degree(), Some(0));
    /// ```
    pub fn one() -> Self {
        Self {
            coefficients: vec![1],
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// assert_eq!(U64Polynomial::two().to_string(), "2");
    /// assert_eq!(U64Polynomial::two().degree(), Some(0));
    /// ```
    pub fn two() -> Self {
        Self {
            coefficients: vec![2],
        }
    }

    /// Returns a reference to a [`U64Polynomial`]'s coefficients, in ascending order.
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// let p = U64Polynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     U64Polynomial::ZERO.coefficients_asc().to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn coefficients_asc(&self) -> &[u64] {
        &self.coefficients
    }

    /// Converts a [`Vec`] of [`u64`]s to a [`U64Polynomial`].
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::u64_polynomial::U64Polynomial;
    /// use u64;
    ///
    /// let p = U64Polynomial::from_coefficients_asc(vec![2, u64::from(3u32), 1]);
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// // The trailing zeros are not part of the polynomial.
    /// let q = U64Polynomial::from_coefficients_asc(vec![2, u64::from(3u32), 1, 0, 0]);
    /// assert_eq!(q.to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     U64Polynomial::from_coefficients_asc(vec![]).to_string(),
    ///     "0"
    /// );
    /// ```
    pub fn from_coefficients_asc(coefficients: Vec<u64>) -> Self {
        let mut p = Self { coefficients };
        p.trim();
        p
    }

    /// Converts a [`U64Polynomial`] to a [`Vec`] of [`u64`]s, in ascending order.
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// let p = U64Polynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.into_coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     U64Polynomial::ZERO
    ///         .into_coefficients_asc()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn into_coefficients_asc(self) -> Vec<u64> {
        self.coefficients
    }

    /// Returns the degree of a [`U64Polynomial`].
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// assert_eq!(U64Polynomial::ZERO.degree(), None);
    /// assert_eq!(U64Polynomial::from_str("5").unwrap().degree(), Some(0));
    /// assert_eq!(U64Polynomial::from_str("x").unwrap().degree(), Some(1));
    /// assert_eq!(
    ///     U64Polynomial::from_str("x^2+3*x+2").unwrap().degree(),
    ///     Some(2)
    /// );
    /// ```
    #[inline]
    pub fn degree(&self) -> Option<u64> {
        self.coefficients.len().checked_sub(1).map(u64::exact_from)
    }

    /// Returns one of a [`U64Polynomial`]'s coefficients.
    ///
    /// The index is the power of the variable the coefficient belongs to, so that index 0 gives the
    /// constant term. An index past the degree gives zero, which is the coefficient a polynomial
    /// has there. A [`u64`] is [`Copy`], so this hands back a value rather than a reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// let p = U64Polynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.coefficient(0), 2);
    /// assert_eq!(p.coefficient(1), 3);
    /// assert_eq!(p.coefficient(2), 1);
    /// assert_eq!(p.coefficient(100), 0);
    /// ```
    #[inline]
    pub fn coefficient(&self, index: u64) -> u64 {
        usize::try_from(index)
            .ok()
            .and_then(|i| self.coefficients.get(i))
            .copied()
            .unwrap_or(0)
    }

    /// Returns a [`U64Polynomial`]'s leading coefficient.
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// let p = U64Polynomial::from_str("7*x^2+3*x+2").unwrap();
    /// assert_eq!(p.leading_coefficient(), 7);
    /// assert_eq!(U64Polynomial::ZERO.leading_coefficient(), 0);
    /// ```
    #[inline]
    pub fn leading_coefficient(&self) -> u64 {
        self.coefficients.last().copied().unwrap_or(0)
    }

    /// Mutates one of a [`U64Polynomial`]'s coefficients using a provided closure, and then returns
    /// whatever the closure returns.
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    /// use u64;
    ///
    /// let mut p = U64Polynomial::from_str("x^2+3*x+2").unwrap();
    ///
    /// let ret = p.mutate_coefficient(1, |c| {
    ///     *c += 1;
    ///     true
    /// });
    /// assert_eq!(p.to_string(), "x^2+4*x+2");
    /// assert_eq!(ret, true);
    ///
    /// // The polynomial grows to reach a coefficient it did not have.
    /// p.mutate_coefficient(5, |c| *c += 1);
    /// assert_eq!(p.to_string(), "x^5+x^2+4*x+2");
    ///
    /// // Clearing the leading coefficient lowers the degree.
    /// p.mutate_coefficient(5, |c| *c = 0);
    /// assert_eq!(p.to_string(), "x^2+4*x+2");
    /// ```
    pub fn mutate_coefficient<F: FnOnce(&mut u64) -> T, T>(&mut self, index: u64, f: F) -> T {
        let index = usize::exact_from(index);
        if index >= self.coefficients.len() {
            self.coefficients.resize(index + 1, 0);
        }
        let out = f(&mut self.coefficients[index]);
        self.trim();
        out
    }
}

impl_named!(U64Polynomial);
