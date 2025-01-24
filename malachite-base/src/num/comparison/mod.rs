// Copyright © 2025 Mikhail Hogrefe
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
/// assert_eq!(123u32.cmp_abs(&456), Less);
/// assert_eq!(123u32.cmp_abs(&123), Equal);
/// assert_eq!(123i32.cmp_abs(&-456), Less);
/// assert_eq!(123i32.cmp_abs(&-123), Equal);
/// ```
pub mod cmp_abs;
/// [`EqAbs`](`traits::EqAbs`), a trait for comparing the absolute values of numbers by equality.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
///
/// assert_eq!(123u32.eq_abs(&456), false);
/// assert_eq!(123u32.eq_abs(&123), true);
/// assert_eq!(123i32.eq_abs(&-456), false);
/// assert_eq!(123i32.eq_abs(&-123), true);
/// ```
pub mod eq_abs;
/// Various traits for comparing numbers.
pub mod traits;
