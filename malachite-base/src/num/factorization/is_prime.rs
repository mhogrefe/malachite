// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008 Peter Shrimpton
//
//      Copyright © 2009 Tom Boothby
//
//      Copyright © 2009, 2010, 2013, 2015, 2016 William Hart
//
//      Copyright © 2010 Fredrik Johansson
//
//      Copyright © 2014 Dana Jacobsen
//
//      Copyright © 2023 Mathieu Gouttenoire
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mod_pow::mul_mod_helper;
use crate::num::arithmetic::traits::{
    Gcd, JacobiSymbol, ModAdd, ModInverse, ModMulPrecomputed, ModMulPrecomputedAssign, ModSub,
    Parity, PowerOf2, WrappingAddAssign, WrappingNegAssign, XMulYToZZ, XXAddYYToZZ,
};
use crate::num::basic::integers::{PrimitiveInt, USIZE_IS_U32};
use crate::num::comparison::traits::PartialOrdAbs;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::factorization::traits::IsPrime;
use crate::num::logic::traits::{BitAccess, LeadingZeros, SignificantBits, TrailingZeros};

// This is FLINT_ODD_PRIME_LOOKUP when FLINT64 is true, from ulong_extras/is_oddprime.c, FLINT
// 3.1.2.
const ODD_PRIME_LOOKUP_U64: [u64; 32] = [
    0x816d129a64b4cb6e,
    0x2196820d864a4c32,
    0xa48961205a0434c9,
    0x4a2882d129861144,
    0x834992132424030,
    0x148a48844225064b,
    0xb40b4086c304205,
    0x65048928125108a0,
    0x80124496804c3098,
    0xc02104c941124221,
    0x804490000982d32,
    0x220825b082689681,
    0x9004265940a28948,
    0x6900924430434006,
    0x12410da408088210,
    0x86122d22400c060,
    0x110d301821b0484,
    0x14916022c044a002,
    0x92094d204a6400c,
    0x4ca2100800522094,
    0xa48b081051018200,
    0x34c108144309a25,
    0x2084490880522502,
    0x241140a218003250,
    0xa41a00101840128,
    0x2926000836004512,
    0x10100480c0618283,
    0xc20c26584822006d,
    0x4520582024894810,
    0x10c0250219002488,
    0x802832ca01140868,
    0x60901300264b0400,
];

// This is FLINT_ODD_PRIME_LOOKUP when FLINT64 is false, from ulong_extras/is_oddprime.c, FLINT
// 3.1.2.
const ODD_PRIME_LOOKUP_U32: [u32; 64] = [
    0x64b4cb6e, 0x816d129a, 0x864a4c32, 0x2196820d, 0x5a0434c9, 0xa4896120, 0x29861144, 0x4a2882d1,
    0x32424030, 0x8349921, 0x4225064b, 0x148a4884, 0x6c304205, 0xb40b408, 0x125108a0, 0x65048928,
    0x804c3098, 0x80124496, 0x41124221, 0xc02104c9, 0x982d32, 0x8044900, 0x82689681, 0x220825b0,
    0x40a28948, 0x90042659, 0x30434006, 0x69009244, 0x8088210, 0x12410da4, 0x2400c060, 0x86122d2,
    0x821b0484, 0x110d301, 0xc044a002, 0x14916022, 0x4a6400c, 0x92094d2, 0x522094, 0x4ca21008,
    0x51018200, 0xa48b0810, 0x44309a25, 0x34c1081, 0x80522502, 0x20844908, 0x18003250, 0x241140a2,
    0x1840128, 0xa41a001, 0x36004512, 0x29260008, 0xc0618283, 0x10100480, 0x4822006d, 0xc20c2658,
    0x24894810, 0x45205820, 0x19002488, 0x10c02502, 0x1140868, 0x802832ca, 0x264b0400, 0x60901300,
];

// This is FLINT_D_BITS when FLINT64 is true, from flint.h, FLINT 3.1.2.
const FLINT_D_BITS: u64 = 53;

// This is n_is_oddprime_small_u64 when FLINT64 is true, from ulong_extras/is_oddprime.c, FLINT
// 3.1.2.
#[inline]
fn is_odd_prime_small_u64(n: u64) -> bool {
    ODD_PRIME_LOOKUP_U64[(n >> 7) as usize].get_bit((n >> 1) & u64::WIDTH_MASK)
}

