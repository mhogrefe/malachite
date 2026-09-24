// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`IntegerPolynomial`](super::IntegerPolynomial), comparing two polynomials by their behavior for
/// large arguments.
pub mod cmp;
/// Implementations of [`EqTruncated`](malachite_base::polynomial::EqTruncated) between
/// [`IntegerPolynomial`](super::IntegerPolynomial)s, and between them and
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial)s and
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial)s.
///
/// # eq_truncated
/// ```
/// use core::str::FromStr;
/// use malachite_base::polynomial::EqTruncated;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let p = IntegerPolynomial::from_str("x^3-2*x^2+3*x+4").unwrap();
/// let q = IntegerPolynomial::from_str("-5*x^3-2*x^2+3*x+4").unwrap();
/// assert!(p.eq_truncated(&q, 3));
/// assert!(!p.eq_truncated(&q, 4));
/// assert!(p.eq_truncated(&q, 0));
///
/// let r = NaturalPolynomial::from_str("7*x^2+3*x+4").unwrap();
/// assert!(p.eq_truncated(&r, 2));
/// assert!(!p.eq_truncated(&r, 3));
/// assert!(r.eq_truncated(&p, 2));
///
/// // Only coefficients below x^len count, even when one polynomial is shorter.
/// let s = UnsignedPolynomial::<u32>::from_str("3*x+4").unwrap();
/// assert!(p.eq_truncated(&s, 2));
/// assert!(!p.eq_truncated(&s, 3));
/// assert!(s.eq_truncated(&p, 2));
/// ```
pub mod eq_truncated;
/// Equality of [`IntegerPolynomial`](super::IntegerPolynomial)s and
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
///
/// assert!(IntegerPolynomial::ZERO == GaussianInteger::ZERO);
/// assert!(IntegerPolynomial::one() == GaussianInteger::ONE);
/// assert!(IntegerPolynomial::negative_one() == GaussianInteger::NEGATIVE_ONE);
/// assert!(IntegerPolynomial::from(-123) == GaussianInteger::from(-123));
/// assert!(IntegerPolynomial::from(-123) != GaussianInteger::from_str("-123+i").unwrap());
/// assert!(IntegerPolynomial::x() != GaussianInteger::ZERO);
///
/// assert!(GaussianInteger::ZERO == IntegerPolynomial::ZERO);
/// assert!(GaussianInteger::from(-123) == IntegerPolynomial::from(-123));
/// assert!(GaussianInteger::from_str("-123-i").unwrap() != IntegerPolynomial::from(-123));
/// ```
pub mod partial_eq_gaussian_integer;
/// Equality of [`IntegerPolynomial`](super::IntegerPolynomial)s and
/// [`Integer`](crate::integer::Integer)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
///
/// assert!(IntegerPolynomial::ZERO == Integer::ZERO);
/// assert!(IntegerPolynomial::ZERO != Integer::NEGATIVE_ONE);
/// assert!(IntegerPolynomial::one() == Integer::ONE);
/// assert!(IntegerPolynomial::negative_one() == Integer::NEGATIVE_ONE);
/// assert!(IntegerPolynomial::from(-123) == Integer::from(-123));
/// assert!(IntegerPolynomial::from(-123) != Integer::from(123));
/// let n = Integer::from_str("-1000000000000000000000000").unwrap();
/// assert!(IntegerPolynomial::from(n.clone()) == n);
/// assert!(IntegerPolynomial::x() != Integer::ZERO);
/// assert!(IntegerPolynomial::from_str("x-1").unwrap() != Integer::NEGATIVE_ONE);
///
/// assert!(Integer::ZERO == IntegerPolynomial::ZERO);
/// assert!(Integer::from(-123) == IntegerPolynomial::from(-123));
/// assert!(Integer::from(123) != IntegerPolynomial::from(-123));
/// ```
pub mod partial_eq_integer;
/// Equality of [`IntegerPolynomial`](super::IntegerPolynomial)s and
/// [`Natural`](crate::natural::Natural)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
/// use malachite_nz::natural::Natural;
///
/// assert!(IntegerPolynomial::ZERO == Natural::ZERO);
/// assert!(IntegerPolynomial::ZERO != Natural::ONE);
/// assert!(IntegerPolynomial::one() == Natural::ONE);
/// assert!(IntegerPolynomial::negative_one() != Natural::ONE);
/// assert!(IntegerPolynomial::from(123) == Natural::from(123u32));
/// assert!(IntegerPolynomial::from(-123) != Natural::from(123u32));
/// assert!(IntegerPolynomial::x() != Natural::ZERO);
///
/// assert!(Natural::ZERO == IntegerPolynomial::ZERO);
/// assert!(Natural::from(123u32) == IntegerPolynomial::from(123));
/// assert!(Natural::from(123u32) != IntegerPolynomial::from(-123));
/// ```
pub mod partial_eq_natural;
/// Equality of [`IntegerPolynomial`](super::IntegerPolynomial)s and
/// [`NaturalPolynomial`](crate::natural_polynomial::NaturalPolynomial)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert!(IntegerPolynomial::ZERO == NaturalPolynomial::ZERO);
/// assert!(IntegerPolynomial::one() == NaturalPolynomial::one());
/// assert!(IntegerPolynomial::x() == NaturalPolynomial::x());
/// assert!(
///     IntegerPolynomial::from_str("3*x^2+1000000000000000000000000").unwrap()
///         == NaturalPolynomial::from_str("3*x^2+1000000000000000000000000").unwrap()
/// );
/// assert!(
///     IntegerPolynomial::from_str("-3*x^2+5").unwrap()
///         != NaturalPolynomial::from_str("3*x^2+5").unwrap()
/// );
/// assert!(IntegerPolynomial::negative_one() != NaturalPolynomial::one());
///
/// assert!(NaturalPolynomial::ZERO == IntegerPolynomial::ZERO);
/// assert!(
///     NaturalPolynomial::from_str("x+1").unwrap() == IntegerPolynomial::from_str("x+1").unwrap()
/// );
/// assert!(NaturalPolynomial::x() != IntegerPolynomial::from_str("x-1").unwrap());
/// ```
pub mod partial_eq_natural_polynomial;
/// Equality of [`IntegerPolynomial`](super::IntegerPolynomial)s and primitive integers.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
///
/// assert!(IntegerPolynomial::ZERO == 0u32);
/// assert!(IntegerPolynomial::ZERO != 1i8);
/// assert!(IntegerPolynomial::one() == 1u64);
/// assert!(IntegerPolynomial::negative_one() == -1i32);
/// assert!(IntegerPolynomial::negative_one() != u64::MAX);
/// assert!(IntegerPolynomial::from(-123) == -123i64);
/// assert!(IntegerPolynomial::from(-123) != 123u8);
/// assert!(IntegerPolynomial::from_str("-1000000000000").unwrap() == -1000000000000i64);
/// assert!(IntegerPolynomial::x() != 0usize);
/// assert!(IntegerPolynomial::from_str("x-1").unwrap() != -1i16);
///
/// assert!(0u32 == IntegerPolynomial::ZERO);
/// assert!(-1i128 != IntegerPolynomial::ZERO);
/// assert!(-123i64 == IntegerPolynomial::from(-123));
/// assert!(123u8 != IntegerPolynomial::from(-123));
/// ```
pub mod partial_eq_primitive_int;
/// Equality of [`IntegerPolynomial`](super::IntegerPolynomial)s and
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::polynomial::Polynomial;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
/// use malachite_nz::integer_polynomial::IntegerPolynomial;
///
/// assert!(IntegerPolynomial::ZERO == UnsignedPolynomial::<u8>::ZERO);
/// assert!(IntegerPolynomial::one() == UnsignedPolynomial::<u64>::one());
/// assert!(
///     IntegerPolynomial::from_str("3*x^2+255").unwrap()
///         == UnsignedPolynomial::<u8>::from_str("3*x^2+255").unwrap()
/// );
/// assert!(
///     IntegerPolynomial::from_str("3*x^2+256").unwrap()
///         != UnsignedPolynomial::<u8>::from_str("3*x^2").unwrap()
/// );
/// assert!(IntegerPolynomial::negative_one() != UnsignedPolynomial::<u8>::from(255));
///
/// assert!(UnsignedPolynomial::<u8>::ZERO == IntegerPolynomial::ZERO);
/// assert!(
///     UnsignedPolynomial::<u16>::from_str("x+1").unwrap()
///         == IntegerPolynomial::from_str("x+1").unwrap()
/// );
/// assert!(UnsignedPolynomial::<u128>::x() != IntegerPolynomial::from_str("x-1").unwrap());
/// ```
pub mod partial_eq_unsigned_polynomial;
/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ShortlexIntegerPolynomial`](super::ShortlexIntegerPolynomial) and
/// [`ShortlexIntegerPolynomialRef`](super::ShortlexIntegerPolynomialRef), comparing two polynomials
/// by degree and then by their coefficients from highest to lowest.
pub mod shortlex_cmp;
