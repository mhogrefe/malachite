// Copyright © 2026 Mikhail Hogrefe
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

use crate::num::arithmetic::traits::XXXAddYYYToZZZ;
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

// A single folded chain per word ((flag_a | flag_b), with the previous flag folded in before
// the next word) lets LLVM fuse the whole sequence into branchless adds/adcs (or subs/sbcs) on
// aarch64 and adc/sbb chains on x86_64; the previous per-word branches did not fuse, and this
// kernel is hot in the fft_small CRT reconstruction.
pub_test! {xxx_add_yyy_to_zzz<T: PrimitiveUnsigned>(
    x_2: T,
    x_1: T,
    x_0: T,
    y_2: T,
    y_1: T,
    y_0: T,
) -> (T, T, T) {
    let (z_0, carry) = x_0.overflowing_add(y_0);
    let (z_1, carry_a) = x_1.overflowing_add(y_1);
    let (z_1, carry_b) = z_1.overflowing_add(if carry { T::ONE } else { T::ZERO });
    let carry = carry_a | carry_b;
    let z_2 = x_2.wrapping_add(y_2).wrapping_add(if carry { T::ONE } else { T::ZERO });
    (z_2, z_1, z_0)
}}

macro_rules! impl_xxx_add_yyy_to_zzz {
    ($t:ident) => {
        impl XXXAddYYYToZZZ for $t {
            /// Adds two numbers, each composed of three `Self` values, returning the sum as a
            /// triple of `Self` values.
            ///
            /// The more significant value always comes first. Addition is wrapping, and overflow is
            /// not indicated.
            ///
            /// $$
            /// f(x_2, x_1, x_0, y_2, y_1, y_0) = (z_2, z_1, z_0),
            /// $$
            /// where $W$ is `Self::WIDTH`,
            ///
            /// $x_2, x_1, x_0, y_2, y_1, y_0, z_2, z_1, z_0 < 2^W$, and
            /// $$
            /// (2^{2W}x_2 + 2^Wx_1 + x_0) + (2^{2W}y_2 + 2^Wy_1 + y_0)
            /// \equiv 2^{2W}z_2 + 2^Wz_1 + z_0 \mod 2^{3W}.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::xxx_add_yyy_to_zzz#xxx_add_yyy_to_zzz).
            ///
            /// This is equivalent to `add_sssaaaaaa` from `longlong.h`, FLINT 2.7.1, where `(sh,
            /// sm, sl)` is returned.
            #[inline]
            fn xxx_add_yyy_to_zzz(
                x_2: $t,
                x_1: $t,
                x_0: $t,
                y_2: $t,
                y_1: $t,
                y_0: $t,
            ) -> ($t, $t, $t) {
                xxx_add_yyy_to_zzz::<$t>(x_2, x_1, x_0, y_2, y_1, y_0)
            }
        }
    };
}

impl_xxx_add_yyy_to_zzz!(u8);
impl_xxx_add_yyy_to_zzz!(u16);
impl_xxx_add_yyy_to_zzz!(u32);
impl_xxx_add_yyy_to_zzz!(u64);
impl_xxx_add_yyy_to_zzz!(u128);

impl XXXAddYYYToZZZ for usize {
    /// Adds two numbers, each composed of three [`usize`] values, returning the sum as a triple of
    /// [`usize`] values.
    ///
    /// The more significant value always comes first. Addition is wrapping, and overflow is not
    /// indicated.
    ///
    /// $$
    /// f(x_2, x_1, x_0, y_2, y_1, y_0) = (z_2, z_1, z_0),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x_2, x_1, x_0, y_2, y_1, y_0, z_2, z_1, z_0 < 2^W$, and
    /// $$
    /// (2^{2W}x_2 + 2^Wx_1 + x_0) + (2^{2W}y_2 + 2^Wy_1 + y_0)
    /// \equiv 2^{2W}z_2 + 2^Wz_1 + z_0 \mod 2^{3W}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::xxx_add_yyy_to_zzz#xxx_add_yyy_to_zzz).
    ///
    /// This is equivalent to `add_sssaaaaaa` from `longlong.h`, FLINT 2.7.1, where `(sh, sm, sl)`
    /// is returned.
    fn xxx_add_yyy_to_zzz(
        x_2: Self,
        x_1: Self,
        x_0: Self,
        y_2: Self,
        y_1: Self,
        y_0: Self,
    ) -> (Self, Self, Self) {
        if USIZE_IS_U32 {
            let (z_2, z_1, z_0) = u32::xxx_add_yyy_to_zzz(
                u32::wrapping_from(x_2),
                u32::wrapping_from(x_1),
                u32::wrapping_from(x_0),
                u32::wrapping_from(y_2),
                u32::wrapping_from(y_1),
                u32::wrapping_from(y_0),
            );
            (
                Self::wrapping_from(z_2),
                Self::wrapping_from(z_1),
                Self::wrapping_from(z_0),
            )
        } else {
            let (z_2, z_1, z_0) = u64::xxx_add_yyy_to_zzz(
                u64::wrapping_from(x_2),
                u64::wrapping_from(x_1),
                u64::wrapping_from(x_0),
                u64::wrapping_from(y_2),
                u64::wrapping_from(y_1),
                u64::wrapping_from(y_0),
            );
            (
                Self::wrapping_from(z_2),
                Self::wrapping_from(z_1),
                Self::wrapping_from(z_0),
            )
        }
    }
}