// This is n_is_oddprime_small_u64 when FLINT64 is false, from ulong_extras/is_oddprime.c, FLINT
// 3.1.2.
#[inline]
fn is_odd_prime_small_u32(n: u32) -> bool {
    ODD_PRIME_LOOKUP_U32[(n >> 6) as usize].get_bit(u64::from(n >> 1) & u32::WIDTH_MASK)
}

// This is n_mod2_preinv when FLINT64 is false, from ulong_extras/mod2_preinv.c, FLINT 3.1.2.
fn mod_preinverted_u32(a: u32, mut n: u32, inverse: u32) -> u32 {
    assert_ne!(n, 0);
    let norm = LeadingZeros::leading_zeros(n);
    n <<= norm;
    let u1 = a >> (u32::WIDTH - norm);
    let u0 = a << norm;
    let (mut q1, mut q0) = u32::x_mul_y_to_zz(inverse, u1);
    (q1, q0) = u32::xx_add_yy_to_zz(q1, q0, u1, u0);
    let mut r = u0 - (q1 + 1) * n;
    if r > q0 {
        r += n;
    }
    if r < n {
        r >> norm
    } else {
        (r - n) >> norm
    }
}

// This is n_mod2_preinv when FLINT64 is true, from ulong_extras/mod2_preinv.c, FLINT 3.1.2.
fn mod_preinverted_u64(a: u64, mut n: u64, inverse: u64) -> u64 {
    assert_ne!(n, 0);
    let norm = LeadingZeros::leading_zeros(n);
    n <<= norm;
    let u1 = a >> (u64::WIDTH - norm);
    let u0 = a << norm;
    let (mut q1, mut q0) = u64::x_mul_y_to_zz(inverse, u1);
    (q1, q0) = u64::xx_add_yy_to_zz(q1, q0, u1, u0);
    let mut r = u0 - (q1 + 1) * n;
    if r > q0 {
        r += n;
    }
    if r < n {
        r >> norm
    } else {
        (r - n) >> norm
    }
}

// This is n_powmod2_ui_preinv when FLINT64 is false, from ulong_extras/powmod2_ui_preinv.c, FLINT
// 3.1.2.
fn mod_pow_preinverted_u32(mut a: u32, mut exp: u32, mut n: u32, inverse: u32) -> u32 {
    assert_ne!(n, 0);
    if exp == 0 {
        // anything modulo 1 is 0
        return u32::from(n != 1);
    }
    if a == 0 {
        return 0;
    }
    if a >= n {
        a = mod_preinverted_u32(a, n, inverse);
    }
    let norm = LeadingZeros::leading_zeros(n);
    a <<= norm;
    n <<= norm;
    while exp.even() {
        a = mul_mod_helper::<u32, u64>(a, a, n, inverse, norm);
        exp >>= 1;
    }
    let mut x = a;
    loop {
        exp >>= 1;
        if exp == 0 {
            break;
        }
        a = mul_mod_helper::<u32, u64>(a, a, n, inverse, norm);
        if exp.odd() {
            x = mul_mod_helper::<u32, u64>(x, a, n, inverse, norm);
        }
    }
    x >> norm
}

// This is n_powmod2_ui_preinv when FLINT64 is true, from ulong_extras/powmod2_ui_preinv.c, FLINT
// 3.1.2.
fn mod_pow_preinverted_u64(mut a: u64, mut exp: u64, mut n: u64, inverse: u64) -> u64 {
    assert_ne!(n, 0);
    if exp == 0 {
        // anything modulo 1 is 0
        return u64::from(n != 1);
    }
    if a == 0 {
        return 0;
    }
    if a >= n {
        a = mod_preinverted_u64(a, n, inverse);
    }
    let norm = LeadingZeros::leading_zeros(n);
    a <<= norm;
    n <<= norm;
    while exp.even() {
        a = mul_mod_helper::<u64, u128>(a, a, n, inverse, norm);
        exp >>= 1;
    }
    let mut x = a;
    loop {
        exp >>= 1;
        if exp == 0 {
            break;
        }
        a = mul_mod_helper::<u64, u128>(a, a, n, inverse, norm);
        if exp.odd() {
            x = mul_mod_helper::<u64, u128>(x, a, n, inverse, norm);
        }
    }
    x >> norm
}

