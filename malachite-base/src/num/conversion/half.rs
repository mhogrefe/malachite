// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{HasHalf, JoinHalves, SplitInHalf, WrappingFrom};

#[inline]
fn join_halves<T: From<H> + PrimitiveUnsigned, H: PrimitiveUnsigned>(upper: H, lower: H) -> T {
    T::from(upper) << H::WIDTH | T::from(lower)
}

#[inline]
fn upper_half<T: PrimitiveUnsigned, H: PrimitiveUnsigned + WrappingFrom<T>>(x: &T) -> H {
    H::wrapping_from(*x >> H::WIDTH)
}

macro_rules! impl_half_traits {
    ($t:ident, $ht: ident) => {
        impl HasHalf for $t {
            /// The primitive integer type whose width is half of `Self`'s.
            type Half = $ht;
        }

        impl JoinHalves for $t {
            /// Joins two unsigned integers to form an unsigned integer with twice the width.
            ///
            /// Let $W$ be the width of `Self` (the output type).
            ///
            /// $f(x, y) = 2^{W/2} x + y$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::half#join_halves).
            #[inline]
            fn join_halves(upper: Self::Half, lower: Self::Half) -> Self {
                join_halves(upper, lower)
            }
        }

        impl SplitInHalf for $t {
            /// Extracts the lower, or least significant, half of an unsigned integer.
            ///
            /// Let $W$ be the width of `Self` (the input type).
            ///
            /// $f(n) = m$, where $m < 2^{W/2}$ and $n + 2^{W/2} k = m$ for some $k \in \Z$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::half#lower_half).
            #[inline]
            fn lower_half(&self) -> Self::Half {
                $ht::wrapping_from(*self)
            }

            /// Extracts the upper, or most-significant, half of an unsigned integer.
            ///
            /// Let $W$ be the width of `Self` (the input type).
            ///
            /// $f(n) = \lfloor \frac{n}{2^{W/2}} \rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::half#upper_half).
            #[inline]
            fn upper_half(&self) -> Self::Half {
                upper_half(self)
            }
        }
    };
}
impl_half_traits!(u16, u8);
impl_half_traits!(u32, u16);
impl_half_traits!(u64, u32);
impl_half_traits!(u128, u64);

#[inline]
fn wide_lower_half<T: PrimitiveUnsigned>(x: T) -> T {
    x.mod_power_of_2(T::WIDTH >> 1)
}

#[inline]
pub(crate) fn wide_upper_half<T: PrimitiveUnsigned>(x: T) -> T {
    x >> (T::WIDTH >> 1)
}

#[inline]
pub(crate) fn wide_split_in_half<T: PrimitiveUnsigned>(x: T) -> (T, T) {
    (wide_upper_half(x), wide_lower_half(x))
}

#[inline]
pub(crate) fn wide_join_halves<T: PrimitiveUnsigned>(hi: T, lo: T) -> T {
    (hi << (T::WIDTH >> 1)) | lo
}
