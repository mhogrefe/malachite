// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2006, 2007, 2008, 2009, 2010, 2016 William Hart
//
//      Copyright © 2008 Peter Shrimpton
//
//      Copyright © 2009 Tom Boothby
//
//      Copyright © 2010, 2021, 2025 Fredrik Johansson
//
//      Copyright © 2014, 2015 Dana Jacobsen
//
//      Copyright © 2015 Kushagra Singh
//
//      Copyright © 2023 Albin Ahlbäck
//
//      Copyright © 2026 Vincent Neiger
//
//      Copyright © 2026 Viorel Wegner
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    Parity, PowerOf2, WrappingAddAssign, WrappingMulAssign, WrappingSubMul, XMulYToZZ,
};
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::factorization::is_prime_tables::{
    BASE_2_PSEUDOPRIMES, BASE_2_PSEUDOPRIMES_HASH_KEYS, MOD_30_PRIME_LOOKUP, ODD_PRIME_LOOKUP,
    WITNESS_BASES,
};
use crate::num::factorization::traits::IsPrime;
use crate::num::modular::traits::{ModAdd, ModMulPrecomputed, ModPowerOf2};

// This is SMALL_ODDPRIME_LIMIT from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
const SMALL_ODD_PRIME_LIMIT: u64 = 32768;

#[cfg(feature = "std")]
#[inline]
fn sqrt_f64(x: f64) -> f64 {
    x.sqrt()
}

#[cfg(not(feature = "std"))]
#[inline]
fn sqrt_f64(x: f64) -> f64 {
    libm::sqrt(x)
}

// The correctly rounded values of $1/\sqrt k$ for $k$ from 2 to 9.
//
// This is SQRTINV from n_is_semiprime_k in ulong_extras/is_prime.c, FLINT 3.7.0-dev.
const SQRT_INVERSES: [f64; 8] = [
    f64::from_bits(0x3fe6a09e667f3bcc),
    f64::from_bits(0x3fe279a74590331d),
    f64::from_bits(0x3fe0000000000000),
    f64::from_bits(0x3fdc9f25c5bfedd9),
    f64::from_bits(0x3fda20bd700c2c3f),
    f64::from_bits(0x3fd83091e6a7f7e6),
    f64::from_bits(0x3fd6a09e667f3bcc),
    f64::from_bits(0x3fd5555555555555),
];

// The correctly rounded value of $1/\sqrt{12}$.
//
// This is SQRTINV12 from n_is_semiprime_k in ulong_extras/is_prime.c, FLINT 3.7.0-dev.
const SQRT_INVERSE_12: f64 = f64::from_bits(0x3fd279a74590331d);

// Detects whether n is of the form pq where q = k(p - 1) + 1, with p = a + 1 and q = ka + 1, for k
// in {2, 3, 4, 5, 6, 7, 8, 9, 12}.
//
// This is n_is_semiprime_k from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
fn is_semiprime_k(n: u64) -> bool {
    // We only need a single square root.
    let sqrt_n = sqrt_f64(n as f64);
    // This is sqrt(n / 12). It is less than 2^32.
    let k = (sqrt_n * SQRT_INVERSE_12) as i64 as u64;
    if (k + 1).wrapping_mul(12 * k + 1) == n {
        return true;
    }
    for (k, &sqrt_inverse) in (2..).zip(SQRT_INVERSES.iter()) {
        // This is sqrt(n / k).
        let a = (sqrt_n * sqrt_inverse) as i64 as u64;
        if (a + 1).wrapping_mul(k * a + 1) == n {
            return true;
        }
    }
    false
}

// A branch-free hash table lookup.
//
// This is u32_is_base2_pseudoprime from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
fn u32_is_base_2_pseudoprime(x: u32) -> bool {
    let c = BASE_2_PSEUDOPRIMES_HASH_KEYS[(x >> 25) as usize];
    let h = ((x ^ c) % 2560) as usize;
    // Linear probing
    (BASE_2_PSEUDOPRIMES[h] == x) | (BASE_2_PSEUDOPRIMES[h + 1] == x)
}

