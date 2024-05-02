// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Comparison of [`Float`](crate::Float)s.
pub mod cmp;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// and [`OrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`) (traits for comparing
/// the absolute values of numbers by order) for [`Float`](crate::Float)s.
pub mod cmp_abs;
/// Equality of [`Float`](crate::Float)s.
pub mod eq;
/// Hashing of [`Float`](crate::Float)s.
pub mod hash;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for [`Float`](crate::Float)s and
/// [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_cmp_abs_integer;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for [`Float`](crate::Float)s and
/// [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_cmp_abs_natural;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for [`Float`](crate::Float)s and
/// primitive floats.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_float::Float;
///
/// assert!(Float::ONE_HALF.gt_abs(&0.4));
/// assert!(Float::ONE_HALF.lt_abs(&0.6));
/// // TODO assert!(Float::ONE_HALF.eq_abs(&-0.5));
///
/// assert!(0.4.lt_abs(&Float::ONE_HALF));
/// assert!(0.6.gt_abs(&Float::ONE_HALF));
/// // TODO assert!((-0.5).eq_abs(&Float::ONE_HALF));
/// ```
pub mod partial_cmp_abs_primitive_float;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for [`Float`](crate::Float)s and
/// primitive integers.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_float::Float;
///
/// assert!(Float::from(80).lt_abs(&100u64));
/// assert!(Float::INFINITY.gt_abs(&100u64));
/// assert!(Float::NEGATIVE_INFINITY.gt_abs(&100u64));
///
/// assert!(100u64.gt_abs(&Float::from(80)));
/// assert!(100u64.lt_abs(&Float::INFINITY));
/// assert!(100u64.lt_abs(&Float::NEGATIVE_INFINITY));
///
/// assert!(Float::from(80).lt_abs(&100i64));
/// assert!(Float::from(-80).lt_abs(&-100i64));
/// assert!(Float::INFINITY.gt_abs(&100i64));
/// assert!(Float::NEGATIVE_INFINITY.gt_abs(&-100i64));
///
/// assert!(100i64.gt_abs(&Float::from(80)));
/// assert!(100i64.lt_abs(&Float::INFINITY));
/// assert!((-100i64).lt_abs(&Float::INFINITY));
/// assert!((-100i64).lt_abs(&Float::NEGATIVE_INFINITY));
/// ```
pub mod partial_cmp_abs_primitive_int;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for [`Float`](crate::Float)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod partial_cmp_abs_rational;
/// Comparison of [`Float`](crate::Float)s and [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_cmp_integer;
/// Comparison of [`Float`](crate::Float)s and [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_cmp_natural;
/// Comparison of [`Float`](crate::Float)s and primitive floats.
///
/// # partial_cmp
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_float::Float;
///
/// assert!(Float::ONE_HALF > 0.4);
/// assert!(Float::ONE_HALF < 0.6);
/// assert!(Float::ONE_HALF > -0.5);
///
/// assert!(0.4 < Float::ONE_HALF);
/// assert!(0.6 > Float::ONE_HALF);
/// assert!(-0.5 < Float::ONE_HALF);
/// ```
pub mod partial_cmp_primitive_float;
/// Comparison of [`Float`](crate::Float)s and primitive integers.
///
/// # partial_cmp
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
/// use malachite_float::Float;
///
/// assert!(Float::from(80) < 100u64);
/// assert!(Float::INFINITY > 100u64);
/// assert!(Float::NEGATIVE_INFINITY < 100u64);
///
/// assert!(100u64 > Float::from(80));
/// assert!(100u64 < Float::INFINITY);
/// assert!(100u64 > Float::NEGATIVE_INFINITY);
///
/// assert!(Float::from(80) < 100i64);
/// assert!(Float::from(-80) > -100i64);
/// assert!(Float::INFINITY > 100i64);
/// assert!(Float::NEGATIVE_INFINITY < -100i64);
///
/// assert!(100i64 > Float::from(80));
/// assert!(-100i64 < Float::from(-80));
/// assert!(-100i64 < Float::INFINITY);
/// assert!(-100i64 > Float::NEGATIVE_INFINITY);
/// ```
pub mod partial_cmp_primitive_int;
/// Comparison of [`Float`](crate::Float)s and [`Rational`](malachite_q::Rational)s.
pub mod partial_cmp_rational;
/// Equality of [`Float`](crate::Float)s and [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_eq_integer;
/// Equality of [`Float`](crate::Float)s and [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_eq_natural;
/// Equality of [`Float`](crate::Float)s and primitive floats.
///
/// # partial_eq
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_float::Float;
///
/// assert!(Float::from(123) == 123.0);
/// assert!(Float::ONE_HALF == 0.5);
/// assert!(Float::ONE_HALF != -0.5);
/// assert!(Float::ONE_HALF != 0.4);
///
/// assert!(123.0 == Float::from(123));
/// assert!(0.5 == Float::ONE_HALF);
/// assert!(-0.5 != Float::ONE_HALF);
/// assert!(0.4 != Float::ONE_HALF);
/// ```
pub mod partial_eq_primitive_float;
/// Equality of [`Float`](crate::Float)s and primitive integers.
///
/// # partial_eq
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_float::Float;
///
/// assert!(Float::from(123) == 123u64);
/// assert!(Float::ONE_HALF != 1u64);
///
/// assert!(Float::from(123) == 123i64);
/// assert!(Float::from(-123) == -123i64);
/// assert!(Float::ONE_HALF != -1i64);
///
/// assert!(123u64 == Float::from(123));
/// assert!(1u64 != Float::ONE_HALF);
///
/// assert!(123i64 == Float::from(123));
/// assert!(-123i64 == Float::from(-123));
/// assert!(-1i64 != Float::ONE_HALF);
/// ```
pub mod partial_eq_primitive_int;
/// Equality of [`Float`](crate::Float)s and [`Rational`](malachite_q::Rational)s.
pub mod partial_eq_rational;
