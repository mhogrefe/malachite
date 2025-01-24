// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_sqrtrem1` contributed to the GNU project by Torbjörn Granlund.
//
//      Copyright © 1999-2002, 2004, 2005, 2008, 2010, 2012, 2015, 2017 Free Software Foundation,
//      Inc.
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, Ln,
    RoundToMultipleOfPowerOf2, ShrRound, Sqrt, SqrtAssign, SqrtAssignRem, SqrtRem,
};
use crate::num::basic::integers::{PrimitiveInt, USIZE_IS_U32};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::logic::traits::SignificantBits;
use crate::rounding_modes::RoundingMode::*;
use core::cmp::Ordering::*;

const U8_SQUARES: [u8; 16] = [0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225];

impl FloorSqrt for u8 {
    type Output = u8;

    /// Returns the floor of the square root of a [`u8`].
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#floor_sqrt).
    ///
    /// # Notes
    /// The [`u8`] implementation uses a lookup table.
    fn floor_sqrt(self) -> u8 {
        u8::wrapping_from(match U8_SQUARES.binary_search(&self) {
            Ok(i) => i,
            Err(i) => i - 1,
        })
    }
}

impl CeilingSqrt for u8 {
    type Output = u8;

    /// Returns the ceiling of the square root of a [`u8`].
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#ceiling_sqrt).
    ///
    /// # Notes
    /// The [`u8`] implementation uses a lookup table.
    fn ceiling_sqrt(self) -> u8 {
        u8::wrapping_from(match U8_SQUARES.binary_search(&self) {
            Ok(i) | Err(i) => i,
        })
    }
}

impl CheckedSqrt for u8 {
    type Output = u8;

    /// Returns the the square root of a [`u8`], or `None` if the [`u8`] is not a perfect square.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#checked_sqrt).
    ///
    /// # Notes
    /// The [`u8`] implementation uses a lookup table.
    fn checked_sqrt(self) -> Option<u8> {
        U8_SQUARES.binary_search(&self).ok().map(u8::wrapping_from)
    }
}

impl SqrtRem for u8 {
    type SqrtOutput = u8;
    type RemOutput = u8;

    /// Returns the floor of the square root of a [`u8`], and the remainder (the difference between
    /// the [`u8`] and the square of the floor).
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#sqrt_rem).
    ///
    /// # Notes
    /// The [`u8`] implementation uses a lookup table.
    fn sqrt_rem(self) -> (u8, u8) {
        match U8_SQUARES.binary_search(&self) {
            Ok(i) => (u8::wrapping_from(i), 0),
            Err(i) => (u8::wrapping_from(i - 1), self - U8_SQUARES[i - 1]),
        }
    }
}

pub(crate) fn floor_inverse_checked_binary<T: PrimitiveUnsigned, F: Fn(T) -> Option<T>>(
    f: F,
    x: T,
    mut low: T,
    mut high: T,
) -> T {
    loop {
        if high <= low {
            return low;
        }
        let mid: T = low.checked_add(high).unwrap().shr_round(1, Ceiling).0;
        match f(mid).map(|mid| mid.cmp(&x)) {
            Some(Equal) => return mid,
            Some(Less) => low = mid,
            Some(Greater) | None => high = mid - T::ONE,
        }
    }
}

pub_test! {floor_sqrt_binary<T: PrimitiveUnsigned>(x: T) -> T {
    if x < T::TWO {
        x
    } else {
        let p = T::power_of_2(x.significant_bits().shr_round(1, Ceiling).0);
        floor_inverse_checked_binary(T::checked_square, x, p >> 1, p)
    }
}}

pub_test! {ceiling_sqrt_binary<T: PrimitiveUnsigned>(x: T) -> T {
    let floor_sqrt = floor_sqrt_binary(x);
    if floor_sqrt.square() == x {
        floor_sqrt
    } else {
        floor_sqrt + T::ONE
    }
}}

pub_test! {checked_sqrt_binary<T: PrimitiveUnsigned>(x: T) -> Option<T> {
    let floor_sqrt = floor_sqrt_binary(x);
    if floor_sqrt.square() == x {
        Some(floor_sqrt)
    } else {
        None
    }
}}