// Checks that the nonzero entries of the pseudoprime table are 2314 distinct composite numbers that
// pass the strong probable prime test to base 2, that the hash lookup finds each of them, and that
// the two small-prime tables agree with trial division.
#[cfg(feature = "test_build")]
pub fn test_is_prime_tables() {
    use crate::test_util::num::factorization::is_prime::is_prime_naive;

    let mut pseudoprimes: alloc::vec::Vec<u32> = BASE_2_PSEUDOPRIMES
        .iter()
        .copied()
        .filter(|&x| x != 0)
        .collect();
    assert_eq!(pseudoprimes.len(), 2314);
    pseudoprimes.sort_unstable();
    pseudoprimes.dedup();
    assert_eq!(pseudoprimes.len(), 2314);
    for &x in &pseudoprimes {
        assert!(u32_is_base_2_pseudoprime(x));
        assert!(x.odd());
        assert!(x < 1 << 20 || u32_is_base_2_probable_prime(x));
        assert!(!is_prime_naive(x));
        assert!(!u32_is_base_2_pseudoprime(x - 2));
    }
    for n in (1..SMALL_ODD_PRIME_LIMIT).step_by(2) {
        assert_eq!(is_odd_prime_small(n), is_prime_naive(n));
    }
    for n in 0..1 << 20 {
        assert_eq!(is_mod_30_prime_small(n), n > 5 && is_prime_naive(n));
    }
}

// Data for dividing by an invariant d >= 1 with a multiplier of one word (Granlund-Montgomery, in
// the variant of Robison).
//
// This is n_div_precomp_struct from ulong_extras.h, FLINT 3.7.0-dev.
struct DivPrecomputed {
    m: u64,
    e: u32,
    c: bool,
}

// This is n_div_precomp_init from ulong_extras.h, FLINT 3.7.0-dev, where
// FLINT_PREINVERT_LIMB_USE_NATIVE is false.
fn div_precompute(d: u64) -> DivPrecomputed {
    let e = d.ilog2();
    if d.is_power_of_two() {
        DivPrecomputed { m: 0, e, c: false }
    } else {
        // From the inverse, without a division: for d not a power of two, floor(2^(64 + e) / d) =
        // (inverse >> 1) | 2^63, and the remainder of that division is -q * d mod 2^64.
        let q = (u64::precompute_mod_mul_data(&d) >> 1) | (1 << 63);
        let r = q.wrapping_mul(d).wrapping_neg();
        if d - r < u64::power_of_2(e.into()) {
            DivPrecomputed {
                m: q + 1,
                e,
                c: false,
            }
        } else {
            DivPrecomputed { m: q, e, c: true }
        }
    }
}

// This is n_rem_precomp_c0 from ulong_extras.h, FLINT 3.7.0-dev.
#[inline]
fn rem_precomputed_c0(x: u64, d: u64, pre: &DivPrecomputed) -> u64 {
    x - d * (u64::x_mul_y_to_zz(x, pre.m).0 >> pre.e)
}

// Assumes x != u64::MAX.
//
// This is n_rem_precomp_c1_bounded from ulong_extras.h, FLINT 3.7.0-dev.
#[inline]
fn rem_precomputed_c1_bounded(x: u64, d: u64, pre: &DivPrecomputed) -> u64 {
    x - d * (u64::x_mul_y_to_zz(x + 1, pre.m).0 >> pre.e)
}

// Computes 2^exp mod n, for n < 2^32, where `rem` reduces its argument modulo n.
//
// This is n_2_powmod_c0 and n_2_powmod_c1 from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
#[inline]
fn two_pow_mod<F: Fn(u64) -> u64>(exp: u32, n: u64, rem: &F) -> u64 {
    if exp < 64 {
        return rem(u64::power_of_2(exp.into()));
    }
    let ebits = 32 - exp.leading_zeros();
    let mut x = rem(u64::power_of_2((exp >> (ebits - 6)).into()));
    if n < 1 << 31 {
        for k in (0..ebits - 6).rev() {
            x = rem(x * x);
            x <<= (exp >> k) & 1;
        }
        if x >= n {
            x -= n;
        }
    } else {
        for k in (0..ebits - 6).rev() {
            x = rem(x * x);
            x <<= (exp >> k) & 1;
            if x >= n {
                x -= n;
            }
        }
    }
    x
}

