// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 William Hart
//
//      Copyright © 2020 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::fail_on_untested_path;
use crate::num::arithmetic::traits::{DivMod, OverflowingAddAssign, Parity, PowerOf2};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::conversion::traits::{ExactFrom, JoinHalves, RoundingFrom, SplitInHalf};
use crate::num::factorization::is_prime::{
    RedcContext128, redc_context_128, redc_mul_128, redc_pow_128, to_redc_128,
};
use crate::num::factorization::traits::{
    IsProbablePrime, IsSquare, IsStrongProbablePrime, Primes, RemovePowerAssign,
};
use crate::num::logic::traits::{BitAccess, NotAssign, SignificantBits};
use crate::num::modular::traits::{
    DivisibleBy, ModAdd, ModInverse, ModMul, ModPowerOf2, ModPowerOf2Assign, ModSub, ModSubAssign,
};
use crate::num::number_theory::traits::{CoprimeWith, Gcd, JacobiSymbol};
use crate::rounding_modes::RoundingMode::*;
use crate::slices::slice_test_zero;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::f64::consts::LN_2;

#[cfg(feature = "std")]
#[inline]
fn ln_f64(x: f64) -> f64 {
    x.ln()
}

#[cfg(not(feature = "std"))]
#[inline]
fn ln_f64(x: f64) -> f64 {
    libm::log(x)
}

const WIDE_LIMBS: usize = 5;

// Which residues mod m are squares, for quickly ruling out most nonsquares.
const fn squares_mod<const M: usize>() -> [bool; M] {
    let mut squares = [false; M];
    let mut i = 0;
    while i < M {
        squares[(i * i) % M] = true;
        i += 1;
    }
    squares
}

const SQUARES_MOD_64: [bool; 64] = squares_mod::<64>();
const SQUARES_MOD_63: [bool; 63] = squares_mod::<63>();
const SQUARES_MOD_65: [bool; 65] = squares_mod::<65>();
const SQUARES_MOD_11: [bool; 11] = squares_mod::<11>();

// A signed 320-bit integer in two's complement, with little-endian limbs, for the few intermediate
// values of the n + 1 test and of Lenstra's divisor search that do not fit in a u128. For n <
// 2^128, all of them are less than 2^270 in absolute value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Wide([u64; WIDE_LIMBS]);

impl Wide {
    const ZERO: Self = Self([0; WIDE_LIMBS]);

    const fn from_u128(x: u128) -> Self {
        let mut xs = [0; WIDE_LIMBS];
        xs[0] = x as u64;
        xs[1] = (x >> 64) as u64;
        Self(xs)
    }

    const fn is_negative(&self) -> bool {
        self.0[WIDE_LIMBS - 1] >> 63 != 0
    }

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    fn add(self, other: Self) -> Self {
        let mut out = [0; WIDE_LIMBS];
        let mut carry = false;
        for (o, (&x, &y)) in out.iter_mut().zip(self.0.iter().zip(other.0.iter())) {
            let (mut s, c1) = x.overflowing_add(y);
            let c2 = s.overflowing_add_assign(u64::from(carry));
            *o = s;
            carry = c1 || c2;
        }
        Self(out)
    }

    fn neg(self) -> Self {
        let mut out = self.0;
        for x in &mut out {
            x.not_assign();
        }
        Self(out).add(Self::from_u128(1))
    }

    fn sub(self, other: Self) -> Self {
        self.add(other.neg())
    }

    fn mul(self, other: Self) -> Self {
        let mut out = [0; WIDE_LIMBS];
        for (i, &x) in self.0.iter().enumerate() {
            let mut carry = 0u128;
            for (j, &y) in other.0[..WIDE_LIMBS - i].iter().enumerate() {
                let t = u128::from(x) * u128::from(y) + u128::from(out[i + j]) + carry;
                out[i + j] = t as u64;
                carry = t >> 64;
            }
        }
        Self(out)
    }

    fn abs(self) -> Self {
        if self.is_negative() { self.neg() } else { self }
    }

    fn cmp_signed(&self, other: &Self) -> Ordering {
        match (self.is_negative(), other.is_negative()) {
            (true, false) => Less,
            (false, true) => Greater,
            _ => self.0.iter().rev().cmp(other.0.iter().rev()),
        }
    }

