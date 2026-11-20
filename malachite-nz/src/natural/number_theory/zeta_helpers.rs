// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Rigorous enclosures of a few real numbers -- pi, the zeta function and Dirichlet's beta function
// at integers -- computed with integer arithmetic alone, and the Bernoulli and Euler numbers that
// they give. `Float`, which has its own pi, lives in a crate that depends on this one, so these are
// computed here from scratch.

use crate::integer::Integer;
use crate::integer_matrix::arithmetic::mul::multi_mod::primes_above;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    AddMul, DivRound, FloorSqrt, ModPowerOf2, MulAddMul, Parity, Pow, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::Primes;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::number_theory::traits::Factorial;
use malachite_base::rounding_modes::RoundingMode::*;

// An enclosure [lo * 2^exp, hi * 2^exp] of a positive real number. The operations round the lower
// end down and the upper end up, so the result encloses the exact result of the operation on any
// numbers in the enclosures of the operands; they keep about `bits` significant bits.
#[derive(Clone, Debug)]
pub(crate) struct Bounds {
    lo: Natural,
    hi: Natural,
    exp: i64,
}

impl Bounds {
    // The exact positive number `n` * 2^`exp`.
    fn exact(n: Natural, exp: i64) -> Self {
        assert_ne!(n, 0u32);
        Self {
            lo: n.clone(),
            hi: n,
            exp,
        }
    }

    // Shifts both ends right until the upper end has at most `bits` significant bits.
    fn normalize(mut self, bits: u64) -> Self {
        let excess = self.hi.significant_bits().saturating_sub(bits);
        if excess != 0 {
            self.lo >>= excess;
            self.hi.shr_round_assign(excess, Ceiling);
            self.exp += i64::exact_from(excess);
        }
        self
    }

    fn mul(&self, other: &Self, bits: u64) -> Self {
        Self {
            lo: &self.lo * &other.lo,
            hi: &self.hi * &other.hi,
            exp: self.exp + other.exp,
        }
        .normalize(bits)
    }

    // The quotient, with at least `bits` significant bits in each end. The divisor's lower end must
    // be positive.
    fn div(&self, other: &Self, bits: u64) -> Self {
        assert_ne!(other.lo, 0u32);
        // lo << shift has at least bits + bits(other.hi) bits, so the quotients have at least
        // `bits`.
        let shift = (bits + other.hi.significant_bits()).saturating_sub(self.lo.significant_bits());
        Self {
            lo: (&self.lo << shift) / &other.hi,
            hi: (&self.hi << shift).div_round(&other.lo, Ceiling).0,
            exp: self.exp - other.exp - i64::exact_from(shift),
        }
        .normalize(bits + 1)
    }

    fn pow(&self, mut n: u64, bits: u64) -> Self {
        let mut result = Self::exact(Natural::ONE, 0);
        let mut power = self.clone();
        loop {
            if n.odd() {
                result = result.mul(&power, bits);
            }
            n >>= 1;
            if n == 0 {
                return result;
            }
            power = power.mul(&power, bits);
        }
    }

    // The two ends with the same exponent: the lower one of `self` and of `other`.
    fn aligned(&self, other: &Self) -> (Natural, Natural, Natural, Natural, i64) {
        let exp = self.exp.min(other.exp);
        let s = u64::exact_from(self.exp - exp);
        let o = u64::exact_from(other.exp - exp);
        (
            &self.lo << s,
            &self.hi << s,
            &other.lo << o,
            &other.hi << o,
            exp,
        )
    }

    // `self` - `other`, which must be positive.
    fn sub(&self, other: &Self, bits: u64) -> Self {
        let (lo, hi, other_lo, other_hi, exp) = self.aligned(other);
        assert!(lo > other_hi, "the difference is not positive");
        Self {
            lo: lo - other_hi,
            hi: hi - other_lo,
            exp,
        }
        .normalize(bits)
    }

    // Widens the enclosure by an upper bound `e` of an absolute error, which must be smaller than
    // the lower end.
    fn widen(&self, e: &Self, bits: u64) -> Self {
        let (lo, hi, _, e_hi, exp) = self.aligned(e);
        assert!(lo > e_hi, "the error is not smaller than the value");
        Self {
            lo: lo - &e_hi,
            hi: hi + e_hi,
            exp,
        }
        .normalize(bits)
    }