// The body of u32_is_base2_probabprime, where `rem` reduces its argument modulo n.
#[inline]
fn u32_is_base_2_probable_prime_helper<F: Fn(u64) -> u64>(n: u64, rem: &F) -> bool {
    let d = n - 1;
    let mut t = d >> d.trailing_zeros();
    // t < 2^32, since n is
    let mut y = two_pow_mod(t as u32, n, rem);
    if y == 1 {
        return true;
    }
    t <<= 1;
    while t != n - 1 && y != n - 1 {
        y = rem(y * y);
        t <<= 1;
    }
    y == n - 1
}

// A strong probable prime test to base 2, with faster arithmetic for a 32-bit n.
//
// This is u32_is_base2_probabprime from ulong_extras/is_prime.c, FLINT 3.7.0-dev, where FLINT_BITS
// is 64.
fn u32_is_base_2_probable_prime(n: u32) -> bool {
    let n = u64::from(n);
    let pre = div_precompute(n);
    if pre.c {
        u32_is_base_2_probable_prime_helper(n, &|x| rem_precomputed_c1_bounded(x, n, &pre))
    } else {
        u32_is_base_2_probable_prime_helper(n, &|x| rem_precomputed_c0(x, n, &pre))
    }
}

// This is get_witness_base from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
fn get_witness_base(n: u64) -> u64 {
    let hash = (n as u32).wrapping_mul(541578969) >> 17;
    u64::from(WITNESS_BASES[hash as usize])
}

// Data for Montgomery arithmetic modulo an odd n, with R = 2^64.
//
// This is nmod_redc_ctx_struct from nmod.h, FLINT 3.7.0-dev.
struct RedcContext {
    n: u64,
    // the inverse of n as computed by `precompute_mod_mul_data`
    n_preinverse: u64,
    norm: u32,
    // -1 / n mod R
    n_redc: u64,
}

// Computes the inverse of an odd a modulo 2^64.
//
// This is n_binvert from ulong_extras.h, FLINT 3.7.0-dev, where FLINT_BITS is 64, but starting
// Newton's iteration from 5 correct bits rather than from a table of 8-bit inverses.
fn binvert(a: u64) -> u64 {
    // correct to 5 bits
    let mut r = a.wrapping_mul(3) ^ 2;
    // 10, 20, 40, and 80 bits
    for _ in 0..4 {
        r.wrapping_mul_assign(2u64.wrapping_sub_mul(a, r));
    }
    r
}

// This is nmod_redc_ctx_init_ui from nmod.h, FLINT 3.7.0-dev.
fn redc_context(n: u64) -> RedcContext {
    RedcContext {
        n,
        n_preinverse: u64::precompute_mod_mul_data(&n),
        norm: n.leading_zeros(),
        n_redc: binvert(n.wrapping_neg()),
    }
}

// Converts a to Montgomery form. Assumes a is already reduced mod n.
//
// This is n_to_redc_preinv from nmod.h, FLINT 3.7.0-dev.
fn to_redc(a: u64, ctx: &RedcContext) -> u64 {
    let n = ctx.n << ctx.norm;
    let a = a << ctx.norm;
    let (mut q1, q0) = u64::x_mul_y_to_zz(ctx.n_preinverse, a);
    q1.wrapping_add_assign(a);
    let mut r = q1.wrapping_add(1).wrapping_mul(n).wrapping_neg();
    if r > q0 {
        r.wrapping_add_assign(n);
    }
    (if r < n { r } else { r - n }) >> ctx.norm
}

// This is nmod_redc_set_ui from nmod.h, FLINT 3.7.0-dev.
#[inline]
fn to_redc_unreduced(x: u64, ctx: &RedcContext) -> u64 {
    to_redc(x % ctx.n, ctx)
}

// Computes ab / R mod n, reducing to [0, n).
//
// This is nmod_redc_mul from nmod.h, FLINT 3.7.0-dev.
#[inline]
fn redc_mul(a: u64, b: u64, ctx: &RedcContext) -> u64 {
    let (hi, lo) = u64::x_mul_y_to_zz(a, b);
    let lo = u64::x_mul_y_to_zz(lo.wrapping_mul(ctx.n_redc.wrapping_neg()), ctx.n).0;
    let diff = hi.wrapping_sub(lo);
    if hi < lo {
        diff.wrapping_add(ctx.n)
    } else {
        diff
    }
}

// Computes ab / R, reducing to [0, 2n). Requires n < 2^62.
//
// This is nmod_redc_fast_mul from nmod.h, FLINT 3.7.0-dev.
#[inline]
fn redc_fast_mul(a: u64, b: u64, ctx: &RedcContext) -> u64 {
    let x = u128::from(a) * u128::from(b);
    let m = (x as u64).wrapping_mul(ctx.n_redc);
    ((u128::from(ctx.n) * u128::from(m) + x) >> 64) as u64
}