pub_test! {sqrt_rem_binary<T: PrimitiveUnsigned>(x: T) -> (T, T) {
    let floor_sqrt = floor_sqrt_binary(x);
    (floor_sqrt, x - floor_sqrt.square())
}}

const INV_SQRT_TAB: [u16; 384] = [
    0xff, 0xfd, 0xfb, 0xf9, 0xf7, 0xf5, 0xf3, 0xf2, 0xf0, 0xee, 0xec, 0xea, 0xe9, 0xe7, 0xe5, 0xe4,
    0xe2, 0xe0, 0xdf, 0xdd, 0xdb, 0xda, 0xd8, 0xd7, 0xd5, 0xd4, 0xd2, 0xd1, 0xcf, 0xce, 0xcc, 0xcb,
    0xc9, 0xc8, 0xc6, 0xc5, 0xc4, 0xc2, 0xc1, 0xc0, 0xbe, 0xbd, 0xbc, 0xba, 0xb9, 0xb8, 0xb7, 0xb5,
    0xb4, 0xb3, 0xb2, 0xb0, 0xaf, 0xae, 0xad, 0xac, 0xaa, 0xa9, 0xa8, 0xa7, 0xa6, 0xa5, 0xa4, 0xa3,
    0xa2, 0xa0, 0x9f, 0x9e, 0x9d, 0x9c, 0x9b, 0x9a, 0x99, 0x98, 0x97, 0x96, 0x95, 0x94, 0x93, 0x92,
    0x91, 0x90, 0x8f, 0x8e, 0x8d, 0x8c, 0x8c, 0x8b, 0x8a, 0x89, 0x88, 0x87, 0x86, 0x85, 0x84, 0x83,
    0x83, 0x82, 0x81, 0x80, 0x7f, 0x7e, 0x7e, 0x7d, 0x7c, 0x7b, 0x7a, 0x79, 0x79, 0x78, 0x77, 0x76,
    0x76, 0x75, 0x74, 0x73, 0x72, 0x72, 0x71, 0x70, 0x6f, 0x6f, 0x6e, 0x6d, 0x6d, 0x6c, 0x6b, 0x6a,
    0x6a, 0x69, 0x68, 0x68, 0x67, 0x66, 0x66, 0x65, 0x64, 0x64, 0x63, 0x62, 0x62, 0x61, 0x60, 0x60,
    0x5f, 0x5e, 0x5e, 0x5d, 0x5c, 0x5c, 0x5b, 0x5a, 0x5a, 0x59, 0x59, 0x58, 0x57, 0x57, 0x56, 0x56,
    0x55, 0x54, 0x54, 0x53, 0x53, 0x52, 0x52, 0x51, 0x50, 0x50, 0x4f, 0x4f, 0x4e, 0x4e, 0x4d, 0x4d,
    0x4c, 0x4b, 0x4b, 0x4a, 0x4a, 0x49, 0x49, 0x48, 0x48, 0x47, 0x47, 0x46, 0x46, 0x45, 0x45, 0x44,
    0x44, 0x43, 0x43, 0x42, 0x42, 0x41, 0x41, 0x40, 0x40, 0x3f, 0x3f, 0x3e, 0x3e, 0x3d, 0x3d, 0x3c,
    0x3c, 0x3b, 0x3b, 0x3a, 0x3a, 0x39, 0x39, 0x39, 0x38, 0x38, 0x37, 0x37, 0x36, 0x36, 0x35, 0x35,
    0x35, 0x34, 0x34, 0x33, 0x33, 0x32, 0x32, 0x32, 0x31, 0x31, 0x30, 0x30, 0x2f, 0x2f, 0x2f, 0x2e,
    0x2e, 0x2d, 0x2d, 0x2d, 0x2c, 0x2c, 0x2b, 0x2b, 0x2b, 0x2a, 0x2a, 0x29, 0x29, 0x29, 0x28, 0x28,
    0x27, 0x27, 0x27, 0x26, 0x26, 0x26, 0x25, 0x25, 0x24, 0x24, 0x24, 0x23, 0x23, 0x23, 0x22, 0x22,
    0x21, 0x21, 0x21, 0x20, 0x20, 0x20, 0x1f, 0x1f, 0x1f, 0x1e, 0x1e, 0x1e, 0x1d, 0x1d, 0x1d, 0x1c,
    0x1c, 0x1b, 0x1b, 0x1b, 0x1a, 0x1a, 0x1a, 0x19, 0x19, 0x19, 0x18, 0x18, 0x18, 0x18, 0x17, 0x17,
    0x17, 0x16, 0x16, 0x16, 0x15, 0x15, 0x15, 0x14, 0x14, 0x14, 0x13, 0x13, 0x13, 0x12, 0x12, 0x12,
    0x12, 0x11, 0x11, 0x11, 0x10, 0x10, 0x10, 0x0f, 0x0f, 0x0f, 0x0f, 0x0e, 0x0e, 0x0e, 0x0d, 0x0d,
    0x0d, 0x0c, 0x0c, 0x0c, 0x0c, 0x0b, 0x0b, 0x0b, 0x0a, 0x0a, 0x0a, 0x0a, 0x09, 0x09, 0x09, 0x09,
    0x08, 0x08, 0x08, 0x07, 0x07, 0x07, 0x07, 0x06, 0x06, 0x06, 0x06, 0x05, 0x05, 0x05, 0x04, 0x04,
    0x04, 0x04, 0x03, 0x03, 0x03, 0x03, 0x02, 0x02, 0x02, 0x02, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00,
];