    fn gt(&self, other: &Self) -> bool {
        self.cmp_signed(other) == Greater
    }

    // Divides a nonnegative value by d, returning the quotient and remainder.
    fn div_mod_u64_nonnegative(self, d: u64) -> (Self, u64) {
        let mut out = [0; WIDE_LIMBS];
        let mut r = 0u128;
        for (o, &x) in out.iter_mut().zip(self.0.iter()).rev() {
            let t = (r << 64) | u128::from(x);
            *o = (t / u128::from(d)) as u64;
            r = t % u128::from(d);
        }
        (Self(out), r as u64)
    }

    // The quotient rounded toward zero, like fmpz_tdiv_q.
    fn div_round_toward_zero_u64(self, d: u64) -> Self {
        let q = self.abs().div_mod_u64_nonnegative(d).0;
        if self.is_negative() { q.neg() } else { q }
    }

    // The remainder of a nonnegative value, like fmpz_mod.
    fn mod_u64(self, d: u64) -> u64 {
        debug_assert!(!self.is_negative());
        self.div_mod_u64_nonnegative(d).1
    }

    fn to_u128(self) -> Option<u128> {
        if self.is_negative() || !slice_test_zero(&self.0[2..]) {
            None
        } else {
            Some(u128::join_halves(self.0[1], self.0[0]))
        }
    }

    fn shr(self, mut bits: usize) -> Self {
        let limbs = bits >> 6;
        bits.mod_power_of_2_assign(6);
        let mut out = [0; WIDE_LIMBS];
        for i in 0..WIDE_LIMBS - limbs {
            let lo = self.0[i + limbs] >> bits;
            let hi = if bits != 0 && i + limbs + 1 < WIDE_LIMBS {
                self.0[i + limbs + 1] << (64 - bits)
            } else {
                0
            };
            out[i] = lo | hi;
        }
        Self(out)
    }

    fn power_of_2(bits: usize) -> Self {
        let mut out = [0; WIDE_LIMBS];
        out[bits >> 6] = u64::power_of_2(u64::exact_from(bits.mod_power_of_2(6)));
        Self(out)
    }

    fn significant_bits(&self) -> usize {
        for i in (0..WIDE_LIMBS).rev() {
            if self.0[i] != 0 {
                return (i << 6) + 64 - self.0[i].leading_zeros() as usize;
            }
        }
        0
    }

    // The floor of the square root of a nonnegative value, and the remainder, digit by digit.
    fn sqrt_rem(self) -> (Self, Self) {
        let mut rem = self;
        let mut root = Self::ZERO;
        let bits = self.significant_bits();
        if bits == 0 {
            return (root, rem);
        }
        let mut bit = Self::power_of_2((bits - 1) & !1);
        while !bit.is_zero() {
            let t = root.add(bit);
            root = if rem.cmp_signed(&t).is_ge() {
                rem = rem.sub(t);
                root.shr(1).add(bit)
            } else {
                root.shr(1)
            };
            bit = bit.shr(2);
        }
        (root, rem)
    }

    fn is_square(self) -> bool {
        if self.is_negative() || !SQUARES_MOD_64[usize::exact_from(self.0[0].mod_power_of_2(6))] {
            return false;
        }
        // 45045 = 63 * 65 * 11
        let r = usize::exact_from(self.div_mod_u64_nonnegative(45045).1);
        SQUARES_MOD_63[r % 63]
            && SQUARES_MOD_65[r % 65]
            && SQUARES_MOD_11[r % 11]
            && self.sqrt_rem().1.is_zero()
    }
}

// This is fmpz_dlog from fmpz/dlog.c, FLINT 3.7.0-dev, for n >= 2^64. mpz_get_d_2exp truncates n to
// 53 significant bits, giving n = s * 2^e with 1/2 <= s < 1.
fn dlog(n: u128) -> f64 {
    let e = n.significant_bits();
    let m = u64::exact_from(n >> (e - 53));
    let s = f64::rounding_from(m, Exact).0 / 9007199254740992.0;
    ln_f64(s) + f64::from(u32::exact_from(e)) * LN_2
}