// This is n_mulmod_precomp when FLINT64 is true, from ulong_extras/mulmod_precomp.c, FLINT 3.1.2.
fn mod_mul_preinverted_float(a: u64, b: u64, n: u64, inverse: f64) -> u64 {
    let q = ((a as f64) * (b as f64) * inverse) as u64;
    let mut r = (a.wrapping_mul(b)).wrapping_sub(q.wrapping_mul(n));
    if r.get_highest_bit() {
        r.wrapping_add_assign(n);
        if r.get_highest_bit() {
            return r.wrapping_add(n);
        }
    } else if r >= n {
        return r - n;
    }
    r
}

// This is n_powmod_ui_precomp when FLINT64 is true, from ulong_extras/powmod_precomp.c, FLINT
// 3.1.2.
fn mod_pow_preinverted_float(a: u64, mut exp: u64, n: u64, inverse: f64) -> u64 {
    if n == 1 {
        return 0;
    }
    let mut x = 1;
    let mut y = a;
    while exp != 0 {
        if exp.odd() {
            x = mod_mul_preinverted_float(x, y, n, inverse);
        }
        exp >>= 1;
        if exp != 0 {
            y = mod_mul_preinverted_float(y, y, n, inverse);
        }
    }
    x
}

// This is n_is_probabprime_fermat when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT
// 3.1.2.
fn is_probable_prime_fermat(n: u64, i: u64) -> bool {
    (if n.significant_bits() <= FLINT_D_BITS {
        mod_pow_preinverted_float(i, n - 1, n, 1.0 / (n as f64))
    } else {
        mod_pow_preinverted_u64(i, n - 1, n, u64::precompute_mod_mul_data(&n))
    }) == 1
}

// This is fchain_precomp when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT 3.1.2.
fn fibonacci_chain_precomputed(m: u64, n: u64, inverse: f64) -> (u64, u64) {
    let mut x = 2;
    let mut y = n - 3;
    let mut power = u64::power_of_2(m.significant_bits() - 1);
    while power != 0 {
        let xy = mod_mul_preinverted_float(x, y, n, inverse).mod_add(3, n);
        (x, y) = if m & power != 0 {
            (
                xy,
                mod_mul_preinverted_float(y, y, n, inverse).mod_sub(2, n),
            )
        } else {
            (
                mod_mul_preinverted_float(x, x, n, inverse).mod_sub(2, n),
                xy,
            )
        };
        power >>= 1;
    }
    (x, y)
}

// This is fchain2_preinv when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT 3.1.2.
fn fibonacci_chain_preinvert(m: u64, n: u64, ninv: u64) -> (u64, u64) {
    let mut x = 2;
    let mut y = n - 3;
    let mut power = u64::power_of_2(m.significant_bits() - 1);
    while power != 0 {
        let xy = x.mod_mul_precomputed(y, n, &ninv).mod_add(3, n);
        (x, y) = if m & power != 0 {
            (xy, y.mod_mul_precomputed(y, n, &ninv).mod_sub(2, n))
        } else {
            (x.mod_mul_precomputed(x, n, &ninv).mod_sub(2, n), xy)
        };
        power >>= 1;
    }
    (x, y)
}

// This is n_is_probabprime_fibonacci when FLINT64 is true, from ulong_extras/is_probabprime.c,
// FLINT 3.1.2.
fn is_probable_prime_fibonacci(n: u64) -> bool {
    if i64::wrapping_from(n).le_abs(&3) {
        return n >= 2;
    }
    // cannot overflow as (5 / n) = 0 for n = 2 ^ 64 - 1
    let m = n.wrapping_sub(u64::wrapping_from(5.jacobi_symbol(n))) >> 1;
    if n.significant_bits() <= FLINT_D_BITS {
        let inverse = 1.0 / (n as f64);
        let (x, y) = fibonacci_chain_precomputed(m, n, inverse);
        mod_mul_preinverted_float(n - 3, x, n, inverse)
            == mod_mul_preinverted_float(2, y, n, inverse)
    } else {
        let inverse = u64::precompute_mod_mul_data(&n);
        let (x, y) = fibonacci_chain_preinvert(m, n, inverse);
        (n - 3).mod_mul_precomputed(x, n, &inverse) == 2.mod_mul_precomputed(y, n, &inverse)
    }
}

