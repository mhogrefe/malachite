// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::named::Named;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::conversion::string::from_string::from_string_with;
use crate::unsigned_polynomial::conversion::string::to_string::Language;
use crate::vars::{Var, VarScheme};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// Traits for arithmetic on [`UnsignedPolynomial`]s.
pub mod arithmetic;
/// Implementations of [`Ord`] and [`PartialOrd`] for [`UnsignedPolynomial`], comparing two
/// polynomials by their behavior for large arguments.
pub mod comparison;
/// Functions for converting a [`UnsignedPolynomial`] to and from other types.
pub mod conversion;
/// Iterators that generate [`UnsignedPolynomial`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`UnsignedPolynomial`]s randomly.
pub mod random;

/// A polynomial in one variable whose coefficients are unsigned primitive integers.
///
/// The coefficients are held in ascending order, so that the coefficient of $x^i$ is the one at
/// index $i$, and the last is the leading one. Trailing zero coefficients are not held at all: the
/// zero polynomial has no coefficients, and every other polynomial's last coefficient is nonzero.
/// That is what makes a polynomial's representation unique, and so what lets [`Eq`] be derived.
///
/// The field is private, since not every [`Vec`] of `T`s is one:
/// [`from_coefficients_asc`](UnsignedPolynomial::from_coefficients_asc) is how a [`Vec`] becomes
/// one.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        try_from = "SerdeUnsignedPolynomial<T>",
        into = "SerdeUnsignedPolynomial<T>"
    )
)]
pub struct UnsignedPolynomial<T: PrimitiveUnsigned> {
    coefficients: Vec<T>,
}

// A `UnsignedPolynomial` is its coefficients, so this is what is serialized: the list of them, in
// the order they are held in. The wrapper is transparent, so the encoding is the list itself and
// nothing around it.
//
// Deserializing goes through `TryFrom`, which rejects a list whose last coefficient is zero: such a
// list is not a `UnsignedPolynomial`'s coefficients, and accepting it would build one that two
// equal polynomials could disagree with.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct SerdeUnsignedPolynomial<T: PrimitiveUnsigned>(pub(crate) Vec<T>);

/// The constant 0.
impl<T: PrimitiveUnsigned> Zero for UnsignedPolynomial<T> {
    const ZERO: Self = Self {
        coefficients: Vec::new(),
    };
}

impl<T: PrimitiveUnsigned> UnsignedPolynomial<T> {
    // Returns true iff `self` is valid.
    //
    // To be valid, its last coefficient, if it has one at all, must be nonzero. All
    // `UnsignedPolynomial`s must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.coefficients.last() != Some(&T::ZERO)
    }

    // Drops the trailing zero coefficients, which is what makes a `Vec` of coefficients the one
    // representation of its polynomial.
    fn trim(&mut self) {
        while self.coefficients.last() == Some(&T::ZERO) {
            self.coefficients.pop();
        }
    }

    /// Returns a reference to a [`UnsignedPolynomial`]'s coefficients, in ascending order.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::ZERO
    ///         .coefficients_asc()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn coefficients_asc(&self) -> &[T] {
        &self.coefficients
    }
}

macro_rules! impl_named_unsigned_polynomial {
    ($t:ident, $name:expr) => {
        impl Named for UnsignedPolynomial<$t> {
            /// The name of this type, with its coefficient type spelled out.
            const NAME: &'static str = $name;
        }
    };
}

impl<T: PrimitiveUnsigned> Polynomial for UnsignedPolynomial<T> {
    type Coefficient = T;
    type CoefficientOutput<'a>
        = T
    where
        Self: 'a;

