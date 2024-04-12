// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedLcm, Lcm, LcmAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

#[inline]
fn lcm<T: PrimitiveUnsigned>(x: T, y: T) -> T {
    checked_lcm(x, y).unwrap()
}

fn checked_lcm<T: PrimitiveUnsigned>(x: T, y: T) -> Option<T> {
    if x == T::ZERO && y == T::ZERO {
        Some(T::ZERO)
    } else {
        (x / x.gcd(y)).checked_mul(y)
    }
}

macro_rules! impl_lcm {
    ($t:ident) => {
        impl Lcm<$t> for $t {
            type Output = $t;

            /// Computes the LCM (least common multiple) of two numbers.
            ///
            /// $$
            /// f(x, y) = \operatorname{lcm}(x, y).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Panics
            /// Panics if the result is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::lcm#lcm).
            #[inline]
            fn lcm(self, other: $t) -> $t {
                lcm(self, other)
            }
        }

        impl LcmAssign<$t> for $t {
            /// Replaces a number with the LCM (least common multiple) of it and another number.
            ///
            /// $$
            /// x \gets \operatorname{lcm}(x, y).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Panics
            /// Panics if the result is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::lcm#lcm_assign).
            #[inline]
            fn lcm_assign(&mut self, other: $t) {
                *self = lcm(*self, other);
            }
        }

        impl CheckedLcm<$t> for $t {
            type Output = $t;

            /// Computes the LCM (least common multiple) of two numbers, returning `None` if the
            /// result is too large to represent.
            ///
            /// $$
            /// f(x, y) = \\begin{cases}
            ///     \operatorname{Some}(\operatorname{lcm}(x, y)) &
            ///         \text{if} \\quad \operatorname{lcm}(x, y) < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad \operatorname{lcm}(x, y) \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n^2)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), other.significant_bits())`.
            ///
            /// # Examples
            /// See [here](super::lcm#checked_lcm).
            #[inline]
            fn checked_lcm(self, other: $t) -> Option<$t> {
                checked_lcm(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_lcm);