// Finds the primes p up to about `limit` with p | n - 1 and the primes p with p | n + 1, in
// increasing order.
//
// This is the trial division from _fmpz_is_prime in fmpz/is_prime.c, FLINT 3.7.0-dev, for n >=
// 2^64. FLINT reduces n modulo the product of each batch of primes that fits in a word, and then
// reduces that remainder modulo each prime of the batch; it tries every prime of a batch whose
// first prime is less than the limit. Here each prime q is tried separately, with n mod q computed
// from the two halves of n in u64 arithmetic (q < 2^32, so the products fit), which is faster than
// a u128 remainder per batch; the primes tried are the same as FLINT's. FLINT's product also wraps
// when a batch reaches past a power of 2 that the limit does not: with limit = 65530, for example,
// a batch holds 4 primes, and 65521 * 65537 * 65539 * 65543 > 2^64.
fn n_pm_1_trial_factors(n: u128, limit: u64) -> (Vec<u64>, Vec<u64>) {
    let mut pm1 = Vec::new();
    let mut pp1 = Vec::new();
    // the number of primes multiplied that will fit in a word
    let num = usize::exact_from(u64::WIDTH / limit.significant_bits());
    let (hi, lo) = n.split_in_half();
    for (i, q) in u64::primes().enumerate() {
        // FLINT stops at the first batch that starts at or above the limit.
        if q >= limit && i.divisible_by(num) {
            break;
        }
        // 2^64 mod q
        let t = (u64::MAX % q + 1) % q;
        let r = ((hi % q) * t + lo % q) % q;
        if r == 1 {
            // n - 1 = 0 mod q
            pm1.push(q);
        }
        if r == q - 1 {
            // n + 1 = 0 mod q
            pp1.push(q);
        }
    }
    (pm1, pp1)
}

// Removes the primes `ps` from m, and also the cofactor if it is proven prime. Returns the factored
// part F, the cofactor R, and the primes dividing F.
fn factor_n_pm_1(
    mut r: u128,
    ps: &[u64],
    prove: fn(u128) -> Option<bool>,
) -> (u128, u128, Vec<u128>) {
    let mut f = 1u128;
    let mut fac = Vec::with_capacity(ps.len() + 1);
    for &p in ps {
        let p = u128::from(p);
        let d = r.remove_power_assign(p);
        f *= p.pow(u32::exact_from(d));
        fac.push(p);
    }
    if r.is_probable_prime() && prove(r) == Some(true) {
        f *= r;
        fac.push(r);
        r = 1;
    }
    (f, r, fac)
}

// The number of bases that the Pocklington and Morrison tests try before giving up.
const MAX_BASES: u128 = 100;

// The Pocklington test for an odd n >= 2^64, given the primes `pm1` dividing n - 1. Returns
// `Some(false)` if n is composite, and `Some(true)` if a base a has a^(n - 1) = 1 mod n and
// gcd(a^((n
// - 1) / q) - 1, n) = 1 for every prime q dividing the factored part F of n - 1. In that case every
// prime factor of n is 1 mod F, so n is prime if F^2 > n. Returns `None` if none of the first
// MAX_BASES bases settles it. Also returns F and the cofactor R = (n - 1) / F. `prove` proves the
// primality of the cofactor of n - 1.
//
// This is fmpz_is_prime_pocklington from fmpz/is_prime_pocklington.c, FLINT 3.7.0-dev, with
// Montgomery arithmetic in place of fmpz arithmetic, except that FLINT tries bases until one
// settles it. For a prime n, the first 100 bases fail only if, for some q, they are all q-th power
// residues; but for a composite n whose Carmichael function divides (n - 1) / q, such as a
// Carmichael number (6k + 1)(12k + 1)(18k + 1) with q | 36k^2 + 11k + 1, every base coprime to n
// fails, and FLINT keeps going until a reaches the least prime factor of n, which can take minutes.
crate_test_fn! {u128_is_prime_pocklington(
    n: u128,
    pm1: &[u64],
    prove: fn(u128) -> Option<bool>,
) -> (Option<bool>, u128, u128) {
    // start with n - 1
    let (f, r, fac) = factor_n_pm_1(n - 1, pm1, prove);
    let ctx = redc_context_128(n);
    let mut c;
    let mut a = 2u128;
    loop {
        if a > const { MAX_BASES + 1 } {
            return (None, f, r);
        }
        // compute a^((n - 1) / F) mod n
        let pow = redc_pow_128(to_redc_128(a, &ctx), r, &ctx);
        // check a^(n - 1) = 1 mod n
        if redc_pow_128(pow, f, &ctx) != ctx.one {
            return (Some(false), f, r);
        }
        c = ctx.one;
        // find values a^((n - 1) / q) - 1 for each prime q dividing F
        let mut valid = true;
        for &q in &fac {
            let pow2 = redc_pow_128(pow, f / q, &ctx).mod_sub(ctx.one, n);
            if pow2 == 0 {
                valid = false;
                break;
            }
            c = redc_mul_128(c, pow2, &ctx);
        }
        if valid {
            break;
        }
        a += 1;
    }
    // check for factors of n
    (Some(n.coprime_with(c)), f, r)
}}