// This is equivalent to `mpn_sqrtrem1` from `mpn/generic/sqrtrem.c`, GMP 6.2.1, where both the
// square root and the remainder are returned.
#[doc(hidden)]
pub fn sqrt_rem_newton<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    n: U,
) -> (U, U) {
    let magic = match U::WIDTH {
        u32::WIDTH => {
            U::wrapping_from(0x100000u32) // 0xfee6f < MAGIC < 0x29cbc8
        }
        u64::WIDTH => {
            U::wrapping_from(0x10000000000u64) // 0xffe7debbfc < magic < 0x232b1850f410
        }
        _ => panic!(),
    };
    assert!(n.leading_zeros() < 2);
    // Use Newton iterations for approximating 1/sqrt(a) instead of sqrt(a), since we can do the
    // former without division. As part of the last iteration convert from 1/sqrt(a) to sqrt(a).
    let i: usize = (n >> (U::WIDTH - 9)).wrapping_into(); // extract bits for table lookup
    let mut inv_sqrt = U::wrapping_from(INV_SQRT_TAB[i - 0x80]);
    inv_sqrt.set_bit(8); // initial 1/sqrt(a)
    let mut sqrt: U = match U::WIDTH {
        u32::WIDTH => {
            let p = inv_sqrt * (n >> 8);
            let t: U = p >> 13;
            let a: U = n << 6;
            let t = S::wrapping_from(a.wrapping_sub(t.wrapping_square()).wrapping_sub(magic)) >> 8;
            p.wrapping_add(U::wrapping_from((S::wrapping_from(inv_sqrt) * t) >> 7)) >> 16
        }
        u64::WIDTH => {
            let a1 = n >> (U::WIDTH - 33);
            let t = (S::wrapping_from(0x2000000000000u64 - 0x30000)
                - S::wrapping_from(a1 * inv_sqrt.square()))
                >> 16;
            let a: U = inv_sqrt << 16;
            inv_sqrt = a.wrapping_add(U::wrapping_from((S::wrapping_from(inv_sqrt) * t) >> 18));
            let p = inv_sqrt * (n >> 24);
            let t: U = p >> 25;
            let a: U = n << 14;
            let t = S::wrapping_from(a.wrapping_sub(t.wrapping_square()).wrapping_sub(magic)) >> 24;
            p.wrapping_add(U::wrapping_from((S::wrapping_from(inv_sqrt) * t) >> 15)) >> 32
        }
        _ => unreachable!(),
    };
    // x0 is now a full limb approximation of sqrt(a0)
    let mut square = sqrt.square();
    if square + (sqrt << 1) < n {
        square += (sqrt << 1) + U::ONE;
        sqrt += U::ONE;
    }
    (sqrt, n - square)
}

// This is equivalent to `n_sqrt` from `ulong_extras/sqrt.c`, FLINT 2.7.1.
fn floor_sqrt_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> T {
    if x >= max_square {
        return T::low_mask(T::WIDTH >> 1);
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Equal => sqrt,
        Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            match square.cmp(&x) {
                Equal => return sqrt,
                Less => {}
                Greater => return sqrt - T::ONE,
            }
        },
        Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            if square <= x {
                return sqrt;
            }
        },
    }
}