    // The enclosure from the lower end of `self` to the upper end of `other`.
    fn hull(&self, other: &Self) -> Self {
        let (lo, _, _, hi, exp) = self.aligned(other);
        Self { lo, hi, exp }
    }

    // The integer in the enclosure, if there is exactly one.
    fn unique_integer(&self) -> Option<Natural> {
        let (lo, hi) = if self.exp >= 0 {
            fail_on_untested_path("Bounds::unique_integer, self.exp >= 0");
            let e = u64::exact_from(self.exp);
            (&self.lo << e, &self.hi << e)
        } else {
            let e = self.exp.unsigned_abs();
            (
                (&self.lo).shr_round(e, Ceiling).0,
                (&self.hi).shr_round(e, Floor).0,
            )
        };
        (lo == hi).then_some(lo)
    }
}

// 2^`e`, as an exact enclosure.
fn power_of_2(e: i64) -> Bounds {
    Bounds::exact(Natural::ONE, e)
}

const FIVE: Natural = Natural::const_from(5);
const SIX: Natural = Natural::const_from(6);
// The constants of the Chudnovsky series: 1/pi = 12 sum_k (-1)^k (6k)! (A + B k) / ((3k)! (k!)^3
// 640320^(3k + 3/2)), and 426880 sqrt(10005) = 640320^(3/2) / 12.
const CHUDNOVSKY_A: Natural = Natural::const_from(13591409);
const CHUDNOVSKY_B: Natural = Natural::const_from(545140134);
const CHUDNOVSKY_C: Natural = Natural::const_from(426880);

// The terms of the Chudnovsky series for 1/pi from index `a` (inclusive) to `b` (exclusive), by
// binary splitting: (P, Q, T), where T / Q is the sum of the terms and P the product of the
// numerators of the ratios of consecutive terms.
fn chudnovsky(a: u64, b: u64) -> (Natural, Natural, Integer) {
    if b - a == 1 {
        let (p, q) = if a == 0 {
            (Natural::ONE, Natural::ONE)
        } else {
            let a = Natural::from(a);
            (
                (SIX * &a - FIVE) * ((&a << 1u32) - Natural::ONE) * (SIX * &a - Natural::ONE),
                (&a).pow(3) * Natural::from(10939058860032000u64),
            )
        };
        let t = Integer::from(&p * CHUDNOVSKY_A.add_mul(CHUDNOVSKY_B, Natural::from(a)));
        (p, q, if a.odd() { -t } else { t })
    } else {
        let m = a + ((b - a) >> 1);
        let (p_am, q_am, t_am) = chudnovsky(a, m);
        let (p_mb, q_mb, t_mb) = chudnovsky(m, b);
        let t = Integer::from(&q_mb).mul_add_mul(t_am, Integer::from(&p_am), t_mb);
        (p_am * p_mb, q_am * q_mb, t)
    }
}

