// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 Tom Boothby
//
//      Copyright © 2008, 2009, 2012 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL as published by the Free Software Foundation; either version
// 3 of the License, or (at your option any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mod_pow::mul_mod_helper;
use crate::num::arithmetic::sqrt::{sqrt_rem_2_newton, sqrt_rem_newton};
use crate::num::arithmetic::traits::{
    DivMod, FloorRoot, FloorSqrt, Gcd, ModMulPrecomputed, ModSub, ModSubAssign, Parity, PowerOf2,
    SqrtRem, Square, WrappingAddAssign, WrappingMulAssign, WrappingSquare, WrappingSubAssign,
    XMulYToZZ, XXDivModYToQR, XXSubYYToZZ,
};
use crate::num::basic::integers::{PrimitiveInt, USIZE_IS_U32};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::factorization::primes::SMALL_PRIMES;
use crate::num::factorization::traits::{Factor, IsPrime, Primes};
use crate::num::logic::traits::{LeadingZeros, LowMask, SignificantBits};
use core::mem::swap;

const MAX_FACTORS_IN_U8: usize = 4;
const MAX_FACTORS_IN_U16: usize = 6;
const MAX_FACTORS_IN_U32: usize = 9;
const MAX_FACTORS_IN_U64: usize = 15;
const MAX_FACTORS_IN_USIZE: usize = 15;

/// A struct that contains the prime factorization of an integer. See implementations of the
/// [`Factor`] trait for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Factors<T: PrimitiveUnsigned, const N: usize> {
    factors: [T; N],
    exponents: [u8; N],
}

/// An iterator over [`Factors`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FactorsIterator<T: PrimitiveUnsigned, const N: usize> {
    i: usize,
    factors: Factors<T, N>,
}

impl<T: PrimitiveUnsigned, const N: usize> Iterator for FactorsIterator<T, N> {
    type Item = (T, u8);

    fn next(&mut self) -> Option<(T, u8)> {
        let e = *self.factors.exponents.get(self.i)?;
        if e == 0 {
            return None;
        }
        let f = self.factors.factors[self.i];
        self.i += 1;
        Some((f, e))
    }
}

impl<T: PrimitiveUnsigned, const N: usize> IntoIterator for Factors<T, N> {
    type IntoIter = FactorsIterator<T, N>;
    type Item = (T, u8);

    fn into_iter(self) -> FactorsIterator<T, N> {
        FactorsIterator {
            i: 0,
            factors: self,
        }
    }
}

impl<T: PrimitiveUnsigned, const N: usize> Factors<T, N> {
    const fn new() -> Factors<T, N> {
        Factors {
            factors: [T::ZERO; N],
            exponents: [0; N],
        }
    }

    // This takes linear time in the number of factors, but that's ok because the number of factors
    // is small.
    //
    // This is n_factor_insert from ulong_extras/factor_insert.c, FLINT 3.1.2, but it also ensures
    // that the factors are ordered least to greatest.
    fn insert(&mut self, factor: T, exp: u8) {
        let mut inserting = false;
        let mut previous_f = T::ZERO;
        let mut previous_e = 0;
        for (f, e) in self.factors.iter_mut().zip(self.exponents.iter_mut()) {
            if inserting {
                swap(&mut previous_f, f);
                swap(&mut previous_e, e);
                if previous_e == 0 {
                    break;
                }
            } else if *e == 0 {
                *f = factor;
                *e = exp;
                break;
            } else if *f == factor {
                *e += exp;
                break;
            } else if *f > factor {
                previous_f = *f;
                previous_e = *e;
                *f = factor;
                *e = exp;
                inserting = true;
            }
        }
    }
}

type FactorsU8 = Factors<u8, MAX_FACTORS_IN_U8>;
type FactorsU16 = Factors<u16, MAX_FACTORS_IN_U16>;
type FactorsU32 = Factors<u32, MAX_FACTORS_IN_U32>;
type FactorsU64 = Factors<u64, MAX_FACTORS_IN_U64>;
type FactorsUsize = Factors<usize, MAX_FACTORS_IN_USIZE>;

// This is n_divrem2_precomp when FLINT64 is false, from ulong_extras/divrem2_precomp.c, FLINT
// 3.1.2, simplified to only include the branches used when factoring a `u32`.
fn div_rem_precomputed_float_for_u32_factorization(a: u32, n: u32, inverse: f64) -> (u32, u32) {
    let mut q = (f64::from(a) * inverse) as u32;
    let r = a.wrapping_sub(q * n);
    let ri = i32::wrapping_from(r);
    if ri >= i32::wrapping_from(n) {
        q += (f64::from(ri) * inverse) as u32;
        (q + 1, a.wrapping_sub(q * n).wrapping_sub(n))
    } else {
        (q, r)
    }
}

