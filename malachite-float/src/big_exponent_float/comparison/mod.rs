// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Comparison of [`BigExponentFloat`](crate::BigExponentFloat)s, and the total order of
/// [`ComparableBigExponentFloat`](crate::big_exponent_float::ComparableBigExponentFloat) and
/// [`ComparableBigExponentFloatRef`](crate::big_exponent_float::ComparableBigExponentFloatRef).
pub mod cmp;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) and
/// [`OrdAbs`](malachite_base::num::comparison::traits::OrdAbs) (traits for comparing the absolute
/// values of numbers by order) for [`BigExponentFloat`](crate::BigExponentFloat)s.
pub mod cmp_abs;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s.
pub mod eq;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) (a trait for
/// comparing the absolute values of numbers for equality) for
/// [`BigExponentFloat`](crate::BigExponentFloat)s.
pub mod eq_abs;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a [`Float`](crate::Float)
/// for equality.
pub mod eq_abs_float;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) for equality.
pub mod eq_abs_gaussian_integer;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`GaussianRational`](malachite_q::gaussian_rational::GaussianRational) for equality.
pub mod eq_abs_gaussian_rational;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and an
/// [`Integer`](malachite_nz::integer::Integer) for equality.
pub mod eq_abs_integer;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`Natural`](malachite_nz::natural::Natural) for equality.
pub mod eq_abs_natural;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a primitive float for
/// equality.
///
/// # eq_abs
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::from(123).eq_abs(&123.0));
/// assert!(BigExponentFloat::ONE_HALF.eq_abs(&-0.5));
/// assert!(BigExponentFloat::ONE_HALF.ne_abs(&0.4));
///
/// assert!(123.0.eq_abs(&BigExponentFloat::from(123)));
/// assert!((-0.5).eq_abs(&BigExponentFloat::ONE_HALF));
/// assert!(0.4.ne_abs(&BigExponentFloat::ONE_HALF));
/// ```
pub mod eq_abs_primitive_float;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a primitive integer for
/// equality.
///
/// # eq_abs
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::from(123).eq_abs(&123u64));
/// assert!(BigExponentFloat::ONE_HALF.ne_abs(&1u64));
///
/// assert!(BigExponentFloat::from(123).eq_abs(&123i64));
/// assert!(BigExponentFloat::from(-123).eq_abs(&123i64));
/// assert!(BigExponentFloat::ONE_HALF.ne_abs(&-1i64));
///
/// assert!(123u64.eq_abs(&BigExponentFloat::from(123)));
/// assert!(1u64.ne_abs(&BigExponentFloat::ONE_HALF));
///
/// assert!(123i64.eq_abs(&BigExponentFloat::from(123)));
/// assert!(123i64.eq_abs(&BigExponentFloat::from(-123)));
/// assert!((-1i64).ne_abs(&BigExponentFloat::ONE_HALF));
/// ```
pub mod eq_abs_primitive_int;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`Rational`](malachite_q::Rational) for equality.
pub mod eq_abs_rational;
/// Hashing of
/// [`ComparableBigExponentFloat`](crate::big_exponent_float::ComparableBigExponentFloat)s and
/// [`ComparableBigExponentFloatRef`](crate::big_exponent_float::ComparableBigExponentFloatRef)s.
pub mod hash;
/// Functions for finding the minimum and maximum of two
/// [`BigExponentFloat`](crate::BigExponentFloat)s, or of a
/// [`BigExponentFloat`](crate::BigExponentFloat) and a [`Rational`](malachite_q::Rational).
pub mod min_max;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`Float`](crate::Float).
pub mod partial_cmp_abs_float;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger).
pub mod partial_cmp_abs_gaussian_integer;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`GaussianRational`](malachite_q::gaussian_rational::GaussianRational).
pub mod partial_cmp_abs_gaussian_rational;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and an
/// [`Integer`](malachite_nz::integer::Integer).
pub mod partial_cmp_abs_integer;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`Natural`](malachite_nz::natural::Natural).
pub mod partial_cmp_abs_natural;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a primitive
/// float.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::ONE_HALF.gt_abs(&0.4));
/// assert!(BigExponentFloat::ONE_HALF.lt_abs(&0.6));
/// assert!(BigExponentFloat::ONE_HALF.eq_abs(&-0.5));
///
/// assert!(0.4.lt_abs(&BigExponentFloat::ONE_HALF));
/// assert!(0.6.gt_abs(&BigExponentFloat::ONE_HALF));
/// assert!((-0.5).eq_abs(&BigExponentFloat::ONE_HALF));
/// ```
pub mod partial_cmp_abs_primitive_float;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a primitive
/// integer.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::from(80).lt_abs(&100u64));
/// assert!(BigExponentFloat::INFINITY.gt_abs(&100u64));
/// assert!(BigExponentFloat::NEGATIVE_INFINITY.gt_abs(&100u64));
///
/// assert!(100u64.gt_abs(&BigExponentFloat::from(80)));
/// assert!(100u64.lt_abs(&BigExponentFloat::INFINITY));
/// assert!(100u64.lt_abs(&BigExponentFloat::NEGATIVE_INFINITY));
///
/// assert!(BigExponentFloat::from(80).lt_abs(&100i64));
/// assert!(BigExponentFloat::from(-80).lt_abs(&-100i64));
/// assert!(BigExponentFloat::INFINITY.gt_abs(&100i64));
/// assert!(BigExponentFloat::NEGATIVE_INFINITY.gt_abs(&-100i64));
///
/// assert!(100i64.gt_abs(&BigExponentFloat::from(80)));
/// assert!(100i64.lt_abs(&BigExponentFloat::INFINITY));
/// assert!((-100i64).lt_abs(&BigExponentFloat::INFINITY));
/// assert!((-100i64).lt_abs(&BigExponentFloat::NEGATIVE_INFINITY));
/// ```
pub mod partial_cmp_abs_primitive_int;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a [`BigExponentFloat`](crate::BigExponentFloat) and a
/// [`Rational`](malachite_q::Rational).
pub mod partial_cmp_abs_rational;
/// Comparison of [`BigExponentFloat`](crate::BigExponentFloat)s and [`Float`](crate::Float)s.
pub mod partial_cmp_float;
/// Comparison of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_cmp_integer;
/// Comparison of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_cmp_natural;
/// Comparison of [`BigExponentFloat`](crate::BigExponentFloat)s and primitive floats.
///
/// # partial_cmp
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::ONE_HALF > 0.4);
/// assert!(BigExponentFloat::ONE_HALF < 0.6);
/// assert!(BigExponentFloat::ONE_HALF > -0.5);
/// assert_eq!(BigExponentFloat::ONE_HALF.partial_cmp(&f64::NAN), None);
///
/// assert!(0.4 < BigExponentFloat::ONE_HALF);
/// assert!(0.6 > BigExponentFloat::ONE_HALF);
/// assert!(-0.5 < BigExponentFloat::ONE_HALF);
/// ```
pub mod partial_cmp_primitive_float;
/// Comparison of [`BigExponentFloat`](crate::BigExponentFloat)s and primitive integers.
///
/// # partial_cmp
/// ```
/// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::from(80) < 100u64);
/// assert!(BigExponentFloat::INFINITY > 100u64);
/// assert!(BigExponentFloat::NEGATIVE_INFINITY < 100u64);
///
/// assert!(100u64 > BigExponentFloat::from(80));
/// assert!(100u64 < BigExponentFloat::INFINITY);
/// assert!(100u64 > BigExponentFloat::NEGATIVE_INFINITY);
///
/// assert!(BigExponentFloat::from(80) < 100i64);
/// assert!(BigExponentFloat::from(-80) > -100i64);
/// assert!(BigExponentFloat::INFINITY > 100i64);
/// assert!(BigExponentFloat::NEGATIVE_INFINITY < -100i64);
///
/// assert!(100i64 > BigExponentFloat::from(80));
/// assert!(-100i64 < BigExponentFloat::from(-80));
/// assert!(-100i64 < BigExponentFloat::INFINITY);
/// assert!(-100i64 > BigExponentFloat::NEGATIVE_INFINITY);
/// ```
pub mod partial_cmp_primitive_int;
/// Comparison of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod partial_cmp_rational;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and [`Float`](crate::Float)s.
pub mod partial_eq_float;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger)s.
pub mod partial_eq_gaussian_integer;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`GaussianRational`](malachite_q::gaussian_rational::GaussianRational)s.
pub mod partial_eq_gaussian_rational;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`Integer`](malachite_nz::integer::Integer)s.
pub mod partial_eq_integer;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`Natural`](malachite_nz::natural::Natural)s.
pub mod partial_eq_natural;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and primitive floats.
///
/// # eq
/// ```
/// use malachite_base::num::basic::traits::{OneHalf, Zero};
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::from(123) == 123.0);
/// assert!(BigExponentFloat::ONE_HALF == 0.5);
/// assert!(BigExponentFloat::ONE_HALF != -0.5);
/// assert!(BigExponentFloat::ZERO == -0.0);
///
/// assert!(123.0 == BigExponentFloat::from(123));
/// assert!(0.5 == BigExponentFloat::ONE_HALF);
/// assert!(-0.5 != BigExponentFloat::ONE_HALF);
/// assert!(-0.0 == BigExponentFloat::ZERO);
/// ```
pub mod partial_eq_primitive_float;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and primitive integers.
///
/// # eq
/// ```
/// use malachite_base::num::basic::traits::OneHalf;
/// use malachite_float::BigExponentFloat;
///
/// assert!(BigExponentFloat::from(123) == 123u64);
/// assert!(BigExponentFloat::ONE_HALF != 1u64);
///
/// assert!(BigExponentFloat::from(123) == 123i64);
/// assert!(BigExponentFloat::from(-123) == -123i64);
/// assert!(BigExponentFloat::ONE_HALF != -1i64);
///
/// assert!(123u64 == BigExponentFloat::from(123));
/// assert!(1u64 != BigExponentFloat::ONE_HALF);
///
/// assert!(123i64 == BigExponentFloat::from(123));
/// assert!(-123i64 == BigExponentFloat::from(-123));
/// assert!(-1i64 != BigExponentFloat::ONE_HALF);
/// ```
pub mod partial_eq_primitive_int;
/// Equality of [`BigExponentFloat`](crate::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod partial_eq_rational;