// This is nmod_redc_fast_mul_two from nmod.h, FLINT 3.7.0-dev.
#[inline]
const fn redc_fast_mul_two(mut x: u64, ctx: &RedcContext) -> u64 {
    if x >= ctx.n {
        x -= ctx.n;
    }
    x << 1
}

// Computes a^exp in Montgomery form, where `mul` multiplies two numbers in that form. Requires exp
// >= 1.
//
// This is _redc_pow_ui and _redc_fast_pow_ui from nmod/pow_impl.h, FLINT 3.7.0-dev.
#[inline]
fn redc_pow<M: Fn(u64, u64) -> u64>(mut a: u64, mut exp: u64, mul: &M) -> u64 {
    while exp.even() {
        a = mul(a, a);
        exp >>= 1;
    }
    let mut x = a;
    loop {
        exp >>= 1;
        if exp == 0 {
            break;
        }
        a = mul(a, a);
        if exp.odd() {
            x = mul(x, a);
        }
    }
    x
}

// Computes 2^exp in Montgomery form, where `mul` multiplies two numbers in that form and `double`
// doubles one.
//
// This is _nmod_redc_2_pow_ui and _nmod_redc_fast_2_pow_ui from nmod/pow.c, FLINT 3.7.0-dev, where
// FLINT_BITS is 64.
#[inline]
fn redc_two_pow<M: Fn(u64, u64) -> u64, D: Fn(u64) -> u64>(
    exp: u64,
    ctx: &RedcContext,
    mul: &M,
    double: &D,
) -> u64 {
    if exp < 64 {
        return to_redc_unreduced(u64::power_of_2(exp), ctx);
    }
    let ebits = 64 - exp.leading_zeros();
    let mut bit = u64::power_of_2((ebits - 6).into());
    let mut x = to_redc_unreduced(u64::power_of_2(exp >> (ebits - 6)), ctx);
    loop {
        bit >>= 1;
        if bit == 0 {
            break;
        }
        x = mul(x, x);
        if bit & exp != 0 {
            x = double(x);
        }
    }
    x
}

// A strong probable prime test to base a, where n - 1 = d * 2^s with d odd, and one_redc is 1 in
// Montgomery form. Assumes that 2 <= a < n - 1. In particular, this is fine with 32-bit bases when
// used for n > 2^32.
//
// This is _n_is_strong_probabprime_redc from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
fn is_strong_probable_prime_redc(a: u64, d: u64, one_redc: u64, ctx: &RedcContext) -> bool {
    let n = ctx.n;
    let mut t = d;
    let minus_one_redc = n - one_redc;
    // This is nmod_redc_can_use_fast from nmod.h, FLINT 3.7.0-dev.
    if ctx.norm >= 2 {
        let mul = |x, y| redc_fast_mul(x, y, ctx);
        // This is redc_fast_equal_redc from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
        let equal = |x: u64, y: u64| x == y || x == y + n;
        let mut y = if a == 2 {
            redc_two_pow(t, ctx, &mul, &|x| redc_fast_mul_two(x, ctx))
        } else {
            redc_pow(to_redc(a, ctx), t, &mul)
        };
        if equal(y, one_redc) {
            return true;
        }
        t <<= 1;
        while t != n - 1 && !equal(y, minus_one_redc) {
            y = mul(y, y);
            t <<= 1;
        }
        equal(y, minus_one_redc)
    } else {
        let mul = |x, y| redc_mul(x, y, ctx);
        let mut y = if a == 2 {
            redc_two_pow(t, ctx, &mul, &|x: u64| x.mod_add(x, n))
        } else {
            redc_pow(to_redc(a, ctx), t, &mul)
        };
        if y == one_redc {
            return true;
        }
        t <<= 1;
        while t != n - 1 && y != minus_one_redc {
            y = mul(y, y);
            t <<= 1;
        }
        y == minus_one_redc
    }
}

// Tests whether an odd n < 2^15 is prime.
//
// This is n_is_oddprime_small2 from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
fn is_odd_prime_small(n: u64) -> bool {
    let q = n >> 1;
    (ODD_PRIME_LOOKUP[(q >> 6) as usize] >> q.mod_power_of_2(6)).odd()
}

