// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// In FLINT's fmpzi multiplication and squaring, the paths that trade multiplications for additions
// are only worthwhile when the real and imaginary parts have similar sizes: within 2 limbs (with
// 64-bit limbs) in fmpzi_mul and fmpzi_sqr alike. The tolerance is expressed here in bits so that
// it does not shift when Malachite is built with 32-bit limbs.
pub(crate) const SIZE_BALANCE_BITS: u64 = 2 * 64;

/// Implementations of [`AbsSquared`](malachite_base::num::arithmetic::traits::AbsSquared) and
/// [`AbsSquaredAssign`](malachite_base::num::arithmetic::traits::AbsSquaredAssign), traits for
/// computing the squared absolute value (norm) of a number.
pub mod abs_squared;
/// Addition of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod add;
/// Implementations of [`Conjugate`](malachite_base::num::arithmetic::traits::Conjugate) and
/// [`ConjugateAssign`](malachite_base::num::arithmetic::traits::ConjugateAssign), traits for
/// computing the complex conjugate of a number: the sign of the imaginary part is flipped.
pub mod conjugate;
/// An implementation of [`IsPowerOf2`](malachite_base::num::arithmetic::traits::IsPowerOf2), a
/// trait for determining whether a number is an integer power of 2.
pub mod is_power_of_2;
/// Multiplication of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod mul;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign) for
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger), negating both the real and
/// imaginary parts.
pub mod neg;
/// An implementation of [`PowerOf2`](malachite_base::num::arithmetic::traits::PowerOf2), a trait
/// for computing a power of 2.
pub mod power_of_2;
/// Left-shifting a [`GaussianInteger`](crate::gaussian_integer::GaussianInteger) (multiplying it by
/// a power of 2).
///
/// # shl
/// ```
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// let x = GaussianInteger::from_str("3-2i").unwrap();
/// assert_eq!((x.clone() << 3u8).to_string(), "24-16i");
/// assert_eq!((&x << 3u64).to_string(), "24-16i");
/// assert_eq!(
///     (x << 100u32).to_string(),
///     "3802951800684688204490109616128-2535301200456458802993406410752i"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// let mut x = GaussianInteger::from_str("1+i").unwrap();
/// x <<= 1u8;
/// x <<= 2u16;
/// x <<= 3u32;
/// x <<= 4u64;
/// assert_eq!(x.to_string(), "1024+1024i");
/// ```
pub mod shl;
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), traits for squaring a
/// number.
pub mod square;
/// Subtraction of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s.
pub mod sub;