// Arithmetic on the Lucas sequences U(A, B) and V(A, B) modulo an odd n, in Montgomery form.
struct LucasContext128 {
    ctx: RedcContext128,
    a: u128,
    b: u128,
    two: u128,
}

impl LucasContext128 {
    #[inline]
    fn mul(&self, x: u128, y: u128) -> u128 {
        redc_mul_128(x, y, &self.ctx)
    }

    #[inline]
    fn sub(&self, x: u128, y: u128) -> u128 {
        x.mod_sub(y, self.ctx.n)
    }

    #[inline]
    fn add(&self, x: u128, y: u128) -> u128 {
        x.mod_add(y, self.ctx.n)
    }

    // Computes V_m and V_{m + 1}, where Q_k = B^k.
    //
    // This is fmpz_lucas_chain_full from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_full(&self, m: u128) -> (u128, u128) {
        let mut q = self.ctx.one;
        let mut vm = self.two;
        let mut vm1 = self.a;
        for i in (0..m.significant_bits()).rev() {
            (vm, vm1, q) = if m.get_bit(i) {
                (
                    self.sub(self.mul(vm1, vm), self.mul(q, self.a)),
                    self.sub(self.mul(vm1, vm1), self.mul(self.add(q, q), self.b)),
                    self.mul(self.mul(q, q), self.b),
                )
            } else {
                (
                    self.sub(self.mul(vm, vm), self.add(q, q)),
                    self.sub(self.mul(vm, vm1), self.mul(q, self.a)),
                    self.mul(q, q),
                )
            };
        }
        (vm, vm1)
    }

    // Computes U_{2m} and U_{2m + 1} from U_m and U_{m + 1}.
    //
    // This is fmpz_lucas_chain_double from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_double(&self, um: u128, um1: u128) -> (u128, u128) {
        // U_m(2U_{m + 1} - AU_m)
        let u2m = self.mul(self.sub(self.add(um1, um1), self.mul(um, self.a)), um);
        // U_{m + 1}^2 - BU_m^2
        let u2m1 = self.sub(self.mul(um1, um1), self.mul(self.mul(um, um), self.b));
        (u2m, u2m1)
    }

    // Computes U_{m + n} and U_{m + n + 1} from U_m, U_{m + 1}, U_n, and U_{n + 1}.
    //
    // This is fmpz_lucas_chain_add from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_add(&self, um: u128, um1: u128, un: u128, un1: u128) -> (u128, u128) {
        // U_n U_{m + 1} - BU_m(AU_n - U_{n + 1}) / B
        let umn = self.add(
            self.mul(self.sub(un1, self.mul(un, self.a)), um),
            self.mul(un, um1),
        );
        // U_{n + 1}U_{m + 1} - BU_mU_n
        let umn1 = self.sub(self.mul(un1, um1), self.mul(self.mul(um, un), self.b));
        (umn, umn1)
    }

    // Computes U_{km} and U_{km + 1} from U_m and U_{m + 1}, for k > 0.
    //
    // This is fmpz_lucas_chain_mul from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_mul(&self, um: u128, um1: u128, k: u128) -> (u128, u128) {
        let b = k.significant_bits();
        let mut ukm = um;
        let mut ukm1 = um1;
        let mut i = 0;
        while !k.get_bit(i) {
            (ukm, ukm1) = self.chain_double(ukm, ukm1);
            i += 1;
        }
        i += 1;
        let mut t = ukm;
        let mut t1 = ukm1;
        while i < b {
            (t, t1) = self.chain_double(t, t1);
            if k.get_bit(i) {
                (ukm, ukm1) = self.chain_add(ukm, ukm1, t, t1);
            }
            i += 1;
        }
        (ukm, ukm1)
    }

    // Computes U_m and U_{m + 1} from V_m and V_{m + 1}, given D^-1 mod n, where D = A^2 - 4B.
    //
    // This is fmpz_lucas_chain_VtoU from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn v_to_u(&self, vm: u128, vm1: u128, d_inv: u128) -> (u128, u128) {
        // (2V_{m + 1} - AV_m) / D
        let um = self.mul(self.sub(self.add(vm1, vm1), self.mul(vm, self.a)), d_inv);
        // (V_m + AU_m) / 2
        let t = self.add(vm, self.mul(um, self.a));
        let um1 = if t.even() {
            t >> 1
        } else {
            // (t + n) / 2, without overflow
            (t >> 1) + (self.ctx.n >> 1) + 1
        };
        (um, um1)
    }
}

