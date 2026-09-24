// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`UnsignedPolynomial`](super::UnsignedPolynomial), comparing two polynomials by their behavior
/// for large arguments.
pub mod cmp;
/// Implementations of [`EqTruncated`](crate::polynomial::EqTruncated) for
/// [`UnsignedPolynomial`](super::UnsignedPolynomial).
///
/// # eq_truncated
/// ```
/// use core::str::FromStr;
/// use malachite_base::polynomial::EqTruncated;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
///
/// let p = UnsignedPolynomial::<u64>::from_str("x^3+2*x^2+3*x+4").unwrap();
/// let q = UnsignedPolynomial::<u64>::from_str("5*x^3+2*x^2+3*x+4").unwrap();
/// assert!(p.eq_truncated(&q, 3));
/// assert!(!p.eq_truncated(&q, 4));
/// assert!(p.eq_truncated(&q, 0));
///
/// // Only coefficients below x^len count, even when one polynomial is shorter.
/// let r = UnsignedPolynomial::<u64>::from_str("3*x+4").unwrap();
/// assert!(p.eq_truncated(&r, 2));
/// assert!(!p.eq_truncated(&r, 3));
/// ```
pub mod eq_truncated;
/// Equality of [`UnsignedPolynomial`](super::UnsignedPolynomial)s and values of their coefficient
/// type.
///
/// # partial_eq
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::polynomial::Polynomial;
/// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
///
/// assert!(UnsignedPolynomial::<u64>::ZERO == 0);
/// assert!(UnsignedPolynomial::<u64>::ZERO != 1);
/// assert!(UnsignedPolynomial::<u64>::one() == 1);
/// assert!(UnsignedPolynomial::<u8>::from(123) == 123);
/// assert!(UnsignedPolynomial::<u8>::from(123) != 5);
/// assert!(UnsignedPolynomial::<u64>::x() != 0);
/// assert!(UnsignedPolynomial::<u64>::from_str("x+1").unwrap() != 1);
///
/// assert!(0 == UnsignedPolynomial::<u64>::ZERO);
/// assert!(1u64 != UnsignedPolynomial::<u64>::ZERO);
/// assert!(123u8 == UnsignedPolynomial::<u8>::from(123));
/// assert!(5u8 != UnsignedPolynomial::<u8>::from(123));
/// ```
pub mod partial_eq_unsigned;
