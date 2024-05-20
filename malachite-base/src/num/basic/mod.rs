// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// The [`PrimitiveFloat`](floats::PrimitiveFloat) trait.
pub mod floats;
/// The [`PrimitiveInt`](integers::PrimitiveInt) trait.
///
/// ```
/// use malachite_base::num::basic::integers::PrimitiveInt;
/// use malachite_base::num::basic::traits::{One, Two, Zero};
///
/// assert_eq!(u32::WIDTH, 32);
/// assert_eq!(u32::LOG_WIDTH, 5);
/// assert_eq!(u32::WIDTH_MASK, 0x1f);
///
/// assert_eq!(u32::ZERO, 0);
/// assert_eq!(u32::ONE, 1);
/// assert_eq!(i16::TWO, 2);
///
/// assert_eq!(u32::MAX, 0xffffffff);
/// assert_eq!(u32::MIN, 0);
/// assert_eq!(i32::MAX, 0x7fffffff);
/// assert_eq!(i32::MIN, -0x80000000);
/// ```
pub mod integers;
/// The [`PrimitiveSigned`](signeds::PrimitiveSigned) trait.
///
/// ```
/// use malachite_base::num::basic::traits::NegativeOne;
///
/// assert_eq!(i16::NEGATIVE_ONE, -1);
/// ```
pub mod signeds;
/// Traits for constants.
pub mod traits;
/// The [`PrimitiveUnsigned`](unsigneds::PrimitiveUnsigned) trait.
pub mod unsigneds;
