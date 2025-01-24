// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::malachite_base::num::arithmetic::traits::{ShlRound, ShlRoundAssign};
use crate::Float;
use core::ops::{Shl, ShlAssign};
use malachite_base::rounding_modes::RoundingMode::*;

macro_rules! impl_shl {
    ($t:ident) => {
        impl Shl<$t> for Float {
            type Output = Float;

            /// Left-shifts a [`Float`] (multiplies it by a power of 2), taking it by value.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// output has the same precision.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// - If $f(x,k)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
            /// - If $f(x,k)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
            /// - If $0<f(x,k)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
            /// - If $2^{-2^{30}-1}<f(x,k)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
            /// - If $-2^{-2^{30}-1}\leq f(x,k)<0$, $-0.0$ is returned instead.
            /// - If $-2^{-2^{30}}<f(x,k)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Float {
                self.shl_round(bits, Nearest).0
            }
        }

        impl Shl<$t> for &Float {
            type Output = Float;

            /// Left-shifts a [`Float`] (multiplies it by a power of 2), taking it by value.
            ///
            /// `NaN`, infinities, and zeros are unchanged. If the [`Float`] has a precision, the
            /// output has the same precision.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// - If $f(x,k)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
            /// - If $f(x,k)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
            /// - If $0<f(x,k)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
            /// - If $2^{-2^{30}-1}<f(x,k)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
            /// - If $-2^{-2^{30}-1}\leq f(x,k)<0$, $-0.0$ is returned instead.
            /// - If $-2^{-2^{30}}<f(x,k)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Float {
                self.shl_round(bits, Nearest).0
            }
        }

        impl ShlAssign<$t> for Float {
            /// Left-shifts a [`Float`] (multiplies it by a power of 2), in place. If the [`Float`]
            /// has a precision, the precision is unchanged.
            ///
            /// `NaN`, infinities, and zeros are unchanged.
            ///
            /// $$
            /// x \gets x2^k.
            /// $$
            ///
            /// - If $f(x,k)\geq 2^{2^{30}-1}$, $\infty$ is assigned instead.
            /// - If $f(x,k)\leq -2^{2^{30}-1}$, $-\infty$ is assigned instead.
            /// - If $0<f(x,k)\leq2^{-2^{30}-1}$, $0.0$ is assigned instead.
            /// - If $2^{-2^{30}-1}<f(x,k)<2^{-2^{30}}$, $2^{-2^{30}}$ is assigned instead.
            /// - If $-2^{-2^{30}-1}\leq f(x,k)<0$, $-0.0$ is assigned instead.
            /// - If $-2^{-2^{30}}<f(x,k)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is assigned instead.
            ///
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                self.shl_round_assign(bits, Nearest);
            }
        }
    };
}
apply_to_primitive_ints!(impl_shl);
