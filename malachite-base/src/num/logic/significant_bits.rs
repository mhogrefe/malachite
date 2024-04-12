// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::UnsignedAbs;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::{LeadingZeros, SignificantBits};

fn significant_bits_unsigned<T: PrimitiveUnsigned>(x: T) -> u64 {
    T::WIDTH - LeadingZeros::leading_zeros(x)
}

macro_rules! impl_significant_bits_unsigned {
    ($t:ident) => {
        impl SignificantBits for $t {
            /// Returns the number of significant bits of an unsigned primitive integer.
            ///
            /// This is the integer's width minus the number of leading zeros.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     0 & \text{if} \\quad n = 0, \\\\
            ///     \lfloor \log_2 n \rfloor + 1 & \text{if} \\quad n > 0.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::significant_bits#significant_bits).
            #[inline]
            fn significant_bits(self) -> u64 {
                significant_bits_unsigned(self)
            }
        }
    };
}
apply_to_unsigneds!(impl_significant_bits_unsigned);

fn significant_bits_signed<U: PrimitiveUnsigned, S: PrimitiveSigned + UnsignedAbs<Output = U>>(
    x: S,
) -> u64 {
    x.unsigned_abs().significant_bits()
}

macro_rules! impl_significant_bits_signed {
    ($u:ident, $s:ident) => {
        /// Returns the number of significant bits of a signed primitive integer.
        ///
        /// This is the integer's width minus the number of leading zeros of its absolute value.
        ///
        /// $$
        /// f(n) = \\begin{cases}
        ///     0 & \text{if} \\quad n = 0, \\\\
        ///     \lfloor \log_2 |n| \rfloor + 1 & \text{if} \\quad n \neq 0.
        /// \\end{cases}
        /// $$
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See [here](super::significant_bits#significant_bits).
        impl SignificantBits for $s {
            #[inline]
            fn significant_bits(self) -> u64 {
                significant_bits_signed(self)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_significant_bits_signed);