// This is n_divrem2_precomp when FLINT64 is true, from ulong_extras/divrem2_precomp.c, FLINT 3.1.2,
// returning both q and r.
fn div_rem_precomputed_float_u64(a: u64, n: u64, npre: f64) -> (u64, u64) {
    if a < n {
        return (0, a);
    }
    if n.get_highest_bit() {
        return (1, a.wrapping_sub(n));
    }
    let (mut q, r) = if n == 1 {
        (a, 0)
    } else {
        let q = ((a as f64) * npre) as u64;
        (q, a.wrapping_sub(q.wrapping_mul(n)))
    };
    let ri = i64::wrapping_from(r);
    let ni = i64::wrapping_from(n);
    if ri < ni.wrapping_neg() {
        q -= ((-(ri as f64)) * npre) as u64;
    } else if ri >= ni {
        q += ((ri as f64) * npre) as u64;
    } else if ri < 0 {
        return (q - 1, r.wrapping_add(n));
    } else {
        return (q, r);
    }
    let r = a.wrapping_sub(q.wrapping_mul(n));
    let ri = i64::wrapping_from(r);
    if ri >= ni {
        (q + 1, r.wrapping_sub(n))
    } else if ri < 0 {
        (q - 1, r.wrapping_add(n))
    } else {
        (q, r)
    }
}

// This is n_remove2_precomp when FLINT64 is false, from ulong_extras/remove2_precomp.c, FLINT
// 3.1.2, returning n and exp.
fn remove_factor_precomputed_float_u32(mut n: u32, p: u32, inverse: f64) -> (u32, u8) {
    if p == 2 {
        let exp = n.trailing_zeros();
        if exp != 0 {
            n >>= exp;
        }
        (n, u8::wrapping_from(exp))
    } else {
        let mut exp = 0;
        while n >= p {
            let (q, r) = div_rem_precomputed_float_for_u32_factorization(n, p, inverse);
            if r != 0 {
                break;
            }
            exp += 1;
            n = q;
        }
        (n, exp)
    }
}

// This is n_remove2_precomp when FLINT64 is true, from ulong_extras/remove2_precomp.c, FLINT 3.1.2,
// returning n and exp.
fn remove_factor_precomputed_float_u64(mut n: u64, p: u64, inverse: f64) -> (u64, u8) {
    if p == 2 {
        let exp = u64::from(n.trailing_zeros());
        if exp != 0 {
            n >>= exp;
        }
        (n, u8::wrapping_from(exp))
    } else {
        let mut exp = 0;
        while n >= p {
            let (q, r) = div_rem_precomputed_float_u64(n, p, inverse);
            if r != 0 {
                break;
            }
            exp += 1;
            n = q;
        }
        (n, exp)
    }
}

// This is n_factor_trial_range when FLINT64 is false, from ulong_extras/factor_trial.c, FLINT
// 3.1.2, where start == 0.
fn factor_trial_range_u32(factors: &mut FactorsU32, mut n: u32, num_primes: usize) -> u32 {
    for p in u32::primes().take(num_primes) {
        if p.square() > n {
            break;
        }
        let exp;
        (n, exp) = remove_factor_precomputed_float_u32(n, p, 1.0 / f64::from(p));
        if exp != 0 {
            factors.insert(p, exp);
        }
    }
    n
}

// This is n_factor_trial_range when FLINT64 is true, from ulong_extras/factor_trial.c, FLINT 3.1.2.
fn factor_trial_range_u64(factors: &mut FactorsU64, mut n: u64, num_primes: usize) -> u64 {
    for p in u64::primes().take(num_primes) {
        if p.square() > n {
            break;
        }
        let exp;
        (n, exp) = remove_factor_precomputed_float_u64(n, p, 1.0 / (p as f64));
        if exp != 0 {
            factors.insert(p, exp);
        }
    }
    n
}

const POWER235_MOD63: [u8; 63] = [
    7, 7, 4, 0, 5, 4, 0, 5, 6, 5, 4, 4, 0, 4, 4, 0, 5, 4, 5, 4, 4, 0, 5, 4, 0, 5, 4, 6, 7, 4, 0, 4,
    4, 0, 4, 6, 7, 5, 4, 0, 4, 4, 0, 5, 4, 4, 5, 4, 0, 5, 4, 0, 4, 4, 4, 6, 4, 0, 5, 4, 0, 4, 6,
];
const POWER235_MOD61: [u8; 61] = [
    7, 7, 0, 3, 1, 1, 0, 0, 2, 3, 0, 6, 1, 5, 5, 1, 1, 0, 0, 1, 3, 4, 1, 2, 2, 1, 0, 3, 2, 4, 0, 0,
    4, 2, 3, 0, 1, 2, 2, 1, 4, 3, 1, 0, 0, 1, 1, 5, 5, 1, 6, 0, 3, 2, 0, 0, 1, 1, 3, 0, 7,
];
const POWER235_MOD44: [u8; 44] = [
    7, 7, 0, 2, 3, 3, 0, 2, 2, 3, 0, 6, 7, 2, 0, 2, 3, 2, 0, 2, 3, 6, 0, 6, 2, 3, 0, 2, 2, 2, 0, 2,
    6, 7, 0, 2, 3, 3, 0, 2, 2, 2, 0, 6,
];
const POWER235_MOD31: [u8; 31] =
    [7, 7, 3, 0, 3, 5, 4, 1, 3, 1, 1, 0, 0, 0, 1, 2, 3, 0, 1, 1, 1, 0, 0, 2, 0, 5, 4, 2, 1, 2, 6];

