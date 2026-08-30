// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ComparableGaussianRational`](crate::gaussian_rational::ComparableGaussianRational) and
/// [`ComparableGaussianRationalRef`](crate::gaussian_rational::ComparableGaussianRationalRef),
/// ordering [`GaussianRational`](crate::gaussian_rational::GaussianRational)s lexicographically:
/// first by real part, then by imaginary part.
pub mod cmp;
/// Implementations of [`OrdAbs`](malachite_base::num::comparison::traits::OrdAbs) and
/// [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational), comparing absolute values
/// (distances from the origin).
pub mod cmp_abs;
/// Equality of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s and
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger)s.
pub mod partial_eq_gaussian_integer;
/// Equality of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s and
/// [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_eq_integer;
/// Equality of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s and
/// [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_eq_natural;
/// Equality of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s and primitive
/// floats.
///
/// # partial_eq
/// ```
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// assert!(GaussianRational::from(123) == 123.0f32);
/// assert!(GaussianRational::from(123) != -5.0f32);
/// assert!(GaussianRational::from_str("1/2").unwrap() == 0.5f32);
/// assert!(GaussianRational::from_str("123+i").unwrap() != 123.0f32);
///
/// assert!(123.0f32 == GaussianRational::from(123));
/// assert!(0.5f32 == GaussianRational::from_str("1/2").unwrap());
/// assert!(-5.0f32 != GaussianRational::from(123));
/// ```
pub mod partial_eq_primitive_float;
/// Equality of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s and primitive
/// integers.
///
/// # partial_eq
/// ```
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// assert!(GaussianRational::from(123) == 123u64);
/// assert!(GaussianRational::from(-123) != 123u64);
/// assert!(GaussianRational::from_str("123+i").unwrap() != 123u64);
/// assert!(GaussianRational::from_str("22/7").unwrap() != 3u64);
///
/// assert!(123u64 == GaussianRational::from(123));
/// assert!(123u64 != GaussianRational::from(-123));
///
/// assert!(-123i64 == GaussianRational::from(-123));
/// assert!(23i64 != GaussianRational::from(123));
/// ```
pub mod partial_eq_primitive_int;
/// Equality of [`GaussianRational`](crate::gaussian_rational::GaussianRational)s and
/// [`Rational`](crate::Rational)s.
pub mod partial_eq_rational;