    /// The constant polynomial 1.
    ///
    /// This is a function rather than an associated constant, and
    /// [`One`](crate::num::basic::traits::One) is not implemented, because a polynomial holds its
    /// coefficients in a [`Vec`] and a [`Vec`] with anything in it cannot be built at compile time.
    /// The zero polynomial has no coefficients, so [`ZERO`](crate::num::basic::traits::Zero::ZERO)
    /// is a constant after all.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::one().to_string(), "1");
    /// assert_eq!(UnsignedPolynomial::<u64>::one().degree(), Some(0));
    /// ```
    fn one() -> Self {
        Self {
            coefficients: vec![T::ONE],
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::two().to_string(), "2");
    /// assert_eq!(UnsignedPolynomial::<u64>::two().degree(), Some(0));
    /// ```
    fn two() -> Self {
        Self {
            coefficients: vec![T::TWO],
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::x().to_string(), "x");
    /// assert_eq!(UnsignedPolynomial::<u64>::x().degree(), Some(1));
    /// ```
    fn x() -> Self {
        Self {
            coefficients: vec![T::ZERO, T::ONE],
        }
    }

    /// Converts a [`Vec`] of [`u64`]s to a [`UnsignedPolynomial`].
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
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use u64;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_coefficients_asc(vec![2, u64::from(3u32), 1]);
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// // The trailing zeros are not part of the polynomial.
    /// let q = UnsignedPolynomial::<u64>::from_coefficients_asc(vec![2, u64::from(3u32), 1, 0, 0]);
    /// assert_eq!(q.to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_coefficients_asc(vec![]).to_string(),
    ///     "0"
    /// );
    /// ```
    fn from_coefficients_asc(coefficients: Vec<T>) -> Self {
        let mut p = Self { coefficients };
        p.trim();
        p
    }

    /// Converts a [`UnsignedPolynomial`] to a [`Vec`] of [`u64`]s, in ascending order.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.into_coefficients_asc().to_debug_string(), "[2, 3, 1]");
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::ZERO
    ///         .into_coefficients_asc()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    fn into_coefficients_asc(self) -> Vec<T> {
        self.coefficients
    }

    /// Returns the degree of a [`UnsignedPolynomial`].
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::ZERO.degree(), None);
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("5").unwrap().degree(),
    ///     Some(0)
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x").unwrap().degree(),
    ///     Some(1)
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .degree(),
    ///     Some(2)
    /// );
    /// ```
    #[inline]
    fn degree(&self) -> Option<u64> {
        self.coefficients.len().checked_sub(1).map(u64::exact_from)
    }

    /// Returns the length of a [`UnsignedPolynomial`]: the number of coefficients it holds.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::ZERO.len(), 0);
    /// assert_eq!(UnsignedPolynomial::<u64>::from_str("5").unwrap().len(), 1);
    /// assert_eq!(UnsignedPolynomial::<u64>::from_str("x").unwrap().len(), 2);
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .len(),
    ///     3
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_length` from `nmod_poly.h`, FLINT 3.6.0.
    #[inline]
    fn len(&self) -> u64 {
        u64::exact_from(self.coefficients.len())
    }

    /// Returns one of a [`UnsignedPolynomial`]'s coefficients.
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
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(p.coefficient(0), 2);
    /// assert_eq!(p.coefficient(1), 3);
    /// assert_eq!(p.coefficient(2), 1);
    /// assert_eq!(p.coefficient(100), 0);
    /// ```
    #[inline]
    fn coefficient(&self, index: u64) -> T {
        usize::try_from(index)
            .ok()
            .and_then(|i| self.coefficients.get(i))
            .copied()
            .unwrap_or(T::ZERO)
    }

    /// Returns a [`UnsignedPolynomial`]'s leading coefficient.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("7*x^2+3*x+2").unwrap();
    /// assert_eq!(p.leading_coefficient(), 7);
    /// assert_eq!(UnsignedPolynomial::<u64>::ZERO.leading_coefficient(), 0);
    /// ```
    #[inline]
    fn leading_coefficient(&self) -> T {
        self.coefficients.last().copied().unwrap_or(T::ZERO)
    }

    /// Determines whether an [`UnsignedPolynomial`] is monic: nonzero, with leading coefficient 1.
    ///
    /// The zero polynomial is not monic.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+3*x+2")
    ///         .unwrap()
    ///         .is_monic()
    /// );
    /// assert!(
    ///     !UnsignedPolynomial::<u8>::from_str("2*x^2+3")
    ///         .unwrap()
    ///         .is_monic()
    /// );
    /// assert!(!UnsignedPolynomial::<u8>::ZERO.is_monic());
    /// ```
    #[inline]
    fn is_monic(&self) -> bool {
        self.coefficients.last() == Some(&T::ONE)
    }

    /// Mutates one of a [`UnsignedPolynomial`]'s coefficients using a provided closure, and then
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
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use u64;
    ///
    /// let mut p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
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
    fn mutate_coefficient<F: FnOnce(&mut T) -> U, U>(&mut self, index: u64, f: F) -> U {
        let index = usize::exact_from(index);
        if index >= self.coefficients.len() {
            self.coefficients.resize(index + 1, T::ZERO);
        }
        let out = f(&mut self.coefficients[index]);
        self.trim();
        out
    }

