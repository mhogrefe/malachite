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
