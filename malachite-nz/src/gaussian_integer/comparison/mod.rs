// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Ord`] and [`PartialOrd`] for
/// [`ComparableGaussianInteger`](crate::gaussian_integer::ComparableGaussianInteger) and
/// [`ComparableGaussianIntegerRef`](crate::gaussian_integer::ComparableGaussianIntegerRef),
/// ordering [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s lexicographically: first
/// by real part, then by imaginary part.
pub mod cmp;
/// Implementations of [`OrdAbs`](malachite_base::num::comparison::traits::OrdAbs) and
/// [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// [`GaussianInteger`](crate::gaussian_integer::GaussianInteger), comparing absolute values
/// (distances from the origin).
pub mod cmp_abs;
/// Equality of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s and
/// [`Integer`](crate::integer::Integer)s.
pub mod partial_eq_integer;
/// Equality of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s and
/// [`Natural`](crate::natural::Natural)s.
pub mod partial_eq_natural;
/// Equality of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s and primitive floats.
///
/// # partial_eq
/// ```
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// assert!(GaussianInteger::from(123) == 123.0f32);
/// assert!(GaussianInteger::from(123) != -5.0f32);
/// assert!(GaussianInteger::from_str("123+i").unwrap() != 123.0f32);
///
/// assert!(123.0f32 == GaussianInteger::from(123));
/// assert!(-5.0f32 != GaussianInteger::from(123));
/// ```
pub mod partial_eq_primitive_float;
/// Equality of [`GaussianInteger`](crate::gaussian_integer::GaussianInteger)s and primitive
/// integers.
///
/// # partial_eq
/// ```
/// use malachite_nz::gaussian_integer::GaussianInteger;
/// use std::str::FromStr;
///
/// assert!(GaussianInteger::from(123) == 123u64);
/// assert!(GaussianInteger::from(-123) != 123u64);
/// assert!(GaussianInteger::from_str("123+i").unwrap() != 123u64);
///
/// assert!(123u64 == GaussianInteger::from(123));
/// assert!(123u64 != GaussianInteger::from(-123));
///
/// assert!(-123i64 == GaussianInteger::from(-123));
/// assert!(23i64 != GaussianInteger::from(123));
/// ```
pub mod partial_eq_primitive_int;
