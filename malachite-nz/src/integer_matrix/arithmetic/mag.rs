// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014, 2015 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, Parity, PowerOf2, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{LeadingZeros, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::*;

// The number of bits in the mantissa of a `Mag`.
const MAG_BITS: u64 = 30;

const MAG_BITS_I64: i64 = MAG_BITS as i64;

// The number of limbs in 64 bits.
const LIMBS_PER_WORD: usize = (u64::WIDTH >> Limb::LOG_WIDTH) as usize;

const MAG_ONE_HALF: u64 = 1 << (MAG_BITS - 1);

// A nonnegative number m * 2^(e - MAG_BITS), with a mantissa m of exactly `MAG_BITS` bits, or zero,
// with m = e = 0. Operations round their results up, so that the value is an upper bound for the
// exact one.
//
// This is FLINT's `mag_t` from `mag.h`, FLINT 3.6.0, without infinity, and with an exponent that
// fits in an `i64`, which is all that the matrix bounds need.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Mag {
    man: u64,
    exp: i64,
}

// The product of two `MAG_BITS`-bit mantissas, shifted right by `MAG_BITS`.
//
// This is `MAG_FIXMUL` from `mag.h`, FLINT 3.6.0.
const fn fix_mul(x: u64, y: u64) -> u64 {
    (x * y) >> MAG_BITS
}

// Bits 64k through 64k + 63 of `x`: the `k`th 64-bit limb, whatever the width of a `Limb`.
fn word(x: &Natural, k: u64) -> u64 {
    let limbs = match x {
        Natural(Small(small)) => core::slice::from_ref(small),
        Natural(Large(limbs)) => limbs.as_slice(),
    };
    let mut w = 0;
    for (i, &limb) in limbs
        .iter()
        .skip(usize::exact_from(k) * LIMBS_PER_WORD)
        .take(LIMBS_PER_WORD)
        .enumerate()
    {
        w |= u64::wrapping_from(limb) << (i << Limb::LOG_WIDTH);
    }
    w
}

// An upper bound m * 2^e for a positive `x`, where m has exactly `bits` bits.
//
// This is `fmpz_abs_ubound_ui_2exp` from `fmpz/abs_ubound_ui_2exp.c`, FLINT 3.6.0, with 64-bit
// limbs.
fn abs_ubound_ui_2exp(x: &Natural, bits: u64) -> (u64, i64) {
    let size = x.significant_bits().div_ceil(u64::WIDTH);
    if size == 1 {
        let mut m = word(x, 0);
        let mut e = i64::wrapping_from(u64::WIDTH - LeadingZeros::leading_zeros(m))
            - i64::wrapping_from(bits);
        if e >= 0 {
            m = (m >> e) + 1;
            // The increment overflowed to the next power of 2.
            if m.is_power_of_2() {
                m = u64::power_of_2(bits - 1);
                e += 1;
            }
        } else {
            m <<= -e;
        }
        (m, e)
    } else {
        let mut e = i64::wrapping_from((size - 1) << u64::LOG_WIDTH);
        let mut m = word(x, size - 1);
        let shift = i64::wrapping_from(u64::WIDTH - LeadingZeros::leading_zeros(m))
            - i64::wrapping_from(bits);
        e += shift;
        m = if shift >= 0 {
            (m >> shift) + 1
        } else {
            // Read a second limb to get an accurate value.
            let m2 = word(x, size - 2);
            ((m << -shift) | (m2 >> (const { u64::WIDTH as i64 } + shift))) + 1
        };
        // The increment overflowed to the next power of 2.
        if m.is_power_of_2() {
            m = u64::power_of_2(bits - 1);
            e += 1;
        }
        (m, e)
    }
}

impl Mag {
    pub(crate) const ZERO: Self = Self { man: 0, exp: 0 };

    pub(crate) const ONE: Self = Self {
        man: MAG_ONE_HALF,
        exp: 1,
    };

    pub(crate) const fn is_zero(self) -> bool {
        self.man == 0 && self.exp == 0
    }