fn ceiling_sqrt_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> T {
    if x > max_square {
        return T::power_of_2(T::WIDTH >> 1);
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Equal => sqrt,
        Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            if square >= x {
                return sqrt;
            }
        },
        Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            match square.cmp(&x) {
                Equal => return sqrt,
                Greater => {}
                Less => return sqrt + T::ONE,
            }
        },
    }
}

fn checked_sqrt_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> Option<T> {
    if x > max_square {
        return None;
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Equal => Some(sqrt),
        Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            match square.cmp(&x) {
                Equal => return Some(sqrt),
                Less => {}
                Greater => return None,
            }
        },
        Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            match square.cmp(&x) {
                Equal => return Some(sqrt),
                Less => return None,
                Greater => {}
            }
        },
    }
}

// This is equivalent to `n_sqrtrem` from `ulong_extras/sqrtrem.c`, FLINT 2.7.1.
fn sqrt_rem_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    max_square: T,
    x: T,
) -> (T, T) {
    if x >= max_square {
        return (T::low_mask(T::WIDTH >> 1), x - max_square);
    }
    let mut sqrt = g(f(x).sqrt());
    let mut square = if let Some(square) = sqrt.checked_square() {
        square
    } else {
        // set to max possible sqrt
        sqrt = T::low_mask(T::WIDTH >> 1);
        sqrt.square()
    };
    match square.cmp(&x) {
        Equal => (sqrt, T::ZERO),
        Less => loop {
            square = square.checked_add((sqrt << 1) + T::ONE).unwrap();
            sqrt += T::ONE;
            match square.cmp(&x) {
                Equal => return (sqrt, T::ZERO),
                Less => {}
                Greater => {
                    square -= (sqrt << 1) - T::ONE;
                    sqrt -= T::ONE;
                    return (sqrt, x - square);
                }
            }
        },
        Greater => loop {
            square -= (sqrt << 1) - T::ONE;
            sqrt -= T::ONE;
            if square <= x {
                return (sqrt, x - square);
            }
        },
    }
}

fn floor_sqrt_newton_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
) -> U {
    if x == U::ZERO {
        return U::ZERO;
    }
    let shift = x
        .leading_zeros()
        .round_to_multiple_of_power_of_2(1, Floor)
        .0;
    sqrt_rem_newton::<U, S>(x << shift).0 >> (shift >> 1)
}

fn ceiling_sqrt_newton_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
) -> U {
    if x == U::ZERO {
        return U::ZERO;
    }
    let shift = x
        .leading_zeros()
        .round_to_multiple_of_power_of_2(1, Floor)
        .0;
    let (mut sqrt, rem) = sqrt_rem_newton::<U, S>(x << shift);
    sqrt >>= shift >> 1;
    if rem != U::ZERO {
        sqrt += U::ONE;
    }
    sqrt
}

fn checked_sqrt_newton_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
) -> Option<U> {
    if x == U::ZERO {
        return Some(U::ZERO);
    }
    let shift = x
        .leading_zeros()
        .round_to_multiple_of_power_of_2(1, Floor)
        .0;
    let (sqrt, rem) = sqrt_rem_newton::<U, S>(x << shift);
    if rem == U::ZERO {
        Some(sqrt >> (shift >> 1))
    } else {
        None
    }
}

fn sqrt_rem_newton_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
) -> (U, U) {
    if x == U::ZERO {
        return (U::ZERO, U::ZERO);
    }
    let shift = x
        .leading_zeros()
        .round_to_multiple_of_power_of_2(1, Floor)
        .0;
    let (mut sqrt, rem) = sqrt_rem_newton::<U, S>(x << shift);
    if shift == 0 {
        (sqrt, rem)
    } else {
        sqrt >>= shift >> 1;
        (sqrt, x - sqrt.square())
    }
}