// An enclosure of pi with about `bits` significant bits.
//
// With the Chudnovsky series 1/pi = 12 sum_k (-1)^k (6k)! (13591409 + 545140134 k) / ((3k)! (k!)^3
// 640320^(3k + 3/2)), pi = 426880 sqrt(10005) / S, where S = sum_k a_k and a_k = (-1)^k (6k)!
// (13591409 + 545140134 k) / ((3k)! (k!)^3 640320^(3k)). The sum of the first K terms is computed
// exactly by binary splitting. Since (6k)! / ((3k)! (k!)^3) = binomial(6k, 3k) (3k)! / (k!)^3 <=
// 64^k 27^k = 1728^k, 13591409 + 545140134 k < 2^30 (k + 1), and 640320^3 / 1728 > 2^47, the
// omitted terms of this alternating series of decreasing terms add up to at most |a_K| < 2^(30 -
// 47K) (K + 1); and S > 2^23.
pub(crate) fn pi_bounds(bits: u64) -> Bounds {
    let w = bits + 64;
    let mut terms = 1;
    // 47K - 30 - bits(K + 1) >= w + 30
    while 47 * terms < w + 60 + (terms + 1).significant_bits() {
        terms += 1;
    }
    let (_, q, t) = chudnovsky(0, terms);
    let t = Natural::exact_from(&t);
    // T / Q, rounded down and up to w fractional bits, widened by the bound on the omitted terms,
    // 2^(31 + bits(K + 1) - 47K) <= 2^(-w - 29).
    let shifted = &t << w;
    let s_lo = &shifted / &q;
    let s_hi = (&shifted).div_round(&q, Ceiling).0;
    let s = Bounds {
        lo: s_lo,
        hi: s_hi,
        exp: -i64::exact_from(w),
    }
    .widen(&power_of_2(-i64::exact_from(w)), w);
    // 426880 sqrt(10005)
    let root = (const { Natural::const_from(10005) } << (w << 1)).floor_sqrt();
    let c = Bounds {
        lo: &root * CHUDNOVSKY_C,
        hi: (root + Natural::ONE) * CHUDNOVSKY_C,
        exp: -i64::exact_from(w),
    };
    c.div(&s, bits + 8)
}

// The number of primes to take in an Euler product for a function whose terms are at most m^(-s),
// for the tail to stay below 2^(-w): the least K with K^(1 - s) / (s - 1) <= 2^(-w), which is what
// the sum of m^(-s) over m > K is at most.
fn euler_product_limit(s: u64, w: u64) -> u64 {
    assert!(s >= 2);
    // K^(s - 1) (s - 1) >= 2^w holds when (s - 1) (bits(K) - 1) + bits(s - 1) - 1 >= w. The callers
    // only use Euler products for an s large enough that K stays small.
    let mut k = 2u64;
    while (s - 1) * (k.significant_bits() - 1) + (s - 1).significant_bits() - 1 < w + 2 {
        assert!(
            k < 1 << 32,
            "an Euler product for s = {s} at {w} bits needs too many primes"
        );
        k <<= 1;
    }
    k
}

// An upper bound on the sum of m^(-s) over m > K, K^(1 - s) / (s - 1), as an enclosure whose upper
// end is that bound.
fn tail_bound(s: u64, k: u64, bits: u64) -> Bounds {
    let denominator = Bounds::exact(Natural::from(k).pow(s - 1) * Natural::from(s - 1), 0);
    Bounds::exact(Natural::ONE, 0).div(&denominator, bits)
}

// An enclosure of zeta(s) = sum_{m >= 1} m^(-s) = prod_p 1 / (1 - p^(-s)) for an integer s >= 2,
// with about `bits` significant bits, by the Euler product over the primes up to K. The omitted
// factors are all greater than 1, and their product is at most 1 / (1 - t), where t >= sum_{m > K}
// m^(-s).
//
// This replaces `arb_zeta_ui_euler_product` from FLINT, which uses ball arithmetic.
pub(crate) fn zeta_bounds(s: u64, bits: u64) -> Bounds {
    let w = bits + (bits.significant_bits() << 1) + 16;
    let k = euler_product_limit(s, w);
    let mut product = Bounds::exact(Natural::ONE, 0);
    for p in u64::primes_less_than_or_equal_to(&k) {
        let power = Natural::from(p).pow(s);
        let factor =
            Bounds::exact(power.clone(), 0).div(&Bounds::exact(power - Natural::ONE, 0), w);
        product = product.mul(&factor, w);
    }
    // The omitted factors multiply the product by a number in [1, 1 / (1 - t)].
    let one = Bounds::exact(Natural::ONE, 0);
    let tail = one.div(&one.sub(&tail_bound(s, k, w), w), w);
    product.hull(&product.mul(&tail, w)).normalize(bits)
}