// This is lchain_precomp when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT 3.1.2.
fn lucas_chain_precomputed(m: u64, a: u64, n: u64, npre: f64) -> (u64, u64) {
    let mut x = 2;
    let mut y = n - 3;
    let mut power = u64::power_of_2(m.significant_bits() - 1);
    while power != 0 {
        let xy = mod_mul_preinverted_float(x, y, n, npre).mod_sub(a, n);
        (x, y) = if m & power != 0 {
            (xy, mod_mul_preinverted_float(y, y, n, npre).mod_sub(2, n))
        } else {
            (mod_mul_preinverted_float(x, x, n, npre).mod_sub(2, n), xy)
        };
        power >>= 1;
    }
    (x, y)
}

// This is lchain2_preinv when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT 3.1.2.
fn lucas_chain_preinvert(m: u64, a: u64, n: u64, ninv: u64) -> (u64, u64) {
    let mut x = 2;
    let mut y = a;
    let mut power = u64::power_of_2(m.significant_bits() - 1);
    while power != 0 {
        let xy = x.mod_mul_precomputed(y, n, &ninv).mod_sub(a, n);
        (x, y) = if m & power != 0 {
            (xy, y.mod_mul_precomputed(y, n, &ninv).mod_sub(2, n))
        } else {
            (x.mod_mul_precomputed(x, n, &ninv).mod_sub(2, n), xy)
        };
        power >>= 1;
    }
    (x, y)
}

// This is n_is_probabprime_lucas when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT
// 3.1.2, where n is odd and greater than 52, and only true or false is returned, rather than 0, 1,
// or -1.
fn is_probable_prime_lucas(n: u64) -> bool {
    let mut d = 0u64;
    if i64::wrapping_from(n).le_abs(&2) {
        return n == 2;
    }
    let mut neg_d = false;
    let mut j = 0;
    for i in 0..100 {
        d = 5 + (i << 1);
        neg_d = false;
        if d.gcd(n % d) == 1 {
            if i.odd() {
                neg_d = true;
            }
            let jacobi = if neg_d {
                (-i128::from(d)).jacobi_symbol(i128::from(n))
            } else {
                d.jacobi_symbol(n)
            };
            if jacobi == -1 {
                break;
            }
        } else if n != d {
            return false;
        }
        j += 1;
    }
    if j == 100 {
        return true;
    }
    if neg_d {
        d.wrapping_neg_assign();
    }
    let mut q = u64::wrapping_from(1i64.wrapping_sub(i64::wrapping_from(d)) / 4);
    if q.get_highest_bit() {
        q.wrapping_add_assign(n);
    }
    let a = q.mod_inverse(n).unwrap().mod_sub(2, n);
    let (left, right) = if n <= FLINT_D_BITS {
        let inverse = 1.0 / (n as f64);
        let (x, y) = lucas_chain_precomputed(n + 1, a, n, inverse);
        (
            mod_mul_preinverted_float(a, x, n, inverse),
            mod_mul_preinverted_float(2, y, n, inverse),
        )
    } else {
        let inverse = u64::precompute_mod_mul_data(&n);
        let (x, y) = lucas_chain_preinvert(n + 1, a, n, inverse);
        (
            a.mod_mul_precomputed(x, n, &inverse),
            2.mod_mul_precomputed(y, n, &inverse),
        )
    };
    left == right
}

// This is n_is_strong_probabprime2_preinv when FLINT64 is false, from
// ulong_extras/is_strong_probabprime2_preinv.c, FLINT 3.1.2.
fn is_strong_probable_prime_preinverted_u32(n: u32, inverse: u32, a: u32, d: u32) -> bool {
    assert!(a < n);
    let nm1 = n - 1;
    if a <= 1 || a == nm1 {
        return true;
    }
    let mut t = d;
    let mut y = mod_pow_preinverted_u32(a, t, n, inverse);
    if y == 1 {
        return true;
    }
    t <<= 1;
    while t != nm1 && y != nm1 {
        y.mod_mul_precomputed_assign(y, n, &inverse);
        t <<= 1;
    }
    y == nm1
}

