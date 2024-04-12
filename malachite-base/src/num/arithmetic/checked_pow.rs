// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedPow, Parity};

macro_rules! impl_checked_pow_unsigned {
    ($t:ident) => {
        impl CheckedPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_pow` functions in the standard library, for
            /// example [this one](u32::checked_pow).
            #[inline]
            fn checked_pow(self, exp: u64) -> Option<$t> {
                if exp == 0 {
                    Some(1)
                } else if self < 2 {
                    Some(self)
                } else {
                    self.checked_pow(u32::try_from(exp).ok()?)
                }
            }
        }
    };
}
apply_to_unsigneds!(impl_checked_pow_unsigned);

macro_rules! impl_checked_pow_signed {
    ($t:ident) => {
        impl CheckedPow<u64> for $t {
            type Output = $t;

            /// This is a wrapper over the `checked_pow` functions in the standard library, for
            /// example [this one](i32::checked_pow).
            #[inline]
            fn checked_pow(self, exp: u64) -> Option<$t> {
                if exp == 0 {
                    Some(1)
                } else if self == 0 || self == 1 {
                    Some(self)
                } else if self == -1 {
                    Some(if exp.even() { 1 } else { -1 })
                } else {
                    self.checked_pow(u32::try_from(exp).ok()?)
                }
            }
        }
    };
}
apply_to_signeds!(impl_checked_pow_signed);