// This is n_factor_power235 when FLINT64 is false, from ulong_extras/factor_power235.c, FLINT
// 3.1.2, returning y and exp, and simplified to only include the branches used when factoring a
// `u32`. In particular, only perfect squares are checked for.
fn factor_square_u32(n: u32) -> (u32, u8) {
    let mut t = POWER235_MOD31[(n % 31) as usize];
    if t == 0 {
        return (0, 0);
    };
    t &= POWER235_MOD44[(n % 44) as usize];
    if t == 0 {
        return (0, 0);
    };
    t &= POWER235_MOD61[(n % 61) as usize];
    if t == 0 {
        return (0, 0);
    };
    t &= POWER235_MOD63[(n % 63) as usize];
    if t.odd() {
        let (y, r) = n.sqrt_rem();
        if r == 0 {
            return (y, 2);
        }
    }
    (0, 0)
}

// This is n_factor_power235 when FLINT64 is true, from ulong_extras/factor_power235.c, FLINT 3.1.2,
// returning y and exp. Fifth powers are not checked for, because this function is only called on
// values with no prime factor less than 27449, and 27449^5 is greater than 2^64.
fn factor_power23_u64(n: u64) -> (u64, u8) {
    let mut t = POWER235_MOD31[(n % 31) as usize];
    if t == 0 {
        return (0, 0);
    };
    t &= POWER235_MOD44[(n % 44) as usize];
    if t == 0 {
        return (0, 0);
    };
    t &= POWER235_MOD61[(n % 61) as usize];
    if t == 0 {
        return (0, 0);
    };
    t &= POWER235_MOD63[(n % 63) as usize];
    if t.odd() {
        let (y, r) = n.sqrt_rem();
        if r == 0 {
            return (y, 2);
        }
    }
    if t & 2 != 0 {
        let y = n.floor_root(3);
        if n == y.pow(3) {
            return (y, 3);
        }
    }
    (0, 0)
}

const IS_SQUARE_MOD64: [bool; 64] = [
    true, true, false, false, true, false, false, false, false, true, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, true, false, false,
    false, false, false, false, false, true, false, false, true, false, false, false, false, true,
    false, false, false, false, false, false, false, true, false, false, false, false, false,
    false, false, true, false, false, false, false, false, false,
];

const IS_SQUARE_MOD65: [bool; 65] = [
    true, true, false, false, true, false, false, false, false, true, true, false, false, false,
    true, false, true, false, false, false, false, false, false, false, false, true, true, false,
    false, true, true, false, false, false, false, true, true, false, false, true, true, false,
    false, false, false, false, false, false, false, true, false, true, false, false, false, true,
    true, false, false, false, false, true, false, false, true,
];

const IS_SQUARE_MOD63: [bool; 63] = [
    true, true, false, false, true, false, false, true, false, true, false, false, false, false,
    true, false, true, false, true, false, false, false, true, false, false, true, false, false,
    true, false, false, false, false, false, false, true, true, true, false, false, false, false,
    false, true, false, false, true, false, false, true, false, false, false, false, false, false,
    true, false, true, false, false, false, false,
];

// This is n_is_square when FLINT64 is false, from ulong_extras/is_square.c, FLINT 3.1.2.
fn is_square_u64(x: u64) -> bool {
    IS_SQUARE_MOD64[(x % 64) as usize]
        && IS_SQUARE_MOD63[(x % 63) as usize]
        && IS_SQUARE_MOD65[(x % 65) as usize]
        && x.floor_sqrt().square() == x
}

const FLINT_ONE_LINE_MULTIPLIER: u32 = 480;

// This is n_factor_one_line when FLINT64 is true, from ulong_extras/factor_one_line.c, FLINT 3.1.2.
fn factor_one_line_u64(mut n: u64, iters: usize) -> u64 {
    let orig_n = n;
    n.wrapping_mul_assign(u64::from(FLINT_ONE_LINE_MULTIPLIER));
    let mut iin = 0;
    let mut inn = n;
    for _ in 0..iters {
        if iin >= inn {
            break;
        }
        let mut sqrti = inn.floor_sqrt() + 1;
        let square = sqrti.square();
        let mmod = square - inn;
        if is_square_u64(mmod) {
            sqrti -= mmod.floor_sqrt();
            let factor = orig_n.gcd(sqrti);
            if factor != 1 {
                return factor;
            }
        }
        iin = inn;
        inn.wrapping_add_assign(n);
    }
    0
}

