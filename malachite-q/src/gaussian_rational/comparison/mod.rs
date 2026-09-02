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
/// An implementation of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational), comparing absolute values
/// (distances from the origin) for equality.
pub mod eq_abs;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) for equality.
pub mod eq_abs_gaussian_integer;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`GaussianRational`](crate::gaussian_rational::GaussianRational) and an
/// [`Integer`](malachite_nz::integer::Integer) for equality.
pub mod eq_abs_integer;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// [`Natural`](malachite_nz::natural::Natural) for equality.
pub mod eq_abs_natural;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// primitive float for equality.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// // |3/10+2i/5| = 1/2
/// let x = GaussianRational::from_str("3/10+2i/5").unwrap();
/// assert!(x.eq_abs(&-0.5f32));
/// assert_eq!(x.eq_abs(&0.4f32), false);
/// assert_eq!(x.eq_abs(&f32::NAN), false);
///
/// assert!((-0.5f32).eq_abs(&x));
/// assert_eq!(0.4f32.eq_abs(&x), false);
/// ```
pub mod eq_abs_primitive_float;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// primitive integer for equality.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// // |3/5+4i/5| = 1
/// let x = GaussianRational::from_str("3/5+4i/5").unwrap();
/// assert!(x.eq_abs(&1u32));
/// assert_eq!(x.eq_abs(&2u32), false);
/// assert!(x.eq_abs(&-1i32));
///
/// assert!(1u32.eq_abs(&x));
/// assert!((-1i32).eq_abs(&x));
/// assert_eq!(2u32.eq_abs(&x), false);
/// ```
pub mod eq_abs_primitive_int;
/// Implementations of [`EqAbs`](malachite_base::num::comparison::traits::EqAbs) for comparing the
/// absolute values of a [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// [`Rational`](crate::Rational) for equality.
pub mod eq_abs_rational;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger).
pub mod partial_cmp_abs_gaussian_integer;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) and an
/// [`Integer`](malachite_nz::integer::Integer).
pub mod partial_cmp_abs_integer;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// [`Natural`](malachite_nz::natural::Natural).
pub mod partial_cmp_abs_natural;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a primitive float.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// // |3/10+2i/5| = 1/2
/// let x = GaussianRational::from_str("3/10+2i/5").unwrap();
/// assert!(x.gt_abs(&-0.4f32));
/// assert!(x.lt_abs(&0.75f32));
/// assert!(x.lt_abs(&f32::NEGATIVE_INFINITY));
/// assert_eq!(x.partial_cmp_abs(&f32::NAN), None);
///
/// assert!((-0.4f32).lt_abs(&x));
/// assert!(0.75f32.gt_abs(&x));
/// assert!(f32::NEGATIVE_INFINITY.gt_abs(&x));
/// ```
pub mod partial_cmp_abs_primitive_float;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a primitive integer.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_q::gaussian_rational::GaussianRational;
/// use std::str::FromStr;
///
/// // |3/5+4i/5| = 1
/// let x = GaussianRational::from_str("3/5+4i/5").unwrap();
/// assert!(x.gt_abs(&0u32));
/// assert!(x.ge_abs(&1u32));
/// assert!(x.le_abs(&1u32));
/// assert!(x.lt_abs(&2u32));
/// assert!(x.le_abs(&-1i32));
/// assert!(x.lt_abs(&-2i32));
///
/// assert!(0u32.lt_abs(&x));
/// assert!(2u32.gt_abs(&x));
/// assert!((-1i32).ge_abs(&x));
/// assert!((-2i32).gt_abs(&x));
/// ```
pub mod partial_cmp_abs_primitive_int;
/// Implementations of [`PartialOrdAbs`](malachite_base::num::comparison::traits::PartialOrdAbs) for
/// comparing the absolute values of a
/// [`GaussianRational`](crate::gaussian_rational::GaussianRational) and a
/// [`Rational`](crate::Rational).
pub mod partial_cmp_abs_rational;
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
