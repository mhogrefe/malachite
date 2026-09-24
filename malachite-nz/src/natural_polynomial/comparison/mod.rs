// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`NaturalPolynomial`](super::NaturalPolynomial), comparing two polynomials by their behavior for
/// large arguments.
pub mod cmp;
/// Implementations of [`EqTruncated`](malachite_base::polynomial::EqTruncated) between
/// [`NaturalPolynomial`](super::NaturalPolynomial)s, and between them and
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial)s.
///
/// # eq_truncated
/// ```
/// use core::str::FromStr;
/// use malachite_base::polynomial::EqTruncated;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// let p = NaturalPolynomial::from_str("x^3+2*x^2+3*x+4").unwrap();
/// let q = NaturalPolynomial::from_str("5*x^3+2*x^2+3*x+4").unwrap();
/// assert!(p.eq_truncated(&q, 3));
/// assert!(!p.eq_truncated(&q, 4));
/// assert!(p.eq_truncated(&q, 0));
///
/// // Only coefficients below x^len count, even when one polynomial is shorter.
/// let r = UnsignedPolynomial::<u8>::from_str("3*x+4").unwrap();
/// assert!(p.eq_truncated(&r, 2));
/// assert!(!p.eq_truncated(&r, 3));
/// assert!(r.eq_truncated(&p, 2));
/// ```
pub mod eq_truncated;
/// Equality of [`NaturalPolynomial`](super::NaturalPolynomial)s and
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert!(NaturalPolynomial::ZERO == GaussianInteger::ZERO);
/// assert!(NaturalPolynomial::one() == GaussianInteger::ONE);
/// assert!(NaturalPolynomial::from(123u32) == GaussianInteger::from(123));
/// assert!(NaturalPolynomial::from(123u32) != GaussianInteger::from(-123));
/// assert!(NaturalPolynomial::from(123u32) != GaussianInteger::from_str("123+i").unwrap());
/// assert!(NaturalPolynomial::x() != GaussianInteger::ZERO);
///
/// assert!(GaussianInteger::ZERO == NaturalPolynomial::ZERO);
/// assert!(GaussianInteger::from(123) == NaturalPolynomial::from(123u32));
/// assert!(GaussianInteger::from_str("123+i").unwrap() != NaturalPolynomial::from(123u32));
/// ```
pub mod partial_eq_gaussian_integer;

/// Equality of [`NaturalPolynomial`](super::NaturalPolynomial)s and
/// [`Integer`](crate::integer::Integer)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert!(NaturalPolynomial::ZERO == Integer::ZERO);
/// assert!(NaturalPolynomial::ZERO != Integer::NEGATIVE_ONE);
/// assert!(NaturalPolynomial::one() == Integer::ONE);
/// assert!(NaturalPolynomial::one() != Integer::NEGATIVE_ONE);
/// assert!(NaturalPolynomial::from(123u32) == Integer::from(123));
/// assert!(NaturalPolynomial::from(123u32) != Integer::from(-123));
/// assert!(NaturalPolynomial::x() != Integer::ZERO);
/// assert!(NaturalPolynomial::from_str("x+1").unwrap() != Integer::ONE);
///
/// assert!(Integer::ZERO == NaturalPolynomial::ZERO);
/// assert!(Integer::from(123) == NaturalPolynomial::from(123u32));
/// assert!(Integer::from(-123) != NaturalPolynomial::from(123u32));
/// ```
pub mod partial_eq_integer;

/// Equality of [`NaturalPolynomial`](super::NaturalPolynomial)s and
/// [`Natural`](crate::natural::Natural)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::{One, Zero};
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert!(NaturalPolynomial::ZERO == Natural::ZERO);
/// assert!(NaturalPolynomial::ZERO != Natural::ONE);
/// assert!(NaturalPolynomial::one() == Natural::ONE);
/// assert!(NaturalPolynomial::from(123u32) == Natural::from(123u32));
/// assert!(NaturalPolynomial::from(123u32) != Natural::from(5u32));
/// let n = Natural::from_str("1000000000000000000000000").unwrap();
/// assert!(NaturalPolynomial::from(n.clone()) == n);
/// assert!(NaturalPolynomial::x() != Natural::ZERO);
/// assert!(NaturalPolynomial::from_str("x+1").unwrap() != Natural::ONE);
///
/// assert!(Natural::ZERO == NaturalPolynomial::ZERO);
/// assert!(Natural::ONE != NaturalPolynomial::ZERO);
/// assert!(Natural::from(123u32) == NaturalPolynomial::from(123u32));
/// assert!(Natural::from(5u32) != NaturalPolynomial::from(123u32));
/// ```
pub mod partial_eq_natural;

/// Equality of [`NaturalPolynomial`](super::NaturalPolynomial)s and unsigned primitive integers.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::polynomial::Polynomial;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert!(NaturalPolynomial::ZERO == 0u32);
/// assert!(NaturalPolynomial::ZERO != 1u8);
/// assert!(NaturalPolynomial::one() == 1u64);
/// assert!(NaturalPolynomial::from(123u32) == 123u8);
/// assert!(NaturalPolynomial::from(123u32) != 5u128);
/// assert!(NaturalPolynomial::from_str("1000000000000").unwrap() == 1000000000000u64);
/// assert!(NaturalPolynomial::from_str("1000000000000").unwrap() != 4096u16);
/// assert!(NaturalPolynomial::x() != 0usize);
/// assert!(NaturalPolynomial::from_str("x+1").unwrap() != 1u32);
///
/// assert!(0u32 == NaturalPolynomial::ZERO);
/// assert!(1u8 != NaturalPolynomial::ZERO);
/// assert!(123u8 == NaturalPolynomial::from(123u32));
/// assert!(5u128 != NaturalPolynomial::from(123u32));
/// ```
pub mod partial_eq_unsigned;

/// Equality of [`NaturalPolynomial`](super::NaturalPolynomial)s and
/// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial)s.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::polynomial::Polynomial;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
/// use malachite_nz::natural_polynomial::NaturalPolynomial;
///
/// assert!(NaturalPolynomial::ZERO == UnsignedPolynomial::<u8>::ZERO);
/// assert!(NaturalPolynomial::one() == UnsignedPolynomial::<u64>::one());
/// assert!(
///     NaturalPolynomial::from_str("3*x^2+255").unwrap()
///         == UnsignedPolynomial::<u8>::from_str("3*x^2+255").unwrap()
/// );
/// assert!(
///     NaturalPolynomial::from_str("3*x^2+256").unwrap()
///         != UnsignedPolynomial::<u8>::from_str("3*x^2").unwrap()
/// );
/// assert!(NaturalPolynomial::x() != UnsignedPolynomial::<u32>::one());
///
/// assert!(UnsignedPolynomial::<u8>::ZERO == NaturalPolynomial::ZERO);
/// assert!(
///     UnsignedPolynomial::<u16>::from_str("x+1").unwrap()
///         == NaturalPolynomial::from_str("x+1").unwrap()
/// );
/// assert!(UnsignedPolynomial::<u128>::x() != NaturalPolynomial::from_str("x+1").unwrap());
/// ```
pub mod partial_eq_unsigned_polynomial;