fn wyhash64(seed: &mut u64) -> u64 {
    seed.wrapping_add_assign(0x60bee2bee120fc15);
    let tmp = u128::from(*seed) * 0xa3b195354a39b70d;
    let tmp = ((tmp >> 64) ^ tmp) * 0x1b03738712fad5c9;
    u64::wrapping_from((tmp >> 64) ^ tmp)
}

struct WyhashRandomU64s {
    seed: u64,
}

impl WyhashRandomU64s {
    const fn new() -> WyhashRandomU64s {
        WyhashRandomU64s {
            seed: 0x452aee49c457bbc3,
        }
    }
}

impl Iterator for WyhashRandomU64s {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(wyhash64(&mut self.seed))
    }
}

// This is n_factor_pp1_table from ulong_extras/factor_pp1.c, FLINT 3.1.2.
const N_FACTOR_PP1_TABLE: [(u16, u8); 34] = [
    (2784, 5),
    (1208, 2),
    (2924, 3),
    (286, 5),
    (58, 5),
    (61, 4),
    (815, 2),
    (944, 2),
    (61, 3),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (0, 0),
    (606, 1),
    (2403, 1),
    (2524, 1),
    (2924, 1),
    (3735, 2),
    (669, 2),
    (6092, 3),
    (2179, 3),
    (3922, 3),
    (6717, 4),
    (4119, 4),
    (2288, 4),
    (9004, 3),
    (9004, 3),
    (9004, 3),
];

// This is n_pp1_pow_ui when FLINT64 is true, from ulong_extras/factor_pp1.c, FLINT 3.1.2, returning
// the new values of x and y. y is not passed in as its initial value is never used.
fn pp1_pow_ui_u64(mut x: u64, exp: u64, n: u64, ninv: u64, norm: u64) -> (u64, u64) {
    let x_orig = x;
    let two = u64::power_of_2(norm + 1);
    let mut y = mul_mod_helper::<u64, u128>(x, x, n, ninv, norm).mod_sub(two, n);
    let mut bit = u64::power_of_2(exp.significant_bits() - 2);
    while bit != 0 {
        (x, y) = if exp & bit != 0 {
            (
                mul_mod_helper::<u64, u128>(x, y, n, ninv, norm).mod_sub(x_orig, n),
                mul_mod_helper::<u64, u128>(y, y, n, ninv, norm).mod_sub(two, n),
            )
        } else {
            (
                mul_mod_helper::<u64, u128>(x, x, n, ninv, norm).mod_sub(two, n),
                mul_mod_helper::<u64, u128>(y, x, n, ninv, norm).mod_sub(x_orig, n),
            )
        };
        bit >>= 1;
    }
    (x, y)
}

// This is n_pp1_factor when FLINT64 is true, from ulong_extras/factor_pp1.c, FLINT 3.1.2.
fn pp1_factor_u64(mut n: u64, mut x: u64, norm: u64) -> u64 {
    if norm != 0 {
        n >>= norm;
        x >>= norm;
    }
    x.mod_sub_assign(2, n);
    if x == 0 {
        0
    } else {
        n.gcd(x)
    }
}

// This is n_pp1_find_power when FLINT64 is true, from ulong_extras/factor_pp1.c, FLINT 3.1.2,
// returning factor and the new values of x and y. y is not passed in as its initial value is never
// used.
fn pp1_find_power_u64(mut x: u64, p: u64, n: u64, ninv: u64, norm: u64) -> (u64, u64, u64) {
    let mut factor = 1;
    let mut y = 0;
    while factor == 1 {
        (x, y) = pp1_pow_ui_u64(x, p, n, ninv, norm);
        factor = pp1_factor_u64(n, x, norm);
    }
    (factor, x, y)
}

