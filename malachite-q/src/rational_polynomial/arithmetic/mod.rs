// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// two polynomials.
pub mod add;
/// Implementations of [`AddTruncated`](malachite_base::polynomial::AddTruncated) and
/// [`AddTruncatedAssign`](malachite_base::polynomial::AddTruncatedAssign), for adding two
/// polynomials and keeping only their low coefficients.
pub mod add_truncated;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// which bring a polynomial into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`ComposePowerOfX`](malachite_base::polynomial::ComposePowerOfX) and
/// [`ComposePowerOfXAssign`](malachite_base::polynomial::ComposePowerOfXAssign), for substituting a
/// power of the variable into a polynomial.
pub mod compose_power_of_x;
/// Implementations of [`Content`](malachite_base::polynomial::Content),
/// [`PrimitivePart`](malachite_base::polynomial::PrimitivePart), and
/// [`ContentAndPrimitivePart`](malachite_base::polynomial::ContentAndPrimitivePart) for
/// [`RationalPolynomial`](super::RationalPolynomial)s.
pub mod content;
/// Implementations of [`DeflatePowerOfX`](malachite_base::polynomial::DeflatePowerOfX) and
/// [`DeflatePowerOfXAssign`](malachite_base::polynomial::DeflatePowerOfXAssign), for undoing the
/// substitution of a power of the variable into a polynomial.
pub mod deflate_power_of_x;
/// Implementations of [`Derivative`](malachite_base::polynomial::Derivative) and
/// [`DerivativeAssign`](malachite_base::polynomial::DerivativeAssign), for differentiating a
/// polynomial.
pub mod derivative;
/// Implementations of [`DivPowerOfX`](malachite_base::polynomial::DivPowerOfX) and
/// [`DivPowerOfXAssign`](malachite_base::polynomial::DivPowerOfXAssign), for dividing a polynomial
/// by a power of its variable and discarding the remainder.
pub mod div_power_of_x;
/// Implementations of [`Evaluate`](malachite_base::polynomial::Evaluate) for
/// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s and
/// [`RationalPolynomial`](super::RationalPolynomial)s at [`Rational`](crate::Rational)s, and for
/// [`RationalPolynomial`](super::RationalPolynomial)s at
/// [`Integer`](malachite_nz::integer::Integer)s.
pub mod evaluate;
/// An implementation of [`ExponentGcd`](malachite_base::polynomial::ExponentGcd), the greatest
/// common divisor of the exponents at which a polynomial has nonzero coefficients.
pub mod exponent_gcd;
/// An implementation of [`Height`](malachite_base::num::arithmetic::traits::Height), the largest of
/// the heights of a polynomial's coefficients.
pub mod height;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
/// An implementation of [`L2NormSquared`](malachite_base::polynomial::L2NormSquared), the sum of
/// the squares of a polynomial's coefficients.
pub mod l2_norm_squared;
/// Implementations of [`MakeMonic`](malachite_base::polynomial::MakeMonic) and
/// [`MakeMonicAssign`](malachite_base::polynomial::MakeMonicAssign), which divide a polynomial by
/// its leading coefficient.
pub mod make_monic;
/// Implementations of [`MulPowerOfX`](malachite_base::polynomial::MulPowerOfX) and
/// [`MulPowerOfXAssign`](malachite_base::polynomial::MulPowerOfXAssign), for multiplying a
/// polynomial by a power of its variable.
pub mod mul_power_of_x;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a polynomial.
pub mod neg;
/// Left-shifting a [`RationalPolynomial`](super::RationalPolynomial) (multiplying it or dividing it
/// by a power of 2), keeping it in lowest terms.
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert_eq!((RationalPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 0u16).to_string(),
///     "1/3*x^2-3/4*x+5"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 2u32).to_string(),
///     "4/3*x^2-3*x+20"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
///     6338253001141147007483516026880"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 3i8).to_string(),
///     "8/3*x^2-6*x+40"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << -2i16).to_string(),
///     "1/12*x^2-3/16*x+5/4"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << -100i64).to_string(),
///     "1/3802951800684688204490109616128*x^2-3/5070602400912917605986812821504*x+\
///     5/1267650600228229401496703205376"
/// );
///
/// assert_eq!((&RationalPolynomial::ZERO << 10u8).to_string(), "0");
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 0u16).to_string(),
///     "1/3*x^2-3/4*x+5"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 2u32).to_string(),
///     "4/3*x^2-3*x+20"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 100u64).to_string(),
///     "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
///     6338253001141147007483516026880"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << 3i8).to_string(),
///     "8/3*x^2-6*x+40"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << -2i16).to_string(),
///     "1/12*x^2-3/16*x+5/4"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() << -100i64).to_string(),
///     "1/3802951800684688204490109616128*x^2-3/5070602400912917605986812821504*x+\
///     5/1267650600228229401496703205376"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// let mut p = RationalPolynomial::ZERO;
/// p <<= 10u8;
/// assert_eq!(p.to_string(), "0");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p <<= 0u16;
/// assert_eq!(p.to_string(), "1/3*x^2-3/4*x+5");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p <<= 2u32;
/// assert_eq!(p.to_string(), "4/3*x^2-3*x+20");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p <<= 3i8;
/// assert_eq!(p.to_string(), "8/3*x^2-6*x+40");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p <<= -2i16;
/// assert_eq!(p.to_string(), "1/12*x^2-3/16*x+5/4");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p <<= 100u64;
/// assert_eq!(
///     p.to_string(),
///     "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
///     6338253001141147007483516026880"
/// );
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p <<= -100i64;
/// assert_eq!(
///     p.to_string(),
///     "1/3802951800684688204490109616128*x^2-3/5070602400912917605986812821504*x+\
///     5/1267650600228229401496703205376"
/// );
/// ```
pub mod shl;
/// Right-shifting a [`RationalPolynomial`](super::RationalPolynomial) (dividing it or multiplying
/// it by a power of 2), keeping it in lowest terms.
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert_eq!((RationalPolynomial::ZERO >> 10u8).to_string(), "0");
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> 0u16).to_string(),
///     "1/3*x^2-3/4*x+5"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> -2i32).to_string(),
///     "4/3*x^2-3*x+20"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> -100i64).to_string(),
///     "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
///     6338253001141147007483516026880"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> -3i8).to_string(),
///     "8/3*x^2-6*x+40"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> 2u16).to_string(),
///     "1/12*x^2-3/16*x+5/4"
/// );
/// assert_eq!(
///     (RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> 100u64).to_string(),
///     "1/3802951800684688204490109616128*x^2-3/5070602400912917605986812821504*x+\
///     5/1267650600228229401496703205376"
/// );
///
/// assert_eq!((&RationalPolynomial::ZERO >> 10u8).to_string(), "0");
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> 0u16).to_string(),
///     "1/3*x^2-3/4*x+5"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> -2i32).to_string(),
///     "4/3*x^2-3*x+20"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> -100i64).to_string(),
///     "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
///     6338253001141147007483516026880"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> -3i8).to_string(),
///     "8/3*x^2-6*x+40"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> 2u16).to_string(),
///     "1/12*x^2-3/16*x+5/4"
/// );
/// assert_eq!(
///     (&RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap() >> 100u64).to_string(),
///     "1/3802951800684688204490109616128*x^2-3/5070602400912917605986812821504*x+\
///     5/1267650600228229401496703205376"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// let mut p = RationalPolynomial::ZERO;
/// p >>= 10u8;
/// assert_eq!(p.to_string(), "0");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p >>= 0u16;
/// assert_eq!(p.to_string(), "1/3*x^2-3/4*x+5");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p >>= -2i32;
/// assert_eq!(p.to_string(), "4/3*x^2-3*x+20");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p >>= -3i8;
/// assert_eq!(p.to_string(), "8/3*x^2-6*x+40");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p >>= 2u16;
/// assert_eq!(p.to_string(), "1/12*x^2-3/16*x+5/4");
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p >>= -100i64;
/// assert_eq!(
///     p.to_string(),
///     "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
///     6338253001141147007483516026880"
/// );
///
/// let mut p = RationalPolynomial::from_str("1/3*x^2-3/4*x+5").unwrap();
/// p >>= 100u64;
/// assert_eq!(
///     p.to_string(),
///     "1/3802951800684688204490109616128*x^2-3/5070602400912917605986812821504*x+\
///     5/1267650600228229401496703205376"
/// );
/// ```
pub mod shr;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting one polynomial from another.
pub mod sub;
/// Implementations of [`SubTruncated`](malachite_base::polynomial::SubTruncated) and
/// [`SubTruncatedAssign`](malachite_base::polynomial::SubTruncatedAssign), for subtracting one
/// polynomial from another and keeping only their low coefficients.
pub mod sub_truncated;
