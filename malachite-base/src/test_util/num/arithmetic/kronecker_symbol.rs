// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1996, 1998, 1999-2002, 2000-2004, 2008, 2010 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{NegAssign, Parity};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{HasHalf, JoinHalves};
use crate::num::logic::traits::NotAssign;
use std::mem::swap;

// This is equivalent to `n_jacobi_unsigned` from `ulong_extras/jacobi.c`, FLINT 2.7.1.
pub fn jacobi_symbol_unsigned_fast_1<T: PrimitiveUnsigned>(x: T, y: T) -> i8 {
    let mut a = x;
    let mut b = y;
    let mut s = 1;
    if a < b && b != T::ONE {
        if a == T::ZERO {
            return 0;
        }
        swap(&mut a, &mut b);
        let exp = b.trailing_zeros();
        b >>= exp;
        if T::wrapping_from(exp)
            .wrapping_mul(a.wrapping_square() - T::ONE)
            .get_bit(3)
            != (a - T::ONE).wrapping_mul(b - T::ONE).get_bit(2)
        {
            s.neg_assign();
        }
    }
    while b != T::ONE {
        if a >> 2 < b {
            let temp = a - b;
            a = b;
            b = if temp < b {
                temp
            } else if temp < b << 1 {
                temp - a
            } else {
                temp - (a << 1)
            }
        } else {
            a %= b;
            swap(&mut a, &mut b);
        }
        if b == T::ZERO {
            return 0;
        }
        let exp = b.trailing_zeros();
        b >>= exp;
        if T::wrapping_from(exp)
            .wrapping_mul(a.wrapping_square() - T::ONE)
            .get_bit(3)
            != (a - T::ONE).wrapping_mul(b - T::ONE).get_bit(2)
        {
            s.neg_assign();
        }
    }
    s
}

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 1` and `result_bit_1` is false.
pub fn jacobi_symbol_unsigned_fast_2_1<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    let a_twos = a.trailing_zeros();
    if a_twos.odd() && ((b >> 1u32) ^ b).get_bit(1) {
        s.neg_assign();
    }
    a >>= a_twos;
    if a == T::ONE {
        return s;
    }
    if a < b {
        if (a & b).get_bit(1) {
            s.neg_assign();
        }
        swap(&mut a, &mut b);
    }
    loop {
        assert!(a.odd());
        assert!(b.odd());
        assert!(a >= b);
        a -= b;
        if a == T::ZERO {
            return 0;
        }
        let a_twos = a.trailing_zeros();
        if a_twos.odd() && ((b >> 1u32) ^ b).get_bit(1) {
            s.neg_assign();
        }
        a >>= a_twos;
        if a == T::ONE {
            return s;
        }
        if a < b {
            if (a & b).get_bit(1) {
                s.neg_assign();
            }
            swap(&mut a, &mut b);
        }
    }
}

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 2` and `result_bit_1` is false.
pub fn jacobi_symbol_unsigned_fast_2_2<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    if a.even() {
        let two = (b >> 1u32) ^ b;
        loop {
            a >>= 1;
            if two.get_bit(1) {
                s.neg_assign();
            }
            if a.odd() {
                break;
            }
        }
    }
    if a == T::ONE {
        return s;
    }
    if a < b {
        if (a & b).get_bit(1) {
            s.neg_assign();
        }
        swap(&mut a, &mut b);
    }
    loop {
        assert!(a.odd());
        assert!(b.odd());
        assert!(a >= b);
        a -= b;
        if a == T::ZERO {
            return 0;
        }
        let two = (b >> 1u32) ^ b;
        loop {
            a >>= 1;
            if two.get_bit(1) {
                s.neg_assign();
            }
            if a.odd() {
                break;
            }
        }
        if a == T::ONE {
            return s;
        }
        if a < b {
            if (a & b).get_bit(1) {
                s.neg_assign();
            }
            swap(&mut a, &mut b);
        }
    }
}

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 3` and `result_bit_1` is false.
pub fn jacobi_symbol_unsigned_fast_2_3<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    let two = (b >> 1u32) ^ b;
    let shift = !a & T::ONE;
    let shift_8: u8 = shift.wrapping_into();
    a >>= shift_8;
    let mask = shift << 1u32;
    if (two & mask).get_bit(1) {
        s.neg_assign();
    }
    let two_bit = two.get_bit(1);
    while a.even() {
        a >>= 1;
        if two_bit {
            s.neg_assign();
        }
    }
    if a == T::ONE {
        return s;
    }
    if a < b {
        if (a & b).get_bit(1) {
            s.neg_assign();
        }
        swap(&mut a, &mut b);
    }
    loop {
        assert!(a.odd());
        assert!(b.odd());
        assert!(a >= b);
        a -= b;
        if a == T::ZERO {
            return 0;
        }
        let two = (b >> 1u32) ^ b;
        let mask = !a & T::TWO;
        a >>= 1;
        if a.even() {
            a >>= 1;
        }
        if (two ^ (two & mask)).get_bit(1) {
            s.neg_assign();
        }
        while a.even() {
            a >>= 1;
            if two.get_bit(1) {
                s.neg_assign();
            }
        }
        if a == T::ONE {
            return s;
        }
        if a < b {
            if (a & b).get_bit(1) {
                s.neg_assign();
            }
            swap(&mut a, &mut b);
        }
    }
}

// This is equivalent to `mpn_jacobi_base` from `mpn/jacbase.c`, GMP 6.2.1, where
// `JACOBI_BASE_METHOD == 4` and `result_bit_1` is false.
pub fn jacobi_symbol_unsigned_fast_2_4<T: PrimitiveUnsigned>(mut a: T, mut b: T) -> i8 {
    assert!(b.odd());
    if b == T::ONE {
        return 1;
    } else if a == T::ZERO {
        return 0;
    }
    let mut s = 1;
    b >>= 1u32;
    let c = a.trailing_zeros();
    if (T::wrapping_from(c) & (b ^ (b >> 1))).odd() {
        s.neg_assign();
    }
    // We may have c == T::WIDTH - 1, so we can't use a >> (c + 1).
    a >>= c;
    a >>= 1;
    while b != T::ZERO {
        let t = a.wrapping_sub(b);
        if t == T::ZERO {
            return 0;
        }
        let bgta = if t.get_highest_bit() { T::MAX } else { T::ZERO };
        // If b > a, invoke reciprocity
        if (bgta & a & b).odd() {
            s.neg_assign();
        }
        // b <-- min (a, b)
        b.wrapping_add_assign(bgta & t);
        // a <-- |a - b|
        a = (t ^ bgta).wrapping_sub(bgta);
        // Number of trailing zeros is the same no matter if we look at t or a, but using t gives
        // more parallelism.
        let c = t.trailing_zeros() + 1;
        // (2/b) = -1 if b = 3 or 5 mod 8
        if (T::wrapping_from(c) & (b ^ (b >> 1))).odd() {
            s.neg_assign();
        }
        a >>= c;
    }
    s
}

pub fn jacobi_symbol_unsigned_double_simple<
    T: PrimitiveUnsigned,
    D: HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned,
>(
    x_1: T,
    x_0: T,
    y_1: T,
    y_0: T,
) -> i8 {
    D::join_halves(x_1, x_0).jacobi_symbol(D::join_halves(y_1, y_0))
}

// Computes (a / b) where b is odd, and a and b are otherwise arbitrary two-limb numbers.
//
// This is equivalent to `mpn_jacobi_2` from `mpn/jacobi_2.c`, GMP 6.2.1, where `JACOBI_2_METHOD ==
// 1` and `bit` is 0.
pub fn jacobi_symbol_unsigned_double_fast_1<T: PrimitiveUnsigned>(
    mut x_1: T,
    mut x_0: T,
    mut y_1: T,
    mut y_0: T,
) -> i8 {
    assert!(y_0.odd());
    y_0 = (y_1 << (T::WIDTH - 1)) | (y_0 >> 1);
    y_1 >>= 1;
    if y_1 == T::ZERO && y_0 == T::ZERO {
        return 1;
    }
    if x_1 == T::ZERO && x_0 == T::ZERO {
        return 0;
    }
    if x_0 == T::ZERO {
        x_0 = x_1;
        x_1 = T::ZERO;
    }
    let mut bit = false;
    let c = x_0.trailing_zeros() + 1;
    if c.even() && (y_0 ^ (y_0 >> 1)).odd() {
        bit.not_assign();
    }
    if c == T::WIDTH {
        x_0 = x_1;
        x_1 = T::ZERO;
    } else {
        x_0 = (x_1 << (T::WIDTH - c)) | (x_0 >> c);
        x_1 >>= c;
    }
    while x_1 != T::ZERO || y_1 != T::ZERO {
        let (diff_1, diff_0) = T::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0);
        if diff_0 == T::ZERO && diff_1 == T::ZERO {
            return 0;
        }
        let mask = if diff_1.get_highest_bit() {
            T::MAX
        } else {
            T::ZERO
        };
        // If y > x, invoke reciprocity
        if (mask & x_0 & y_0).odd() {
            bit.not_assign();
        }
        // y <- min(x, y)
        (y_1, y_0) = T::xx_add_yy_to_zz(y_1, y_0, diff_1 & mask, diff_0 & mask);
        if y_1 == T::ZERO && y_0 == T::ZERO {
            return if bit { -1 } else { 1 };
        }
        // x <- |x - y|
        x_0 = (mask ^ diff_0).wrapping_sub(mask);
        x_1 = mask ^ diff_1;
        if x_0 == T::ZERO {
            // If y > x, x_0 == 0 implies that we have a carry to propagate.
            x_0 = x_1.wrapping_sub(mask);
            x_1 = T::ZERO;
        }
        let c = x_0.trailing_zeros() + 1;
        if c.odd() && (y_0 ^ (y_0 >> 1)).odd() {
            bit.not_assign();
        }
        if c == T::WIDTH {
            x_0 = x_1;
            x_1 = T::ZERO;
        } else {
            x_0 = (x_1 << (T::WIDTH - c)) | (x_0 >> c);
            x_1 >>= c;
        }
    }
    assert_ne!(y_0, T::ZERO);
    while (x_0 | y_0).get_highest_bit() {
        // Need an extra comparison to get the mask.
        let t = x_0.wrapping_sub(y_0);
        let mask = if y_0 > x_0 { T::MAX } else { T::ZERO };
        if t == T::ZERO {
            return 0;
        }
        // If y > x, invoke reciprocity
        if (mask & x_0 & y_0).odd() {
            bit.not_assign();
        }
        // y <- min(x, y)
        y_0.wrapping_add_assign(mask & t);
        // x <- |x - y|
        x_0 = (t ^ mask).wrapping_sub(mask);
        // Number of trailing zeros is the same no matter if we look at t or x, but using t gives
        // more parallelism.
        let c = t.trailing_zeros() + 1;
        // (2 / y) = -1 if y = 3 or 5 mod 8
        if c.odd() && (y_0 ^ (y_0 >> 1)).odd() {
            bit.not_assign();
        }
        if c == T::WIDTH {
            return if bit { -1 } else { 1 };
        }
        x_0 >>= c;
    }
    let j = ((x_0 << 1u32) | T::ONE).jacobi_symbol((y_0 << 1u32) | T::ONE);
    if bit {
        -j
    } else {
        j
    }
}