// This is n_factor_pp1 when FLINT64 is false, from ulong_extras/factor_pp1.c, FLINT 3.1.2. It is
// assumed that n is odd.
fn factor_pp1_u64(mut n: u64, b1: u64, c: u64) -> u64 {
    let mut primes = u64::primes();
    let sqrt = b1.floor_sqrt();
    let bits0 = b1.significant_bits();
    let norm = LeadingZeros::leading_zeros(n);
    n <<= norm;
    let n_inverse = u64::precompute_mod_mul_data(&n);
    let mut x = c << norm;
    // mul by various prime powers
    let mut p = 0;
    let mut old_p = 0;
    let mut i = 0;
    let mut old_x = 0;
    while p < b1 {
        let j = i + 1024;
        old_p = p;
        old_x = x;
        while i < j {
            p = primes.next().unwrap();
            x = if p < sqrt {
                pp1_pow_ui_u64(
                    x,
                    p.pow(u32::wrapping_from(bits0 / p.significant_bits())),
                    n,
                    n_inverse,
                    norm,
                )
                .0
            } else {
                pp1_pow_ui_u64(x, p, n, n_inverse, norm).0
            };
            i += 1;
        }
        let factor = pp1_factor_u64(n, x, norm);
        if factor == 0 {
            break;
        }
        if factor != 1 {
            return factor;
        }
    }
    if p < b1 {
        // factor = 0
        primes.jump_after(old_p);
        x = old_x;
        loop {
            p = primes.next().unwrap();
            old_x = x;
            x = if p < sqrt {
                pp1_pow_ui_u64(
                    x,
                    p.pow(u32::wrapping_from(bits0 / p.significant_bits())),
                    n,
                    n_inverse,
                    norm,
                )
                .0
            } else {
                pp1_pow_ui_u64(x, p, n, n_inverse, norm).0
            };
            let factor = pp1_factor_u64(n, x, norm);
            if factor == 0 {
                break;
            }
            if factor != 1 {
                return factor;
            }
        }
    } else {
        return 0;
    }
    // factor still 0
    pp1_find_power_u64(old_x, p, n, n_inverse, norm).0
}

// This is n_factor_pp1_wrapper when FLINT64 is true, from ulong_extras/factor_pp1.c, FLINT 3.1.2.
fn factor_pp1_wrapper_u64(n: u64) -> u64 {
    let bits = n.significant_bits();
    // silently fail if trial factoring would always succeed
    if bits < 31 {
        return 0;
    }
    let (b1, count) = N_FACTOR_PP1_TABLE[usize::wrapping_from(bits) - 31];
    let b1 = u64::from(b1);
    let mut state = WyhashRandomU64s::new();
    let mask = u64::low_mask((n - 4).significant_bits());
    let limit = n - 3;
    for _ in 0..count {
        let mut randint = u64::MAX;
        while randint >= limit {
            randint = state.next().unwrap() & mask;
        }
        let factor = factor_pp1_u64(n, b1, randint + 3);
        if factor != 0 {
            return factor;
        }
    }
    0
}

// This is equivalent to `mpn_sqrtrem` from `mpn/generic/sqrtrem.c`, GMP 6.2.1, where `rp` is not
// `NULL` and `Limb == u64`, and the input has two limbs. One limb of the square root and two limbs
// of the remainder are returned.
#[doc(hidden)]
fn limbs_sqrt_rem_to_out_u64(xs_hi: u64, xs_lo: u64) -> (u64, u64, u64, usize) {
    let high = if xs_hi == 0 { xs_lo } else { xs_hi };
    assert_ne!(high, 0);
    let shift = LeadingZeros::leading_zeros(high) >> 1;
    let two_shift = shift << 1;
    if xs_hi == 0 {
        let (sqrt_lo, rem_lo) = if shift == 0 {
            sqrt_rem_newton::<u64, i64>(high)
        } else {
            let sqrt = sqrt_rem_newton::<u64, i64>(high << two_shift).0 >> shift;
            (sqrt, high - sqrt.square())
        };
        (sqrt_lo, 0, rem_lo, usize::from(rem_lo != 0))
    } else if shift == 0 {
        let (sqrt_lo, rem_hi, rem_lo) = sqrt_rem_2_newton::<u64, i64>(xs_hi, xs_lo);
        if rem_hi {
            (sqrt_lo, 1, rem_lo, 2)
        } else {
            (sqrt_lo, 0, rem_lo, usize::from(rem_lo != 0))
        }
    } else {
        let mut lo = xs_lo;
        let hi = (high << two_shift) | (lo >> (u64::WIDTH - two_shift));
        let sqrt_lo = sqrt_rem_2_newton::<u64, i64>(hi, lo << two_shift).0 >> shift;
        lo.wrapping_sub_assign(sqrt_lo.wrapping_square());
        (sqrt_lo, 0, lo, usize::from(lo != 0))
    }
}

const FACTOR_SQUFOF_ITERS: usize = 50_000;
const FACTOR_ONE_LINE_ITERS: usize = 40_000;

