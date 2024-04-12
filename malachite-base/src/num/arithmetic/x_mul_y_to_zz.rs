// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-1994, 1996, 1997, 1999-2005, 2007-2009, 2011-2020 Free Software
//      Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::XMulYToZZ;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::half::{wide_join_halves, wide_split_in_half, wide_upper_half};
use crate::num::conversion::traits::{HasHalf, SplitInHalf, WrappingFrom};

fn implicit_x_mul_y_to_zz<T, DT: From<T> + HasHalf<Half = T> + PrimitiveUnsigned + SplitInHalf>(
    x: T,
    y: T,
) -> (T, T) {
    (DT::from(x) * DT::from(y)).split_in_half()
}

pub_test! {explicit_x_mul_y_to_zz<T: PrimitiveUnsigned>(x: T, y: T) -> (T, T) {
    let (x_1, x_0) = wide_split_in_half(x);
    let (y_1, y_0) = wide_split_in_half(y);
    let x_0_y_0 = x_0 * y_0;
    let mut x_0_y_1 = x_0 * y_1;
    let x_1_y_0 = x_1 * y_0;
    let mut x_1_y_1 = x_1 * y_1;
    let (x_0_y_0_1, x_0_y_0_0) = wide_split_in_half(x_0_y_0);
    x_0_y_1.wrapping_add_assign(x_0_y_0_1);
    if x_0_y_1.overflowing_add_assign(x_1_y_0) {
        x_1_y_1.wrapping_add_assign(T::power_of_2(T::WIDTH >> 1));
    }
    let z_1 = x_1_y_1.wrapping_add(wide_upper_half(x_0_y_1));
    let z_0 = wide_join_halves(x_0_y_1, x_0_y_0_0);
    (z_1, z_0)
}}

macro_rules! implicit_x_mul_y_to_zz {
    ($t:ident, $dt:ident) => {
        impl XMulYToZZ for $t {
            /// Multiplies two numbers, returning the product as a pair of `Self` values.
            ///
            /// The more significant value always comes first.
            ///
            /// $$
            /// f(x, y) = (z_1, z_0),
            /// $$
            /// where $W$ is `Self::WIDTH`,
            ///
            /// $x, y, z_1, z_0 < 2^W$, and
            /// $$
            /// xy = 2^Wz_1 + z_0.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::x_mul_y_to_zz#x_mul_y_to_zz).
            ///
            /// This is equivalent to `umul_ppmm` from `longlong.h`, GMP 6.2.1, where `(w1, w0)` is
            /// returned.
            #[inline]
            fn x_mul_y_to_zz(x: $t, y: $t) -> ($t, $t) {
                implicit_x_mul_y_to_zz::<$t, $dt>(x, y)
            }
        }
    };
}

implicit_x_mul_y_to_zz!(u8, u16);
implicit_x_mul_y_to_zz!(u16, u32);
implicit_x_mul_y_to_zz!(u32, u64);
implicit_x_mul_y_to_zz!(u64, u128);

impl XMulYToZZ for usize {
    /// Multiplies two numbers, returning the product as a pair of [`usize`] values.
    ///
    /// The more significant value always comes first.
    ///
    /// $$
    /// f(x, y) = (z_1, z_0),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x, y, z_1, z_0 < 2^W$, and
    /// $$
    /// xy = 2^Wz_1 + z_0.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::x_mul_y_to_zz#x_mul_y_to_zz).
    ///
    /// This is equivalent to `umul_ppmm` from `longlong.h`, GMP 6.2.1, where `(w1, w0)` is
    /// returned.
    fn x_mul_y_to_zz(x: usize, y: usize) -> (usize, usize) {
        if usize::WIDTH == u32::WIDTH {
            let (z_1, z_0) = u32::x_mul_y_to_zz(u32::wrapping_from(x), u32::wrapping_from(y));
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        } else {
            let (z_1, z_0) = u64::x_mul_y_to_zz(u64::wrapping_from(x), u64::wrapping_from(y));
            (usize::wrapping_from(z_1), usize::wrapping_from(z_0))
        }
    }
}

impl XMulYToZZ for u128 {
    /// Multiplies two numbers, returning the product as a pair of [`u128`] values.
    ///
    /// The more significant value always comes first.
    ///
    /// $$
    /// f(x, y) = (z_1, z_0),
    /// $$
    /// where $W$ is `Self::WIDTH`,
    ///
    /// $x, y, z_1, z_0 < 2^W$, and
    /// $$
    /// xy = 2^Wz_1 + z_0.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::x_mul_y_to_zz#x_mul_y_to_zz).
    ///
    /// This is equivalent to `umul_ppmm` from `longlong.h`, GMP 6.2.1, where `(w1, w0)` is
    /// returned.
    #[inline]
    fn x_mul_y_to_zz(x: u128, y: u128) -> (u128, u128) {
        explicit_x_mul_y_to_zz(x, y)
    }
}
