// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`PartialOrdAbs`](`traits::PartialOrdAbs`) and [`OrdAbs`](`traits::OrdAbs`), traits for
/// comparing the absolute values of numbers by order.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(123i32.partial_cmp_abs(&-456), Some(Less));
/// assert_eq!(123i32.partial_cmp_abs(&-123), Some(Equal));
/// ```
///
/// # cmp_abs
/// ```
/// use malachite_base::num::comparison::traits::OrdAbs;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(123i32.cmp_abs(&-456), Less);
/// assert_eq!(123i32.cmp_abs(&-123), Equal);
/// ```
pub mod cmp_abs;
/// [`OrdDouble`](traits::OrdDouble) and [`OrdAbsDouble`](traits::OrdAbsDouble), traits for
/// comparing a number with twice another number without computing the doubled value.
///
/// # cmp_double
/// ```
/// use malachite_base::num::comparison::traits::OrdDouble;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(4u32.cmp_double(&2), Equal);
/// assert_eq!(3u32.cmp_double(&2), Less);
/// assert_eq!(5u32.cmp_double(&2), Greater);
/// // the doubling would overflow, but the comparison still works
/// assert_eq!(u32::MAX.cmp_double(&(1 << 31)), Less);
/// ```
///
/// # cmp_abs_double
/// ```
/// use malachite_base::num::comparison::traits::OrdAbsDouble;
/// use std::cmp::Ordering::*;
///
/// assert_eq!((-4i32).cmp_abs_double(&2), Equal);
/// assert_eq!(3i32.cmp_abs_double(&-2), Less);
/// assert_eq!((-5i32).cmp_abs_double(&2), Greater);
/// // the most negative value, whose magnitude is not representable
/// assert_eq!(i32::MIN.cmp_abs_double(&(i32::MIN >> 1)), Equal);
/// ```
pub mod cmp_double;
/// [`EqAbs`](`traits::EqAbs`), a trait for comparing the absolute values of numbers by equality.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
///
/// assert_eq!(123i32.eq_abs(&-456), false);
/// assert_eq!(123i32.eq_abs(&-123), true);
/// assert_eq!(1.0.eq_abs(&-1.0), true);
/// assert_eq!(1.0.eq_abs(&f64::NAN), false);
/// ```
pub mod eq_abs;
/// Various traits for comparing numbers.
pub mod traits;