// This is _ll_factor_SQUFOF when FLINT64 is true, from ulong_extras/factor_SQUFOF.c, FLINT 3.1.2.
fn ll_factor_squfof_u64(n_hi: u64, n_lo: u64, max_iters: usize) -> u64 {
    let (mut sqrt_lo, mut rem_lo, num) = if n_hi != 0 {
        let (sqrt_lo, _, rem_lo, size) = limbs_sqrt_rem_to_out_u64(n_hi, n_lo);
        (sqrt_lo, rem_lo, size)
    } else {
        let (sqrt_lo, rem_lo) = n_lo.sqrt_rem();
        (sqrt_lo, rem_lo, usize::from(sqrt_lo != 0))
    };
    let sqroot = sqrt_lo;
    let mut p = sqroot;
    let mut q = rem_lo;
    if q == 0 || num == 0 {
        return sqroot;
    }
    let l = 1 + ((p << 1).floor_sqrt() << 1);
    let l2 = l >> 1;
    let mut qupto = 0;
    let mut qlast = 1u64;
    let mut qarr = [0; 50];
    let mut r = 0;
    let mut finished_loop = true;
    for i in 0..max_iters {
        let iq = (sqroot + p) / q;
        let pnext = iq * q - p;
        if q <= l {
            if q.even() {
                qarr[qupto] = q >> 1;
                qupto += 1;
                if qupto >= 50 {
                    return 0;
                }
            } else if q <= l2 {
                qarr[qupto] = q;
                qupto += 1;
                if qupto >= 50 {
                    return 0;
                }
            }
        }
        let t = qlast.wrapping_add(iq.wrapping_mul(p.wrapping_sub(pnext)));
        qlast = q;
        q = t;
        p = pnext;
        if i.odd() || !is_square_u64(q) {
            continue;
        }
        r = q.floor_sqrt();
        if qupto == 0 {
            finished_loop = false;
            break;
        }
        let mut done = true;
        for &q in &qarr[..qupto] {
            if r == q {
                done = false;
                break;
            }
        }
        if done {
            finished_loop = false;
            break;
        }
        if r == 1 {
            return 0;
        }
    }
    if finished_loop {
        return 0;
    }
    qlast = r;
    p = p + r * ((sqroot - p) / r);
    let rem_hi;
    (rem_hi, rem_lo) = u64::x_mul_y_to_zz(p, p);
    let sqrt_hi;
    (sqrt_hi, sqrt_lo) = u64::xx_sub_yy_to_zz(n_hi, n_lo, rem_hi, rem_lo);
    q = if sqrt_hi != 0 {
        let norm = LeadingZeros::leading_zeros(qlast);
        u64::xx_div_mod_y_to_qr(
            (sqrt_hi << norm) + (sqrt_lo >> (u64::WIDTH - norm)),
            sqrt_lo << norm,
            qlast << norm,
        )
        .0
    } else {
        sqrt_lo / qlast
    };
    let mut finished_loop = true;
    for _ in 0..max_iters {
        let iq = (sqroot + p) / q;
        let pnext = iq * q - p;
        if p == pnext {
            finished_loop = false;
            break;
        }
        let t = qlast.wrapping_add(iq.wrapping_mul(p.wrapping_sub(pnext)));
        qlast = q;
        q = t;
        p = pnext;
    }
    if finished_loop {
        0
    } else if q.even() {
        q >> 1
    } else {
        q
    }
}

// This is n_factor_SQUFOF when FLINT64 is true, from ulong_extras/factor_SQUFOF.c, FLINT 3.1.2.
fn factor_squfof_u64(n: u64, iters: usize) -> u64 {
    let mut factor = ll_factor_squfof_u64(0, n, iters);
    let mut finished_loop = true;
    for &p in &SMALL_PRIMES[1..] {
        if factor != 0 {
            finished_loop = false;
            break;
        }
        let multiplier = u64::from(p);
        let (multn_1, multn_0) = u64::x_mul_y_to_zz(multiplier, n);
        factor = ll_factor_squfof_u64(multn_1, multn_0, iters);
        if factor != 0 {
            let (quot, rem) = factor.div_mod(multiplier);
            if rem == 0 {
                factor = quot;
            }
            if factor == 1 || factor == n {
                factor = 0;
            }
        }
    }
    if finished_loop {
        0
    } else {
        factor
    }
}

const FACTOR_TRIAL_PRIMES: usize = 3000;
const FACTOR_TRIAL_CUTOFF: u32 = 27449 * 27449;

impl Factor for u8 {
    type FACTORS = FactorsU8;