// This is n_is_strong_probabprime2_preinv when FLINT64 is true, from
// ulong_extras/is_strong_probabprime2_preinv.c, FLINT 3.1.2.
fn is_strong_probable_prime_preinverted_u64(n: u64, inverse: u64, a: u64, d: u64) -> bool {
    assert!(a < n);
    let nm1 = n - 1;
    if a <= 1 || a == nm1 {
        return true;
    }
    let mut t = d;
    let mut y = mod_pow_preinverted_u64(a, t, n, inverse);
    if y == 1 {
        return true;
    }
    t <<= 1;
    while t != nm1 && y != nm1 {
        y.mod_mul_precomputed_assign(y, n, &inverse);
        t <<= 1;
    }
    y == nm1
}

// This is n_mod2_precomp when FLINT64 is true, from ulong_extras/mod2_precomp.c, FLINT 3.1.2.
fn mod_preinverted_float(a: u64, n: u64, inverse: f64) -> u64 {
    if a < n {
        return a;
    }
    let ni = i64::wrapping_from(n);
    if ni < 0 {
        return a.wrapping_sub(n);
    }
    let (mut q, mut r) = if n == 1 {
        (a, 0)
    } else {
        let q = ((a as f64) * inverse) as u64;
        (
            q,
            i64::wrapping_from(a).wrapping_sub(i64::wrapping_from(q.wrapping_mul(n))),
        )
    };
    if r < ni.wrapping_neg() {
        q -= ((r.wrapping_neg() as f64) * inverse) as u64;
    } else if r >= ni {
        q += ((r as f64) * inverse) as u64;
    } else if r < 0 {
        return u64::wrapping_from(r + ni);
    } else {
        return u64::wrapping_from(r);
    }
    r = i64::wrapping_from(a) - i64::wrapping_from(q.wrapping_mul(n));
    u64::wrapping_from(if r >= ni {
        r.wrapping_sub(ni)
    } else if r < 0 {
        r.wrapping_add(ni)
    } else {
        r
    })
}

// This is n_is_strong_probabprime_precomp when FLINT64 is true, from
// ulong_extras/is_strong_probabprime_precomp.c, FLINT 3.1.2.
fn is_strong_probable_prime_preinverted_float(n: u64, inverse: f64, mut a: u64, d: u64) -> bool {
    // Map large base to range 2 ... n - 1
    if a >= n {
        a = mod_preinverted_float(a, n, inverse);
    }
    let nm1 = n - 1;
    if a <= 1 || a == nm1 {
        return true;
    }
    let mut t = d;
    let mut y = mod_pow_preinverted_float(a, t, n, inverse);
    if y == 1 {
        return true;
    }
    t <<= 1;
    while t != nm1 && y != nm1 {
        y = mod_mul_preinverted_float(y, y, n, inverse);
        t <<= 1;
    }
    y == nm1
}

// This is n_is_probabprime_BPSW when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT
// 3.1.2, where n is odd and greater than 1.
fn is_probable_prime_bpsw(n: u64) -> bool {
    let nm10 = n % 10;
    if nm10 == 3 || nm10 == 7 {
        return is_probable_prime_fermat(n, 2) && is_probable_prime_fibonacci(n);
    }
    let mut d = n - 1;
    while d.even() {
        d >>= 1;
    }
    let result = if n.significant_bits() <= FLINT_D_BITS {
        is_strong_probable_prime_preinverted_float(n, 1.0 / (n as f64), 2, d)
    } else {
        is_strong_probable_prime_preinverted_u64(n, u64::precompute_mod_mul_data(&n), 2, d)
    };
    if !result {
        return false;
    }
    is_probable_prime_lucas(n)
}

const FLINT_ODDPRIME_SMALL_CUTOFF: u32 = 4096;

// This is n_is_probabprime when FLINT64 is false, from ulong_extras/is_probabprime.c, FLINT 3.1.2,
// assuming n is odd and greater than 2.
fn is_probable_prime_u32(n: u32) -> bool {
    if n < FLINT_ODDPRIME_SMALL_CUTOFF {
        return is_odd_prime_small_u32(n);
    }
    let mut d = n - 1;
    d >>= TrailingZeros::trailing_zeros(d);
    // For 32-bit, just the 2-base or 3-base Miller-Rabin is enough.
    let inverse = u32::precompute_mod_mul_data(&n);
    if n < 9080191 {
        is_strong_probable_prime_preinverted_u32(n, inverse, 31, d)
            && is_strong_probable_prime_preinverted_u32(n, inverse, 73, d)
    } else {
        is_strong_probable_prime_preinverted_u32(n, inverse, 2, d)
            && is_strong_probable_prime_preinverted_u32(n, inverse, 7, d)
            && is_strong_probable_prime_preinverted_u32(n, inverse, 61, d)
    }
}