// An enclosure of Dirichlet's beta function beta(s) = sum_{m >= 0} (-1)^m (2m + 1)^(-s) = prod_{p
// odd} 1 / (1 - chi(p) p^(-s)) for an integer s >= 2, with about `bits` significant bits, where
// chi(p) is 1 for p = 1 mod 4 and -1 for p = 3 mod 4, by the Euler product over the odd primes up
// to K. The product of the omitted factors lies between 1 - t and 1 / (1 - t), where t >= sum_{m >
// K} m^(-s).
pub(crate) fn dirichlet_beta_bounds(s: u64, bits: u64) -> Bounds {
    let w = bits + (bits.significant_bits() << 1) + 16;
    let k = euler_product_limit(s, w);
    let mut product = Bounds::exact(Natural::ONE, 0);
    for p in u64::primes_less_than_or_equal_to(&k).skip(1) {
        let power = Natural::from(p).pow(s);
        let denominator = if p.mod_power_of_2(2) == 1 {
            &power - Natural::ONE
        } else {
            &power + Natural::ONE
        };
        let factor = Bounds::exact(power, 0).div(&Bounds::exact(denominator, 0), w);
        product = product.mul(&factor, w);
    }
    // The omitted factors multiply the product by a number in [1 - t, 1 / (1 - t)].
    let one = Bounds::exact(Natural::ONE, 0);
    let low = one.sub(&tail_bound(s, k, w), w);
    let high = one.div(&low, w);
    product.mul(&low.hull(&high), w).normalize(bits)
}

// An estimate of the number of bits of the numerator of |B_n| * D for an even n, where D is the
// denominator of B_n: log2(2 n! / (2 pi)^n), by Stirling's formula, plus a few bits.
//
// This is arith_bernoulli_number_size from arith/bernoulli_number_size.c, FLINT 3.6.0.
#[doc(hidden)]
pub fn bernoulli_number_size(n: u64) -> f64 {
    // |B_n| < 2
    if n <= 14 {
        return 1.0;
    }
    let n_f = n as f64;
    let mut x = 2.0 + (n_f + 1.0) * libm::log(n_f + 1.0) * core::f64::consts::LOG2_E;
    // log2(2 pi e)
    x -= n_f * 4.094_191_170_361_282;
    x + 2.0
}

// An estimate of the number of bits of |E_n|.
//
// This is arith_euler_number_size from arith/euler_number_size.c, FLINT 3.6.0.
#[doc(hidden)]
pub fn euler_number_size(n: u64) -> f64 {
    let n_f = n as f64;
    let mut x = n_f + 2.0;
    x += ((n_f + 1.0) * libm::log(n_f + 1.0) - n_f) * core::f64::consts::LOG2_E;
    // log2(pi)
    x -= 1.6514961294723 * (n_f + 1.0);
    x + 2.0
}

// The working precision for an integer of about `size` bits: enough for its enclosure to be
// narrower than 1, with room for the rounding errors of the n-th power of pi.
fn working_bits(size: f64, n: u64) -> u64 {
    // `size` is positive and far below 2^53
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let size = size.ceil() as u64;
    size + (n.significant_bits() << 1) + 32
}

// |B_n| times the denominator `den` of B_n, for an even n >= 2: 2 n! zeta(n) / (2 pi)^n times
// `den`, which is an integer. It is computed from enclosures of zeta(n) and pi, at a precision that
// grows until the enclosure of the result contains only one integer.
//
// This replaces _bernoulli_fmpq_ui_zeta from bernoulli/fmpq_ui_zeta.c, FLINT 3.6.0, which uses ball
// arithmetic.
#[doc(hidden)]
pub fn bernoulli_number_numerator_abs(n: u64, den: &Natural) -> Natural {
    assert!(n >= 2 && n.even());
    let size = bernoulli_number_size(n) + den.significant_bits() as f64;
    let mut bits = working_bits(size, n);
    let numerator = Bounds::exact((Natural::factorial(n) * den) << 1u32, 0);
    loop {
        let zeta = zeta_bounds(n, bits);
        let two_pi = pi_bounds(bits + n.significant_bits()).mul(&power_of_2(1), bits + 8);
        let x = numerator
            .mul(&zeta, bits)
            .div(&two_pi.pow(n, bits + n.significant_bits() + 8), bits);
        if let Some(result) = x.unique_integer() {
            return result;
        }
        fail_on_untested_path("bernoulli_number_numerator_abs, no unique integer");
        bits += bits >> 1;
    }
}