    /// Returns the prime factorization of a `u8`. The return value is iterable, and produces pairs
    /// $(p,e)$ of type `(u8, u8)`, where the $p$ is prime and $e$ is the exponent of $p$. The
    /// primes are in ascending order.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::num::factorization::traits::Factor;
    ///
    /// assert_eq!(251u8.factor().into_iter().collect_vec(), &[(251, 1)]);
    /// assert_eq!(
    ///     120u8.factor().into_iter().collect_vec(),
    ///     &[(2, 3), (3, 1), (5, 1)]
    /// );
    /// ```
    fn factor(&self) -> FactorsU8 {
        assert_ne!(*self, 0);
        let mut n = *self;
        let mut factors = FactorsU8::new();
        if n == 1 {
            return factors;
        }
        let zeros = n.trailing_zeros();
        if zeros != 0 {
            factors.insert(2, zeros as u8);
            n >>= zeros;
            if n == 1 {
                return factors;
            }
        }
        let mut e = 0;
        let (q, r) = n.div_mod(3);
        if r == 0 {
            e += 1;
            n = q;
            let (q, r) = n.div_mod(3);
            if r == 0 {
                e += 1;
                n = q;
                let (q, r) = n.div_mod(3);
                if r == 0 {
                    e += 1;
                    n = q;
                    let (q, r) = n.div_mod(3);
                    if r == 0 {
                        e += 1;
                        n = q;
                        if n == 3 {
                            e += 1;
                            n = 1;
                        }
                    }
                }
            }
            factors.insert(3, e);
            if n == 1 {
                return factors;
            }
        }
        e = 0;
        let (q, r) = n.div_mod(5);
        if r == 0 {
            e += 1;
            n = q;
            let (q, r) = n.div_mod(5);
            if r == 0 {
                e += 1;
                n = q;
                if n == 5 {
                    e += 1;
                    n = 1;
                }
            }
            factors.insert(5, e);
            if n == 1 {
                return factors;
            }
        }
        e = 0;
        let (q, r) = n.div_mod(7);
        if r == 0 {
            e += 1;
            n = q;
            if n == 7 {
                e += 1;
                n = 1;
            }
            factors.insert(7, e);
            if n == 1 {
                return factors;
            }
        }
        match n {
            121 => {
                factors.insert(11, 2);
            }
            143 => {
                factors.insert(11, 1);
                factors.insert(13, 1);
            }
            169 => {
                factors.insert(13, 2);
            }
            187 => {
                factors.insert(11, 1);
                factors.insert(17, 1);
            }
            209 => {
                factors.insert(11, 1);
                factors.insert(19, 1);
            }
            221 => {
                factors.insert(13, 1);
                factors.insert(17, 1);
            }
            247 => {
                factors.insert(13, 1);
                factors.insert(19, 1);
            }
            253 => {
                factors.insert(11, 1);
                factors.insert(23, 1);
            }
            _ => {
                factors.insert(n, 1);
            }
        }
        factors
    }
}

impl Factor for u16 {
    type FACTORS = FactorsU16;

    /// Returns the prime factorization of a `u16`. The return value is iterable, and produces pairs
    /// $(p,e)$ of type `(u16, u8)`, where the $p$ is prime and $e$ is the exponent of $p$. The
    /// primes are in ascending order.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(e^{n/4})$
    ///
    /// $M(n) = O(e^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::num::factorization::traits::Factor;
    ///
    /// assert_eq!(65521u16.factor().into_iter().collect_vec(), &[(65521, 1)]);
    /// assert_eq!(
    ///     40320u16.factor().into_iter().collect_vec(),
    ///     &[(2, 7), (3, 2), (5, 1), (7, 1)]
    /// );
    /// ```
    fn factor(&self) -> FactorsU16 {
        let mut factors = FactorsU16::new();
        for (f, e) in u32::from(*self).factor() {
            factors.insert(f as u16, e);
        }
        factors
    }
}

impl Factor for u32 {
    type FACTORS = FactorsU32;

    /// Returns the prime factorization of a `u32`. The return value is iterable, and produces pairs
    /// $(p,e)$ of type `(u32, u8)`, where the $p$ is prime and $e$ is the exponent of $p$. The
    /// primes are in ascending order.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(e^{n/4})$
    ///
    /// $M(n) = O(e^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::num::factorization::traits::Factor;
    ///
    /// assert_eq!(
    ///     4294967291u32.factor().into_iter().collect_vec(),
    ///     &[(4294967291, 1)]
    /// );
    /// assert_eq!(
    ///     479001600u32.factor().into_iter().collect_vec(),
    ///     &[(2, 10), (3, 5), (5, 2), (7, 1), (11, 1)]
    /// );
    /// ```
    ///
    /// This is n_factor when FLINT64 is false, from ulong_extras/factor.c, FLINT 3.1.2.
    fn factor(&self) -> FactorsU32 {
        let n = *self;
        assert_ne!(n, 0);
        let mut factors = FactorsU32::new();
        let cofactor = factor_trial_range_u32(&mut factors, n, FACTOR_TRIAL_PRIMES);
        if cofactor == 1 {
            return factors;
        }
        if cofactor.is_prime() {
            factors.insert(cofactor, 1);
            return factors;
        }
        let mut factor_arr = [0; MAX_FACTORS_IN_U32];
        let mut exp_arr = [0; MAX_FACTORS_IN_U32];
        factor_arr[0] = cofactor;
        let mut factors_left = 1;
        exp_arr[0] = 1;
        let cutoff = FACTOR_TRIAL_CUTOFF;
        while factors_left != 0 {
            let mut factor = factor_arr[factors_left - 1];
            if factor >= cutoff {
                let (mut cofactor, exp) = factor_square_u32(factor);
                if cofactor != 0 {
                    exp_arr[factors_left - 1] *= exp;
                    factor = cofactor;
                    factor_arr[factors_left - 1] = factor;
                }
                if factor >= cutoff && !factor.is_prime() {
                    cofactor = u32::exact_from(factor_one_line_u64(
                        u64::from(factor),
                        FACTOR_ONE_LINE_ITERS,
                    ));
                    exp_arr[factors_left] = exp_arr[factors_left - 1];
                    factor_arr[factors_left] = cofactor;
                    factor_arr[factors_left - 1] /= cofactor;
                    factors_left += 1;
                } else {
                    factors.insert(factor, exp_arr[factors_left - 1]);
                    factors_left -= 1;
                }
            } else {
                factors.insert(factor, exp_arr[factors_left - 1]);
                factors_left -= 1;
            }
        }
        factors
    }
}

