// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL as published by the Free Software Foundation; either version
// 3 of the License, or (at your option any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{RootRem, SqrtRem};
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::factorization::traits::{ExpressAsPower, IsPower, IsSquare};

// The following arrays are bitmasks indicating whether an integer is a 2, 3, or 5th power residue.
// For example, modulo 31 we have:
// - squares:    {0, 1, 2, 4, 5, 7, 8, 9, 10, 14, 16, 18, 19, 20, 25, 28}
// - cubes:      {0, 1, 2, 4, 8, 15, 16, 23, 27, 29, 30}
// - 5th powers: {0, 1, 5, 6, 25, 26, 30}
// Since 2 is a square, cube, but not a 5th power mod 31, we encode it as 011 = 3. Then MOD31[2] =
// 3.

const MOD63: [u8; 63] = [
    7, 7, 4, 0, 5, 4, 0, 5, 6, 5, 4, 4, 0, 4, 4, 0, 5, 4, 5, 4, 4, 0, 5, 4, 0, 5, 4, 6, 7, 4, 0, 4,
    4, 0, 4, 6, 7, 5, 4, 0, 4, 4, 0, 5, 4, 4, 5, 4, 0, 5, 4, 0, 4, 4, 4, 6, 4, 0, 5, 4, 0, 4, 6,
];

const MOD61: [u8; 61] = [
    7, 7, 0, 3, 1, 1, 0, 0, 2, 3, 0, 6, 1, 5, 5, 1, 1, 0, 0, 1, 3, 4, 1, 2, 2, 1, 0, 3, 2, 4, 0, 0,
    4, 2, 3, 0, 1, 2, 2, 1, 4, 3, 1, 0, 0, 1, 1, 5, 5, 1, 6, 0, 3, 2, 0, 0, 1, 1, 3, 0, 7,
];

const MOD44: [u8; 44] = [
    7, 7, 0, 2, 3, 3, 0, 2, 2, 3, 0, 6, 7, 2, 0, 2, 3, 2, 0, 2, 3, 6, 0, 6, 2, 3, 0, 2, 2, 2, 0, 2,
    6, 7, 0, 2, 3, 3, 0, 2, 2, 2, 0, 6,
];

const MOD31: [u8; 31] =
    [7, 7, 3, 0, 3, 5, 4, 1, 3, 1, 1, 0, 0, 0, 1, 2, 3, 0, 1, 1, 1, 0, 0, 2, 0, 5, 4, 2, 1, 2, 6];

const MOD72: [u8; 72] = [
    7, 7, 0, 0, 0, 7, 0, 7, 7, 7, 0, 7, 0, 7, 0, 0, 7, 7, 0, 7, 0, 0, 0, 7, 0, 7, 0, 7, 0, 7, 0, 7,
    7, 0, 0, 7, 0, 7, 0, 0, 7, 7, 0, 7, 0, 7, 0, 7, 0, 7, 0, 0, 0, 7, 0, 7, 7, 0, 0, 7, 0, 7, 0, 7,
    7, 7, 0, 7, 0, 0, 0, 7,
];