// The Morrison test for an odd n >= 2^64, given the primes `pp1` dividing n + 1. Returns
// `Some(false)` if n is composite, and `Some(true)` if a Lucas sequence U has U_{n + 1} = 0 mod n
// and gcd(U_{(n + 1) / q}, n) = 1 for every prime q dividing the factored part F of n + 1. In that
// case every prime factor of n is ±1 mod F, so n is prime if F - 1 > sqrt(n). Returns `None` if
// none of the first MAX_BASES values of B settles it. Also returns F and the cofactor R = (n + 1) /
// F. `prove` proves the primality of the cofactor of n + 1.
//
// This is fmpz_is_prime_morrison from fmpz/is_prime_morrison.c, FLINT 3.7.0-dev, with Montgomery
// arithmetic in place of fmpz arithmetic, except that FLINT tries values of B until one settles it;
// see u128_is_prime_pocklington. FLINT would loop forever if n were a square; but n has passed the
// base-2 strong probable prime test, and the square of a prime p is a base-2 strong probable prime
// only if p is a Wieferich prime, and the only Wieferich primes below 2^64 are 1093 and 3511.
crate_test_fn! {u128_is_prime_morrison(
    n: u128,
    pp1: &[u64],
    prove: fn(u128) -> Option<bool>,
) -> (Option<bool>, u128, u128) {
    // start with n + 1
    let (f, r, fac) = factor_n_pm_1(n + 1, pp1, prove);
    let mut c;
    // Want D = A^2 - 4B where A = a, B = b, such that (D / n) = -1.
    let mut b = 1u128;
    loop {
        if b > MAX_BASES {
            return (None, f, r);
        }
        // need gcd(n, b) = 1. FLINT also skips any b with gcd(b, n) = n, but here b <= MAX_BASES <
        // n.
        let g = b.gcd(n);
        if g != 1 {
            // found a factor of n
            fail_on_untested_path("u128_is_prime_morrison, gcd(b, n) is a proper factor of n");
            return (Some(false), f, r);
        }
        let mut a = 2u128;
        // D mod n
        let d = loop {
            a += 1;
            let d = i128::exact_from(a * a) - i128::exact_from(b << 2);
            let r = d.unsigned_abs() % n;
            let d = if d < 0 && r != 0 { n - r } else { r };
            if d.jacobi_symbol(n) == -1 {
                break d;
            }
        };
        let d_inv = d.mod_inverse(n).unwrap();
        let ctx = redc_context_128(n);
        let two = to_redc_128(2, &ctx);
        let lucas = LucasContext128 {
            a: to_redc_128(a, &ctx),
            b: to_redc_128(b, &ctx),
            two,
            ctx,
        };
        let d_inv = to_redc_128(d_inv, &lucas.ctx);
        // compute U((n + 1) / F) mod n
        let (vm, vm1) = lucas.chain_full(r);
        let (um, um1) = lucas.v_to_u(vm, vm1, d_inv);
        // check U(n + 1) = 0 mod n
        if lucas.chain_mul(um, um1, f).0 != 0 {
            return (Some(false), f, r);
        }
        c = lucas.ctx.one;
        // find values U((n + 1) / q) for each prime q dividing F
        let mut valid = true;
        for &q in &fac {
            let ukm = lucas.chain_mul(um, um1, f / q).0;
            if ukm == 0 {
                valid = false;
                break;
            }
            c = lucas.mul(c, ukm);
        }
        if valid {
            break;
        }
        b += 1;
    }
    // check for factors of n
    let g = n.gcd(c);
    if g != 1 {
        fail_on_untested_path("u128_is_prime_morrison, gcd(c, n) > 1");
    }
    (Some(g == 1), f, r)
}}

