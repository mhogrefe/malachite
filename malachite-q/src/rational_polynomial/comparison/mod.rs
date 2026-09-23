// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`RationalPolynomial`](super::RationalPolynomial), comparing two polynomials by their behavior
/// for large arguments.
pub mod cmp;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == GaussianInteger::ZERO);
/// assert!(RationalPolynomial::one() == GaussianInteger::ONE);
/// assert!(RationalPolynomial::negative_one() == GaussianInteger::NEGATIVE_ONE);
/// assert!(RationalPolynomial::one_half() != GaussianInteger::ZERO);
/// assert!(RationalPolynomial::from(-123) == GaussianInteger::from(-123));
/// assert!(RationalPolynomial::from(-123) != GaussianInteger::from_str("-123+i").unwrap());
/// assert!(RationalPolynomial::x() != GaussianInteger::ZERO);
///
/// assert!(GaussianInteger::ZERO == RationalPolynomial::ZERO);
/// assert!(GaussianInteger::from(-123) == RationalPolynomial::from(-123));
/// assert!(GaussianInteger::from_str("-123-i").unwrap() != RationalPolynomial::from(-123));
/// ```
pub mod partial_eq_gaussian_integer;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{OneHalf, Zero};
/// use malachite_q::gaussian_rational::GaussianRational;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == GaussianRational::ZERO);
/// assert!(RationalPolynomial::one_half() == GaussianRational::ONE_HALF);
/// assert!(
///     RationalPolynomial::from_str("-22/7").unwrap()
///         == GaussianRational::from_str("-22/7").unwrap()
/// );
/// assert!(
///     RationalPolynomial::from_str("-22/7").unwrap()
///         != GaussianRational::from_str("-22/7+i/2").unwrap()
/// );
/// assert!(RationalPolynomial::x() != GaussianRational::ZERO);
///
/// assert!(GaussianRational::ZERO == RationalPolynomial::ZERO);
/// assert!(GaussianRational::ONE_HALF == RationalPolynomial::one_half());
/// assert!(GaussianRational::from_str("i/2").unwrap() != RationalPolynomial::ZERO);
/// ```
pub mod partial_eq_gaussian_rational;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`Integer`](malachite_nz::integer::Integer)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == Integer::ZERO);
/// assert!(RationalPolynomial::ZERO != Integer::NEGATIVE_ONE);
/// assert!(RationalPolynomial::one() == Integer::ONE);
/// assert!(RationalPolynomial::negative_one() == Integer::NEGATIVE_ONE);
/// assert!(RationalPolynomial::one_half() != Integer::ZERO);
/// assert!(RationalPolynomial::one_half() != Integer::ONE);
/// assert!(RationalPolynomial::from(-123) == Integer::from(-123));
/// assert!(RationalPolynomial::from(-123) != Integer::from(123));
/// assert!(RationalPolynomial::x() != Integer::ZERO);
///
/// assert!(Integer::ZERO == RationalPolynomial::ZERO);
/// assert!(Integer::from(-123) == RationalPolynomial::from(-123));
/// assert!(Integer::ONE != RationalPolynomial::one_half());
/// ```
pub mod partial_eq_integer;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`IntegerPolynomial`](malachite_nz::integer_polynomial::IntegerPolynomial)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == IntegerPolynomial::ZERO);
/// assert!(RationalPolynomial::negative_one() == IntegerPolynomial::negative_one());
/// assert!(RationalPolynomial::x() == IntegerPolynomial::x());
/// assert!(
///     RationalPolynomial::from_str("-3*x^2+5").unwrap()
///         == IntegerPolynomial::from_str("-3*x^2+5").unwrap()
/// );
/// assert!(
///     RationalPolynomial::from_str("1/2*x+1").unwrap()
///         != IntegerPolynomial::from_str("x+1").unwrap()
/// );
/// assert!(RationalPolynomial::one_half() != IntegerPolynomial::ZERO);
///
/// assert!(IntegerPolynomial::ZERO == RationalPolynomial::ZERO);
/// assert!(
///     IntegerPolynomial::from_str("x-1").unwrap() == RationalPolynomial::from_str("x-1").unwrap()
/// );
/// assert!(IntegerPolynomial::one() != RationalPolynomial::one_half());
/// ```
pub mod partial_eq_integer_polynomial;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`Natural`](malachite_nz::natural::Natural)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == Natural::ZERO);
/// assert!(RationalPolynomial::ZERO != Natural::ONE);
/// assert!(RationalPolynomial::one() == Natural::ONE);
/// assert!(RationalPolynomial::negative_one() != Natural::ONE);
/// assert!(RationalPolynomial::one_half() != Natural::ZERO);
/// assert!(RationalPolynomial::from(123) == Natural::from(123u32));
/// assert!(RationalPolynomial::x() != Natural::ZERO);
///
/// assert!(Natural::ZERO == RationalPolynomial::ZERO);
/// assert!(Natural::from(123u32) == RationalPolynomial::from(123));
/// assert!(Natural::ONE != RationalPolynomial::one_half());
/// ```
pub mod partial_eq_natural;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`NaturalPolynomial`](malachite_nz::natural_polynomial::NaturalPolynomial)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == NaturalPolynomial::ZERO);
/// assert!(RationalPolynomial::one() == NaturalPolynomial::one());
/// assert!(RationalPolynomial::x() == NaturalPolynomial::x());
/// assert!(
///     RationalPolynomial::from_str("3*x^2+5").unwrap()
///         == NaturalPolynomial::from_str("3*x^2+5").unwrap()
/// );
/// assert!(
///     RationalPolynomial::from_str("-3*x^2+5").unwrap()
///         != NaturalPolynomial::from_str("3*x^2+5").unwrap()
/// );
/// assert!(RationalPolynomial::one_half() != NaturalPolynomial::ZERO);
///
/// assert!(NaturalPolynomial::ZERO == RationalPolynomial::ZERO);
/// assert!(
///     NaturalPolynomial::from_str("x+1").unwrap() == RationalPolynomial::from_str("x+1").unwrap()
/// );
/// assert!(NaturalPolynomial::one() != RationalPolynomial::one_half());
/// ```
pub mod partial_eq_natural_polynomial;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and primitive integers.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == 0u32);
/// assert!(RationalPolynomial::ZERO != 1i8);
/// assert!(RationalPolynomial::one() == 1u64);
/// assert!(RationalPolynomial::negative_one() == -1i32);
/// assert!(RationalPolynomial::negative_one() != u64::MAX);
/// assert!(RationalPolynomial::one_half() != 0u8);
/// assert!(RationalPolynomial::one_half() != 1u8);
/// assert!(RationalPolynomial::from(-123) == -123i64);
/// assert!(RationalPolynomial::from(-123) != 123u8);
/// assert!(RationalPolynomial::from_str("-1000000000000").unwrap() == -1000000000000i64);
/// assert!(RationalPolynomial::x() != 0usize);
/// assert!(RationalPolynomial::from_str("1/2*x+1").unwrap() != 1i16);
///
/// assert!(0u32 == RationalPolynomial::ZERO);
/// assert!(-1i128 != RationalPolynomial::ZERO);
/// assert!(-123i64 == RationalPolynomial::from(-123));
/// assert!(1u8 != RationalPolynomial::one_half());
/// ```
pub mod partial_eq_primitive_int;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`Rational`](crate::Rational)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Zero};
/// use malachite_q::Rational;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == Rational::ZERO);
/// assert!(RationalPolynomial::one() == Rational::ONE);
/// assert!(RationalPolynomial::negative_one() == Rational::NEGATIVE_ONE);
/// assert!(RationalPolynomial::one_half() == Rational::ONE_HALF);
/// assert!(RationalPolynomial::one_half() != Rational::from_signeds(-1, 2));
/// assert!(RationalPolynomial::one_half() != Rational::from_signeds(1, 3));
/// assert!(RationalPolynomial::from_str("-22/7").unwrap() == Rational::from_str("-22/7").unwrap());
/// assert!(RationalPolynomial::x() != Rational::ZERO);
/// assert!(RationalPolynomial::from_str("1/2*x+1/2").unwrap() != Rational::ONE_HALF);
///
/// assert!(Rational::ZERO == RationalPolynomial::ZERO);
/// assert!(Rational::ONE_HALF == RationalPolynomial::one_half());
/// assert!(Rational::from_signeds(-1, 2) != RationalPolynomial::one_half());
/// ```
pub mod partial_eq_rational;
/// Equality of [`RationalPolynomial`](super::RationalPolynomial)s and
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
/// use malachite_q::rational_polynomial::RationalPolynomial;
///
/// assert!(RationalPolynomial::ZERO == UnsignedPolynomial::<u8>::ZERO);
/// assert!(RationalPolynomial::one() == UnsignedPolynomial::<u64>::one());
/// assert!(
///     RationalPolynomial::from_str("3*x^2+255").unwrap()
///         == UnsignedPolynomial::<u8>::from_str("3*x^2+255").unwrap()
/// );
/// assert!(
///     RationalPolynomial::from_str("3*x^2+256").unwrap()
///         != UnsignedPolynomial::<u8>::from_str("3*x^2").unwrap()
/// );
/// assert!(RationalPolynomial::negative_one() != UnsignedPolynomial::<u8>::from(255));
/// assert!(RationalPolynomial::one_half() != UnsignedPolynomial::<u8>::ZERO);
///
/// assert!(UnsignedPolynomial::<u8>::ZERO == RationalPolynomial::ZERO);
/// assert!(
///     UnsignedPolynomial::<u16>::from_str("x+1").unwrap()
///         == RationalPolynomial::from_str("x+1").unwrap()
/// );
/// assert!(UnsignedPolynomial::<u128>::x() != RationalPolynomial::from_str("1/2*x").unwrap());
/// ```
pub mod partial_eq_unsigned_polynomial;
/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexRationalPolynomial`](super::ShortlexRationalPolynomial) and
/// [`ShortlexRationalPolynomialRef`](super::ShortlexRationalPolynomialRef), comparing two
/// polynomials by degree and then by their coefficients from highest to lowest.
pub mod shortlex_cmp;