    /// Sets the coefficients of a [`UnsignedPolynomial`] of $x^i$ for $i$ in `start..end` to zero.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p;
    /// p = UnsignedPolynomial::<u64>::from_str("x^4+x^3+x^2+x+1").unwrap();
    /// p.zero_coefficients(1, 3);
    /// assert_eq!(p.to_string(), "x^4+x^3+1");
    /// p = UnsignedPolynomial::<u64>::from_str("x^4+x^3+x^2+x+1").unwrap();
    /// p.zero_coefficients(2, 10);
    /// assert_eq!(p.to_string(), "x+1");
    /// p = UnsignedPolynomial::<u64>::from_str("x^4+x^3+x^2+x+1").unwrap();
    /// p.zero_coefficients(5, 10);
    /// assert_eq!(p.to_string(), "x^4+x^3+x^2+x+1");
    /// ```
    ///
    /// FLINT has no counterpart for `nmod_poly`; this is the counterpart of `fmpz_poly_zero_coeffs`
    /// from `fmpz_poly/zero_coeffs.c`, FLINT 3.6.0.
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
            self.coefficients[start..end].fill(T::ZERO);
        }
    }

    /// Truncates a [`UnsignedPolynomial`] to its first `len` coefficients, taking the polynomial by
    /// reference and returning the result.
    ///
    /// The result is the polynomial reduced modulo $x^{\mathrm{len}}$: every term of degree `len`
    /// or more is dropped, and then any zeros left at the top go too, so the result may have fewer
    /// than `len` coefficients. A polynomial with at most `len` coefficients is returned unchanged.
    ///
    /// $$
    /// f(p, n) = p \bmod x^n.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(len, self.len())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^3+2*x^2+3*x+4")
    ///         .unwrap()
    ///         .truncate(2)
    ///         .to_string(),
    ///     "3*x+4"
    /// );
    /// // A polynomial with no more than len coefficients is unchanged.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^3+2*x^2+3*x+4")
    ///         .unwrap()
    ///         .truncate(10)
    ///         .to_string(),
    ///     "x^3+2*x^2+3*x+4"
    /// );
    /// // Truncating can uncover zeros, which are dropped too.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^3+3*x+4")
    ///         .unwrap()
    ///         .truncate(3)
    ///         .to_string(),
    ///     "3*x+4"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^3+2*x^2+3*x+4")
    ///         .unwrap()
    ///         .truncate(0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_set_trunc` from `nmod_poly/set_trunc.c`, FLINT 3.6.0.
    fn truncate(&self, len: u64) -> Self {
        let kept = usize::try_from(len).map_or(self.coefficients.len(), |len| {
            len.min(self.coefficients.len())
        });
        // Skip the zeros that truncating leaves at the top rather than copying them and trimming.
        let kept = self.coefficients[..kept]
            .iter()
            .rposition(|c| *c != T::ZERO)
            .map_or(0, |i| i + 1);
        Self {
            coefficients: self.coefficients[..kept].to_vec(),
        }
    }

    /// Truncates a [`UnsignedPolynomial`] to its first `len` coefficients, in place.
    ///
    /// See [`truncate`](Self::truncate) for what the result is.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p;
    /// p = UnsignedPolynomial::<u64>::from_str("x^3+2*x^2+3*x+4").unwrap();
    /// p.truncate_assign(2);
    /// assert_eq!(p.to_string(), "3*x+4");
    /// // A polynomial with no more than len coefficients is unchanged.
    /// p = UnsignedPolynomial::<u64>::from_str("x^3+2*x^2+3*x+4").unwrap();
    /// p.truncate_assign(10);
    /// assert_eq!(p.to_string(), "x^3+2*x^2+3*x+4");
    /// // Truncating can uncover zeros, which are dropped too.
    /// p = UnsignedPolynomial::<u64>::from_str("x^3+3*x+4").unwrap();
    /// p.truncate_assign(3);
    /// assert_eq!(p.to_string(), "3*x+4");
    /// p = UnsignedPolynomial::<u64>::from_str("x^3+2*x^2+3*x+4").unwrap();
    /// p.truncate_assign(0);
    /// assert_eq!(p.to_string(), "0");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_truncate` from `nmod_poly.h`, FLINT 3.6.0.
    fn truncate_assign(&mut self, len: u64) {
        if let Ok(len) = usize::try_from(len)
            && len < self.coefficients.len()
        {
            self.coefficients.truncate(len);
            self.trim();
        }
    }

    /// Reverses the coefficients of a [`UnsignedPolynomial`], considered as having length `len`,
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `len` exceeds `self.len()` and does not fit in a [`usize`], which cannot happen on
    /// a target with 64-bit pointers.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Polynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+2*x+3")
    ///         .unwrap()
    ///         .reverse(3)
    ///         .to_string(),
    ///     "3*x^2+2*x+1"
    /// );
    /// // Padding to length 5 adds low zeros.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+2*x+3")
    ///         .unwrap()
    ///         .reverse(5)
    ///         .to_string(),
    ///     "3*x^4+2*x^3+x^2"
    /// );
    /// // Truncating to length 2 drops x^2 first.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+2*x+3")
    ///         .unwrap()
    ///         .reverse(2)
    ///         .to_string(),
    ///     "3*x+2"
    /// );
    /// // A zero constant term becomes a trailing zero, and is dropped.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_str("x^2+2*x")
    ///         .unwrap()
    ///         .reverse(3)
    ///         .to_string(),
    ///     "2*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_reverse` from `nmod_poly/reverse.c`, FLINT 3.6.0.
    fn reverse(&self, len: u64) -> Self {
        let kept = usize::try_from(len).map_or(self.coefficients.len(), |len| {
            len.min(self.coefficients.len())
        });
        if kept == 0 {
            return Self::ZERO;
        }
        // The coefficients past the kept ones become the result's low zeros.
        let mut coefficients = vec![T::ZERO; usize::exact_from(len) - kept];
        coefficients.extend(self.coefficients[..kept].iter().rev().copied());
        Self::from_coefficients_asc(coefficients)
    }

    /// Reverses the coefficients of a [`UnsignedPolynomial`], considered as having length `len`, in
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p;
    /// p = UnsignedPolynomial::<u64>::from_str("x^2+2*x+3").unwrap();
    /// p.reverse_assign(3);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+1");
    /// // Padding to length 5 adds low zeros.
    /// p = UnsignedPolynomial::<u64>::from_str("x^2+2*x+3").unwrap();
    /// p.reverse_assign(5);
    /// assert_eq!(p.to_string(), "3*x^4+2*x^3+x^2");
    /// // Truncating to length 2 drops x^2 first.
    /// p = UnsignedPolynomial::<u64>::from_str("x^2+2*x+3").unwrap();
    /// p.reverse_assign(2);
    /// assert_eq!(p.to_string(), "3*x+2");
    /// // A zero constant term becomes a trailing zero, and is dropped.
    /// p = UnsignedPolynomial::<u64>::from_str("x^2+2*x").unwrap();
    /// p.reverse_assign(3);
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_reverse` from `nmod_poly/reverse.c`, FLINT 3.6.0.
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
        self.coefficients.resize(len, T::ZERO);
        self.coefficients.rotate_right(len - kept);
        // The polynomial's low zeros, if any, are now at the top.
        self.trim();
    }

    /// Converts a [`UnsignedPolynomial`] to a [`String`], naming its variable with any
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    /// use malachite_base::vars::list::ListVars;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
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

    /// Converts a [`UnsignedPolynomial`] to a LaTeX math-mode fragment, naming its variable with
    /// any [`VarScheme`].
    ///
    /// The fragment is the one [`ToLatex`](crate::strings::latex::ToLatex) writes, which that
    /// implementation describes; the only difference is that the variable is whichever one is
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
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

    /// Converts a [`UnsignedPolynomial`] to a Typst math-mode fragment, naming its variable with
    /// any [`VarScheme`].
    ///
    /// The fragment is the one [`ToTypst`](crate::strings::typst::ToTypst) writes, which that
    /// implementation describes; the only difference is that the variable is whichever one is
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::indexed::IndexedVars;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
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

    /// Converts a string to a [`UnsignedPolynomial`], with its variable named by any [`VarScheme`].
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use malachite_base::vars::VarScheme;
    /// use malachite_base::vars::greek::GreekVars;
    /// use malachite_base::vars::list::ListVars;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_string_with(GreekVars.var(0), "α^2+3*α+2").unwrap();
    /// assert_eq!(p.to_string(), "x^2+3*x+2");
    ///
    /// let vars = ListVars::new(["t"]);
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from_string_with(vars.var(0), "t^2+1")
    ///         .unwrap()
    ///         .to_string(),
    ///     "x^2+1"
    /// );
    ///
    /// // The variable must be the one that was asked for.
    /// assert!(UnsignedPolynomial::<u64>::from_string_with(GreekVars.var(0), "β^2").is_none());
    /// ```
    #[inline]
    fn from_string_with<S: VarScheme + ?Sized>(var: Var<'_, S>, s: &str) -> Option<Self> {
        from_string_with(var, s)
    }
}

impl_named_unsigned_polynomial!(u8, "UnsignedPolynomial<u8>");
impl_named_unsigned_polynomial!(u16, "UnsignedPolynomial<u16>");
impl_named_unsigned_polynomial!(u32, "UnsignedPolynomial<u32>");
impl_named_unsigned_polynomial!(u64, "UnsignedPolynomial<u64>");
impl_named_unsigned_polynomial!(u128, "UnsignedPolynomial<u128>");
impl_named_unsigned_polynomial!(usize, "UnsignedPolynomial<usize>");
