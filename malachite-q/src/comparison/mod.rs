// Copyright © 2024 Mikhail Hogrefe
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