// This is n_is_probabprime when FLINT64 is true, from ulong_extras/is_probabprime.c, FLINT 3.1.2,
// assuming n is odd and greater than 2.
fn is_probable_prime_u64(n: u64) -> bool {
    if n < u64::from(FLINT_ODDPRIME_SMALL_CUTOFF) {
        return is_odd_prime_small_u64(n);
    } else if n >= 1050535501 {
        // Avoid the unnecessary inverse
        return is_probable_prime_bpsw(n);
    }
    let mut d = n - 1;
    d >>= TrailingZeros::trailing_zeros(d);
    let inverse = 1.0 / (n as f64);
    // For 64-bit, BPSW seems to be a little bit faster than 3 bases.
    if n < 341531 {
        is_strong_probable_prime_preinverted_float(n, inverse, 9345883071009581737, d)
    } else {
        is_strong_probable_prime_preinverted_float(n, inverse, 336781006125, d)
            && is_strong_probable_prime_preinverted_float(n, inverse, 9639812373923155, d)
    }
}

impl IsPrime for u8 {
    /// Tests whether a `u8` is prime.
    ///
    /// This implementation just does a few divisibility checks.
    ///
    /// If you want to generate many small primes, try using
    /// [`u8::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u8.is_prime(), true);
    /// assert_eq!(6u8.is_prime(), false);
    /// ```
    fn is_prime(&self) -> bool {
        let n = *self;
        if n < 11 {
            n == 2 || n == 3 || n == 5 || n == 7
        } else if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
            false
        } else {
            n < 121 || n % 11 != 0 && n % 13 != 0
        }
    }
}

impl IsPrime for u16 {
    /// Tests whether a `u16` is prime.
    ///
    /// This implementation does a few divisibility checks, then performs strong probable prime
    /// tests with bases 31 and 73, which is enough to prove primality for any integer less than
    /// $2^{16}$.
    ///
    /// If you want to generate many small primes, try using
    /// [`u16::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u16.is_prime(), true);
    /// assert_eq!(6u16.is_prime(), false);
    /// assert_eq!(65521u16.is_prime(), true);
    /// ```
    fn is_prime(&self) -> bool {
        // Flint's "BPSW" (which Malachite's code is based on) checked against Feitsma and Galway's
        // database [1, 2] up to 2^64 by Dana Jacobsen.
        // - [1] http://www.janfeitsma.nl/math/psp2/database
        // - [2] http://www.cecm.sfu.ca/Pseudoprimes/index-2-to-64.html
        let n = *self;
        if n < 11 {
            n == 2 || n == 3 || n == 5 || n == 7
        } else if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
            false
        } else if n < 121 {
            // 11*11
            true
        } else if n % 11 == 0
            || n % 13 == 0
            || n % 17 == 0
            || n % 19 == 0
            || n % 23 == 0
            || n % 29 == 0
            || n % 31 == 0
            || n % 37 == 0
            || n % 41 == 0
            || n % 43 == 0
            || n % 47 == 0
            || n % 53 == 0
        {
            false
        } else {
            n < 3481 || is_probable_prime_u32(u32::from(n))
        }
    }
}

