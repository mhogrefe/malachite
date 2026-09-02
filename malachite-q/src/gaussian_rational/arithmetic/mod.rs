// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`AbsSquared`](malachite_base::num::arithmetic::traits::AbsSquared) and
/// [`AbsSquaredAssign`](malachite_base::num::arithmetic::traits::AbsSquaredAssign), traits for
/// computing the squared absolute value (norm) of a number.
pub mod abs_squared;
/// Addition of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s.
pub mod add;
/// An implementation of
/// [`CanonicalUnitIPow`](malachite_base::num::arithmetic::traits::CanonicalUnitIPow), a trait for
/// finding the power of $i$ that brings a number into canonical unit form.
pub mod canonical_unit_i_pow;
/// Implementations of
/// [`CanonicalizeUnit`](malachite_base::num::arithmetic::traits::CanonicalizeUnit) and
/// [`CanonicalizeUnitAssign`](malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign),
/// traits for bringing a number into canonical unit form.
pub mod canonicalize_unit;
/// Implementations of [`Conjugate`](malachite_base::num::arithmetic::traits::Conjugate) and
/// [`ConjugateAssign`](malachite_base::num::arithmetic::traits::ConjugateAssign), traits for
/// computing the complex conjugate of a number: the sign of the imaginary part is flipped.
pub mod conjugate;
/// Division of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s, including
/// [`CheckedDiv`](malachite_base::num::arithmetic::traits::CheckedDiv).
pub mod div;
/// Implementations of [`DivI`](malachite_base::num::arithmetic::traits::DivI) and
/// [`DivIAssign`](malachite_base::num::arithmetic::traits::DivIAssign), traits for dividing a
/// number by $i$: a clockwise quarter turn.
pub mod div_i;
/// An implementation of [`IsPowerOf2`](malachite_base::num::arithmetic::traits::IsPowerOf2), a
/// trait for determining whether a number is an integer power of 2.
pub mod is_power_of_2;
/// An implementation of [`IsUnit`](malachite_base::num::arithmetic::traits::IsUnit), a trait for
/// determining whether a number is a unit of its ring.
pub mod is_unit;
/// Multiplication of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s.
pub mod mul;
/// Implementations of [`MulI`](malachite_base::num::arithmetic::traits::MulI) and
/// [`MulIAssign`](malachite_base::num::arithmetic::traits::MulIAssign), traits for multiplying a
/// number by $i$: a counterclockwise quarter turn.
pub mod mul_i;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign) for
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational), negating both the real and
/// imaginary parts.
pub mod neg;
/// Implementations of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2), a trait for
/// computing a power of 2.
pub mod power_of_2;
/// Implementations of [`Reciprocal`](malachite_base::num::arithmetic::traits::Reciprocal) and
/// [`ReciprocalAssign`](malachite_base::num::arithmetic::traits::ReciprocalAssign), traits for
/// computing the reciprocal of a number.
pub mod reciprocal;
/// Left-shifting a [`GaussianRational`](crate::gaussian_rational::GaussianRational) (multiplying it
/// by a power of 2). A negative shift amount divides by a power of 2 instead.
///
/// # shl
/// ```
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let x = GaussianRational::from_str("7/22-i").unwrap();
/// assert_eq!((x.clone() << 2u8).to_string(), "14/11-4i");
/// assert_eq!((&x << 2u64).to_string(), "14/11-4i");
/// assert_eq!((&x << 2i8).to_string(), "14/11-4i");
/// assert_eq!((&x << -2i64).to_string(), "7/88-i/4");
/// assert_eq!((x << -1i32).to_string(), "7/44-i/2");
/// ```
///
/// # shl_assign
/// ```
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let mut x = GaussianRational::from_str("1+i").unwrap();
/// x <<= 1u8;
/// x <<= 2u16;
/// assert_eq!(x.to_string(), "8+8i");
/// x <<= -4i8;
/// x <<= -1i64;
/// assert_eq!(x.to_string(), "1/4+i/4");
/// ```
pub mod shl;
/// Right-shifting a [`GaussianRational`](crate::gaussian_rational::GaussianRational) (dividing it
/// by a power of 2). A negative shift amount multiplies by a power of 2 instead.
///
/// # shr
/// ```
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let x = GaussianRational::from_str("14/11-4i").unwrap();
/// assert_eq!((x.clone() >> 2u8).to_string(), "7/22-i");
/// assert_eq!((&x >> 2u64).to_string(), "7/22-i");
/// assert_eq!((&x >> 2i8).to_string(), "7/22-i");
/// assert_eq!((&x >> -2i64).to_string(), "56/11-16i");
/// assert_eq!((x >> -1i32).to_string(), "28/11-8i");
/// ```
///
/// # shr_assign
/// ```
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// let mut x = GaussianRational::from_str("8+8i").unwrap();
/// x >>= 1u8;
/// x >>= 2u16;
/// assert_eq!(x.to_string(), "1+i");
/// x >>= -4i8;
/// x >>= -1i64;
/// assert_eq!(x.to_string(), "32+32i");
/// ```
pub mod shr;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), traits for squaring a
/// number.
pub mod square;
/// Subtraction of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s.
pub mod sub;
