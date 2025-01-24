// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;

impl Natural {
    /// Returns the number of limbs of a [`Natural`].
    ///
    /// Zero has 0 limbs.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::integers::PrimitiveInt;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::platform::Limb;
    ///
    /// if Limb::WIDTH == u32::WIDTH {
    ///     assert_eq!(Natural::ZERO.limb_count(), 0);
    ///     assert_eq!(Natural::from(123u32).limb_count(), 1);
    ///     assert_eq!(Natural::from(10u32).pow(12).limb_count(), 2);
    /// }
    /// ```
    pub fn limb_count(&self) -> u64 {
        match *self {
            Natural::ZERO => 0,
            Natural(Small(_)) => 1,
            Natural(Large(ref limbs)) => u64::wrapping_from(limbs.len()),
        }
    }
}