// This is wheel_index from n_is_mod30prime_small in ulong_extras/is_prime.c, FLINT 3.7.0-dev.
const WHEEL_INDEX: [u8; 30] =
    [0, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 0, 0, 0, 5, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8];

// Tests whether n < 2^20 is a prime greater than 5.
//
// This is n_is_mod30prime_small from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
fn is_mod_30_prime_small(n: u32) -> bool {
    let index = WHEEL_INDEX[(n % 30) as usize];
    index != 0 && (MOD_30_PRIME_LOOKUP[(n / 30) as usize] >> (index - 1)).odd()
}

// Tests whether an odd n is prime, without first looking for small factors.
//
// This is n_is_prime_odd_no_trial from ulong_extras/is_prime.c, FLINT 3.7.0-dev, where FLINT_BITS
// is 64.
fn is_prime_odd_no_trial(n: u64) -> bool {
    if let Ok(n) = u32::try_from(n) {
        if u64::from(n) < SMALL_ODD_PRIME_LIMIT {
            is_odd_prime_small(u64::from(n))
        } else if n < 1 << 20 {
            is_mod_30_prime_small(n)
        } else {
            u32_is_base_2_probable_prime(n) && !u32_is_base_2_pseudoprime(n)
        }
    } else {
        let ctx = redc_context(n);
        let one_redc = to_redc_unreduced(1, &ctx);
        let d = n - 1;
        let d = d >> d.trailing_zeros();
        is_strong_probable_prime_redc(2, d, one_redc, &ctx)
            && !is_semiprime_k(n)
            && is_strong_probable_prime_redc(get_witness_base(n), d, one_redc, &ctx)
    }
}

// 3, 5 (checked separately), and 32 more primes
//
// This is NUM_ODD_TRIAL_PRIMES from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
const NUM_ODD_TRIAL_PRIMES: usize = 34;

// For each odd trial prime p, the inverse of p modulo 2^32 and the floor of (2^32 - 1) / p: n is
// divisible by p exactly when n times the inverse, modulo 2^32, is at most the floor.
//
// This is trial_inv_32 from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
const TRIAL_INVERSES_32: [(u32, u32); NUM_ODD_TRIAL_PRIMES] = [
    (0xaaaaaaab, 0x55555555), // 3
    (0xcccccccd, 0x33333333), // 5
    (0xb6db6db7, 0x24924924), // 7
    (0xba2e8ba3, 0x1745d174), // 11
    (0xc4ec4ec5, 0x13b13b13), // 13
    (0xf0f0f0f1, 0xf0f0f0f),  // 17
    (0x286bca1b, 0xd79435e),  // 19
    (0xe9bd37a7, 0xb21642c),  // 23
    (0x4f72c235, 0x8d3dcb0),  // 29
    (0xbdef7bdf, 0x8421084),  // 31
    (0x914c1bad, 0x6eb3e45),  // 37
    (0xc18f9c19, 0x63e7063),  // 41
    (0x2fa0be83, 0x5f417d0),  // 43
    (0x677d46cf, 0x572620a),  // 47
    (0x8c13521d, 0x4d4873e),  // 53
    (0xa08ad8f3, 0x456c797),  // 59
    (0xc10c9715, 0x4325c53),  // 61
    (0x7a44c6b, 0x3d22635),   // 67
    (0xe327a977, 0x39b0ad1),  // 71
    (0xc7e3f1f9, 0x381c0e0),  // 73
    (0x613716af, 0x33d91d2),  // 79
    (0x2b2e43db, 0x3159721),  // 83
    (0xfa3f47e9, 0x2e05c0b),  // 89
    (0x5f02a3a1, 0x2a3a0fd),  // 97
    (0x7c32b16d, 0x288df0c),  // 101
    (0xd3431b57, 0x27c4597),  // 103
    (0x8d28ac43, 0x2647c69),  // 107
    (0xda6c0965, 0x2593f69),  // 109
    (0xfdbc091, 0x243f6f0),   // 113
    (0xefdfbf7f, 0x2040810),  // 127
    (0xc9484e2b, 0x1f44659),  // 131
    (0x77975b9, 0x1de5d6e),   // 137
    (0x70586723, 0x1d77b65),  // 139
    (0x8ce2cabd, 0x1b7d6c3),  // 149
];

