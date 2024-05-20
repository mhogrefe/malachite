// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Comparison of [`Natural`](crate::natural::Natural)s.
pub mod cmp;
/// Equality of the absolute values of a [`Natural`](crate::natural::Natural) and a primitive float.
///
/// # eq_abs
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(123u32).eq_abs(&123.0), true);
/// assert_eq!(Natural::from(123u32).eq_abs(&5.0), false);
/// assert_eq!(Natural::from(123u32).eq_abs(&-123.0), true);
/// assert_eq!(Natural::from(123u32).eq_abs(&-5.0), false);
/// assert_eq!(Natural::ZERO.eq_abs(&0.0), true);
/// assert_eq!(Natural::ZERO.eq_abs(&-0.0), true);
/// assert_eq!(Natural::ZERO.eq_abs(&f64::NAN), false);
/// assert_eq!(Natural::ZERO.eq_abs(&f64::INFINITY), false);
/// assert_eq!(Natural::ZERO.eq_abs(&f64::NEGATIVE_INFINITY), false);
///
/// assert_eq!(123.0.eq_abs(&Natural::from(123u32)), true);
/// assert_eq!(5.0.eq_abs(&Natural::from(123u32)), false);
/// assert_eq!((-123.0).eq_abs(&Natural::from(123u32)), true);
/// assert_eq!((-5.0).eq_abs(&Natural::from(123u32)), false);
/// assert_eq!(0.0.eq_abs(&Natural::ZERO), true);
/// assert_eq!((-0.0).eq_abs(&Natural::ZERO), true);
/// assert_eq!(f64::NAN.eq_abs(&Natural::ZERO), false);
/// assert_eq!(f64::INFINITY.eq_abs(&Natural::ZERO), false);
/// assert_eq!(f64::NEGATIVE_INFINITY.eq_abs(&Natural::ZERO), false);
/// ```
pub mod eq_abs_primitive_float;
/// Equality of the absolute values of a [`Natural`](crate::natural::Natural) and a primitive
/// integer.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(123u32).eq_abs(&123u32), true);
/// assert_eq!(Natural::from(123u32).eq_abs(&5u32), false);
///
/// assert_eq!(Natural::from(123u32).eq_abs(&123u64), true);
/// assert_eq!(Natural::from(123u32).eq_abs(&5u64), false);
///
/// assert_eq!(Natural::from(123u32).eq_abs(&123i64), true);
/// assert_eq!(Natural::from(123u32).eq_abs(&-123i64), true);
///
/// assert_eq!(123u8.eq_abs(&Natural::from(123u32)), true);
/// assert_eq!(5u8.eq_abs(&Natural::from(123u32)), false);
///
/// assert_eq!(123u64.eq_abs(&Natural::from(123u32)), true);
/// assert_eq!(5u64.eq_abs(&Natural::from(123u32)), false);
///
/// assert_eq!(123i64.eq_abs(&Natural::from(123u32)), true);
/// assert_eq!((-123i64).eq_abs(&Natural::from(123u32)), true);
/// ```
pub mod eq_abs_primitive_int;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Natural`](crate::natural::Natural)s and primitive floats.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32).gt_abs(&-122.5f32));
/// assert!(Natural::from(123u32).lt_abs(&f32::NEGATIVE_INFINITY));
/// assert!((-122.5f32).lt_abs(&Natural::from(123u32)));
/// assert!(f32::NEGATIVE_INFINITY.gt_abs(&Natural::from(123u32)));
/// ```
pub mod partial_cmp_abs_primitive_float;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Natural`](crate::natural::Natural)s and primitive integers.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(122u32).lt_abs(&123u64));
/// assert!(Natural::from(122u32).le_abs(&123u64));
/// assert!(Natural::from(123u32).lt_abs(&124u64));
/// assert!(Natural::from(123u32).le_abs(&124u64));
/// assert!(Natural::from(10u32).pow(12).gt_abs(&123u64));
/// assert!(Natural::from(10u32).pow(12).ge_abs(&123u64));
///
/// assert!(Natural::from(122u32).lt_abs(&-123i64));
/// assert!(Natural::from(122u32).le_abs(&-123i64));
/// assert!(Natural::from(124u32).gt_abs(&-123i64));
/// assert!(Natural::from(124u32).ge_abs(&-123i64));
/// assert!(Natural::from(10u32).pow(12).gt_abs(&123i64));
/// assert!(Natural::from(10u32).pow(12).ge_abs(&123i64));
///
/// assert!(123u64.gt_abs(&Natural::from(122u32)));
/// assert!(123u64.ge_abs(&Natural::from(122u32)));
/// assert!(124u64.gt_abs(&Natural::from(123u32)));
/// assert!(124u64.ge_abs(&Natural::from(123u32)));
/// assert!(123u64.lt_abs(&Natural::from(10u32).pow(12)));
/// assert!(123u64.le_abs(&Natural::from(10u32).pow(12)));
///
/// assert!((-123i64).gt_abs(&Natural::from(122u32)));
/// assert!((-123i64).ge_abs(&Natural::from(122u32)));
/// assert!((-123i64).lt_abs(&Natural::from(124u32)));
/// assert!((-123i64).le_abs(&Natural::from(124u32)));
/// assert!(123i64.lt_abs(&Natural::from(10u32).pow(12)));
/// assert!(123i64.le_abs(&Natural::from(10u32).pow(12)));
/// ```
pub mod partial_cmp_abs_primitive_int;
/// Comparison of [`Natural`](crate::natural::Natural)s and primitive floats.
///
/// # partial_cmp
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) > 122.5f32);
/// assert!(Natural::from(123u32) < f32::INFINITY);
/// assert!(122.5f32 < Natural::from(123u32));
/// assert!(f32::INFINITY > Natural::from(123u32));
/// ```
pub mod partial_cmp_primitive_float;
/// Comparison of [`Natural`](crate::natural::Natural)s and primitive integers.
///
/// # partial_cmp
/// ```
/// use malachite_base::num::arithmetic::traits::Pow;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) > 122u32);
/// assert!(Natural::from(123u32) >= 122u32);
/// assert!(Natural::from(123u32) < 124u32);
/// assert!(Natural::from(123u32) <= 124u32);
/// assert!(Natural::from(10u32).pow(12) > 123u32);
/// assert!(Natural::from(10u32).pow(12) >= 123u32);
///
/// assert!(Natural::from(123u32) > 122u8);
/// assert!(Natural::from(123u32) >= 122u8);
/// assert!(Natural::from(123u32) < 124u8);
/// assert!(Natural::from(123u32) <= 124u8);
/// assert!(Natural::from(10u32).pow(12) > 123u8);
/// assert!(Natural::from(10u32).pow(12) >= 123u8);
///
/// assert!(Natural::from(123u32) > 122u64);
/// assert!(Natural::from(123u32) >= 122u64);
/// assert!(Natural::from(123u32) < 124u64);
/// assert!(Natural::from(123u32) <= 124u64);
/// assert!(Natural::from(10u32).pow(12) > 123u64);
/// assert!(Natural::from(10u32).pow(12) >= 123u64);
///
/// assert!(Natural::from(123u32) > 122i64);
/// assert!(Natural::from(123u32) >= 122i64);
/// assert!(Natural::from(123u32) < 124i64);
/// assert!(Natural::from(123u32) <= 124i64);
/// assert!(Natural::from(123u32) > -124i64);
/// assert!(Natural::from(123u32) >= -124i64);
/// assert!(Natural::from(10u32).pow(12) > 123i64);
/// assert!(Natural::from(10u32).pow(12) >= 123i64);
///
/// assert!(122u32 < Natural::from(123u32));
/// assert!(122u32 <= Natural::from(123u32));
/// assert!(124u32 > Natural::from(123u32));
/// assert!(124u32 >= Natural::from(123u32));
/// assert!(123u32 < Natural::from(10u32).pow(12));
/// assert!(123u32 <= Natural::from(10u32).pow(12));
///
/// assert!(122u8 < Natural::from(123u32));
/// assert!(122u8 <= Natural::from(123u32));
/// assert!(124u8 > Natural::from(123u32));
/// assert!(124u8 >= Natural::from(123u32));
/// assert!(123u8 < Natural::from(10u32).pow(12));
/// assert!(123u8 <= Natural::from(10u32).pow(12));
///
/// assert!(122u64 < Natural::from(123u32));
/// assert!(122u64 <= Natural::from(123u32));
/// assert!(124u64 > Natural::from(123u32));
/// assert!(124u64 >= Natural::from(123u32));
/// assert!(123u64 < Natural::from(10u32).pow(12));
/// assert!(123u64 <= Natural::from(10u32).pow(12));
///
/// assert!(122i64 < Natural::from(123u32));
/// assert!(122i64 <= Natural::from(123u32));
/// assert!(124i64 > Natural::from(123u32));
/// assert!(124i64 >= Natural::from(123u32));
/// assert!(-124i64 < Natural::from(123u32));
/// assert!(-124i64 <= Natural::from(123u32));
/// assert!(123i64 < Natural::from(10u32).pow(12));
/// assert!(123i64 <= Natural::from(10u32).pow(12));
/// ```
pub mod partial_cmp_primitive_int;
/// Equality of [`Natural`](crate::natural::Natural)s and primitive floats.
///
/// # partial_eq
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) == 123.0f32);
/// assert!(Natural::from(123u32) != -5.0f32);
///
/// assert!(123.0f32 == Natural::from(123u32));
/// assert!(-5.0f32 != Natural::from(123u32));
/// ```
pub mod partial_eq_primitive_float;
/// Equality of [`Natural`](crate::natural::Natural)s and primitive integers.
///
/// # partial_eq
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) == 123u32);
/// assert!(Natural::from(123u32) != 5u32);
///
/// assert!(Natural::from(123u32) == 123u64);
/// assert!(Natural::from(123u32) != 5u64);
///
/// assert!(Natural::from(123u32) == 123i64);
/// assert!(Natural::from(123u32) != -123i64);
///
/// assert!(123u8 == Natural::from(123u32));
/// assert!(5u8 != Natural::from(123u32));
///
/// assert!(123u64 == Natural::from(123u32));
/// assert!(5u64 != Natural::from(123u32));
///
/// assert!(123i64 == Natural::from(123u32));
/// assert!(-123i64 != Natural::from(123u32));
/// ```
pub mod partial_eq_primitive_int;
