// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`IsPower`](malachite_base::num::factorization::traits::IsPower) and
/// [`ExpressAsPower`](malachite_base::num::factorization::traits::ExpressAsPower), traits for
/// testing if a number is a perfect power and, if it is, expressing it as such.
pub mod is_power;
/// Implementations of [`RemovePower`](malachite_base::num::factorization::traits::RemovePower) and
/// [`RemovePowerAssign`](malachite_base::num::factorization::traits::RemovePowerAssign), traits for
/// dividing out the largest power of a factor.
///
/// # remove_power
/// ```
/// use malachite_base::num::factorization::traits::RemovePower;
/// use malachite_nz::integer::Integer;
///
/// let (q, k) = Integer::from(-12).remove_power(Integer::from(2));
/// assert_eq!(q, -3);
/// assert_eq!(k, 2);
///
/// // a negative factor raised to an odd power flips the sign
/// let (q, k) = Integer::from(-8).remove_power(Integer::from(-2));
/// assert_eq!(q, 1);
/// assert_eq!(k, 3);
/// ```
///
/// # remove_power_assign
/// ```
/// use malachite_base::num::factorization::traits::RemovePowerAssign;
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::from(-12);
/// assert_eq!(x.remove_power_assign(Integer::from(2)), 2);
/// assert_eq!(x, -3);
/// ```
pub mod remove_power;