// Searches for a divisor of n congruent to r mod s, other than 1 and n, where gcd(r, s) = 1 and
// n^(1/3) < s <= n^(1/2), so that n has at most 11 such divisors. Returns the first one it finds.
//
// This is fmpz_divisor_in_residue_class_lenstra from fmpz/divisor_in_residue_class_lenstra.c, FLINT
// 3.7.0-dev, for n >= 2^64.
crate_test_fn! {u128_divisor_in_residue_class_lenstra(n: u128, r: u128, s: u128) -> Option<u128> {
    let s64 = u64::exact_from(s);
    let ws = Wide::from_u128(s);
    let wr = Wide::from_u128(r);
    let wn = Wide::from_u128(n);
    // ns2 = n / s^2
    let ns2 = Wide::from_u128(n / (s * s));
    // r1 = r^-1 mod s
    let r1 = r.mod_inverse(s).unwrap();
    // r2 = r1 * n mod s
    let r2 = r1.mod_mul(n % s, s);
    let wr2 = Wide::from_u128(r2);
    let mut a0 = s;
    // a1 = r1 * r2 mod s, with 0 < a1 <= s
    let mut a1 = r1.mod_mul(r2, s);
    if a1 == 0 {
        a1 = s;
    }
    let mut b0 = Wide::ZERO;
    let mut b1 = Wide::from_u128(1);
    let mut c0 = 0u128;
    // c1 = (n - r * r2) / s * r1 mod s
    let mut c1 = ((n - r * r2) / s % s).mod_mul(r1, s);
    let is_divisor = |fac: Wide| {
        fac.to_u128().is_some_and(|fac| fac != 0 && fac != 1 && fac != n && n.divisible_by(fac))
    };
    // deal with a0, b0, c0
    if r != 1 && n != r && n.divisible_by(r) {
        return Some(r);
    }
    for i in 1usize.. {
        let wa1 = Wide::from_u128(a1);
        let (mut s1, s2) = if i.even() {
            (Wide::from_u128(c1 % s), ws.neg())
        } else {
            let s2 = wa1.mul(b1);
            // s1 = a1 * b1 + n / s^2
            let mut s1 = s2.add(ns2);
            // s1 is the largest integer less than a1 * b1 + n / s^2 congruent to c1 mod s. When i
            // is odd, b1 > 0, so s1 >= 0.
            let q = u128::from(s1.mod_u64(s64));
            if q < c1 {
                s1 = s1.sub(ws);
            }
            s1 = s1.sub(Wide::from_u128(q)).add(Wide::from_u128(c1));
            // s2 = 2 * a1 * b1 - 1
            (s1, s2.add(s2).sub(Wide::from_u128(1)))
        };
        // for each value s1 in range
        while s1.gt(&s2) {
            // d = (s1 * s + a1 * r + b1 * r2)^2 - 4 * a1 * b1 * n
            let d1 = ws.mul(s1).add(wa1.mul(wr)).add(b1.mul(wr2));
            let d = d1.mul(d1).sub(wa1.mul(b1).mul(wn).mul(Wide::from_u128(4)));
            if d.is_square() {
                // A divisor exists; the roots are (d1 ± sqrt(d)) / 2.
                let d = d.sqrt_rem().0.add(d1).div_round_toward_zero_u64(2);
                // FLINT also handles b1 == 0, but b1 is never 0: the b's alternate in sign, and
                // |b_{k+1}| = |b_{k-1}| + q|b_k| with |b_1| = 1.
                if a1 == 0 {
                    // y * b1 = s1; check whether y * s + r2 is a factor
                    let b1_abs = u64::exact_from(b1.abs().to_u128().unwrap());
                    let y = s1.div_round_toward_zero_u64(b1_abs);
                    let y = if b1.is_negative() { y.neg() } else { y };
                    let fac = y.mul(ws).add(wr2).abs();
                    if is_divisor(fac) {
                        return fac.to_u128();
                    }
                } else {
                    // either d / a1 or d / b1 is a divisor of n
                    let fac = d.div_round_toward_zero_u64(u64::exact_from(a1)).abs();
                    if is_divisor(fac) {
                        return fac.to_u128();
                    }
                    let b1_abs = u64::exact_from(b1.abs().to_u128().unwrap());
                    let fac = d.div_round_toward_zero_u64(b1_abs).abs();
                    if is_divisor(fac) {
                        return fac.to_u128();
                    }
                }
            }
            s1 = s1.sub(ws);
        }
        // the Euclidean chain has terminated
        if a1 == 0 {
            break;
        }
        // Euclidean chain: a1, a0 = a0 - q * a1, a1, where 0 <= a1 < a0 for i even and 0 < a1 <= a0
        // for i odd
        let (mut q, r0) = a0.div_mod(a1);
        a0 = r0;
        if i.even() && a0 == 0 {
            q -= 1;
            a0 += a1;
        }
        (a0, a1) = (a1, a0);
        // b1, b0 = b0 - q * b1, b1
        b0 = b0.sub(Wide::from_u128(q).mul(b1));
        (b0, b1) = (b1, b0);
        // c1, c0 = c0 - q * c1, c1 mod s
        c0.mod_sub_assign((q % s).mod_mul(c1, s), s);
        (c0, c1) = (c1, c0);
    }
    None
}}