macro_rules! impl_sqrt_newton {
    ($u: ident, $s: ident) => {
        impl FloorSqrt for $u {
            type Output = $u;

            /// Returns the floor of the square root of an integer.
            ///
            /// $f(x) = \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sqrt#floor_sqrt).
            ///
            /// # Notes
            /// For [`u32`] and [`u64`], the square root is computed using Newton's method.
            #[inline]
            fn floor_sqrt(self) -> $u {
                floor_sqrt_newton_helper::<$u, $s>(self)
            }
        }

        impl CeilingSqrt for $u {
            type Output = $u;

            /// Returns the ceiling of the square root of an integer.
            ///
            /// $f(x) = \lceil\sqrt{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sqrt#ceiling_sqrt).
            ///
            /// # Notes
            /// For [`u32`] and [`u64`], the square root is computed using Newton's method.
            #[inline]
            fn ceiling_sqrt(self) -> $u {
                ceiling_sqrt_newton_helper::<$u, $s>(self)
            }
        }

        impl CheckedSqrt for $u {
            type Output = $u;

            /// Returns the the square root of an integer, or `None` if the integer is not a perfect
            /// square.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
            ///     \operatorname{None} & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sqrt#checked_sqrt).
            ///
            /// # Notes
            /// For [`u32`] and [`u64`], the square root is computed using Newton's method.
            #[inline]
            fn checked_sqrt(self) -> Option<$u> {
                checked_sqrt_newton_helper::<$u, $s>(self)
            }
        }

        impl SqrtRem for $u {
            type SqrtOutput = $u;
            type RemOutput = $u;

            /// Returns the floor of the square root of an integer, and the remainder (the
            /// difference between the integer and the square of the floor).
            ///
            /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sqrt#sqrt_rem).
            ///
            /// # Notes
            /// For [`u32`] and [`u64`], the square root is computed using Newton's method.
            #[inline]
            fn sqrt_rem(self) -> ($u, $u) {
                sqrt_rem_newton_helper::<$u, $s>(self)
            }
        }
    };
}
impl_sqrt_newton!(u32, i32);
impl_sqrt_newton!(u64, i64);

impl FloorSqrt for u16 {
    type Output = u16;

    /// Returns the floor of the square root of a [`u16`].
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#floor_sqrt).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn floor_sqrt(self) -> u16 {
        u16::wrapping_from(u32::from(self).floor_sqrt())
    }
}

impl CeilingSqrt for u16 {
    type Output = u16;

    /// Returns the ceiling of the square root of a [`u16`].
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#ceiling_sqrt).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn ceiling_sqrt(self) -> u16 {
        u16::wrapping_from(u32::from(self).ceiling_sqrt())
    }
}

impl CheckedSqrt for u16 {
    type Output = u16;

    /// Returns the the square root of a [`u16`], or `None` if the integer is not a perfect square.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#checked_sqrt).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn checked_sqrt(self) -> Option<u16> {
        u32::from(self).checked_sqrt().map(u16::wrapping_from)
    }
}

impl SqrtRem for u16 {
    type SqrtOutput = u16;
    type RemOutput = u16;

    /// Returns the floor of the square root of a [`u16`], and the remainder (the difference between
    /// the [`u16`] and the square of the floor).
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#sqrt_rem).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn sqrt_rem(self) -> (u16, u16) {
        let (sqrt, rem) = u32::from(self).sqrt_rem();
        (u16::wrapping_from(sqrt), u16::wrapping_from(rem))
    }
}

impl FloorSqrt for usize {
    type Output = usize;

    /// Returns the floor of the square root of a [`usize`].
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#floor_sqrt).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn floor_sqrt(self) -> usize {
        if USIZE_IS_U32 {
            usize::wrapping_from(u32::wrapping_from(self).floor_sqrt())
        } else {
            usize::wrapping_from(u64::wrapping_from(self).floor_sqrt())
        }
    }
}

impl CeilingSqrt for usize {
    type Output = usize;

    /// Returns the ceiling of the square root of a [`usize`].
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#ceiling_sqrt).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn ceiling_sqrt(self) -> usize {
        if USIZE_IS_U32 {
            usize::wrapping_from(u32::wrapping_from(self).ceiling_sqrt())
        } else {
            usize::wrapping_from(u64::wrapping_from(self).ceiling_sqrt())
        }
    }
}

impl CheckedSqrt for usize {
    type Output = usize;

    /// Returns the the square root of a [`usize`], or `None` if the [`usize`] is not a perfect
    /// square.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#checked_sqrt).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn checked_sqrt(self) -> Option<usize> {
        if USIZE_IS_U32 {
            u32::wrapping_from(self)
                .checked_sqrt()
                .map(usize::wrapping_from)
        } else {
            u64::wrapping_from(self)
                .checked_sqrt()
                .map(usize::wrapping_from)
        }
    }
}

