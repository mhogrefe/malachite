// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Traits for taking the base-$b$ logarithm of a number.
///
/// The traits are [`FloorLogBase`](traits::FloorLogBase),
/// [`CeilingLogBase`](traits::CeilingLogBase), and [`CheckedLogBase`](traits::CheckedLogBase).
///
/// # floor_log_base
/// ```
/// use malachite_base::num::exponential::traits::FloorLogBase;
///
/// assert_eq!(1u8.floor_log_base(5), 0);
/// assert_eq!(125u8.floor_log_base(5), 3);
/// assert_eq!(99u64.floor_log_base(10), 1);
/// assert_eq!(100u64.floor_log_base(10), 2);
/// assert_eq!(101u64.floor_log_base(10), 2);
/// ```
///
/// # ceiling_log_base
/// ```
/// use malachite_base::num::exponential::traits::CeilingLogBase;
///
/// assert_eq!(1u8.ceiling_log_base(5), 0);
/// assert_eq!(125u8.ceiling_log_base(5), 3);
/// assert_eq!(99u64.ceiling_log_base(10), 2);
/// assert_eq!(100u64.ceiling_log_base(10), 2);
/// assert_eq!(101u64.ceiling_log_base(10), 3);
/// ```
///
/// # checked_log_base
/// ```
/// use malachite_base::num::exponential::traits::CheckedLogBase;
///
/// assert_eq!(1u8.checked_log_base(5), Some(0));
/// assert_eq!(125u8.checked_log_base(5), Some(3));
/// assert_eq!(99u64.checked_log_base(10), None);
/// assert_eq!(100u64.checked_log_base(10), Some(2));
/// assert_eq!(101u64.checked_log_base(10), None);
/// ```
pub mod log_base;
/// Traits for taking the base-2 logarithm of a number.
///
/// The traits are [`FloorLogBase2`](traits::FloorLogBase2),
/// [`CeilingLogBase2`](traits::CeilingLogBase2), and [`CheckedLogBase2`](traits::CheckedLogBase2).
///
/// # floor_log_base_2
/// ```
/// use malachite_base::num::exponential::traits::FloorLogBase2;
///
/// assert_eq!(1u8.floor_log_base_2(), 0);
/// assert_eq!(100u64.floor_log_base_2(), 6);
///
/// assert_eq!(1.0f32.floor_log_base_2(), 0);
/// assert_eq!(100.0f32.floor_log_base_2(), 6);
/// assert_eq!(0.1f32.floor_log_base_2(), -4);
/// ```
///
/// # ceiling_log_base_2
/// ```
/// use malachite_base::num::exponential::traits::CeilingLogBase2;
///
/// assert_eq!(1u8.ceiling_log_base_2(), 0);
/// assert_eq!(100u64.ceiling_log_base_2(), 7);
///
/// assert_eq!(1.0f32.ceiling_log_base_2(), 0);
/// assert_eq!(100.0f32.ceiling_log_base_2(), 7);
/// assert_eq!(0.1f32.ceiling_log_base_2(), -3);
/// ```
///
/// # checked_log_base_2
/// ```
/// use malachite_base::num::exponential::traits::CheckedLogBase2;
///
/// assert_eq!(1u8.checked_log_base_2(), Some(0));
/// assert_eq!(100u64.checked_log_base_2(), None);
/// assert_eq!(128u64.checked_log_base_2(), Some(7));
///
/// assert_eq!(1.0f32.checked_log_base_2(), Some(0));
/// assert_eq!(100.0f32.checked_log_base_2(), None);
/// assert_eq!(128.0f32.checked_log_base_2(), Some(7));
/// assert_eq!(0.1f32.checked_log_base_2(), None);
/// assert_eq!(0.0625f32.checked_log_base_2(), Some(-4));
/// ```
pub mod log_base_2;
/// Traits for taking the base-$2^k$ logarithm of a number.
///
/// The traits are [`FloorLogBasePowerOf2`](traits::FloorLogBasePowerOf2),
/// [`CeilingLogBasePowerOf2`](traits::CeilingLogBasePowerOf2), and
/// [`CheckedLogBasePowerOf2`](traits::CheckedLogBasePowerOf2).
///
/// # floor_log_base_power_of_2
/// ```
/// use malachite_base::num::exponential::traits::FloorLogBasePowerOf2;
///
/// assert_eq!(1u8.floor_log_base_power_of_2(4), 0);
/// assert_eq!(100u64.floor_log_base_power_of_2(2), 3);
///
/// assert_eq!(0.1f32.floor_log_base_power_of_2(2), -2);
/// ```
///
/// # ceiling_log_base_power_of_2
/// ```
/// use malachite_base::num::exponential::traits::CeilingLogBasePowerOf2;
///
/// assert_eq!(1u8.ceiling_log_base_power_of_2(4), 0);
/// assert_eq!(100u64.ceiling_log_base_power_of_2(2), 4);
///
/// assert_eq!(0.1f32.ceiling_log_base_power_of_2(2), -1);
/// ```
///
/// # checked_log_base_power_of_2
/// ```
/// use malachite_base::num::exponential::traits::CheckedLogBasePowerOf2;
///
/// assert_eq!(1u8.checked_log_base_power_of_2(4), Some(0));
/// assert_eq!(100u64.checked_log_base_power_of_2(4), None);
/// assert_eq!(256u64.checked_log_base_power_of_2(4), Some(2));
///
/// assert_eq!(0.1f32.checked_log_base_power_of_2(2), None);
/// assert_eq!(0.0625f32.checked_log_base_power_of_2(2), Some(-2));
/// ```
pub mod log_base_power_of_2;
/// Various traits for exponential and logarithmic functions.
pub mod traits;
