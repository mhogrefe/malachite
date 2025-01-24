// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Comparison of [`Integer`](crate::integer::Integer)s.
pub mod cmp;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// and [`OrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`) (traits for comparing
/// the absolute values of numbers by order) for [`Integer`](crate::integer::Integer)s.
pub mod cmp_abs;
/// Equality of the absolute values of two [`Integer`](crate::integer::Integer)s.
pub mod eq_abs;
/// Equality of the absolute values of an [`Integer`](crate::integer::Integer) and a
/// [`Natural`](crate::integer::Natural).
pub mod eq_abs_natural;
/// Equality of the absolute values of an [`Integer`](crate::integer::Integer) and a primitive
/// float.
///
/// # eq_abs
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123).eq_abs(&123.0), true);
/// assert_eq!(Integer::from(123).eq_abs(&5.0), false);
/// assert_eq!(Integer::from(123).eq_abs(&-123.0), true);
/// assert_eq!(Integer::from(123).eq_abs(&-5.0), false);
/// assert_eq!(Integer::from(-123).eq_abs(&123.0), true);
/// assert_eq!(Integer::from(-123).eq_abs(&5.0), false);
/// assert_eq!(Integer::from(-123).eq_abs(&-123.0), true);
/// assert_eq!(Integer::from(-123).eq_abs(&-5.0), false);
/// assert_eq!(Integer::ZERO.eq_abs(&0.0), true);
/// assert_eq!(Integer::ZERO.eq_abs(&-0.0), true);
/// assert_eq!(Integer::ZERO.eq_abs(&f64::NAN), false);
/// assert_eq!(Integer::ZERO.eq_abs(&f64::INFINITY), false);
/// assert_eq!(Integer::ZERO.eq_abs(&f64::NEGATIVE_INFINITY), false);
///
/// assert_eq!(123.0.eq_abs(&Integer::from(123)), true);
/// assert_eq!(5.0.eq_abs(&Integer::from(123)), false);
/// assert_eq!((-123.0).eq_abs(&Integer::from(123)), true);
/// assert_eq!((-5.0).eq_abs(&Integer::from(123)), false);
/// assert_eq!(123.0.eq_abs(&Integer::from(-123)), true);
/// assert_eq!(5.0.eq_abs(&Integer::from(-123)), false);
/// assert_eq!((-123.0).eq_abs(&Integer::from(-123)), true);
/// assert_eq!((-5.0).eq_abs(&Integer::from(-123)), false);
/// assert_eq!(0.0.eq_abs(&Integer::ZERO), true);
/// assert_eq!((-0.0).eq_abs(&Integer::ZERO), true);
/// assert_eq!(f64::NAN.eq_abs(&Integer::ZERO), false);
/// assert_eq!(f64::INFINITY.eq_abs(&Integer::ZERO), false);
/// assert_eq!(f64::NEGATIVE_INFINITY.eq_abs(&Integer::ZERO), false);
/// ```
pub mod eq_abs_primitive_float;
/// Equality of the absolute values of an [`Integer`](crate::integer::Integer) and a primitive
/// integer.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(Integer::from(123).eq_abs(&123u32), true);
/// assert_eq!(Integer::from(-123).eq_abs(&123u32), true);
/// assert_eq!(Integer::from(123).eq_abs(&5u32), false);
/// assert_eq!(Integer::from(-123).eq_abs(&5u32), false);
///
/// assert_eq!(Integer::from(123).eq_abs(&123u64), true);
/// assert_eq!(Integer::from(-123).eq_abs(&123u64), true);
/// assert_eq!(Integer::from(123).eq_abs(&5u64), false);
/// assert_eq!(Integer::from(-123).eq_abs(&5u64), false);
///
/// assert_eq!(Integer::from(123).eq_abs(&123i64), true);
/// assert_eq!(Integer::from(-123).eq_abs(&123i64), true);
/// assert_eq!(Integer::from(123).eq_abs(&-123i64), true);
/// assert_eq!(Integer::from(-123).eq_abs(&-123i64), true);
///
/// assert_eq!(123u8.eq_abs(&Integer::from(123)), true);
/// assert_eq!(123u8.eq_abs(&Integer::from(-123)), true);
/// assert_eq!(5u8.eq_abs(&Integer::from(123)), false);
/// assert_eq!(5u8.eq_abs(&Integer::from(-123)), false);
///
/// assert_eq!(123u64.eq_abs(&Integer::from(123)), true);
/// assert_eq!(123u64.eq_abs(&Integer::from(-123)), true);
/// assert_eq!(5u64.eq_abs(&Integer::from(123)), false);
/// assert_eq!(5u64.eq_abs(&Integer::from(-123)), false);
///
/// assert_eq!(123i64.eq_abs(&Integer::from(123)), true);
/// assert_eq!(123i64.eq_abs(&Integer::from(-123)), true);
/// assert_eq!((-123i64).eq_abs(&Integer::from(123)), true);
/// assert_eq!((-123i64).eq_abs(&Integer::from(-123)), true);
/// ```
pub mod eq_abs_primitive_int;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Integer`](crate::integer::Integer)s and [`Natural`](crate::natural::Natural)s.
pub mod partial_cmp_abs_natural;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Integer`](crate::integer::Integer)s and primitive floats.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-123).gt_abs(&-122.5f32));
/// assert!(Integer::from(123).lt_abs(&f32::NEGATIVE_INFINITY));
///
/// assert!((-122.5f32).lt_abs(&Integer::from(-123)));
/// assert!(f32::NEGATIVE_INFINITY.gt_abs(&Integer::from(123)));
/// ```
pub mod partial_cmp_abs_primitive_float;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Integer`](crate::integer::Integer)s and primitive integers.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-122).lt_abs(&123u64));
/// assert!(Integer::from(-122).le_abs(&123u64));
/// assert!(Integer::from(-123).lt_abs(&124u64));
/// assert!(Integer::from(-123).le_abs(&124u64));
/// assert!(Integer::from(10u32).pow(12).gt_abs(&123u64));
/// assert!(Integer::from(10u32).pow(12).ge_abs(&123u64));
/// assert!((-Integer::from(10u32).pow(12)).gt_abs(&123u64));
/// assert!((-Integer::from(10u32).pow(12)).ge_abs(&123u64));
///
/// assert!(Integer::from(-122).lt_abs(&-123i64));
/// assert!(Integer::from(-122).le_abs(&-123i64));
/// assert!(Integer::from(-124).gt_abs(&-123i64));
/// assert!(Integer::from(-124).ge_abs(&-123i64));
/// assert!(Integer::from(10u32).pow(12).gt_abs(&123i64));
/// assert!(Integer::from(10u32).pow(12).ge_abs(&123i64));
/// assert!((-Integer::from(10u32).pow(12)).gt_abs(&123i64));
/// assert!((-Integer::from(10u32).pow(12)).ge_abs(&123i64));
///
/// assert!(123u64.gt_abs(&Integer::from(-122)));
/// assert!(123u64.ge_abs(&Integer::from(-122)));
/// assert!(124u64.gt_abs(&Integer::from(-123)));
/// assert!(124u64.ge_abs(&Integer::from(-123)));
/// assert!(123u64.lt_abs(&Integer::from(10u32).pow(12)));
/// assert!(123u64.le_abs(&Integer::from(10u32).pow(12)));
/// assert!(123u64.lt_abs(&-Integer::from(10u32).pow(12)));
/// assert!(123u64.le_abs(&-Integer::from(10u32).pow(12)));
///
/// assert!((-123i64).gt_abs(&Integer::from(-122)));
/// assert!((-123i64).ge_abs(&Integer::from(-122)));
/// assert!((-123i64).lt_abs(&Integer::from(-124)));
/// assert!((-123i64).le_abs(&Integer::from(-124)));
/// assert!(123i64.lt_abs(&Integer::from(10u32).pow(12)));
/// assert!(123i64.le_abs(&Integer::from(10u32).pow(12)));
/// assert!(123i64.lt_abs(&-Integer::from(10u32).pow(12)));
/// assert!(123i64.le_abs(&-Integer::from(10u32).pow(12)));
/// ```
pub mod partial_cmp_abs_primitive_int;
/// Comparison of [`Integer`](crate::integer::Integer)s and [`Natural`](crate::natural::Natural)s.
pub mod partial_cmp_natural;
/// Comparison of [`Integer`](crate::integer::Integer)s and primitive floats.
///
/// # partial_cmp
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-123) < -122.5f32);
/// assert!(Integer::from(123) < f32::INFINITY);
/// assert!(-122.5f32 > Integer::from(-123));
/// assert!(f32::INFINITY > Integer::from(123));
/// ```
pub mod partial_cmp_primitive_float;
/// Comparison of [`Integer`](crate::integer::Integer)s and primitive integers.
///
/// # partial_cmp
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-123) < 122u64);
/// assert!(Integer::from(-123) <= 122u64);
/// assert!(Integer::from(-123) < 124u64);
/// assert!(Integer::from(-123) <= 124u64);
/// assert!(Integer::from(10u32).pow(12) > 123u64);
/// assert!(Integer::from(10u32).pow(12) >= 123u64);
/// assert!(-Integer::from(10u32).pow(12) < 123u64);
/// assert!(-Integer::from(10u32).pow(12) <= 123u64);
///
/// assert!(Integer::from(-123) < -122i64);
/// assert!(Integer::from(-123) <= -122i64);
/// assert!(Integer::from(-123) > -124i64);
/// assert!(Integer::from(-123) >= -124i64);
/// assert!(Integer::from(10u32).pow(12) > 123i64);
/// assert!(Integer::from(10u32).pow(12) >= 123i64);
/// assert!(-Integer::from(10u32).pow(12) < 123i64);
/// assert!(-Integer::from(10u32).pow(12) <= 123i64);
///
/// assert!(122u64 > Integer::from(-123));
/// assert!(122u64 >= Integer::from(-123));
/// assert!(124u64 > Integer::from(-123));
/// assert!(124u64 >= Integer::from(-123));
/// assert!(123u64 < Integer::from(10u32).pow(12));
/// assert!(123u64 <= Integer::from(10u32).pow(12));
/// assert!(123u64 > -Integer::from(10u32).pow(12));
/// assert!(123u64 >= -Integer::from(10u32).pow(12));
///
/// assert!(-122i64 > Integer::from(-123));
/// assert!(-122i64 >= Integer::from(-123));
/// assert!(-124i64 < Integer::from(-123));
/// assert!(-124i64 <= Integer::from(-123));
/// assert!(123i64 < Integer::from(10u32).pow(12));
/// assert!(123i64 <= Integer::from(10u32).pow(12));
/// assert!(123i64 > -Integer::from(10u32).pow(12));
/// assert!(123i64 >= -Integer::from(10u32).pow(12));
/// ```
pub mod partial_cmp_primitive_int;
/// Equality of [`Integer`](crate::integer::Integer)s and [`Natural`](crate::natural::Natural)s.
pub mod partial_eq_natural;
/// Equality of [`Integer`](crate::integer::Integer)s and primitive floats.
///
/// # partial_eq
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(123u32) == 123.0f32);
/// assert!(Integer::from(123u32) != -5.0f32);
///
/// assert!(123.0f32 == Integer::from(123u32));
/// assert!(-5.0f32 != Integer::from(123u32));
/// ```
pub mod partial_eq_primitive_float;
/// Equality of [`Integer`](crate::integer::Integer)s and primitive integers.
///
/// # partial_eq
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(123) == 123u64);
/// assert!(Integer::from(-123) != 123u64);
///
/// assert!(123u64 == Integer::from(123));
/// assert!(123u64 != Integer::from(-123));
///
/// assert!(123u64 == Integer::from(123));
/// assert!(123u64 != Integer::from(-123));
///
/// assert!(23i64 != Integer::from(123));
/// assert!(-123i64 == Integer::from(-123));
/// ```
pub mod partial_eq_primitive_int;
