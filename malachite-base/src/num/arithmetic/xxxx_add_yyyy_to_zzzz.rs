// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 1991, 1992, 1993, 1994, 1996, 1997, 1999, 2000, 2001, 2002, 2003, 2004, 2005
//      Free Software Foundation, Inc.
//
//      Copyright © 2009, 2015, 2016 William Hart
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::XXXXAddYYYYToZZZZ;
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

#[allow(clippy::too_many_arguments)]
fn xxxx_add_yyyy_to_zzzz<T: PrimitiveUnsigned>(
    x_3: T,
    x_2: T,
    x_1: T,
    x_0: T,
    y_3: T,
    y_2: T,
    y_1: T,
    y_0: T,
) -> (T, T, T, T) {
    let (z_0, carry_1) = x_0.overflowing_add(y_0);
    let (mut z_1, mut carry_2) = x_1.overflowing_add(y_1);
    if carry_1 {
        carry_2 |= z_1.overflowing_add_assign(T::ONE);
    }
    let (mut z_2, mut carry_3) = x_2.overflowing_add(y_2);
    if carry_2 {
        carry_3 |= z_2.overflowing_add_assign(T::ONE);
    }
    let mut z_3 = x_3.wrapping_add(y_3);
    if carry_3 {
        z_3.wrapping_add_assign(T::ONE);
    }
    (z_3, z_2, z_1, z_0)
}

macro_rules! impl_xxxx_add_yyyy_to_zzzz {
    ($t:ident) => {
        impl XXXXAddYYYYToZZZZ for $t {
            /// Adds two numbers, each composed of four `Self` values, returning the sum as a
            /// quadruple of `Self` values.
            ///
            /// The more significant value always comes first. Addition is wrapping, and overflow is
            /// not indicated.
            ///
            /// $$
            /// f(x_3, x_2, x_1, x_0, y_2, y_2, y_1, y_0) = (z_3, z_2, z_1, z_0),
            /// $$
            /// where $W$ is `Self::WIDTH`,
            ///
            /// $x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0, z_3, z_2, z_1, z_0 < 2^W$, and
            /// $$
            /// (2^{3W}x_3 + 2^{2W}x_2 + 2^Wx_1 + x_0) + (2^{3W}y_3 + 2^{2W}y_2 + 2^Wy_1 + y_0)
            /// \equiv 2^{3W}z_3 + 2^{2W}z_2 + 2^Wz_1 + z_0 \mod 2^{4W}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::xxxx_add_yyyy_to_zzzz#xxxx_add_yyyy_to_zzzz).
            ///
            /// This is equivalent to `add_ssssaaaaaaaa` from `longlong.h`, FLINT 2.7.1, where `(s3,
            /// s2, s1, s0)` is returned.
            #[inline]
            fn xxxx_add_yyyy_to_zzzz(
                x_3: $t,
                x_2: $t,
                x_1: $t,
                x_0: $t,
                y_3: $t,
                y_2: $t,
                y_1: $t,
                y_0: $t,
            ) -> ($t, $t, $t, $t) {
                xxxx_add_yyyy_to_zzzz::<$t>(x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0)
            }
        }
    };
}

impl_xxxx_add_yyyy_to_zzzz!(u8);
impl_xxxx_add_yyyy_to_zzzz!(u16);
impl_xxxx_add_yyyy_to_zzzz!(u32);
impl_xxxx_add_yyyy_to_zzzz!(u64);
impl_xxxx_add_yyyy_to_zzzz!(u128);

impl XXXXAddYYYYToZZZZ for usize {
    /// Adds two numbers, each composed of four [`usize`] values, returning the sum as a quadruple
    /// of [`usize`] values.
    ///
    /// The more significant value always comes first. Addition is wrapping, and overflow is not
    /// indicated.
    ///
    /// $$
    /// f(x_3, x_2, x_1, x_0, y_2, y_2, y_1, y_0) = (z_3, z_2, z_1, z_0),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x_3, x_2, x_1, x_0, y_3, y_2, y_1, y_0, z_3, z_2, z_1, z_0 < 2^W$, and
    /// $$
    /// (2^{3W}x_3 + 2^{2W}x_2 + 2^Wx_1 + x_0) + (2^{3W}y_3 + 2^{2W}y_2 + 2^Wy_1 + y_0)
    /// \equiv 2^{3W}z_3 + 2^{2W}z_2 + 2^Wz_1 + z_0 \mod 2^{4W}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::xxxx_add_yyyy_to_zzzz#xxxx_add_yyyy_to_zzzz).
    ///
    /// This is equivalent to `add_ssssaaaaaaaa` from `longlong.h`, FLINT 2.7.1, where `(s3, s2, s1,
    /// s0)` is returned.
    fn xxxx_add_yyyy_to_zzzz(
        x_3: usize,
        x_2: usize,
        x_1: usize,
        x_0: usize,
        y_3: usize,
        y_2: usize,
        y_1: usize,
        y_0: usize,
    ) -> (usize, usize, usize, usize) {
        if USIZE_IS_U32 {
            let (z_3, z_2, z_1, z_0) = u32::xxxx_add_yyyy_to_zzzz(
                u32::wrapping_from(x_3),
                u32::wrapping_from(x_2),
                u32::wrapping_from(x_1),
                u32::wrapping_from(x_0),
                u32::wrapping_from(y_3),
                u32::wrapping_from(y_2),
                u32::wrapping_from(y_1),
                u32::wrapping_from(y_0),
            );
            (
                usize::wrapping_from(z_3),
                usize::wrapping_from(z_2),
                usize::wrapping_from(z_1),
                usize::wrapping_from(z_0),
            )
        } else {
            let (z_3, z_2, z_1, z_0) = u64::xxxx_add_yyyy_to_zzzz(
                u64::wrapping_from(x_3),
                u64::wrapping_from(x_2),
                u64::wrapping_from(x_1),
                u64::wrapping_from(x_0),
                u64::wrapping_from(y_3),
                u64::wrapping_from(y_2),
                u64::wrapping_from(y_1),
                u64::wrapping_from(y_0),
            );
            (
                usize::wrapping_from(z_3),
                usize::wrapping_from(z_2),
                usize::wrapping_from(z_1),
                usize::wrapping_from(z_0),
            )
        }
    }
}