const MOD49: [u8; 49] = [
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

const MOD67: [u8; 67] = [
    2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0,
    0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2,
];

const MOD79: [u8; 79] = [
    4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
];

// This is n_is_power when FLINT64 is false, from ulong_extras/is_power.c, FLINT 3.1.2.
fn get_perfect_power_u32(n: u32) -> Option<(u32, u64)> {
    // Check for powers 2, 3, 5
    let mut t = MOD31[(n % 31) as usize];
    t &= MOD44[(n % 44) as usize];
    t &= MOD61[(n % 61) as usize];
    t &= MOD63[(n % 63) as usize];
    // Check for perfect square
    if t & 1 != 0 {
        let (rt, rem) = n.sqrt_rem();
        if rem == 0 {
            return Some((rt, 2));
        }
    }
    // Check for perfect cube
    if t & 2 != 0 {
        let (rt, rem) = n.root_rem(3);
        if rem == 0 {
            return Some((rt, 3));
        }
    }
    // Check for perfect fifth power
    if t & 4 != 0 {
        let (rt, rem) = n.root_rem(5);
        if rem == 0 {
            return Some((rt, 5));
        }
    }
    // Check for powers 7, 11, 13
    t = MOD49[(n % 49) as usize];
    t |= MOD67[(n % 67) as usize];
    t |= MOD79[(n % 79) as usize];
    t &= MOD72[(n % 72) as usize];

    // Check for perfect 7th power
    if t & 1 != 0 {
        let (rt, rem) = n.root_rem(7);
        if rem == 0 {
            return Some((rt, 7));
        }
    }
    // Check for perfect 11th power
    if t & 2 != 0 {
        let (rt, rem) = n.root_rem(11);
        if rem == 0 {
            return Some((rt, 11));
        }
    }
    // Check for perfect 13th power
    if t & 13 != 0 {
        let (rt, rem) = n.root_rem(13);
        if rem == 0 {
            return Some((rt, 13));
        }
    }
    // Handle powers of 2
    let count = u64::from(n.trailing_zeros());
    let mut n = n >> count;
    if n == 1 {
        return if count == 1 {
            None // Just 2^1 = 2, not a perfect power
        } else {
            Some((2, count))
        };
    }
    // Check other powers (exp >= 17, root <= 13 and odd)
    let mut exp = 0;
    while n % 3 == 0 {
        n /= 3;
        exp += 1;
    }
    if exp > 0 {
        if n == 1 && exp > 1 {
            if count == 0 {
                return Some((3, exp));
            } else if count == exp {
                return Some((6, exp));
            } else if count == 2 * exp {
                return Some((12, exp));
            }
        }
        return None;
    }
    None
}

// This is n_is_power when FLINT64 is false, from ulong_extras/is_power.c, FLINT 3.1.2, returning
// only whether n can be expressed as a nontrivial perfect power.
fn get_perfect_power_u32_bool(n: u32) -> bool {
    // Check for powers 2, 3, 5
    let mut t = MOD31[(n % 31) as usize];
    t &= MOD44[(n % 44) as usize];
    t &= MOD61[(n % 61) as usize];
    t &= MOD63[(n % 63) as usize];
    // Check for perfect square
    if t & 1 != 0 && n.is_square() {
        return true;
    }
    // Check for perfect cube
    if t & 2 != 0 && n.root_rem(3).1 == 0 {
        return true;
    }
    // Check for perfect fifth power
    if t & 4 != 0 && n.root_rem(5).1 == 0 {
        return true;
    }
    // Check for powers 7, 11, 13
    t = MOD49[(n % 49) as usize];
    t |= MOD67[(n % 67) as usize];
    t |= MOD79[(n % 79) as usize];
    t &= MOD72[(n % 72) as usize];
    // Check for perfect 7th power
    if t & 1 != 0 && n.root_rem(7).1 == 0 {
        return true;
    }
    // Check for perfect 11th power
    if t & 2 != 0 && n.root_rem(11).1 == 0 {
        return true;
    }
    // Check for perfect 13th power
    if t & 13 != 0 && n.root_rem(13).1 == 0 {
        return true;
    }
    // Handle powers of 2
    let count = n.trailing_zeros();
    let mut n = n >> n.trailing_zeros();
    if n == 1 {
        return count != 1;
    }
    // Check other powers (exp >= 17, root <= 13 and odd)
    let mut exp = 0;
    while n % 3 == 0 {
        n /= 3;
        exp += 1;
    }
    exp > 0 && n == 1 && exp > 1 && (count == 0 || count == exp || count == exp << 1)
}

// This is n_is_power when FLINT64 is true, from ulong_extras/is_power.c, FLINT 3.1.2.
fn get_perfect_power_u64(n: u64) -> Option<(u64, u64)> {
    // Check for powers 2, 3, 5
    let mut t = MOD31[(n % 31) as usize];
    t &= MOD44[(n % 44) as usize];
    t &= MOD61[(n % 61) as usize];
    t &= MOD63[(n % 63) as usize];
    // Check for perfect square
    if t & 1 != 0 {
        let (rt, rem) = n.sqrt_rem();
        if rem == 0 {
            return Some((rt, 2));
        }
    }
    // Check for perfect cube
    if t & 2 != 0 {
        let (rt, rem) = n.root_rem(3);
        if rem == 0 {
            return Some((rt, 3));
        }
    }
    // Check for perfect fifth power
    if t & 4 != 0 {
        let (rt, rem) = n.root_rem(5);
        if rem == 0 {
            return Some((rt, 5));
        }
    }
    // Check for powers 7, 11, 13
    t = MOD49[(n % 49) as usize];
    t |= MOD67[(n % 67) as usize];
    t |= MOD79[(n % 79) as usize];
    t &= MOD72[(n % 72) as usize];
    // Check for perfect 7th power
    if t & 1 != 0 {
        let (rt, rem) = n.root_rem(7);
        if rem == 0 {
            return Some((rt, 7));
        }
    }
    // Check for perfect 11th power
    if t & 2 != 0 {
        let (rt, rem) = n.root_rem(11);
        if rem == 0 {
            return Some((rt, 11));
        }
    }
    // Check for perfect 13th power
    if t & 13 != 0 {
        let (rt, rem) = n.root_rem(13);
        if rem == 0 {
            return Some((rt, 13));
        }
    }
    // Handle powers of 2
    let count = u64::from(n.trailing_zeros());
    let mut n = n >> count;
    if n == 1 {
        return if count == 1 {
            None // Just 2^1 = 2, not a perfect power
        } else {
            Some((2, count))
        };
    }
    // Check other powers (exp >= 17, root <= 13 and odd)
    let mut exp = 0;
    while n % 3 == 0 {
        n /= 3;
        exp += 1;
    }
    if exp > 0 {
        if n == 1 && exp > 1 {
            if count == 0 {
                return Some((3, exp));
            } else if count == exp {
                return Some((6, exp));
            } else if count == 2 * exp {
                return Some((12, exp));
            }
        }
        return None;
    }
    // Check powers of 5
    exp = 0;
    while n % 5 == 0 {
        n /= 5;
        exp += 1;
    }
    if exp > 0 {
        if n == 1 && exp > 1 {
            if count == 0 {
                return Some((5, exp));
            } else if count == exp {
                return Some((10, exp));
            }
        }
        return None;
    }
    if count > 0 {
        return None;
    }
    // Check powers of 7
    exp = 0;
    while n % 7 == 0 {
        n /= 7;
        exp += 1;
    }
    if exp > 0 {
        if n == 1 && exp > 1 {
            return Some((7, exp));
        }
        return None;
    }
    // Check powers of 11
    exp = 0;
    while n % 11 == 0 {
        n /= 11;
        exp += 1;
    }
    if exp > 0 {
        if n == 1 && exp > 1 {
            return Some((11, exp));
        }
        return None;
    }
    // Check powers of 13
    exp = 0;
    while n % 13 == 0 {
        n /= 13;
        exp += 1;
    }
    if exp > 0 {
        if n == 1 && exp > 1 {
            return Some((13, exp));
        }
        return None;
    }
    None
}

// This is n_is_power when FLINT64 is true, from ulong_extras/is_power.c, FLINT 3.1.2, returning
// only whether n can be expressed as a nontrivial perfect power.
fn get_perfect_power_u64_bool(n: u64) -> bool {
    // Check for powers 2, 3, 5
    let mut t = MOD31[(n % 31) as usize];
    t &= MOD44[(n % 44) as usize];
    t &= MOD61[(n % 61) as usize];
    t &= MOD63[(n % 63) as usize];
    // Check for perfect square
    if t & 1 != 0 && n.is_square() {
        return true;
    }
    // Check for perfect cube
    if t & 2 != 0 && n.root_rem(3).1 == 0 {
        return true;
    }
    // Check for perfect fifth power
    if t & 4 != 0 && n.root_rem(5).1 == 0 {
        return true;
    }
    // Check for powers 7, 11, 13
    t = MOD49[(n % 49) as usize];
    t |= MOD67[(n % 67) as usize];
    t |= MOD79[(n % 79) as usize];
    t &= MOD72[(n % 72) as usize];
    // Check for perfect 7th power
    if t & 1 != 0 && n.root_rem(7).1 == 0 {
        return true;
    }
    // Check for perfect 11th power
    if t & 2 != 0 && n.root_rem(11).1 == 0 {
        return true;
    }
    // Check for perfect 13th power
    if t & 13 != 0 && n.root_rem(13).1 == 0 {
        return true;
    }
    // Handle powers of 2
    let count = u64::from(n.trailing_zeros());
    let mut n = n >> count;
    if n == 1 {
        return count != 1;
    }
    // Check other powers (exp >= 17, root <= 13 and odd)
    let mut exp = 0;
    while n % 3 == 0 {
        n /= 3;
        exp += 1;
    }
    if exp > 0 {
        return n == 1 && exp > 1 && (count == 0 || count == exp || count == exp << 1);
    }
    // Check powers of 5
    exp = 0;
    while n % 5 == 0 {
        n /= 5;
        exp += 1;
    }
    if exp > 0 {
        return n == 1 && exp > 1 && (count == 0 || count == exp);
    }
    if count > 0 {
        return false;
    }
    // Check powers of 7
    exp = 0;
    while n % 7 == 0 {
        n /= 7;
        exp += 1;
    }
    if exp > 0 {
        return n == 1 && exp > 1;
    }
    // Check powers of 11
    exp = 0;
    while n % 11 == 0 {
        n /= 11;
        exp += 1;
    }
    if exp > 0 {
        return n == 1 && exp > 1;
    }
    // Check powers of 13
    exp = 0;
    while n % 13 == 0 {
        n /= 13;
        exp += 1;
    }
    n == 1 && exp > 1
}

fn express_as_power_u32(n: u32) -> Option<(u32, u64)> {
    if n <= 1 {
        return Some((n, 2));
    }
    // continue until we have largest possible exponent
    let (mut base, mut exp) = get_perfect_power_u32(n)?;
    while base > 3 {
        match get_perfect_power_u32(base) {
            Some((base2, exp2)) => {
                base = base2;
                exp *= exp2;
            }
            None => {
                return Some((base, exp));
            }
        }
    }
    Some((base, exp))
}

fn express_as_power_u64(n: u64) -> Option<(u64, u64)> {
    if n <= 1 {
        return Some((n, 2));
    }
    // continue until we have largest possible exponent
    let (mut base, mut exp) = get_perfect_power_u64(n)?;
    while base > 3 {
        match get_perfect_power_u64(base) {
            Some((base2, exp2)) => {
                base = base2;
                exp *= exp2;
            }
            None => {
                return Some((base, exp));
            }
        }
    }
    Some((base, exp))
}

fn express_as_power_i32(n: i32) -> Option<(i32, u64)> {
    if n == 0 || n == 1 {
        return Some((n, 2));
    }
    // continue until we have largest possible exponent
    let (mut base, mut exp) = get_perfect_power_u32(n.unsigned_abs())?;
    while base > 3 {
        match get_perfect_power_u32(base) {
            Some((base2, exp2)) => {
                base = base2;
                exp *= exp2;
            }
            None => break,
        }
    }
    // handle negative input
    if n < 0 && (exp & 1) == 0 {
        while (exp & 1) == 0 {
            base *= base;
            exp >>= 1;
        }
        if exp == 1 {
            return None;
        }
    }
    let ibase = i32::exact_from(base);
    Some((if n >= 0 { ibase } else { -ibase }, exp))
}

fn express_as_power_i64(n: i64) -> Option<(i64, u64)> {
    if n == 0 || n == 1 {
        return Some((n, 2));
    }
    // continue until we have largest possible exponent
    let (mut base, mut exp) = get_perfect_power_u64(n.unsigned_abs())?;
    while base > 3 {
        match get_perfect_power_u64(base) {
            Some((base2, exp2)) => {
                base = base2;
                exp *= exp2;
            }
            None => break,
        }
    }
    // handle negative input
    if n < 0 && (exp & 1) == 0 {
        while (exp & 1) == 0 {
            base *= base;
            exp >>= 1;
        }
        if exp == 1 {
            return None;
        }
    }
    let ibase = i64::exact_from(base);
    Some((if n >= 0 { ibase } else { -ibase }, exp))
}

#[inline]
fn is_power_u32(n: u32) -> bool {
    n <= 1 || get_perfect_power_u32_bool(n)
}

#[inline]
fn is_power_u64(n: u64) -> bool {
    n <= 1 || get_perfect_power_u64_bool(n)
}

fn is_power_i32(n: i32) -> bool {
    if n == 0 || n == 1 {
        return true;
    }
    // continue until we have largest possible exponent
    let (mut base, mut exp) = if let Some((base, exp)) = get_perfect_power_u32(n.unsigned_abs()) {
        (base, exp)
    } else {
        return false;
    };
    while base > 3 {
        match get_perfect_power_u32(base) {
            Some((base2, exp2)) => {
                base = base2;
                exp *= exp2;
            }
            None => break,
        }
    }
    // handle negative input
    if n < 0 && (exp & 1) == 0 {
        while (exp & 1) == 0 {
            base *= base;
            exp >>= 1;
        }
        exp != 1
    } else {
        true
    }
}

fn is_power_i64(n: i64) -> bool {
    if n == 0 || n == 1 {
        return true;
    }
    // continue until we have largest possible exponent
    let (mut base, mut exp) = if let Some((base, exp)) = get_perfect_power_u64(n.unsigned_abs()) {
        (base, exp)
    } else {
        return false;
    };
    while base > 3 {
        match get_perfect_power_u64(base) {
            Some((base2, exp2)) => {
                base = base2;
                exp *= exp2;
            }
            None => break,
        }
    }
    // handle negative input
    if n < 0 && (exp & 1) == 0 {
        while (exp & 1) == 0 {
            base *= base;
            exp >>= 1;
        }
        exp != 1
    } else {
        true
    }
}

impl ExpressAsPower for u64 {
    /// Expresses a number as a perfect power, if such a representation exists. We define a perfect
    /// power as any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In
    /// particular, 0 and 1 are considered perfect powers. If a number has more than one
    /// representation as a power, the representation with the smallest base is returned. For
    /// example, $64=2^6=4^3=8^2$, but this function returns `(2,6)` rather than `(4,3)` or `(8,2)`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#express_as_power).
    ///
    /// # Notes
    /// - This returns an [`Option`] which is either `Some((base, exp))` if the input is a perfect
    ///   power equal to $\text{base}^\text{exp}$, otherwise `None`.
    /// - For 0 this returns `Some((0, 2))` and for 1 this returns `Some((1, 2))`.
    #[inline]
    fn express_as_power(&self) -> Option<(u64, u64)> {
        express_as_power_u64(*self)
    }
}

impl ExpressAsPower for usize {
    /// Expresses a number as a perfect power, if such a representation exists. We define a perfect
    /// power as any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In
    /// particular, 0 and 1 are considered perfect powers. If a number has more than one
    /// representation as a power, the representation with the smallest base is returned. For
    /// example, $64=2^6=4^3=8^2$, but this function returns `(2,6)` rather than `(4,3)` or `(8,2)`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#express_as_power).
    ///
    /// # Notes
    /// - This returns an [`Option`] which is either `Some((base, exp))` if the input is a perfect
    ///   power equal to $\text{base}^\text{exp}$, otherwise `None`.
    /// - For 0 this returns `Some((0, 2))` and for 1 this returns `Some((1, 2))`.
    fn express_as_power(&self) -> Option<(usize, u64)> {
        if USIZE_IS_U32 {
            match express_as_power_u32(u32::wrapping_from(*self)) {
                Some((base, exp)) => Some((usize::wrapping_from(base), exp)),
                _ => None,
            }
        } else {
            match express_as_power_u64(u64::wrapping_from(*self)) {
                Some((base, exp)) => Some((usize::wrapping_from(base), exp)),
                _ => None,
            }
        }
    }
}

impl IsPower for u64 {
    /// Determines whether an integer is a perfect power. We define a perfect power as any number of
    /// the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In particular 0 and 1 are
    /// considered perfect powers.
    ///
    /// $f(x) = (\exists b \in \Z, e \in \N : e > 1 \ \text{and} \ b^e = x)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#is_power).
    #[inline]
    fn is_power(&self) -> bool {
        is_power_u64(*self)
    }
}

impl IsPower for usize {
    /// Determines whether an integer is a perfect power. We define a perfect power as any number of
    /// the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In particular 0 and 1 are
    /// considered perfect powers.
    ///
    /// $f(x) = (\exists b \in \Z, e \in \N : e > 1 \ \text{and} \ b^e = x)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#is_power).
    fn is_power(&self) -> bool {
        if USIZE_IS_U32 {
            is_power_u32(u32::wrapping_from(*self))
        } else {
            is_power_u64(u64::wrapping_from(*self))
        }
    }
}

macro_rules! impl_unsigned_32 {
    ($t: ident) => {
        impl ExpressAsPower for $t {
            /// Expresses a number as a perfect power, if such a representation exists. We define a
            /// perfect power as any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both
            /// integers. In particular, 0 and 1 are considered perfect powers. If a number has more
            /// than one representation as a power, the representation with the smallest base is
            /// returned. For example, $64=2^6=4^3=8^2$, but this function returns `(2,6)` rather
            /// than `(4,3)` or `(8,2)`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_power#express_as_power).
            ///
            /// # Notes
            /// - This returns an [`Option`] which is either `Some((base, exp))` if the input is a
            ///   perfect power equal to $\text{base}^\text{exp}$, otherwise `None`.
            /// - For 0 this returns `Some((0, 2))` and for 1 this returns `Some((1, 2))`.
            fn express_as_power(&self) -> Option<($t, u64)> {
                match express_as_power_u32(u32::from(*self)) {
                    Some((base, exp)) => Some(($t::exact_from(base), exp)),
                    _ => None,
                }
            }
        }

        impl IsPower for $t {
            /// Determines whether an integer is a perfect power. We define a perfect power as any
            /// number of the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In
            /// particular 0 and 1 are considered perfect powers.
            ///
            /// $f(x) = (\exists b \in \Z, e \in \N : e > 1 \ \text{and} \ b^e = x)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_power#is_power).
            #[inline]
            fn is_power(&self) -> bool {
                is_power_u32(u32::from(*self))
            }
        }
    };
}
impl_unsigned_32!(u8);
impl_unsigned_32!(u16);
impl_unsigned_32!(u32);

impl ExpressAsPower for i64 {
    /// Expresses a number as a perfect power, if such a representation exists. We define a perfect
    /// power as any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In
    /// particular, 0 and 1 are considered perfect powers. If a number has more than one
    /// representation as a power, the representation with the smallest base is returned. For
    /// example, $64=2^6=4^3=8^2$, but this function returns `(2,6)` rather than `(4,3)` or `(8,2)`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#express_as_power).
    ///
    /// # Notes
    /// - This returns an [`Option`] which is either `Some((base, exp))` if the input is a perfect
    ///   power equal to $\text{base}^\text{exp}$, otherwise `None`.
    /// - For 0 this returns `Some((0, 2))` and for 1 this returns `Some((1, 2))`.
    #[inline]
    fn express_as_power(&self) -> Option<(i64, u64)> {
        express_as_power_i64(*self)
    }
}

impl ExpressAsPower for isize {
    /// Expresses a number as a perfect power, if such a representation exists. We define a perfect
    /// power as any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In
    /// particular, 0 and 1 are considered perfect powers. If a number has more than one
    /// representation as a power, the representation with the smallest base is returned. For
    /// example, $64=2^6=4^3=8^2$, but this function returns `(2,6)` rather than `(4,3)` or `(8,2)`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#express_as_power).
    ///
    /// # Notes
    /// - This returns an [`Option`] which is either `Some((base, exp))` if the input is a perfect
    ///   power equal to $\text{base}^\text{exp}$, otherwise `None`.
    /// - For 0 this returns `Some((0, 2))` and for 1 this returns `Some((1, 2))`.
    fn express_as_power(&self) -> Option<(isize, u64)> {
        if USIZE_IS_U32 {
            match express_as_power_i32(i32::wrapping_from(*self)) {
                Some((base, exp)) => Some((isize::wrapping_from(base), exp)),
                _ => None,
            }
        } else {
            match express_as_power_i64(i64::wrapping_from(*self)) {
                Some((base, exp)) => Some((isize::wrapping_from(base), exp)),
                _ => None,
            }
        }
    }
}

impl IsPower for i64 {
    /// Determines whether an integer is a perfect power. We define a perfect power as any number of
    /// the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In particular 0 and 1 are
    /// considered perfect powers.
    ///
    /// $f(x) = (\exists b \in \Z, e \in \N : e > 1 \ \text{and} \ b^e = x)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#is_power).
    #[inline]
    fn is_power(&self) -> bool {
        is_power_i64(*self)
    }
}

impl IsPower for isize {
    /// Determines whether an integer is a perfect power. We define a perfect power as any number of
    /// the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In particular 0 and 1 are
    /// considered perfect powers.
    ///
    /// $f(x) = (\exists b \in \Z, e \in \N : e > 1 \ \text{and} \ b^e = x)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_power#is_power).
    fn is_power(&self) -> bool {
        if USIZE_IS_U32 {
            is_power_i32(i32::wrapping_from(*self))
        } else {
            is_power_i64(i64::wrapping_from(*self))
        }
    }
}

macro_rules! impl_signed_32 {
    ($t: ident) => {
        impl ExpressAsPower for $t {
            /// Expresses a number as a perfect power, if such a representation exists. We define a
            /// perfect power as any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both
            /// integers. In particular, 0 and 1 are considered perfect powers. If a number has more
            /// than one representation as a power, the representation with the smallest base is
            /// returned. For example, $64=2^6=4^3=8^2$, but this function returns `(2,6)` rather
            /// than `(4,3)` or `(8,2)`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_power#express_as_power).
            ///
            /// # Notes
            /// - This returns an [`Option`] which is either `Some((base, exp))` if the input is a
            ///   perfect power equal to $\text{base}^\text{exp}$, otherwise `None`.
            /// - For 0 this returns `Some((0, 2))` and for 1 this returns `Some((1, 2))`.
            fn express_as_power(&self) -> Option<($t, u64)> {
                match express_as_power_i32(i32::from(*self)) {
                    Some((base, exp)) => Some(($t::exact_from(base), exp)),
                    _ => None,
                }
            }
        }

        impl IsPower for $t {
            /// Determines whether an integer is a perfect power. We define a perfect power as any
            /// number of the form $a^x$ where $x > 1$, with $a$ and $x$ both integers. In
            /// particular 0 and 1 are considered perfect powers.
            ///
            /// $f(x) = (\exists b \in \Z, e \in \N : e > 1 \ \text{and} \ b^e = x)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_power#is_power).
            #[inline]
            fn is_power(&self) -> bool {
                is_power_i32(i32::from(*self))
            }
        }
    };
}
impl_signed_32!(i8);
impl_signed_32!(i16);
impl_signed_32!(i32);