// |E_n| for an even n >= 2: 2^(n + 2) n! beta(n + 1) / pi^(n + 1), which is an integer, computed
// like `bernoulli_number_numerator_abs`.
//
// This replaces arb_fmpz_euler_number_ui from arb/euler_number_ui.c, FLINT 3.6.0, which uses ball
// arithmetic.
#[doc(hidden)]
pub fn euler_number_abs(n: u64) -> Natural {
    assert!(n >= 2 && n.even());
    let mut bits = working_bits(euler_number_size(n), n);
    let numerator = Bounds::exact(Natural::factorial(n), i64::exact_from(n + 2));
    loop {
        let beta = dirichlet_beta_bounds(n + 1, bits);
        let pi = pi_bounds(bits + n.significant_bits());
        let x = numerator
            .mul(&beta, bits)
            .div(&pi.pow(n + 1, bits + n.significant_bits() + 8), bits);
        if let Some(result) = x.unique_integer() {
            return result;
        }
        fail_on_untested_path("euler_number_abs, no unique integer");
        bits += bits >> 1;
    }
}

// The number of bits of the primes of the multimodular algorithm.
const MULTI_MOD_PRIME_BITS: u64 = 63;

// The primes of the multimodular algorithm are the ones above this power of 2.
const MULTI_MOD_PRIME_FLOOR: u64 = 1 << MULTI_MOD_PRIME_BITS;

// The number of primes above 2^63 whose product exceeds 2^`size`.
fn multi_mod_prime_count(size: f64) -> usize {
    // `size` is positive and far below 2^53
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let size = size.ceil() as u64;
    usize::exact_from(size.div_ceil(MULTI_MOD_PRIME_BITS))
}

// A vector of `len` nonnegative integers, by the multimodular algorithm: `residues(p)` returns the
// vector modulo p, for a prime p above 2^63, and each entry is recombined by the Chinese remainder
// theorem from its residues modulo just the primes that its size needs. Entry k has fewer than
// `size(k)` bits, and none has `max_size` bits or more.
//
// This is the recombination step of _arith_bernoulli_number_vec_multi_mod from
// arith/bernoulli_number_vec_multi_mod.c and of __euler_number_vec_multi_mod from
// arith/euler_number_vec.c, FLINT 3.6.0.
#[doc(hidden)]
pub fn multi_mod_vec(
    len: usize,
    max_size: f64,
    size: impl Fn(usize) -> f64,
    residues: impl Fn(u64) -> Vec<u64>,
) -> Vec<Natural> {
    let prime_count = multi_mod_prime_count(max_size);
    let primes = primes_above(MULTI_MOD_PRIME_FLOOR, prime_count);
    let residues: Vec<Vec<u64>> = primes.iter().map(|&p| residues(p)).collect();
    let moduli: Vec<Natural> = primes.into_iter().map(Natural::from).collect();
    (0..len)
        .map(|k| {
            let count = multi_mod_prime_count(size(k)).min(prime_count);
            let values: Vec<Natural> = residues[..count]
                .iter()
                .map(|r| Natural::from(r[k]))
                .collect();
            Natural::multi_crt(&moduli[..count], &values).unwrap()
        })
        .collect()
}

crate_test_fn! {
    // The lower and upper ends of an enclosure of pi with about `bits` significant bits, as
    // (numerator, exponent) pairs: pi lies in [lo * 2^exp, hi * 2^exp].
    pi_enclosure(bits: u64) -> (Natural, Natural, i64) {
        let b = pi_bounds(bits);
        (b.lo, b.hi, b.exp)
    }
}

crate_test_fn! {
    // The ends of an enclosure of zeta(s), like `pi_enclosure`.
    zeta_enclosure(s: u64, bits: u64) -> (Natural, Natural, i64) {
        let b = zeta_bounds(s, bits);
        (b.lo, b.hi, b.exp)
    }
}

crate_test_fn! {
    // The ends of an enclosure of beta(s), like `pi_enclosure`.
    dirichlet_beta_enclosure(s: u64, bits: u64) -> (Natural, Natural, i64) {
        let b = dirichlet_beta_bounds(s, bits);
        (b.lo, b.hi, b.exp)
    }
}
