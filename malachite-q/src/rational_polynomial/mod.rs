// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec::Vec;
use malachite_base::named::Named;
#[cfg(feature = "test_build")]
use malachite_base::num::arithmetic::traits::CoprimeWith;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd, GcdAssign, LcmAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

/// Iterators that generate [`RationalPolynomial`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`RationalPolynomial`]s randomly.
pub mod random;

/// Functions for converting a [`RationalPolynomial`] to and from other types.
pub mod conversion;

/// A polynomial in one variable whose coefficients are [`Rational`]s.
///
/// The coefficients are not stored one by one. A polynomial over the rationals can always be
/// written as an integer polynomial over a single denominator, and that is what is kept: a
/// numerator [`IntegerPolynomial`] and one positive [`Natural`] denominator, reduced so that no
/// factor is shared by the denominator and every numerator coefficient. This is how
/// [FLINT](https://flintlib.org/) stores an `fmpq_poly_t`, and it is what makes the arithmetic
/// worth doing: one denominator to carry rather than one per coefficient.
///
/// The consequence for this type's interface is that a coefficient does not exist anywhere to point
/// at — $c_i$ is $n_i/d$, and the division has not been done. So where
/// [`NaturalPolynomial`](malachite_nz::natural_polynomial::NaturalPolynomial) hands out a
/// reference, this hands out a value: [`coefficient`](Self::coefficient) returns a [`Rational`],
/// and [`to_coefficients_asc`](Self::to_coefficients_asc) builds a [`Vec`] rather than lending a
/// slice.
///
/// Trailing zero coefficients are not held at all, so the zero polynomial has no coefficients and
/// every other polynomial's leading coefficient is nonzero. The zero polynomial's denominator is 1.
/// Together these make a polynomial's representation unique, which is what lets [`Eq`] be derived.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeRationalPolynomial", into = "SerdeRationalPolynomial")
)]
pub struct RationalPolynomial {
    numerator: IntegerPolynomial,
    denominator: Natural,
}

// A `RationalPolynomial` is a numerator and a denominator, so both are serialized, under short
// names since an encoding is not read by people. Serializing the coefficients instead would make
// the encoding uniform with the other polynomial types, but it would also throw away the shared
// denominator and make deserializing clear the coefficients' denominators all over again.
//
// Deserializing goes through `TryFrom`, which checks the three things that make the pair canonical.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct SerdeRationalPolynomial {
    #[cfg_attr(feature = "serde", serde(rename = "n"))]
    pub(crate) numerator: IntegerPolynomial,
    #[cfg_attr(feature = "serde", serde(rename = "d"))]
    pub(crate) denominator: Natural,
}

/// The constant 0.
impl Zero for RationalPolynomial {
    const ZERO: Self = Self {
        numerator: IntegerPolynomial::ZERO,
        denominator: Natural::ONE,
    };
}