// For each odd trial prime p, the inverse of p modulo 2^64 and the floor of (2^64 - 1) / p: n is
// divisible by p exactly when n times the inverse, modulo 2^64, is at most the floor.
//
// This is trial_inv_64 from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
const TRIAL_INVERSES_64: [(u64, u64); NUM_ODD_TRIAL_PRIMES] = [
    (0xaaaaaaaaaaaaaaab, 0x5555555555555555), // 3
    (0xcccccccccccccccd, 0x3333333333333333), // 5
    (0x6db6db6db6db6db7, 0x2492492492492492), // 7
    (0x2e8ba2e8ba2e8ba3, 0x1745d1745d1745d1), // 11
    (0x4ec4ec4ec4ec4ec5, 0x13b13b13b13b13b1), // 13
    (0xf0f0f0f0f0f0f0f1, 0xf0f0f0f0f0f0f0f),  // 17
    (0x86bca1af286bca1b, 0xd79435e50d79435),  // 19
    (0xd37a6f4de9bd37a7, 0xb21642c8590b216),  // 23
    (0x34f72c234f72c235, 0x8d3dcb08d3dcb08),  // 29
    (0xef7bdef7bdef7bdf, 0x842108421084210),  // 31
    (0x14c1bacf914c1bad, 0x6eb3e45306eb3e4),  // 37
    (0x8f9c18f9c18f9c19, 0x63e7063e7063e70),  // 41
    (0x82fa0be82fa0be83, 0x5f417d05f417d05),  // 43
    (0x51b3bea3677d46cf, 0x572620ae4c415c9),  // 47
    (0x21cfb2b78c13521d, 0x4d4873ecade304d),  // 53
    (0xcbeea4e1a08ad8f3, 0x456c797dd49c341),  // 59
    (0x4fbcda3ac10c9715, 0x4325c53ef368eb0),  // 61
    (0xf0b7672a07a44c6b, 0x3d226357e16ece5),  // 67
    (0x193d4bb7e327a977, 0x39b0ad12073615a),  // 71
    (0x7e3f1f8fc7e3f1f9, 0x381c0e070381c0e),  // 73
    (0x9b8b577e613716af, 0x33d91d2a2067b23),  // 79
    (0xa3784a062b2e43db, 0x3159721ed7e7534),  // 83
    (0xf47e8fd1fa3f47e9, 0x2e05c0b81702e05),  // 89
    (0xa3a0fd5c5f02a3a1, 0x2a3a0fd5c5f02a3),  // 97
    (0x3a4c0a237c32b16d, 0x288df0cac5b3f5d),  // 101
    (0xdab7ec1dd3431b57, 0x27c45979c95204f),  // 103
    (0x77a04c8f8d28ac43, 0x2647c69456217ec),  // 107
    (0xa6c0964fda6c0965, 0x2593f69b02593f6),  // 109
    (0x90fdbc090fdbc091, 0x243f6f0243f6f02),  // 113
    (0x7efdfbf7efdfbf7f, 0x204081020408102),  // 127
    (0x3e88cb3c9484e2b, 0x1f44659e4a42715),   // 131
    (0xe21a291c077975b9, 0x1de5d6e3f8868a4),  // 137
    (0x3aef6ca970586723, 0x1d77b654b82c339),  // 139
    (0xdf5b0f768ce2cabd, 0x1b7d6c3dda338b2),  // 149
];

// This is TRIAL32 from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
#[inline]
const fn trial_32(n: u32, i: usize) -> bool {
    n.wrapping_mul(TRIAL_INVERSES_32[i].0) <= TRIAL_INVERSES_32[i].1
}

// This is TRIAL64 from ulong_extras/is_prime.c, FLINT 3.7.0-dev.
#[inline]
const fn trial_64(n: u64, i: usize) -> bool {
    n.wrapping_mul(TRIAL_INVERSES_64[i].0) <= TRIAL_INVERSES_64[i].1
}