    // An upper bound for `x`.
    //
    // This is `mag_set_fmpz` from `mag.h` and `mag_set_fmpz_2exp_fmpz` from
    // `mag/set_fmpz_2exp_fmpz.c`, FLINT 3.6.0.
    pub(crate) fn from_natural(x: &Natural) -> Self {
        if *x == 0u32 {
            Self::ZERO
        } else {
            let (man, exp) = abs_ubound_ui_2exp(x, MAG_BITS);
            Self {
                man,
                exp: exp + MAG_BITS_I64,
            }
        }
    }

    // Shifts the mantissa left by one bit if it has one bit too few.
    //
    // This is `MAG_FAST_ADJUST_ONE_TOO_SMALL` from `mag.h`, FLINT 3.6.0.
    fn adjust_one_too_small(&mut self) {
        let t = u64::from(self.man < MAG_ONE_HALF);
        self.man <<= t;
        self.exp -= i64::wrapping_from(t);
    }

    // Shifts the mantissa right by one bit, rounding up, if it has one bit too many.
    //
    // This is `MAG_FAST_ADJUST_ONE_TOO_LARGE` from `mag.h`, FLINT 3.6.0.
    fn adjust_one_too_large(&mut self) {
        let t = self.man >> MAG_BITS;
        self.man = (self.man >> t) + (t & self.man);
        self.exp += i64::wrapping_from(t);
    }

    // An upper bound for `x * y`.
    //
    // This is `mag_mul` from `mag/mul.c` and `mag_fast_mul` from `mag.h`, FLINT 3.6.0, which agree
    // on finite inputs.
    pub(crate) fn mul(x: Self, y: Self) -> Self {
        if x.man == 0 || y.man == 0 {
            Self::ZERO
        } else {
            let mut z = Self {
                man: fix_mul(x.man, y.man) + 1,
                exp: x.exp + y.exp,
            };
            z.adjust_one_too_small();
            z
        }
    }

    // Replaces `self` with an upper bound for `self + x * y`.
    //
    // This is `mag_fast_addmul` from `mag.h`, FLINT 3.6.0.
    pub(crate) fn add_mul_assign(&mut self, x: Self, y: Self) {
        if self.man == 0 {
            *self = Self::mul(x, y);
        } else if x.man != 0 && y.man != 0 {
            // x * y < 2^e
            let e = x.exp + y.exp;
            let shift = self.exp - e;
            let product = fix_mul(x.man, y.man);
            if shift >= 0 {
                if shift >= MAG_BITS_I64 {
                    self.man += 1;
                } else {
                    self.man += (product >> shift) + 1;
                }
            } else {
                let shift = -shift;
                self.exp = e;
                self.man = if shift >= MAG_BITS_I64 {
                    product + 2
                } else {
                    product + (self.man >> shift) + 2
                };
                self.adjust_one_too_small();
            }
            self.adjust_one_too_large();
        }
    }

    // An upper bound for `x`.
    //
    // This is `mag_set_ui` from `mag/set_ui.c`, FLINT 3.6.0.
    pub(crate) fn from_u64(x: u64) -> Self {
        if x == 0 {
            return Self::ZERO;
        }
        let mut bits = u64::WIDTH - LeadingZeros::leading_zeros(x);
        let mut x = x;
        if bits <= MAG_BITS {
            x <<= MAG_BITS - bits;
        } else {
            x = (x >> (bits - MAG_BITS)) + 1;
            let overflow = x >> MAG_BITS;
            bits += overflow;
            x >>= overflow;
        }
        Self {
            man: x,
            exp: i64::wrapping_from(bits),
        }
    }

    // A lower bound for `x`.
    //
    // This is `mag_set_ui_lower` from `mag/set_ui.c`, FLINT 3.6.0.
    pub(crate) fn from_u64_lower(x: u64) -> Self {
        if x == 0 {
            return Self::ZERO;
        }
        let bits = u64::WIDTH - LeadingZeros::leading_zeros(x);
        let x = if bits <= MAG_BITS {
            x << (MAG_BITS - bits)
        } else {
            x >> (bits - MAG_BITS)
        };
        Self {
            man: x,
            exp: i64::wrapping_from(bits),
        }
    }

    // An upper bound for `self * y`.
    //
    // This is `mag_mul_ui` from `mag.h`, FLINT 3.6.0.
    pub(crate) fn mul_u64(self, y: u64) -> Self {
        Self::mul(self, Self::from_u64(y))
    }

