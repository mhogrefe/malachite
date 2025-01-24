// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::LowMask;

fn low_mask_unsigned<T: PrimitiveUnsigned>(bits: u64) -> T {
    assert!(bits <= T::WIDTH);
    if bits == T::WIDTH {
        T::MAX
    } else {
        T::power_of_2(bits) - T::ONE
    }
}

macro_rules! impl_low_mask_unsigned {
    ($t:ident) => {
        impl LowMask for $t {
            /// Returns a number whose least significant $b$ bits are `true` and whose other bits
            /// are `false`.
            ///
            /// $f(b) = 2^b - 1$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `bits` is greater than the width of of the type.
            ///
            /// # Examples
            /// See [here](super::low_mask#low_mask).
            #[inline]
            fn low_mask(bits: u64) -> $t {
                low_mask_unsigned(bits)
            }
        }
    };
}
apply_to_unsigneds!(impl_low_mask_unsigned);

fn low_mask_signed<T: PrimitiveSigned>(bits: u64) -> T {
    assert!(bits <= T::WIDTH);
    if bits == T::WIDTH {
        T::NEGATIVE_ONE
    } else if bits == T::WIDTH - 1 {
        T::MAX
    } else {
        T::power_of_2(bits) - T::ONE
    }
}

macro_rules! impl_low_mask_signed {
    ($t:ident) => {
        impl LowMask for $t {
            /// Returns a number whose least significant $b$ bits are `true` and whose other bits
            /// are `false`.
            ///
            /// $$
            /// f(b) = \\begin{cases}
            ///     2^b - 1 & \text{if} \\quad 0 \leq n < W, \\\\
            ///     -1 & \text{if} \\quad n = W,
            /// \\end{cases}
            /// $$
            /// where $W$ is the width of the type.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `bits` is greater than the width of the type.
            ///
            /// # Examples
            /// See [here](super::low_mask#low_mask).
            #[inline]
            fn low_mask(bits: u64) -> $t {
                low_mask_signed(bits)
            }
        }
    };
}
apply_to_signeds!(impl_low_mask_signed);