impl Default for RationalPolynomial {
    /// The zero polynomial.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl RationalPolynomial {
    // Returns true iff `self` is valid.
    //
    // To be valid, its numerator must be valid, its denominator must be positive, and no factor may
    // be shared by the denominator and every numerator coefficient. The zero polynomial's
    // denominator must be 1. All `RationalPolynomial`s must be valid.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        if !self.numerator.is_valid() || self.denominator == 0u32 {
            return false;
        }
        if self.numerator == IntegerPolynomial::ZERO {
            return self.denominator == 1u32;
        }
        content(&self.numerator).coprime_with(&self.denominator)
    }

    // Reduces a numerator and denominator to the one pair that stands for their polynomial.
    //
    // The denominator is positive to begin with, since it is built from denominators of
    // `Rational`s; all that is left is to divide out what the numerator's coefficients share with
    // it, and to give the zero polynomial the denominator 1.
    fn canonicalize(numerator: IntegerPolynomial, denominator: Natural) -> Self {
        if numerator == IntegerPolynomial::ZERO {
            return Self::ZERO;
        }
        let gcd = content(&numerator).gcd(&denominator);
        if gcd == 1u32 {
            return Self {
                numerator,
                denominator,
            };
        }
        let gcd = Integer::from(gcd);
        Self {
            numerator: IntegerPolynomial::from_coefficients_asc(
                numerator
                    .into_coefficients_asc()
                    .into_iter()
                    .map(|c| c.div_exact(&gcd))
                    .collect(),
            ),
            denominator: denominator.div_exact(Natural::exact_from(gcd)),
        }
    }

    /// The constant polynomial 1.
    ///
    /// This is a function rather than an associated constant, because a nonzero polynomial holds
    /// its coefficients in a [`Vec`] and a [`Vec`] with anything in it cannot be built at compile
    /// time. The zero polynomial has no coefficients, so
    /// [`ZERO`](malachite_base::num::basic::traits::Zero::ZERO) is a constant after all.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::one().to_string(), "1");
    /// assert_eq!(RationalPolynomial::one().degree(), Some(0));
    /// ```
    pub fn one() -> Self {
        Self {
            numerator: IntegerPolynomial::one(),
            denominator: Natural::ONE,
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
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::two().to_string(), "2");
    /// ```
    pub fn two() -> Self {
        Self {
            numerator: IntegerPolynomial::two(),
            denominator: Natural::ONE,
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
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::negative_one().to_string(), "-1");
    /// ```
    pub fn negative_one() -> Self {
        Self {
            numerator: IntegerPolynomial::negative_one(),
            denominator: Natural::ONE,
        }
    }

    /// A reference to the numerator of a [`RationalPolynomial`].
    ///
    /// The polynomial is this [`IntegerPolynomial`] divided by
    /// [`denominator_ref`](Self::denominator_ref).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// assert_eq!(p.numerator_ref().to_string(), "3*x+2");
    /// assert_eq!(*p.denominator_ref(), 6);
    /// ```
    #[inline]
    pub const fn numerator_ref(&self) -> &IntegerPolynomial {
        &self.numerator
    }

    /// A reference to the denominator of a [`RationalPolynomial`], which is positive.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](Self::numerator_ref).
    #[inline]
    pub const fn denominator_ref(&self) -> &Natural {
        &self.denominator
    }

    /// Converts a [`RationalPolynomial`] to its numerator and denominator.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (n, d) = RationalPolynomial::from_str("1/2*x+1/3")
    ///     .unwrap()
    ///     .into_numerator_and_denominator();
    /// assert_eq!(n.to_string(), "3*x+2");
    /// assert_eq!(d, 6);
    /// ```
    #[inline]
    pub fn into_numerator_and_denominator(self) -> (IntegerPolynomial, Natural) {
        (self.numerator, self.denominator)
    }

    /// Converts an [`IntegerPolynomial`] and a positive [`Natural`] to a [`RationalPolynomial`].
    ///
    /// The result is the polynomial divided by the denominator, reduced to the one numerator and
    /// denominator that stand for it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerator's coefficients and the denominator.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let n = IntegerPolynomial::from_str("3*x+2").unwrap();
    /// let p = RationalPolynomial::from_numerator_and_denominator(n, Natural::from(6u32));
    /// assert_eq!(p.to_string(), "1/2*x+1/3");
    ///
    /// // The pair is reduced, so a common factor makes no difference.
    /// let n = IntegerPolynomial::from_str("6*x+4").unwrap();
    /// let q = RationalPolynomial::from_numerator_and_denominator(n, Natural::from(12u32));
    /// assert_eq!(p, q);
    /// ```
    #[inline]
    pub fn from_numerator_and_denominator(
        numerator: IntegerPolynomial,
        denominator: Natural,
    ) -> Self {
        assert_ne!(denominator, 0u32, "the denominator may not be zero");
        Self::canonicalize(numerator, denominator)
    }

    /// Converts a [`Vec`] of [`Rational`]s to a [`RationalPolynomial`].
    ///
    /// The coefficients are in ascending order, so that the first is the constant term. Trailing
    /// zeros are dropped, since a polynomial does not hold them; the [`Vec`] may therefore end with
    /// as many as it likes, and the empty [`Vec`] is the zero polynomial.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `coefficients.len()`, and $m$ is the
    /// largest number of bits of any coefficient.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_coefficients_asc(vec![
    ///     Rational::from_signeds(1, 3),
    ///     Rational::ONE_HALF,
    /// ]);
    /// assert_eq!(p.to_string(), "1/2*x+1/3");
    /// assert_eq!(p.numerator_ref().to_string(), "3*x+2");
    /// assert_eq!(*p.denominator_ref(), 6);
    /// ```
    pub fn from_coefficients_asc(coefficients: Vec<Rational>) -> Self {
        // One denominator has to serve every coefficient, and the smallest that does is their least
        // common multiple.
        let mut denominator = Natural::ONE;
        for c in &coefficients {
            denominator.lcm_assign(c.denominator_ref());
        }
        let numerator = IntegerPolynomial::from_coefficients_asc(
            coefficients
                .into_iter()
                .map(|c| {
                    let negative = c < 0u32;
                    let (n, d) = c.into_numerator_and_denominator();
                    let n = Integer::from(n) * Integer::from((&denominator).div_exact(d));
                    if negative { -n } else { n }
                })
                .collect(),
        );
        Self::canonicalize(numerator, denominator)
    }

    /// Converts a [`RationalPolynomial`] to a [`Vec`] of [`Rational`]s, in ascending order.
    ///
    /// The first is the constant term and the last is the leading coefficient, so the [`Vec`] is
    /// what [`from_coefficients_asc`](Self::from_coefficients_asc) would take back. It holds no
    /// trailing zeros, and for the zero polynomial it is empty.
    ///
    /// A coefficient is not stored anywhere, so this builds one [`Rational`] per coefficient rather
    /// than lending a slice.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of coefficients, and $m$ is
    /// the largest number of bits of any of them.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// assert_eq!(p.to_coefficients_asc().to_debug_string(), "[1/3, 1/2]");
    /// assert_eq!(p.into_coefficients_asc().to_debug_string(), "[1/3, 1/2]");
    /// ```
    pub fn to_coefficients_asc(&self) -> Vec<Rational> {
        let denominator = Integer::from(self.denominator.clone());
        self.numerator
            .coefficients_asc()
            .iter()
            .map(|n| Rational::from_integers_ref(n, &denominator))
            .collect()
    }

    /// Converts a [`RationalPolynomial`] to a [`Vec`] of [`Rational`]s, in ascending order, taking
    /// it by value.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`to_coefficients_asc`](Self::to_coefficients_asc).
    ///
    /// # Examples
    /// See [here](Self::to_coefficients_asc).
    pub fn into_coefficients_asc(self) -> Vec<Rational> {
        let denominator = Integer::from(self.denominator);
        self.numerator
            .into_coefficients_asc()
            .into_iter()
            .map(|n| Rational::from_integers_ref(&n, &denominator))
            .collect()
    }

    /// Returns the degree of a [`RationalPolynomial`].
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
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::ZERO.degree(), None);
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2").unwrap().degree(),
    ///     Some(0)
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x+1/3").unwrap().degree(),
    ///     Some(1)
    /// );
    /// ```
    #[inline]
    pub fn degree(&self) -> Option<u64> {
        self.numerator.degree()
    }

    /// Returns one of a [`RationalPolynomial`]'s coefficients.
    ///
    /// The index is the power of the variable the coefficient belongs to, so that index 0 gives the
    /// constant term. An index past the degree gives zero, which is the coefficient a polynomial
    /// has there.
    ///
    /// A coefficient is not stored anywhere — it is a numerator coefficient over the shared
    /// denominator, and the division has not been done — so this returns a value rather than a
    /// reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of bits of the
    /// coefficient and the denominator.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// assert_eq!(p.coefficient(0).to_string(), "1/3");
    /// assert_eq!(p.coefficient(1).to_string(), "1/2");
    /// assert_eq!(p.coefficient(100).to_string(), "0");
    /// ```
    #[inline]
    pub fn coefficient(&self, index: u64) -> Rational {
        Rational::from_integers_ref(
            self.numerator.coefficient(index),
            &Integer::from(self.denominator.clone()),
        )
    }

    /// Returns a [`RationalPolynomial`]'s leading coefficient.
    ///
    /// This is the coefficient of the highest power of the variable that the polynomial has one
    /// for. The zero polynomial has no such power, and gives zero, which is what every one of its
    /// coefficients is.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of [`coefficient`](Self::coefficient).
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// assert_eq!(p.leading_coefficient().to_string(), "1/2");
    /// assert_eq!(
    ///     RationalPolynomial::ZERO.leading_coefficient().to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    pub fn leading_coefficient(&self) -> Rational {
        Rational::from_integers_ref(
            self.numerator.leading_coefficient(),
            &Integer::from(self.denominator.clone()),
        )
    }

    /// Mutates one of a [`RationalPolynomial`]'s coefficients using a provided closure, and then
    /// returns whatever the closure returns.
    ///
    /// The index is the power of the variable the coefficient belongs to. An index past the degree
    /// is not an error: the polynomial grows to reach it, and the closure is handed the zero that
    /// was there all along.
    ///
    /// Since no coefficient is stored, the closure is handed a [`Rational`] built for it and the
    /// polynomial is rebuilt around whatever comes back. That costs a reduction, and it is why this
    /// is a closure rather than a `&mut Rational` the caller could hold onto.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of coefficients after the
    /// mutation, and $m$ is the largest number of bits of any of them.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// let ret = p.mutate_coefficient(0, |c| {
    ///     *c = Rational::from_signeds(1, 6);
    ///     true
    /// });
    /// assert_eq!(p.to_string(), "1/2*x+1/6");
    /// assert_eq!(ret, true);
    ///
    /// // The polynomial grows to reach a coefficient it did not have.
    /// p.mutate_coefficient(3, |c| *c = Rational::from_signeds(2, 3));
    /// assert_eq!(p.to_string(), "2/3*x^3+1/2*x+1/6");
    /// ```
    pub fn mutate_coefficient<F: FnOnce(&mut Rational) -> T, T>(&mut self, index: u64, f: F) -> T {
        let mut coefficients = self.to_coefficients_asc();
        let index = usize::exact_from(index);
        if index >= coefficients.len() {
            coefficients.resize(index + 1, Rational::ZERO);
        }
        let out = f(&mut coefficients[index]);
        *self = Self::from_coefficients_asc(coefficients);
        out
    }
}

// The greatest common divisor of a polynomial's coefficients, which is what it shares with its
// denominator. The zero polynomial has no content worth speaking of; this is never asked for it.
fn content(p: &IntegerPolynomial) -> Natural {
    let mut gcd = Natural::ZERO;
    for c in p.coefficients_asc() {
        gcd.gcd_assign(c.unsigned_abs_ref());
    }
    gcd
}

impl_named!(RationalPolynomial);