    // An upper bound for `x / y`, where `y` is positive.
    //
    // This is `mag_div` from `mag/div.c`, FLINT 3.6.0, for a positive divisor.
    fn div(x: Self, y: Self) -> Self {
        assert_ne!(y.man, 0);
        if x.man == 0 {
            return Self::ZERO;
        }
        let mut q = (x.man << MAG_BITS) / y.man + 1;
        let mut fix = q >> MAG_BITS;
        q = (q >> fix) + fix;
        // This could overflow to the next power of 2.
        if q >> MAG_BITS != 0 {
            q >>= 1;
            fix += 1;
        }
        Self {
            man: q,
            exp: x.exp - y.exp + i64::wrapping_from(fix),
        }
    }

    // An upper bound for `self / y`, where `y` is positive.
    //
    // This is `mag_div_ui` from `mag.h`, FLINT 3.6.0.
    pub(crate) fn div_u64(self, y: u64) -> Self {
        Self::div(self, Self::from_u64_lower(y))
    }

    // Compares `x` and `y`.
    //
    // This is `mag_cmp` from `mag/cmp.c`, FLINT 3.6.0, for finite inputs.
    pub(crate) fn cmp(x: Self, y: Self) -> Ordering {
        if x == y {
            Equal
        } else if x.man == 0 {
            Less
        } else if y.man == 0 {
            Greater
        } else if x.exp != y.exp {
            x.exp.cmp(&y.exp)
        } else if x.man < y.man {
            Less
        } else {
            Greater
        }
    }

    // The larger of `x` and `y`, or `x` if they are equal.
    //
    // This is `mag_max` from `mag.h`, FLINT 3.6.0.
    pub(crate) fn max(x: Self, y: Self) -> Self {
        if Self::cmp(x, y) == Less { y } else { x }
    }

    // The smaller of `x` and `y`, or `x` if they are equal.
    //
    // This is `mag_min` from `mag.h` and `mag_cmp` from `mag/cmp.c`, FLINT 3.6.0.
    pub(crate) fn min(x: Self, y: Self) -> Self {
        let x_le_y = if x == y || x.man == 0 {
            true
        } else if y.man == 0 {
            false
        } else {
            (x.exp, x.man) < (y.exp, y.man)
        };
        if x_le_y { x } else { y }
    }

    // An upper bound for the square root of `self`.
    //
    // This is `mag_sqrt` from `mag/sqrt.c`, FLINT 3.6.0, for an exponent of less than 2^61 in
    // absolute value (FLINT's "lagom" exponents), the only ones an `i64` exponent can reach here.
    pub(crate) fn sqrt(self) -> Self {
        if self.man == 0 {
            return self;
        }
        // The mantissa has 30 bits, so this conversion and scaling are exact.
        let mut t = self.man as f64 / (u64::power_of_2(MAG_BITS) as f64);
        let mut e = self.exp;
        if e.odd() {
            e = (e - 1) >> 1;
            t *= 2.0;
        } else {
            e >>= 1;
        }
        t = libm::sqrt(t) * (1.0 + 1e-13);
        // This is `MAG_SET_D_2EXP` from `mag.h`, FLINT 3.6.0.
        let (x, x_exp) = libm::frexp(t);
        let mut man = (x * u64::power_of_2(MAG_BITS) as f64) as u64 + 1;
        let fix = man >> MAG_BITS;
        man = (man >> fix) + fix;
        Self {
            man,
            exp: e + i64::from(x_exp) + i64::wrapping_from(fix),
        }
    }

    // The smallest integer that is at least `self`, which must be 0 or at least 1, so that its
    // exponent is positive.
    //
    // This is `mag_get_fmpz` from `mag/get_fmpz.c`, FLINT 3.6.0.
    pub(crate) fn ceiling(self) -> Natural {
        if self.man == 0 {
            return Natural::ZERO;
        }
        let shift = self.exp - MAG_BITS_I64;
        if shift >= 0 {
            Natural::from(self.man) << shift
        } else {
            Natural::from(self.man.shr_round(-shift, Ceiling).0)
        }
    }
}