const FACTOR_ONE_LINE_MAX: u64 = 1 << 39;

impl Factor for u64 {
    type FACTORS = FactorsU64;

    /// Returns the prime factorization of a `u64`. The return value is iterable, and produces pairs
    /// $(p,e)$ of type `(u64, u8)`, where the $p$ is prime and $e$ is the exponent of $p$. The
    /// primes are in ascending order.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(e^{n/4})$
    ///
    /// $M(n) = O(e^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::num::factorization::traits::Factor;
    ///
    /// assert_eq!(
    ///     18446744073709551557u64.factor().into_iter().collect_vec(),
    ///     &[(18446744073709551557, 1)]
    /// );
    /// assert_eq!(
    ///     2432902008176640000u64.factor().into_iter().collect_vec(),
    ///     &[(2, 18), (3, 8), (5, 4), (7, 2), (11, 1), (13, 1), (17, 1), (19, 1)]
    /// );
    /// ```
    ///
    /// This is n_factor when FLINT64 is true, from ulong_extras/factor.c, FLINT 3.1.2.
    fn factor(&self) -> FactorsU64 {
        let n = *self;
        assert_ne!(n, 0);
        let mut factors = FactorsU64::new();
        let cofactor = factor_trial_range_u64(&mut factors, n, FACTOR_TRIAL_PRIMES);
        if cofactor == 1 {
            return factors;
        }
        if cofactor.is_prime() {
            factors.insert(cofactor, 1);
            return factors;
        }
        let mut factor_arr = [0; MAX_FACTORS_IN_U64];
        let mut exp_arr = [0; MAX_FACTORS_IN_U64];
        factor_arr[0] = cofactor;
        let mut factors_left = 1;
        exp_arr[0] = 1;
        const CUTOFF: u64 = FACTOR_TRIAL_CUTOFF as u64;
        while factors_left != 0 {
            let mut factor = factor_arr[factors_left - 1];
            if factor >= CUTOFF {
                let (mut cofactor, exp) = factor_power23_u64(factor);
                if cofactor != 0 {
                    exp_arr[factors_left - 1] *= exp;
                    factor = cofactor;
                    factor_arr[factors_left - 1] = factor;
                }
                if factor >= CUTOFF && !factor.is_prime() {
                    cofactor = 0;
                    if factor < FACTOR_ONE_LINE_MAX {
                        cofactor = factor_one_line_u64(factor, FACTOR_ONE_LINE_ITERS);
                    }
                    if cofactor == 0 {
                        cofactor = factor_pp1_wrapper_u64(factor);
                        if cofactor == 0 {
                            cofactor = factor_squfof_u64(factor, FACTOR_SQUFOF_ITERS);
                            assert_ne!(cofactor, 0);
                        }
                    }
                    exp_arr[factors_left] = exp_arr[factors_left - 1];
                    factor_arr[factors_left] = cofactor;
                    factor_arr[factors_left - 1] /= cofactor;
                    factors_left += 1;
                } else {
                    factors.insert(factor, exp_arr[factors_left - 1]);
                    factors_left -= 1;
                }
            } else {
                factors.insert(factor, exp_arr[factors_left - 1]);
                factors_left -= 1;
            }
        }
        factors
    }
}

impl Factor for usize {
    type FACTORS = FactorsUsize;

    /// Returns the prime factorization of a `usize`. The return value is iterable, and produces
    /// pairs $(p,e)$ of type `(usize, u8)`, where the $p$ is prime and $e$ is the exponent of $p$.
    /// The primes are in ascending order.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(e^{n/4})$
    ///
    /// $M(n) = O(e^n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use itertools::Itertools;
    /// use malachite_base::num::factorization::traits::Factor;
    ///
    /// assert_eq!(
    ///     4294967291usize.factor().into_iter().collect_vec(),
    ///     &[(4294967291, 1)]
    /// );
    /// assert_eq!(
    ///     479001600usize.factor().into_iter().collect_vec(),
    ///     &[(2, 10), (3, 5), (5, 2), (7, 1), (11, 1)]
    /// );
    /// ```
    fn factor(&self) -> FactorsUsize {
        let mut factors = FactorsUsize::new();
        if USIZE_IS_U32 {
            for (f, e) in (*self as u32).factor() {
                factors.insert(f as usize, e);
            }
        } else {
            for (f, e) in (*self as u64).factor() {
                factors.insert(f as usize, e);
            }
        }
        factors
    }
}
