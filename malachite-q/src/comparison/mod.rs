// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Comparison of [`Rational`](crate::Rational)s.
pub mod cmp;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// and [`OrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`) (traits for comparing
/// the absolute values of numbers by order) for [`Rational`](crate::Rational)s.
pub mod cmp_abs;
/// Equality of the absolute values of two [`Rational`](crate::Rational)s.
pub mod eq_abs;
/// Equality of the absolute values of a [`Rational`](crate::Rational) and an
/// [`Integer`](malachite_nz::integer::Integer).
pub mod eq_abs_integer;
/// Equality of the absolute values of a [`Rational`](crate::Rational) and a
/// [`Natural`](malachite_nz::natural::Natural).
pub mod eq_abs_natural;
/// Equality of the absolute values of a [`Rational`](crate::Rational) and a primitive float.
///
/// # eq_abs
/// ```
/// use malachite_base::num::basic::traits::{NegativeInfinity, Zero};
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_q::Rational;
///
/// assert_eq!(Rational::from(123).eq_abs(&123.0), true);
/// assert_eq!(Rational::from(123).eq_abs(&5.0), false);
/// assert_eq!(Rational::from(123).eq_abs(&-123.0), true);
/// assert_eq!(Rational::from(123).eq_abs(&-5.0), false);
/// assert_eq!(Rational::from(-123).eq_abs(&123.0), true);
/// assert_eq!(Rational::from(-123).eq_abs(&5.0), false);
/// assert_eq!(Rational::from(-123).eq_abs(&-123.0), true);
/// assert_eq!(Rational::from(-123).eq_abs(&-5.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&123.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&5.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&-123.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&-5.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&123.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&5.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&-123.0), false);
/// assert_eq!(Rational::from_signeds(22, 7).eq_abs(&-5.0), false);
/// assert_eq!(Rational::ZERO.eq_abs(&0.0), true);
/// assert_eq!(Rational::ZERO.eq_abs(&-0.0), true);
/// assert_eq!(Rational::ZERO.eq_abs(&f64::NAN), false);
/// assert_eq!(Rational::ZERO.eq_abs(&f64::INFINITY), false);
/// assert_eq!(Rational::ZERO.eq_abs(&f64::NEGATIVE_INFINITY), false);
///
/// assert_eq!(123.0.eq_abs(&Rational::from(123)), true);
/// assert_eq!(5.0.eq_abs(&Rational::from(123)), false);
/// assert_eq!((-123.0).eq_abs(&Rational::from(123)), true);
/// assert_eq!((-5.0).eq_abs(&Rational::from(123)), false);
/// assert_eq!(123.0.eq_abs(&Rational::from(-123)), true);
/// assert_eq!(5.0.eq_abs(&Rational::from(-123)), false);
/// assert_eq!((-123.0).eq_abs(&Rational::from(-123)), true);
/// assert_eq!((-5.0).eq_abs(&Rational::from(-123)), false);
/// assert_eq!(123.0.eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!(5.0.eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!((-123.0).eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!((-5.0).eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!(123.0.eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!(5.0.eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!((-123.0).eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!((-5.0).eq_abs(&Rational::from_signeds(22, 7)), false);
/// assert_eq!(0.0.eq_abs(&Rational::ZERO), true);
/// assert_eq!((-0.0).eq_abs(&Rational::ZERO), true);
/// assert_eq!(f64::NAN.eq_abs(&Rational::ZERO), false);
/// assert_eq!(f64::INFINITY.eq_abs(&Rational::ZERO), false);
/// assert_eq!(f64::NEGATIVE_INFINITY.eq_abs(&Rational::ZERO), false);
/// ```
pub mod eq_abs_primitive_float;
/// Equality of the absolute values of a [`Rational`](crate::Rational) and a primitive integer.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_q::Rational;
///
/// assert!(Rational::from(123).eq_abs(&123u64));
/// assert!(Rational::from(-123).eq_abs(&123u64));
/// assert!(Rational::from_signeds(22, 7).ne_abs(&123u64));
/// assert!(Rational::from_signeds(-22, 7).ne_abs(&123u64));
///
/// assert!(Rational::from(123).eq_abs(&123i64));
/// assert!(Rational::from(123).eq_abs(&-123i64));
/// assert!(Rational::from_signeds(22, 7).ne_abs(&-123i64));
/// assert!(Rational::from_signeds(-22, 7).ne_abs(&-123i64));
///
/// assert!(123u64.eq_abs(&Rational::from(123)));
/// assert!(123u64.eq_abs(&Rational::from(-123)));
/// assert!(123u64.ne_abs(&Rational::from_signeds(22, 7)));
/// assert!(123u64.ne_abs(&Rational::from_signeds(-22, 7)));
///
/// assert!(123i64.eq_abs(&Rational::from(123)));
/// assert!(123i64.eq_abs(&Rational::from(-123)));
/// assert!((-123i64).ne_abs(&Rational::from_signeds(22, 7)));
/// assert!((-123i64).ne_abs(&Rational::from_signeds(-22, 7)));
/// ```
pub mod eq_abs_primitive_int;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Rational`](crate::Rational)s and [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_cmp_abs_integer;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Rational`](crate::Rational)s and [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_cmp_abs_natural;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Rational`](crate::Rational)s and primitive floats.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_q::Rational;
///
/// assert!(Rational::from_signeds(1, 3).gt_abs(&-0.33f32));
/// assert!(Rational::from_signeds(1, 3).lt_abs(&-0.34f32));
///
/// assert!((-0.33f32).lt_abs(&Rational::from_signeds(1, 3)));
/// assert!((-0.34f32).gt_abs(&Rational::from_signeds(1, 3)));
/// ```
pub mod partial_cmp_abs_primitive_float;
/// Implementations of [`PartialOrdAbs`](`malachite_base::num::comparison::traits::PartialOrdAbs`)
/// (a trait for comparing the absolute values of numbers by order) for
/// [`Rational`](crate::Rational)s and primitive integers.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_q::Rational;
///
/// assert!(Rational::from_signeds(22, 7).gt_abs(&3u32));
/// assert!(Rational::from_signeds(22, 7).lt_abs(&4u32));
/// assert!(Rational::from_signeds(-22, 7).gt_abs(&3u32));
/// assert!(Rational::from_signeds(-22, 7).lt_abs(&4u32));
///
/// assert!(Rational::from_signeds(22, 7).gt_abs(&3i32));
/// assert!(Rational::from_signeds(22, 7).lt_abs(&4i32));
/// assert!(Rational::from_signeds(-22, 7).gt_abs(&-3i32));
/// assert!(Rational::from_signeds(-22, 7).lt_abs(&-4i32));
///
/// assert!(3u32.lt_abs(&Rational::from_signeds(22, 7)));
/// assert!(4u32.gt_abs(&Rational::from_signeds(22, 7)));
/// assert!(3u32.lt_abs(&Rational::from_signeds(-22, 7)));
/// assert!(4u32.gt_abs(&Rational::from_signeds(-22, 7)));
///
/// assert!(3i32.lt_abs(&Rational::from_signeds(22, 7)));
/// assert!(4i32.gt_abs(&Rational::from_signeds(22, 7)));
/// assert!((-3i32).lt_abs(&Rational::from_signeds(-22, 7)));
/// assert!((-4i32).gt_abs(&Rational::from_signeds(-22, 7)));
/// ```
pub mod partial_cmp_abs_primitive_int;
/// Comparison of [`Rational`](crate::Rational)s and [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_cmp_integer;
/// Comparison of [`Rational`](crate::Rational)s and [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_cmp_natural;
/// Comparison of [`Rational`](crate::Rational)s and primitive floats.
///
/// # partial_cmp
/// ```
/// use malachite_q::Rational;
///
/// assert!(Rational::from_signeds(1, 3) > 0.33f32);
/// assert!(Rational::from_signeds(1, 3) < 0.34f32);
///
/// assert!(0.33f32 < Rational::from_signeds(1, 3));
/// assert!(0.34f32 > Rational::from_signeds(1, 3));
/// ```
pub mod partial_cmp_primitive_float;
/// Comparison of [`Rational`](crate::Rational)s and primitive integers.
///
/// # partial_cmp
/// ```
/// use malachite_q::Rational;
///
/// assert!(Rational::from_signeds(22, 7) > 3u32);
/// assert!(Rational::from_signeds(22, 7) < 4u32);
/// assert!(Rational::from_signeds(-22, 7) < 3u32);
/// assert!(Rational::from_signeds(-22, 7) < 4u32);
///
/// assert!(Rational::from_signeds(22, 7) > 3i32);
/// assert!(Rational::from_signeds(22, 7) < 4i32);
/// assert!(Rational::from_signeds(-22, 7) < -3i32);
/// assert!(Rational::from_signeds(-22, 7) > -4i32);
///
/// assert!(3u32 < Rational::from_signeds(22, 7));
/// assert!(4u32 > Rational::from_signeds(22, 7));
/// assert!(3u32 > Rational::from_signeds(-22, 7));
/// assert!(4u32 > Rational::from_signeds(-22, 7));
///
/// assert!(3i32 < Rational::from_signeds(22, 7));
/// assert!(4i32 > Rational::from_signeds(22, 7));
/// assert!(-3i32 > Rational::from_signeds(-22, 7));
/// assert!(-4i32 < Rational::from_signeds(-22, 7));
/// ```
pub mod partial_cmp_primitive_int;
/// Equality of [`Rational`](crate::Rational)s and [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_eq_integer;
/// Equality of [`Rational`](crate::Rational)s and [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_eq_natural;
/// Equality of [`Rational`](crate::Rational)s and primitive floats.
///
/// # partial_eq
/// ```
/// use malachite_q::Rational;
///
/// assert!(Rational::from_signeds(3, 2) == 1.5f32);
/// assert!(Rational::from_signeds(3, 2) != 1.4f32);
///
/// assert!(1.5f32 == Rational::from_signeds(3, 2));
/// assert!(1.4f32 != Rational::from_signeds(3, 2));
/// ```
pub mod partial_eq_primitive_float;
/// Equality of [`Rational`](crate::Rational)s and primitive integers.
///
/// # partial_eq
/// ```
/// use malachite_q::Rational;
///
/// assert!(Rational::from(123) == 123u64);
/// assert!(Rational::from(-123) != 123u64);
/// assert!(Rational::from_signeds(22, 7) != 123u64);
/// assert!(Rational::from_signeds(-22, 7) != 123u64);
///
/// assert!(Rational::from(123) == 123i64);
/// assert!(Rational::from(-123) == -123i64);
/// assert!(Rational::from_signeds(22, 7) != -123i64);
/// assert!(Rational::from_signeds(-22, 7) != -123i64);
///
/// assert!(123u64 == Rational::from(123));
/// assert!(123u64 != Rational::from(-123));
/// assert!(123u64 != Rational::from_signeds(22, 7));
/// assert!(123u64 != Rational::from_signeds(-22, 7));
///
/// assert!(123i64 == Rational::from(123));
/// assert!(-123i64 == Rational::from(-123));
/// assert!(-123i64 != Rational::from_signeds(22, 7));
/// assert!(-123i64 != Rational::from_signeds(-22, 7));
/// ```
pub mod partial_eq_primitive_int;
