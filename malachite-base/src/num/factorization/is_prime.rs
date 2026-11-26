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
//      Copyright © 2010, 2021, 2025, 2026 Fredrik Johansson
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
    OverflowingAddAssign, Parity, PowerOf2, WrappingAddAssign, WrappingMulAssign, WrappingSubMul,
    XMulYToZZ,
};
use crate::num::basic::integers::{PrimitiveInt, USIZE_IS_U32};
use crate::num::conversion::half::{wide_join_halves, wide_split_in_half, wide_upper_half};
use crate::num::conversion::traits::{ExactFrom, JoinHalves, SplitInHalf, WrappingFrom};
use crate::num::factorization::is_prime_aprcl::u128_is_prime_aprcl;
use crate::num::factorization::is_prime_n_pm_1::u128_is_prime_n_pm_1;
use crate::num::factorization::is_prime_tables::{
    BASE_2_PSEUDOPRIMES, BASE_2_PSEUDOPRIMES_HASH_KEYS, MOD_30_PRIME_LOOKUP, ODD_PRIME_LOOKUP,
    WITNESS_BASES,
};
use crate::num::factorization::traits::IsPrime;
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::num::modular::traits::{ModAdd, ModAddAssign, ModMulPrecomputed, ModPowerOf2};

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
pub(crate) struct RedcContext {
    pub(crate) n: u64,
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
pub(crate) fn redc_context(n: u64) -> RedcContext {
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
pub(crate) fn to_redc(a: u64, ctx: &RedcContext) -> u64 {
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
pub(crate) fn to_redc_unreduced(x: u64, ctx: &RedcContext) -> u64 {
    to_redc(x % ctx.n, ctx)
}

// Computes ab / R mod n, reducing to [0, n).
//
// This is nmod_redc_mul from nmod.h, FLINT 3.7.0-dev.
#[inline]
pub(crate) fn redc_mul(a: u64, b: u64, ctx: &RedcContext) -> u64 {
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
pub(crate) fn is_strong_probable_prime_redc(
    a: u64,
    d: u64,
    one_redc: u64,
    ctx: &RedcContext,
) -> bool {
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
pub(crate) fn u64_is_prime(n: u64) -> bool {
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

// Divides the three-limb value [n2, n1, n0] by a normalized (top bit set) two-limb d, returning the
// one-limb quotient and the remainder. Requires [n2, n1] < d.
//
// FLINT does this with flint_mpn_tdiv_qr; this is Knuth's Algorithm D specialized to one quotient
// limb, for which the two-limb correction test makes the estimate exact.
fn div_3_by_2_normalized(n2: u64, n1: u64, n0: u64, d: u128) -> (u64, u128) {
    let (d1, d0) = wide_split_in_half(d);
    let top = u128::join_halves(n2, n1);
    let mut q = if u128::from(n2) >= d1 {
        u128::MAX >> u64::WIDTH
    } else {
        top / d1
    };
    let mut r = top - q * d1;
    while r.upper_half() == 0 && q * d0 > u128::join_halves(r.lower_half(), n0) {
        q -= 1;
        r += d1;
    }
    let q = q.lower_half();
    (
        q,
        u128::join_halves(n1, n0).wrapping_sub_mul(d, u128::from(q)),
    )
}

// Computes floor(2^192 / m) for 2^64 < m < 2^128. The result fits in two limbs.
//
// This is n_ll_small_preinv from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
fn ll_small_preinv(m: u128) -> u128 {
    let s = m.leading_zeros();
    let d = m << s;
    // 2^(192 + s) / d, one quotient limb at a time
    let (q1, r) = div_3_by_2_normalized(u64::power_of_2(s.into()), 0, 0, d);
    let (r1, r0) = r.split_in_half();
    let q0 = div_3_by_2_normalized(r1, r0, 0, d).0;
    u128::join_halves(q1, q0)
}

// Returns the low three limbs of a^2, as the top limb and the bottom two.
//
// This is sqrlo_3x2x2 from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
#[inline]
fn sqrlo_3x2x2(a: u128) -> (u64, u128) {
    let (a1, a0) = wide_split_in_half(a);
    let s = a0 * a0;
    let t = a0 * a1;
    let high = wide_join_halves(a1 * a1, wide_upper_half(s))
        .wrapping_add(t)
        .wrapping_add(t);
    let (high_1, high_0) = high.split_in_half();
    (high_1, u128::join_halves(high_0, s.lower_half()))
}

// An approximation, at most a few units too small, of floor(ab / 2^128).
//
// This is mulhi_2x2_sloppy from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
#[inline]
fn mulhi_2x2_sloppy(a: u128, b: u128) -> u128 {
    let (a1, a0) = wide_split_in_half(a);
    let (b1, b0) = wide_split_in_half(b);
    let s2 = wide_upper_half(a0 * b1);
    let u2 = wide_upper_half(a1 * b0);
    (s2 + u2).wrapping_add(a1 * b1)
}

// Reduces a two-limb x to a noncanonical residue mod m, typically in [0, 2m).
//
// This is n_ll_small_reduce2_sloppy from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
#[inline]
fn ll_small_reduce2_sloppy(x: u128, m: u128, minv: u128) -> u128 {
    let q0 = wide_upper_half(wide_upper_half(x) * wide_upper_half(minv));
    x.wrapping_sub_mul(m, q0)
}

// Reduces a two-limb x to [0, m).
//
// This is n_ll_small_reduce2_precise from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
#[inline]
fn ll_small_reduce2_precise(x: u128, m: u128, minv: u128) -> u128 {
    let mut x = ll_small_reduce2_sloppy(x, m, minv);
    while x >= m {
        x -= m;
    }
    x
}

// Squares a noncanonical residue mod m, returning a noncanonical residue in [0, 6m).
//
// This is n_ll_small_sqrmod_sloppy from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
#[inline]
fn ll_small_sqrmod_sloppy(a: u128, m: u128, minv: u128) -> u128 {
    let (t2, t10) = sqrlo_3x2x2(a);
    let q = mulhi_2x2_sloppy(u128::join_halves(t2, t10.upper_half()), minv);
    t10.wrapping_sub_mul(q, m)
}

// Squares a residue mod m, reducing to [0, m).
//
// This is n_ll_small_sqrmod from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
#[inline]
fn ll_small_sqrmod(a: u128, m: u128, minv: u128) -> u128 {
    let mut t = ll_small_sqrmod_sloppy(a, m, minv);
    while t >= m {
        t -= m;
    }
    t
}

// The largest b for which b^7 fits in a limb.
const MAX_7TH_ROOT: u64 = 565;

// Computes b1^exp, b2^exp, and b3^exp mod m simultaneously, for b1 <= b2 <= b3 <= 41 and m < 2^82,
// to benefit from instruction-level parallelism.
//
// This is n_ll_small_powmod_triple from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev, with the
// exponent's bits indexed within a u128 rather than limb by limb.
fn ll_small_powmod_triple(
    b1: u64,
    b2: u64,
    b3: u64,
    exp: u128,
    m: u128,
    minv: u128,
) -> (u128, u128, u128) {
    let mut x = 1;
    let mut y = 1;
    let mut z = 1;
    let mut ebits = exp.significant_bits();
    // Do the first two rounds without modular reduction, since b^7 fits in a limb.
    if ebits >= 3 && b3 <= MAX_7TH_ROOT {
        let mut x0 = b1 * b1;
        let mut y0 = b2 * b2;
        let mut z0 = b3 * b3;
        if exp.get_bit(ebits - 2) {
            x0 *= b1;
            y0 *= b2;
            z0 *= b3;
        }
        x0 *= x0;
        y0 *= y0;
        z0 *= z0;
        if exp.get_bit(ebits - 3) {
            x0 *= b1;
            y0 *= b2;
            z0 *= b3;
        }
        x = u128::from(x0);
        y = u128::from(y0);
        z = u128::from(z0);
        ebits -= 3;
    }
    for bit in (0..ebits).rev() {
        x = ll_small_sqrmod_sloppy(x, m, minv);
        y = ll_small_sqrmod_sloppy(y, m, minv);
        z = ll_small_sqrmod_sloppy(z, m, minv);
        if exp.get_bit(bit) {
            x.wrapping_mul_assign(u128::from(b1));
            y.wrapping_mul_assign(u128::from(b2));
            z.wrapping_mul_assign(u128::from(b3));
        }
    }
    (
        ll_small_reduce2_precise(x, m, minv),
        ll_small_reduce2_precise(y, m, minv),
        ll_small_reduce2_precise(z, m, minv),
    )
}

// Computes 2^exp mod m, for m < 2^82.
//
// This is n_ll_small_2_powmod from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev, where FLINT_BITS is
// 64, with the exponent's bits indexed within a u128 rather than limb by limb.
fn ll_small_2_powmod(exp: u128, m: u128, minv: u128) -> u128 {
    let mut x = 1;
    let mut ebits = exp.significant_bits();
    // The top six bits of the exponent give a power of 2 that fits in a limb.
    if ebits >= 6 {
        ebits -= 6;
        x = u128::power_of_2(u64::wrapping_from(exp >> ebits));
    }
    for bit in (0..ebits).rev() {
        x = ll_small_sqrmod_sloppy(x, m, minv);
        if exp.get_bit(bit) {
            x.wrapping_add_assign(x);
        }
    }
    ll_small_reduce2_precise(x, m, minv)
}

// The second stage of a strong probable prime test: y = a^d mod n, where n - 1 = d * 2^s.
//
// This is n_ll_small_sprp_stage2 from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
fn ll_small_sprp_stage2(mut y: u128, s: u32, nm1: u128, n: u128, ninv: u128) -> bool {
    if y == 1 || y == nm1 {
        return true;
    }
    for _ in 1..s {
        y = ll_small_sqrmod(y, n, ninv);
        if y == nm1 {
            return true;
        }
    }
    false
}

// Bases for the Sorenson–Webster test; together with 2, they prove primality below
// SORENSON_WEBSTER_BOUND.
const LL_ODD_BASES: [u64; 12] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

// Miller–Rabin for odd 2^64 < n < 2^128 with n < 357913941 * 2^64, so that the noncanonical
// residues never overflow. Returns `Some(false)` for a composite, `Some(true)` for a proven prime
// (valid when n < SORENSON_WEBSTER_BOUND), and `None` when `base_2_only` is set and n is a base-2
// strong probable prime.
//
// This is n_ll_small_is_prime_mr from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev.
fn ll_small_is_prime_mr(n: u128, base_2_only: bool) -> Option<bool> {
    let nm1 = n - 1;
    let s = nm1.trailing_zeros();
    let t = nm1 >> s;
    let ninv = ll_small_preinv(n);
    if !ll_small_sprp_stage2(ll_small_2_powmod(t, n, ninv), s, nm1, n, ninv) {
        return Some(false);
    }
    if base_2_only {
        return None;
    }
    for &[b1, b2, b3] in LL_ODD_BASES.as_chunks::<3>().0 {
        let (y1, y2, y3) = ll_small_powmod_triple(b1, b2, b3, t, n, ninv);
        if !ll_small_sprp_stage2(y1, s, nm1, n, ninv)
            || !ll_small_sprp_stage2(y2, s, nm1, n, ninv)
            || !ll_small_sprp_stage2(y3, s, nm1, n, ninv)
        {
            return Some(false);
        }
    }
    Some(true)
}

// The Sorenson–Webster bound, 3317044064679887385961981: an odd n below it that is a strong
// probable prime to the bases 2, 3, 5, ..., 41 is prime.
const SORENSON_WEBSTER_BOUND: u128 = 0x2be69_51adc5b22410a5fd;

// For n of at least this high limb, the noncanonical residues of the small-modulus arithmetic could
// overflow three limbs: 6 * 2 * (357913941 * 2^64) < 2^192.
const LL_SMALL_LIMIT_HIGH: u64 = 357913941;

const LL_NUM_TRIAL_PRIMES: usize = 64;

// For each odd prime p from 3 to 313: the inverse of p mod 2^128, and floor((2^128 - 1) / p). p
// divides n exactly when n times the inverse, mod 2^128, is at most the second value.
//
// This is ll_trial_primes from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev, with each pair of limbs
// joined into a u128.
const LL_TRIAL_INVERSES: [(u128, u128); LL_NUM_TRIAL_PRIMES] = [
    // 3
    (
        0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab,
        0x55555555555555555555555555555555,
    ),
    // 5
    (
        0xcccccccccccccccccccccccccccccccd,
        0x33333333333333333333333333333333,
    ),
    // 7
    (
        0xb6db6db6db6db6db6db6db6db6db6db7,
        0x24924924924924924924924924924924,
    ),
    // 11
    (
        0xa2e8ba2e8ba2e8ba2e8ba2e8ba2e8ba3,
        0x1745d1745d1745d1745d1745d1745d17,
    ),
    // 13
    (
        0xc4ec4ec4ec4ec4ec4ec4ec4ec4ec4ec5,
        0x13b13b13b13b13b13b13b13b13b13b13,
    ),
    // 17
    (
        0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f1,
        0x0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f,
    ),
    // 19
    (
        0xbca1af286bca1af286bca1af286bca1b,
        0x0d79435e50d79435e50d79435e50d794,
    ),
    // 23
    (
        0x4de9bd37a6f4de9bd37a6f4de9bd37a7,
        0x0b21642c8590b21642c8590b21642c85,
    ),
    // 29
    (
        0xc234f72c234f72c234f72c234f72c235,
        0x08d3dcb08d3dcb08d3dcb08d3dcb08d3,
    ),
    // 31
    (
        0xdef7bdef7bdef7bdef7bdef7bdef7bdf,
        0x08421084210842108421084210842108,
    ),
    // 37
    (
        0xc1bacf914c1bacf914c1bacf914c1bad,
        0x06eb3e45306eb3e45306eb3e45306eb3,
    ),
    // 41
    (
        0x18f9c18f9c18f9c18f9c18f9c18f9c19,
        0x063e7063e7063e7063e7063e7063e706,
    ),
    // 43
    (
        0xbe82fa0be82fa0be82fa0be82fa0be83,
        0x05f417d05f417d05f417d05f417d05f4,
    ),
    // 47
    (
        0x3677d46cefa8d9df51b3bea3677d46cf,
        0x0572620ae4c415c9882b9310572620ae,
    ),
    // 53
    (
        0x13521cfb2b78c13521cfb2b78c13521d,
        0x04d4873ecade304d4873ecade304d487,
    ),
    // 59
    (
        0x8f2fba9386822b63cbeea4e1a08ad8f3,
        0x0456c797dd49c34115b1e5f75270d045,
    ),
    // 61
    (
        0x14fbcda3ac10c9714fbcda3ac10c9715,
        0x04325c53ef368eb04325c53ef368eb04,
    ),
    // 67
    (
        0xc2dd9ca81e9131abf0b7672a07a44c6b,
        0x03d226357e16ece540f4898d5f85bb39,
    ),
    // 71
    (
        0x4f52edf8c9ea5dbf193d4bb7e327a977,
        0x039b0ad12073615a240e6c2b4481cd85,
    ),
    // 73
    (
        0x3f1f8fc7e3f1f8fc7e3f1f8fc7e3f1f9,
        0x0381c0e070381c0e070381c0e070381c,
    ),
    // 79
    (
        0xd5df984dc5abbf309b8b577e613716af,
        0x033d91d2a2067b23a5440cf6474a8819,
    ),
    // 83
    (
        0x2818acb90f6bf3a9a3784a062b2e43db,
        0x03159721ed7e75346f0940c565c87b5f,
    ),
    // 89
    (
        0xd1fa3f47e8fd1fa3f47e8fd1fa3f47e9,
        0x02e05c0b81702e05c0b81702e05c0b81,
    ),
    // 97
    (
        0x5f02a3a0fd5c5f02a3a0fd5c5f02a3a1,
        0x02a3a0fd5c5f02a3a0fd5c5f02a3a0fd,
    ),
    // 101
    (
        0xc32b16cfd7720f353a4c0a237c32b16d,
        0x0288df0cac5b3f5dc83cd4e930288df0,
    ),
    // 103
    (
        0xd0c6d5bf60ee9a18dab7ec1dd3431b57,
        0x027c45979c95204f88b2f392a409f116,
    ),
    // 107
    (
        0xa2b10bf66e0e5aea77a04c8f8d28ac43,
        0x02647c69456217ecdc1cb5d4ef40991f,
    ),
    // 109
    (
        0xc0964fda6c0964fda6c0964fda6c0965,
        0x02593f69b02593f69b02593f69b02593,
    ),
    // 113
    (
        0xc090fdbc090fdbc090fdbc090fdbc091,
        0x0243f6f0243f6f0243f6f0243f6f0243,
    ),
    // 127
    (
        0xbf7efdfbf7efdfbf7efdfbf7efdfbf7f,
        0x02040810204081020408102040810204,
    ),
    // 131
    (
        0xf82ee6986d6f63aa03e88cb3c9484e2b,
        0x01f44659e4a427157f05dcd30dadec75,
    ),
    // 137
    (
        0x21a291c077975b8fe21a291c077975b9,
        0x01de5d6e3f8868a4701de5d6e3f8868a,
    ),
    // 139
    (
        0xa2126ad1f4f31ba03aef6ca970586723,
        0x01d77b654b82c33917f14424d5a3e9e6,
    ),
    // 149
    (
        0x93c225cc74d50c06df5b0f768ce2cabd,
        0x01b7d6c3dda338b2af3f920a4f089731,
    ),
    // 151
    (
        0x26fe4dfc9bf937f26fe4dfc9bf937f27,
        0x01b2036406c80d901b2036406c80d901,
    ),
    // 157
    (
        0x0685b4fe5e92c0685b4fe5e92c0685b5,
        0x01a16d3f97a4b01a16d3f97a4b01a16d,
    ),
    // 163
    (
        0x8bc775ca99ea03241f693a1c451ab30b,
        0x01920fb49d0e228d59857f36f825b178,
    ),
    // 167
    (
        0x513ed9ad38b7f3bc8d07aa27db35a717,
        0x01886e5f0abb04994b1d20310dcbe157,
    ),
    // 173
    (
        0x133caba736c05eb4882383b30d516325,
        0x017ad2208e0ecc35458c93fa14b77dc7,
    ),
    // 179
    (
        0x0e4d3aa30a02dc3eed6866f8d962ae7b,
        0x016e1f76b4337c6cb1573d7f48f044a5,
    ),
    // 181
    (
        0x6fbc1c498c05a84f3454dca410f8ed9d,
        0x016a13cd153729043e3b673fa57b0cba,
    ),
    // 191
    (
        0x7749b79f7f5470961d7ca632ee936f3f,
        0x01571ed3c506b39a22d9218202ae3da7,
    ),
    // 193
    (
        0x90948f40feac6f6b70bf015390948f41,
        0x015390948f40feac6f6b70bf01539094,
    ),
    // 197
    (
        0x0bb207cc0532ae21c96bdb9d3d137e0d,
        0x014cab88725af6e74f44df833facd51d,
    ),
    // 199
    (
        0x7a3607b7f5b5630e2697cc8aef46c0f7,
        0x0149539e3b2d066ea21727e120292a73,
    ),
    // 211
    (
        0x2f514a026d31be7bc0e8f2a76e68575b,
        0x013698df3de0747953b7342bad7f64b3,
    ),
    // 223
    (
        0xdd8f7f6d0eec7bfb687763dfdb43bb1f,
        0x0125e22708092f113840497889c2024b,
    ),
    // 227
    (
        0x766a024168e18cf81b10ea929ba144cb,
        0x0120b470c67c0d8875494dd0a2657f6f,
    ),
    // 229
    (
        0x0c4c0478bbcecfee1d10c4c0478bbced,
        0x011e2ef3b3fb8744313011e2ef3b3fb8,
    ),
    // 233
    (
        0x758fee6bac7f735d63fb9aeb1fdcd759,
        0x0119453808ca29c046514e02328a7011,
    ),
    // 239
    (
        0x077f76e538c5167e64afaa4f437b2e0f,
        0x0112358e75d30336a0ab617909a3e202,
    ),
    // 241
    (
        0x10fef010fef010fef010fef010fef011,
        0x010fef010fef010fef010fef010fef01,
    ),
    // 251
    (
        0xa020a32fefae680828cbfbeb9a020a33,
        0x0105197f7d734041465fdf5cd0105197,
    ),
    // 257
    (
        0xff00ff00ff00ff00ff00ff00ff00ff01,
        0x00ff00ff00ff00ff00ff00ff00ff00ff,
    ),
    // 263
    (
        0xf836826ef73d52bcd624fd1470e99cb7,
        0x00f92fb2211855a8653b605d71e2cc69,
    ),
    // 269
    (
        0x3ce8354b2ea1c8cd8fb3ddbd6205b5c5,
        0x00f3a0d52cba8723363ecf76f58816d7,
    ),
    // 271
    (
        0x8715ba188f963302d57da36ca27acdef,
        0x00f1d48bcee0d399fa5504b926bb0a64,
    ),
    // 277
    (
        0xb25e4463cff13686ee70c03b25e4463d,
        0x00ec979118f3fc4da1bb9c300ec97911,
    ),
    // 281
    (
        0x6c69ae01d272ca3fc5b1a6b80749cb29,
        0x00e939651fe2d8d35c03a4e5947f8b63,
    ),
    // 283
    (
        0xf26e5c44bfc61b2347768073c9b97113,
        0x00e79372e225fe30d91a3bb4039e4dcb,
    ),
    // 293
    (
        0xb07dd0d1b15d7cf12591e94884ce32ad,
        0x00dfac1f74346c575f3c49647a522133,
    ),
    // 307
    (
        0xd2f87ebfcaa1c5a0f02806abc74be1fb,
        0x00d578e97c3f5fe550e2d078140355e3,
    ),
    // 311
    (
        0xbe25dd6d7aa646ca7ec3e8f3a7198487,
        0x00d2ba083b445250ab3726b02782e18b,
    ),
    // 313
    (
        0xbc1d71afd8bdc03458550f8a39409d09,
        0x00d161543e28e5027423fcba7aaf075c,
    ),
];

// FLINT compares only the high limbs below a bound on n, and both limbs above it; comparing the
// whole u128 is exact everywhere.
#[inline]
const fn ll_trial(n: u128, i: usize) -> bool {
    n.wrapping_mul(LL_TRIAL_INVERSES[i].0) <= LL_TRIAL_INVERSES[i].1
}

// Tests an n >= 2^64 for primality. Returns `Some(false)` for a composite, `Some(true)` for a
// proven prime, and `None` when n >= SORENSON_WEBSTER_BOUND has no small factor and is a base-2
// strong probable prime.
//
// This is n_ll_is_prime from ulong_extras/ll_is_prime.c, FLINT 3.7.0-dev, where FLINT_BITS is 64,
// except that FLINT runs the base-2 test for the largest n with fmpz arithmetic, and Malachite with
// two-limb Montgomery arithmetic.
crate_test_fn! {ll_is_prime(n: u128) -> Option<bool> {
    assert!(n.upper_half() != 0);
    if n.even() || (0..LL_NUM_TRIAL_PRIMES >> 1).any(|i| ll_trial(n, i)) {
        return Some(false);
    }
    if n < SORENSON_WEBSTER_BOUND {
        return ll_small_is_prime_mr(n, false);
    }
    if n.upper_half() < LL_SMALL_LIMIT_HIGH {
        return ll_small_is_prime_mr(n, true);
    }
    // The base-2 test is slower now, so do some extra trial division.
    if (LL_NUM_TRIAL_PRIMES >> 1..LL_NUM_TRIAL_PRIMES).any(|i| ll_trial(n, i)) {
        return Some(false);
    }
    if u128_is_strong_probable_prime_odd(n, 2) {
        None
    } else {
        Some(false)
    }
}}

// Data for Montgomery arithmetic modulo an odd n > 1, with R = 2^128.
pub(crate) struct RedcContext128 {
    pub(crate) n: u128,
    // -1 / n mod R
    n_redc: u128,
    // R mod n: 1 in Montgomery form
    pub(crate) one: u128,
    // R^2 mod n
    r_squared: u128,
}

// Computes the inverse of an odd a modulo 2^128, by Newton's iteration from 5 correct bits.
fn binvert_128(a: u128) -> u128 {
    // correct to 5 bits
    let mut r = a.wrapping_mul(3) ^ 2;
    // 10, 20, 40, 80, and 160 bits
    for _ in 0..5 {
        r.wrapping_mul_assign(2u128.wrapping_sub_mul(a, r));
    }
    r
}

// Computes [hi, lo] mod n, for hi < n.
fn rem_256_by_128(hi: u128, lo: u128, n: u128) -> u128 {
    let s = n.leading_zeros();
    let d = n << s;
    // Since hi < n, the top two limbs of the shifted value are less than d.
    let (h, l) = if s == 0 {
        (hi, lo)
    } else {
        ((hi << s) | (lo >> (u128::WIDTH - u64::from(s))), lo << s)
    };
    let (h1, h0) = h.split_in_half();
    let (l1, l0) = l.split_in_half();
    let (r1, r0) = div_3_by_2_normalized(h1, h0, l1, d).1.split_in_half();
    div_3_by_2_normalized(r1, r0, l0, d).1 >> s
}

pub(crate) fn redc_context_128(n: u128) -> RedcContext128 {
    // R mod n = (R - 1) mod n + 1, which is already reduced: (R - 1) mod n = n - 1 would mean that
    // n divides R, but n is odd and greater than 1.
    let one = u128::MAX % n + 1;
    let (hi, lo) = u128::x_mul_y_to_zz(one, one);
    RedcContext128 {
        n,
        n_redc: binvert_128(n).wrapping_neg(),
        one,
        r_squared: rem_256_by_128(hi, lo, n),
    }
}

// Computes ab / R mod n, for a, b < n, reducing to [0, n).
#[inline]
pub(crate) fn redc_mul_128(a: u128, b: u128, ctx: &RedcContext128) -> u128 {
    let (hi, lo) = u128::x_mul_y_to_zz(a, b);
    let m = lo.wrapping_mul(ctx.n_redc);
    let mn_hi = u128::x_mul_y_to_zz(m, ctx.n).0;
    // lo + the low half of mn is 0 mod R, and carries exactly when lo is nonzero.
    let mut t = hi;
    let overflow_1 = t.overflowing_add_assign(mn_hi);
    let overflow_2 = t.overflowing_add_assign(u128::from(lo != 0));
    if overflow_1 || overflow_2 || t >= ctx.n {
        t.wrapping_sub(ctx.n)
    } else {
        t
    }
}

// Converts a < n to Montgomery form.
#[inline]
pub(crate) fn to_redc_128(a: u128, ctx: &RedcContext128) -> u128 {
    redc_mul_128(a, ctx.r_squared, ctx)
}

// Computes x^exp in Montgomery form, left to right.
pub(crate) fn redc_pow_128(x: u128, exp: u128, ctx: &RedcContext128) -> u128 {
    let mut y = ctx.one;
    for bit in (0..exp.significant_bits()).rev() {
        y = redc_mul_128(y, y, ctx);
        if exp.get_bit(bit) {
            y = redc_mul_128(y, x, ctx);
        }
    }
    y
}

// Computes 2^exp in Montgomery form, left to right. The top 7 bits of exp give a power of 2 that
// fits in a u128, and each remaining bit costs a squaring and at most a doubling, rather than a
// multiplication.
fn redc_two_pow_128(exp: u128, ctx: &RedcContext128) -> u128 {
    let n = ctx.n;
    if exp < 128 {
        return to_redc_128(u128::power_of_2(u64::exact_from(exp)) % n, ctx);
    }
    let bits = exp.significant_bits();
    let mut y = to_redc_128(
        u128::power_of_2(u64::exact_from(exp >> (bits - 7))) % n,
        ctx,
    );
    for bit in (0..bits - 7).rev() {
        y = redc_mul_128(y, y, ctx);
        if exp.get_bit(bit) {
            y.mod_add_assign(y, n);
        }
    }
    y
}

// A strong probable prime test to base a, for odd n > 1 and a reduced mod n, with a not in {0, 1, n
// - 1}.
crate_test_fn! {u128_is_strong_probable_prime_odd(n: u128, a: u128) -> bool {
    let ctx = redc_context_128(n);
    let nm1 = n - 1;
    let s = nm1.trailing_zeros();
    let minus_one = n - ctx.one;
    let d = nm1 >> s;
    let mut y = if a == 2 {
        redc_two_pow_128(d, &ctx)
    } else {
        redc_pow_128(to_redc_128(a, &ctx), d, &ctx)
    };
    if y == ctx.one || y == minus_one {
        return true;
    }
    for _ in 1..s {
        y = redc_mul_128(y, y, &ctx);
        if y == minus_one {
            return true;
        }
    }
    false
}}

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
    /// This implementation looks the answer up in a table, after checking for divisibility by 2, 3,
    /// and 5 when the input is at least $2^{15}$.
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

impl IsPrime for u128 {
    /// Tests whether a `u128` is prime.
    ///
    /// Inputs less than $2^{64}$ are tested as `u64`s. A larger input is checked for divisibility
    /// by the odd primes up to 137, or up to 313 above $357913941 \cdot 2^{64}$, and must pass a
    /// strong probable prime test to base 2. Below $3317044064679887385961981 \approx 2^{81.5}$,
    /// strong probable prime tests to the bases 2, 3, 5, …, 41 then prove primality, by a result
    /// of Sorenson and Webster. A larger input is proven prime or composite by the $n - 1$ and $n +
    /// 1$ tests of Pocklington, Morrison, and Brillhart, Lehmer, and Selfridge, which use the
    /// primes found by trial division of $n - 1$ and $n + 1$, together with Lenstra's search for
    /// divisors in residue classes. If those tests cannot settle it, the APR-CL test with Jacobi
    /// sums, of Adleman, Pomerance, Rumely, Cohen, and Lenstra, does.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: each test
    /// is a bounded number of modular powerings, each costing a bounded number of modular
    /// multiplications per bit.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    ///
    /// assert_eq!(5u128.is_prime(), true);
    /// assert_eq!(6u128.is_prime(), false);
    /// // 2^127 - 1
    /// assert_eq!(170141183460469231731687303715884105727u128.is_prime(), true);
    /// // the largest prime less than 2^128
    /// assert_eq!(340282366920938463463374607431768211297u128.is_prime(), true);
    /// // a strong pseudoprime to each of the first 13 prime bases
    /// assert_eq!(3317044064679887385961981u128.is_prime(), false);
    /// ```
    ///
    /// This is equivalent to `fmpz_is_prime` from `fmpz.h`, FLINT 3.7.0-dev, for two-limb inputs.
    fn is_prime(&self) -> bool {
        let n = *self;
        if let Ok(n) = u64::try_from(n) {
            return n.is_prime();
        }
        ll_is_prime(n)
            .or_else(|| u128_is_prime_n_pm_1(n, |m| Some(m.is_prime())))
            .unwrap_or_else(|| u128_is_prime_aprcl(n))
    }
}