// This is n_is_prime from ulong_extras/is_prime.c, FLINT 3.7.0-dev, where FLINT_BITS is 64.
fn u64_is_prime(n: u64) -> bool {
    if n < SMALL_ODD_PRIME_LIMIT {
        return if n.odd() {
            is_odd_prime_small(n)
        } else {
            n == 2
        };
    }
    // Trial division by 2, 3 and 5
    if n.even() || trial_64(n, 0) || trial_64(n, 1) {
        return false;
    }
    if let Ok(n) = u32::try_from(n) {
        if n < 1 << 20 {
            return is_mod_30_prime_small(n);
        }
        // The compiler should unroll and convert this to a few vector instructions when e.g. AVX2
        // is available; a branchy return isn't worth it.
        let mut c = false;
        for i in 2..NUM_ODD_TRIAL_PRIMES {
            c |= trial_32(n, i);
        }
        if c {
            return false;
        }
    } else {
        for i in 2..NUM_ODD_TRIAL_PRIMES {
            if trial_64(n, i) {
                return false;
            }
        }
    }
    is_prime_odd_no_trial(n)
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
        } else if n.even() || n.is_multiple_of(3) || n.is_multiple_of(5) || n.is_multiple_of(7) {
            false
        } else {
            n < 121 || !n.is_multiple_of(11) && !n.is_multiple_of(13)
        }
    }
}

impl IsPrime for u16 {
    /// Tests whether a `u16` is prime.
    ///
    /// This implementation looks the answer up in a table, after checking for divisibility by 2,
    /// 3, and 5 when the input is at least $2^{15}$.
    ///
    /// If you want to generate many small primes, try using
    /// [`u16::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u16.is_prime(), true);
    /// assert_eq!(6u16.is_prime(), false);
    /// assert_eq!(65521u16.is_prime(), true);
    /// ```
    #[inline]
    fn is_prime(&self) -> bool {
        u64_is_prime(u64::from(*self))
    }
}

impl IsPrime for u32 {
    /// Tests whether a `u32` is prime.
    ///
    /// For inputs less than $2^{20}$, this implementation looks the answer up in a table. For
    /// larger inputs, it checks for divisibility by the primes up to 149, performs a strong
    /// probable prime test to base 2, and then looks the input up in a table of the 2314 composite
    /// numbers less than $2^{32}$ that pass that test. This is enough to prove primality for any
    /// integer less than $2^{32}$.
    ///
    /// If you want to generate many small primes, try using
    /// [`u32::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: trial
    /// division by a bounded set of small primes, then one strong probable prime test, a modular
    /// powering costing one modular multiplication per exponent bit.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u32.is_prime(), true);
    /// assert_eq!(6u32.is_prime(), false);
    /// assert_eq!(4294967291u32.is_prime(), true);
    /// ```
    #[inline]
    fn is_prime(&self) -> bool {
        u64_is_prime(u64::from(*self))
    }
}

impl IsPrime for u64 {
    /// Tests whether a `u64` is prime.
    ///
    /// For inputs less than $2^{32}$, this implementation does what the `u32` one does. For larger
    /// inputs, it checks for divisibility by the primes up to 149 and performs a strong probable
    /// prime test to base 2. It then rules out the semiprimes $pq$ with $q = k(p - 1) + 1$ for $k$
    /// in $\\{2, 3, 4, 5, 6, 7, 8, 9, 12\\}$, and performs a second strong probable prime test, to
    /// a base chosen by hashing the input into a table. The table was built, by the authors of
    /// FLINT, from the composite numbers less than $2^{64}$ that pass the test to base 2, so that
    /// each of them is caught. This is enough to prove primality for any integer less than
    /// $2^{64}$.
    ///
    /// If you want to generate many small primes, try using
    /// [`u64::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: trial
    /// division by a bounded set of small primes, then at most two strong probable prime tests,
    /// each a modular powering costing one or two modular multiplications per exponent bit.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u64.is_prime(), true);
    /// assert_eq!(6u64.is_prime(), false);
    /// assert_eq!(5509785649208481923u64.is_prime(), true);
    /// ```
    #[inline]
    fn is_prime(&self) -> bool {
        u64_is_prime(*self)
    }
}

impl IsPrime for usize {
    /// Tests whether a `usize` is prime.
    ///
    /// This implementation does what the `u32` or the `u64` one does, depending on the width of a
    /// `usize`. This is enough to prove primality for any integer that fits in a `usize`.
    ///
    /// If you want to generate many small primes, try using
    /// [`usize::primes`][crate::num::factorization::traits::Primes::primes] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: trial
    /// division by a bounded set of small primes, then at most two strong probable prime tests,
    /// each a modular powering costing one or two modular multiplications per exponent bit.
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
