// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::BitAccess;

fn get_bit_unsigned<T: PrimitiveUnsigned>(x: &T, index: u64) -> bool {
    index < T::WIDTH && (*x >> index).odd()
}

fn set_bit_unsigned<T: PrimitiveUnsigned>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x |= T::power_of_2(index);
    } else {
        panic!(
            "Cannot set bit {} in non-negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

fn clear_bit_unsigned<T: PrimitiveUnsigned>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x &= !T::power_of_2(index);
    }
}

macro_rules! impl_bit_access_unsigned {
    ($t:ident) => {
        impl BitAccess for $t {
            /// Determines whether the $i$th bit of an unsigned primitive integer, or the
            /// coefficient of $2^i$ in its binary expansion, is 0 or 1.
            ///
            /// `false` means 0 and `true` means 1. Getting bits beyond the type's width is allowed;
            /// those bits are false.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^\infty 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the
            /// rest are 0. Then $f(n, j) = (b_j = 1)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_access#get_bit).
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                get_bit_unsigned(self, index)
            }

            /// Sets the $i$th bit of an unsigned primitive integer, or the coefficient of $2^i$ in
            /// its binary expansion, to 1.
            ///
            /// Setting bits beyond the type's width is disallowed.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$, and $W$ is the width of the type. Then
            /// $$
            /// n \gets \\begin{cases}
            ///     n + 2^j & \text{if} \\quad b_j = 0, \\\\
            ///     n & \text{otherwise},
            /// \\end{cases}
            /// $$
            /// where $j < W$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $i \geq W$, where $i$ is `index` and $W$ is `$t::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::bit_access#set_bit).
            #[inline]
            fn set_bit(&mut self, index: u64) {
                set_bit_unsigned(self, index)
            }

            /// Sets the $i$th bit of an unsigned primitive integer, or the coefficient of $2^i$ in
            /// its binary expansion, to 0.
            ///
            /// Clearing bits beyond the type's width is allowed; since those bits are already
            /// `false`, clearing them does nothing.
            ///
            /// Let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$, and $W$ is the width of the type. Then
            /// $$
            /// n \gets \\begin{cases}
            ///     n - 2^j & \text{if} \\quad b_j = 1, \\\\
            ///     n & \text{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_access#clear_bit).
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                clear_bit_unsigned(self, index)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_access_unsigned);

fn get_bit_signed<T: PrimitiveSigned>(x: &T, index: u64) -> bool {
    if index < T::WIDTH {
        (*x >> index).odd()
    } else {
        *x < T::ZERO
    }
}

fn set_bit_signed<T: PrimitiveSigned>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x |= T::ONE << index;
    } else if *x >= T::ZERO {
        panic!(
            "Cannot set bit {} in non-negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

fn clear_bit_signed<T: PrimitiveSigned>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x &= !(T::ONE << index);
    } else if *x < T::ZERO {
        panic!(
            "Cannot clear bit {} in negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

macro_rules! impl_bit_access_signed {
    ($t:ident) => {
        impl BitAccess for $t {
            /// Determines whether the $i$th bit of a signed primitive integer is 0 or 1.
            ///
            /// `false` means 0 and `true` means 1. Getting bits beyond the type's width is allowed;
            /// those bits are `true` if the value is negative, and `false` otherwise.
            ///
            /// If $n \geq 0$, let
            /// $$
            /// n = \sum_{i=0}^\infty 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$; so finitely many of the bits are 1, and the
            /// rest are 0. Then $f(n, i) = (b_i = 1)$.
            ///
            /// If $n < 0$, let
            /// $$
            /// 2^W + n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where
            /// - $W$ is the type's width
            /// - for all $i$, $b_i\in \\{0, 1\\}$, and $b_i = 1$ for $i \geq W$.
            ///
            /// Then $f(n, j) = (b_j = 1)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::bit_access#get_bit).
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                get_bit_signed(self, index)
            }

            /// Sets the $i$th bit of a signed primitive integer to 1.
            ///
            /// Setting bits beyond the type's width is disallowed if the number is non-negative.
            ///
            /// If $n \geq 0$ and $j \neq W - 1$, let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i};
            /// $$
            /// but if $n < 0$ or $j = W - 1$, let
            /// $$
            /// 2^W + n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$, and $W$ is the width of the type. Then
            /// $$
            /// n \gets \\begin{cases}
            ///     n + 2^j & \text{if} \\quad b_j = 0, \\\\
            ///     n & \text{otherwise},
            /// \\end{cases}
            /// $$
            /// where $n < 0$ or $j < W$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $n \geq 0$ and $i \geq W$, where $n$ is `self`, $i$ is `index` and $W$ is
            /// the width of the type.
            ///
            /// # Examples
            /// See [here](super::bit_access#set_bit).
            #[inline]
            fn set_bit(&mut self, index: u64) {
                set_bit_signed(self, index)
            }

            /// Sets the $i$th bit of a signed primitive integer to 0.
            ///
            /// Clearing bits beyond the type's width is disallowed if the number is negative.
            ///
            /// If $n \geq 0$ or $j = W - 1$, let
            /// $$
            /// n = \sum_{i=0}^{W-1} 2^{b_i};
            /// $$
            /// but if $n < 0$ or $j = W - 1$, let
            /// $$
            /// 2^W + n = \sum_{i=0}^{W-1} 2^{b_i},
            /// $$
            /// where for all $i$, $b_i\in \\{0, 1\\}$ and $W$ is the width of the type. Then
            /// $$
            /// n \gets \\begin{cases}
            ///     n - 2^j & \text{if} \\quad b_j = 1, \\\\
            ///     n & \text{otherwise},
            /// \\end{cases}
            /// $$
            /// where $n \geq 0$ or $j < W$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $n < 0$ and $i \geq W$, where $n$ is `self`, $i$ is `index` and $W$ is the
            /// width of the type.
            ///
            /// # Examples
            /// See [here](super::bit_access#clear_bit).
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                clear_bit_signed(self, index)
            }
        }
    };
}
apply_to_signeds!(impl_bit_access_signed);