impl IsPrime for u32 {
    /// Tests whether a `u32` is prime.
    ///
    /// This implementation does a few divisibility checks, then performs a few strong probable
    /// prime tests, which is enough to prove primality for any integer less than $2^{32}$.
    ///
    /// If you want to generate many small primes, try using
    /// [`u32::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u32.is_prime(), true);
    /// assert_eq!(6u32.is_prime(), false);
    /// assert_eq!(4294967291u32.is_prime(), true);
    /// ```
    fn is_prime(&self) -> bool {
        // Flint's "BPSW" (which Malachite's code is based on) checked against Feitsma and Galway's
        // database [1, 2] up to 2^64 by Dana Jacobsen.
        // - [1] http://www.janfeitsma.nl/math/psp2/database
        // - [2] http://www.cecm.sfu.ca/Pseudoprimes/index-2-to-64.html
        let n = *self;
        if n < 11 {
            n == 2 || n == 3 || n == 5 || n == 7
        } else if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
            false
        } else if n < 121 {
            // 11*11
            true
        } else if n % 11 == 0
            || n % 13 == 0
            || n % 17 == 0
            || n % 19 == 0
            || n % 23 == 0
            || n % 29 == 0
            || n % 31 == 0
            || n % 37 == 0
            || n % 41 == 0
            || n % 43 == 0
            || n % 47 == 0
            || n % 53 == 0
        {
            false
        } else if n < 3481 {
            // 59*59
            true
        } else if n > 1000000
            && (n % 59 == 0
                || n % 61 == 0
                || n % 67 == 0
                || n % 71 == 0
                || n % 73 == 0
                || n % 79 == 0
                || n % 83 == 0
                || n % 89 == 0
                || n % 97 == 0
                || n % 101 == 0
                || n % 103 == 0
                || n % 107 == 0
                || n % 109 == 0
                || n % 113 == 0
                || n % 127 == 0
                || n % 131 == 0
                || n % 137 == 0
                || n % 139 == 0
                || n % 149 == 0)
        {
            false
        } else {
            is_probable_prime_u32(n)
        }
    }
}

impl IsPrime for u64 {
    /// Tests whether a `u64` is prime.
    ///
    /// This implementation first does a few divisibility checks. Then, depending on the input, it
    /// either runs a few strong probable prime tests or the Baillie–PSW test. This is enough to
    /// prove primality for any integer less than $2^{64}$.
    ///
    /// If you want to generate many small primes, try using
    /// [`u64::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u64.is_prime(), true);
    /// assert_eq!(6u64.is_prime(), false);
    /// assert_eq!(5509785649208481923u64.is_prime(), true);
    /// ```
    fn is_prime(&self) -> bool {
        // Flint's "BPSW" (which Malachite's code is based on) checked against Feitsma and Galway's
        // database [1, 2] up to 2^64 by Dana Jacobsen.
        // - [1] http://www.janfeitsma.nl/math/psp2/database
        // - [2] http://www.cecm.sfu.ca/Pseudoprimes/index-2-to-64.html
        let n = *self;
        if n < 11 {
            n == 2 || n == 3 || n == 5 || n == 7
        } else if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
            false
        } else if n < 121 {
            // 11*11
            true
        } else if n % 11 == 0
            || n % 13 == 0
            || n % 17 == 0
            || n % 19 == 0
            || n % 23 == 0
            || n % 29 == 0
            || n % 31 == 0
            || n % 37 == 0
            || n % 41 == 0
            || n % 43 == 0
            || n % 47 == 0
            || n % 53 == 0
        {
            false
        } else if n < 3481 {
            // 59*59
            true
        } else if n > 1000000
            && (n % 59 == 0
                || n % 61 == 0
                || n % 67 == 0
                || n % 71 == 0
                || n % 73 == 0
                || n % 79 == 0
                || n % 83 == 0
                || n % 89 == 0
                || n % 97 == 0
                || n % 101 == 0
                || n % 103 == 0
                || n % 107 == 0
                || n % 109 == 0
                || n % 113 == 0
                || n % 127 == 0
                || n % 131 == 0
                || n % 137 == 0
                || n % 139 == 0
                || n % 149 == 0)
        {
            false
        } else {
            is_probable_prime_u64(n)
        }
    }
}

impl IsPrime for usize {
    /// Tests whether a `usize` is prime.
    ///
    /// This implementation first does a few divisibility checks. Then, depending on the input, it
    /// either runs a few strong probable prime tests or the Baillie–PSW test. This is enough to
    /// prove primality for any integer that fits in a `usize`.
    ///
    /// If you want to generate many small primes, try using
    /// [`usize::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5usize.is_prime(), true);
    /// assert_eq!(6usize.is_prime(), false);
    /// assert_eq!(4294967291usize.is_prime(), true);
    /// ```
    #[inline]
    fn is_prime(&self) -> bool {
        if USIZE_IS_U32 {
            u32::wrapping_from(*self).is_prime()
        } else {
            u64::wrapping_from(*self).is_prime()
        }
    }
}