// The n - 1 and n + 1 stage of the primality proof for an odd n >= 2^64 with no small factors that
// is a base-2 strong probable prime. Returns `Some(true)` if n is proven prime, `Some(false)` if n
// is proven composite, and `None` if neither test can settle it. `prove` proves the primality of
// the cofactors of n - 1 and n + 1.
//
// This is the proof stage of _fmpz_is_prime from fmpz/is_prime.c, FLINT 3.7.0-dev, for n >= 2^64,
// except that it returns `None` where FLINT calls _fmpz_is_prime_proof. FLINT repeats the stage
// with ten times the trial division limit while the result is unknown, but every branch settles the
// result, so the stage runs once.
crate_test_fn! {u128_is_prime_n_pm_1(n: u128, prove: fn(u128) -> Option<bool>) -> Option<bool> {
    let logd = dlog(n);
    let mut limit = u64::rounding_from(logd * logd * logd / 100.0, Floor).0 + 20;
    // Below about 260 bits, a 10x larger trial factor base measurably increases the fraction of
    // inputs proven by the n ± 1 tests alone.
    if logd < 180.0 {
        limit *= 10;
    }
    let (pm1, pp1) = n_pm_1_trial_factors(n, limit);
    // Feasibility check: with F1 | n - 1 and F2 | n + 1 the fully factored parts, every proof below
    // requires lcm(F1, F2)^3 > n, except when a cofactor of n - 1 or n + 1 is itself prime. `None`
    // stands for a product of at least 2^128.
    let mut fc = Some(1u128);
    let mut feasible = false;
    for (pf, mut rc) in [(&pm1, n - 1), (&pp1, n + 1)] {
        for &p in pf {
            let p = u128::from(p);
            let e = rc.remove_power_assign(p);
            fc = fc.and_then(|fc| fc.checked_mul(p.checked_pow(u32::exact_from(e))?));
        }
        // A prime cofactor makes n ∓ 1 fully factorable.
        if rc == 1 || rc.is_strong_probable_prime(2) {
            feasible = true;
            break;
        }
    }
    if !feasible {
        // Both 2-parts were included, so lcm(F1, F2) >= Fc / 2.
        feasible = fc.is_none_or(|fc| {
            let h = fc >> 1;
            h.checked_mul(h).and_then(|t| t.checked_mul(h)).is_none_or(|t| t > n)
        });
    }
    if !feasible {
        return None;
    }
    // p - 1 test
    let (res, f1, _) = u128_is_prime_pocklington(n, &pm1, prove);
    if res != Some(true) {
        return res;
    }
    let Some(f1_sqr) = f1.checked_mul(f1).filter(|&s| s < n) else {
        return Some(true);
    };
    if f1_sqr.checked_mul(f1).is_none_or(|c| c >= n) {
        // Brillhart, Lehmer, Selfridge test. n = 1 mod F1, so let n = c2 * F1^2 + c1 * F1 + 1, and
        // check whether c1^2 - 4 * c2 is a square.
        let (c2, c1) = ((n - 1) / f1).div_mod(f1);
        let c1_sqr = c1 * c1;
        let c2_4 = c2 << 2;
        return Some(c1_sqr < c2_4 || !(c1_sqr - c2_4).is_square());
    }
    // p + 1 test
    let (res, f2, r) = u128_is_prime_morrison(n, &pp1, prove);
    if res != Some(true) {
        return res;
    }
    // need F2 - 1 > sqrt(n)
    let f2_m1 = f2 - 1;
    let Some(f2_m1_sqr) = f2_m1.checked_mul(f2_m1).filter(|&s| s <= n) else {
        return Some(true);
    };
    if f2_m1_sqr.checked_mul(f2_m1).is_none_or(|c| c > n) {
        // Improved n + 1 test. Let R = r1 * F2 + r0.
        let (r1, r0) = r.div_mod(f2);
        let wr0 = Wide::from_u128(r0);
        let four = Wide::from_u128(4);
        // Check whether x^2 + r0 * x - r1 has a positive integral root.
        let (b, rem) = wr0.mul(wr0).add(four.mul(Wide::from_u128(r1))).sqrt_rem();
        let mut composite = rem.is_zero() && b.gt(&wr0);
        if composite {
            fail_on_untested_path("u128_is_prime_n_pm_1, improved n + 1 test, first root");
        }
        // Check whether x^2 + (r0 - F2) * x - r1 - 1 has a positive integral root.
        let wr0 = wr0.sub(Wide::from_u128(f2));
        let (b, rem) = wr0.mul(wr0).add(four.mul(Wide::from_u128(r1 + 1))).sqrt_rem();
        if rem.is_zero() && b.gt(&wr0) {
            fail_on_untested_path("u128_is_prime_n_pm_1, improved n + 1 test, second root");
            composite = true;
        }
        return Some(!composite);
    }
    // Brillhart, Lehmer, Selfridge combined p - 1, p + 1 test. F = lcm(F1, F2), where F1 | n - 1
    // and F2 | n + 1.
    let mut f = f1 * f2;
    if f1.even() && f2.even() {
        f >>= 1;
    }
    let f_sqr = f.checked_mul(f);
    if f_sqr.is_none_or(|s| s > n) {
        // lcm(F1, F2) > sqrt(n); check that n mod F is not a factor of n
        let n_mod_f = n % f;
        if n_mod_f != n && n_mod_f != 1 && n.divisible_by(n_mod_f) {
            fail_on_untested_path("u128_is_prime_n_pm_1, n mod F divides n");
            return Some(false);
        }
        return Some(true);
    }
    if f_sqr.unwrap().checked_mul(f).is_none_or(|c| c > n) {
        // Lenstra's divisors in residue classes
        let composite = u128_divisor_in_residue_class_lenstra(n, 1, f).is_some()
            || u128_divisor_in_residue_class_lenstra(n, n % f, f).is_some();
        if composite {
            fail_on_untested_path("u128_is_prime_n_pm_1, Lenstra's search finds a divisor");
        }
        return Some(!composite);
    }
    None
}}