impl SqrtRem for usize {
    type SqrtOutput = usize;
    type RemOutput = usize;

    /// Returns the floor of the square root of a [`usize`], and the remainder (the difference
    /// between the [`usize`] and the square of the floor).
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::sqrt#sqrt_rem).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn sqrt_rem(self) -> (usize, usize) {
        if USIZE_IS_U32 {
            let (sqrt, rem) = u32::wrapping_from(self).sqrt_rem();
            (usize::wrapping_from(sqrt), usize::wrapping_from(rem))
        } else {
            let (sqrt, rem) = u64::wrapping_from(self).sqrt_rem();
            (usize::wrapping_from(sqrt), usize::wrapping_from(rem))
        }
    }
}

// TODO tune
const U128_SQRT_THRESHOLD: u64 = 125;
const U128_MAX_SQUARE: u128 = 0xfffffffffffffffe0000000000000001;

impl FloorSqrt for u128 {
    type Output = u128;

    /// Returns the floor of the square root of a [`u128`].
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::sqrt#floor_sqrt).
    ///
    /// # Notes
    /// For [`u128`], using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large [`u128`]s. To overcome this, large
    /// [`u128`]s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that $2^{14} \leq
    /// \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn floor_sqrt(self) -> u128 {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            floor_sqrt_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            floor_sqrt_binary(self)
        }
    }
}

impl CeilingSqrt for u128 {
    type Output = u128;

    /// Returns the ceiling of the square root of a [`u128`].
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::sqrt#ceiling_sqrt).
    ///
    /// # Notes
    /// For [`u128`], using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large [`u128`]s. To overcome this, large
    /// [`u128`]s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that $2^{14} \leq
    /// \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn ceiling_sqrt(self) -> u128 {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            ceiling_sqrt_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            ceiling_sqrt_binary(self)
        }
    }
}

impl CheckedSqrt for u128 {
    type Output = u128;

    /// Returns the the square root of a [`u128`], or `None` if the [`u128`] is not a perfect
    /// square.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::sqrt#checked_sqrt).
    ///
    /// # Notes
    /// For [`u128`], using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large [`u128`]s. To overcome this, large
    /// [`u128`]s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that $2^{14} \leq
    /// \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn checked_sqrt(self) -> Option<u128> {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            checked_sqrt_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            checked_sqrt_binary(self)
        }
    }
}

impl SqrtRem for u128 {
    type SqrtOutput = u128;
    type RemOutput = u128;

    /// Returns the floor of the square root of a [`u128`], and the remainder (the difference
    /// between the [`u128`] and the square of the floor).
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// See [here](super::sqrt#sqrt_rem).
    ///
    /// # Notes
    /// For [`u128`], using a floating-point approximation and refining the result works, but the
    /// number of necessary adjustments becomes large for large [`u128`]s. To overcome this, large
    /// [`u128`]s switch to a binary search algorithm. To get decent starting bounds, the following
    /// fact is used:
    ///
    /// If $x$ is nonzero and has $b$ significant bits, then
    ///
    /// $2^{b-1} \leq x \leq 2^b-1$,
    ///
    /// $2^{b-1} \leq x \leq 2^b$,
    ///
    /// $2^{2\lfloor (b-1)/2 \rfloor} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $2^{2(\lceil b/2 \rceil-1)} \leq x \leq 2^{2\lceil b/2 \rceil}$,
    ///
    /// $\lfloor\sqrt{2^{2(\lceil b/2 \rceil-1)}}\rfloor \leq \lfloor\sqrt{x}\rfloor \leq
    /// \lfloor\sqrt{2^{2\lceil b/2 \rceil}}\rfloor$, since $x \mapsto \lfloor\sqrt{x}\rfloor$ is
    /// weakly increasing,
    ///
    /// $2^{\lceil b/2 \rceil-1} \leq \lfloor\sqrt{x}\rfloor \leq 2^{\lceil b/2 \rceil}$.
    ///
    /// For example, since $10^9$ has 30 significant bits, we know that $2^{14} \leq
    /// \lfloor\sqrt{10^9}\rfloor \leq 2^{15}$.
    fn sqrt_rem(self) -> (u128, u128) {
        if self.significant_bits() < U128_SQRT_THRESHOLD {
            sqrt_rem_approx_and_refine(|x| x as f64, |x| x as u128, U128_MAX_SQUARE, self)
        } else {
            sqrt_rem_binary(self)
        }
    }
}

macro_rules! impl_sqrt_signed {
    ($u: ident, $s: ident) => {
        impl FloorSqrt for $s {
            type Output = $s;

            /// Returns the floor of the square root of an integer.
            ///
            /// $f(x) = \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See [here](super::sqrt#floor_sqrt).
            #[inline]
            fn floor_sqrt(self) -> Self {
                if self >= 0 {
                    $s::wrapping_from(self.unsigned_abs().floor_sqrt())
                } else {
                    panic!("Cannot take square root of {}", self)
                }
            }
        }

        impl CeilingSqrt for $s {
            type Output = $s;

            /// Returns the ceiling of the square root of an integer.
            ///
            /// $f(x) = \lceil\sqrt{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See [here](super::sqrt#ceiling_sqrt).
            #[inline]
            fn ceiling_sqrt(self) -> $s {
                if self >= 0 {
                    $s::wrapping_from(self.unsigned_abs().ceiling_sqrt())
                } else {
                    panic!("Cannot take square root of {}", self)
                }
            }
        }

        impl CheckedSqrt for $s {
            type Output = $s;

            /// Returns the the square root of an integer, or `None` if the integer is not a perfect
            /// square.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
            ///     \operatorname{None} & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See [here](super::sqrt#checked_sqrt).
            #[inline]
            fn checked_sqrt(self) -> Option<$s> {
                if self >= 0 {
                    self.unsigned_abs().checked_sqrt().map($s::wrapping_from)
                } else {
                    panic!("Cannot take square root of {}", self)
                }
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_sqrt_signed);

macro_rules! impl_sqrt_assign_rem_unsigned {
    ($t: ident) => {
        impl SqrtAssignRem for $t {
            type RemOutput = $t;

            /// Replaces an integer with the floor of its square root, and returns the remainder
            /// (the difference between the original integer and the square of the floor).
            ///
            /// $f(x) = x - \lfloor\sqrt{x}\rfloor^2$,
            ///
            /// $x \gets \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sqrt#sqrt_assign_rem).
            #[inline]
            fn sqrt_assign_rem(&mut self) -> $t {
                let rem;
                (*self, rem) = self.sqrt_rem();
                rem
            }
        }
    };
}
apply_to_unsigneds!(impl_sqrt_assign_rem_unsigned);

macro_rules! impl_sqrt_assign {
    ($t: ident) => {
        impl FloorSqrtAssign for $t {
            /// Replaces an integer with the floor of its square root.
            ///
            /// $x \gets \lfloor\sqrt{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See [here](super::sqrt#floor_sqrt_assign).
            #[inline]
            fn floor_sqrt_assign(&mut self) {
                *self = self.floor_sqrt();
            }
        }

        impl CeilingSqrtAssign for $t {
            /// Replaces an integer with the ceiling of its square root.
            ///
            /// $x \gets \lceil\sqrt{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative.
            ///
            /// # Examples
            /// See [here](super::sqrt#ceiling_sqrt_assign).
            #[inline]
            fn ceiling_sqrt_assign(&mut self) {
                *self = self.ceiling_sqrt();
            }
        }
    };
}
apply_to_primitive_ints!(impl_sqrt_assign);

macro_rules! impl_sqrt_primitive_float {
    ($f:ident) => {
        impl Sqrt for $f {
            type Output = Self;

            #[inline]
            fn sqrt(self) -> $f {
                libm::Libm::<$f>::sqrt(self)
            }
        }

        // TODO move to better location
        impl Ln for $f {
            type Output = Self;

            #[inline]
            fn ln(self) -> $f {
                libm::Libm::<$f>::log(self)
            }
        }

        impl SqrtAssign for $f {
            /// Replaces a number with its square root.
            ///
            /// $x \gets \sqrt x$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::sqrt#sqrt_assign).
            #[inline]
            fn sqrt_assign(&mut self) {
                *self = self.sqrt();
            }
        }
    };
}
apply_to_primitive_floats!(impl_sqrt_primitive_float);
