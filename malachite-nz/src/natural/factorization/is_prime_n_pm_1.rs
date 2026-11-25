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

use crate::integer::Integer;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::factorization::is_probable_prime::limbs_factor_trial;
use crate::natural::modular::mod_op::limbs_mod_limb;
use crate::natural::modular::mod_pow::LimbsRedc;
use crate::natural::{LIMB_HIGH_BIT, Natural};
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::f64::consts::LN_2;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    AddMul, DivMod, DivModPrecomputed, DivRound, MulAddMul, Parity, Pow, SqrtRem, Square, SubMul,
    SubMulAssign, SubMulShl, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::comparison::traits::OrdProduct;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::factorization::is_prime_n_pm_1::u128_prove_prime;
use malachite_base::num::factorization::traits::{
    IsProbablePrime, IsSquare, IsStrongProbablePrime, Primes, RemovePowerAssign,
};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::num::modular::traits::{
    DivisibleBy, Mod, ModInverse, ModMul, ModMulAssign, ModPow, ModSub, ModSubAssign,
};
use malachite_base::num::number_theory::traits::{CoprimeWith, Gcd, JacobiSymbol};
use malachite_base::rounding_modes::RoundingMode::*;

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

// This is fmpz_dlog from fmpz/dlog.c, FLINT 3.7.0-dev, for n >= 2^64. mpz_get_d_2exp truncates n to
// 53 significant bits, giving n = s * 2^e with 1/2 <= s < 1.
fn dlog(n: &Natural) -> f64 {
    let e = n.significant_bits();
    let m = u64::exact_from(&(n >> (e - 53)));
    let s = f64::rounding_from(m, Exact).0 / 9007199254740992.0;
    ln_f64(s) + f64::rounding_from(e, Exact).0 * LN_2
}

// Finds the primes p up to about `limit` with p | n - 1 and the primes p with p | n + 1, in
// increasing order.
//
// This is the trial division from _fmpz_is_prime in fmpz/is_prime.c, FLINT 3.7.0-dev, except that
// the product of each batch of primes is a u128 rather than a u64, and that a product that does not
// fit in a limb is reduced with Natural arithmetic; see the u128 version in malachite-base.
// Requires n to have at least two limbs.
fn n_pm_1_trial_factors(n: &Natural, limit: u64) -> (Vec<u64>, Vec<u64>) {
    let mut pm1 = Vec::new();
    let mut pp1 = Vec::new();
    // the number of primes multiplied that will fit in a word
    let num = usize::exact_from(64 / limit.significant_bits());
    let mut primes = u64::primes();
    let mut batch = Vec::with_capacity(num);
    loop {
        batch.clear();
        batch.extend(primes.by_ref().take(num));
        if batch[0] >= limit {
            break;
        }
        let mut p = 1u128;
        for &q in &batch {
            p *= u128::from(q);
        }
        // multi-modular reduction
        let p = if let Ok(p) = Limb::try_from(p) {
            u64::exact_from(limbs_mod_limb::<DoubleLimb, Limb>(n.as_limbs_asc(), p))
        } else {
            u64::exact_from(&(n % Natural::from(p)))
        };
        // check for factors
        for &q in &batch {
            let r = p % q;
            if r == 1 {
                // n - 1 = 0 mod q
                pm1.push(q);
            }
            if r == q - 1 {
                // n + 1 = 0 mod q
                pp1.push(q);
            }
        }
    }
    (pm1, pp1)
}

// Removes the primes `ps` from r, and also the cofactor if it is proven prime. Returns the factored
// part F, the cofactor R, and the primes dividing F.
fn factor_n_pm_1(
    mut r: Natural,
    ps: &[u64],
    prove: fn(&Natural) -> Option<bool>,
) -> (Natural, Natural, Vec<Natural>) {
    let mut f = Natural::ONE;
    let mut fac = Vec::with_capacity(ps.len() + 1);
    for &p in ps {
        let p = Natural::from(p);
        let d = r.remove_power_assign(&p);
        f *= (&p).pow(d);
        fac.push(p);
    }
    if r.is_probable_prime() && prove(&r) == Some(true) {
        f *= &r;
        fac.push(r);
        r = Natural::ONE;
    }
    (f, r, fac)
}

// The number of bases that the Pocklington and Morrison tests try before giving up.
const MAX_BASES: u64 = 100;

// The Pocklington test for an odd n, given the primes `pm1` dividing n - 1. Returns `Some(false)`
// if n is composite, and `Some(true)` if a base a has a^(n - 1) = 1 mod n and gcd(a^((n - 1) / q) -
// 1, n) = 1 for every prime q dividing the factored part F of n - 1. In that case every prime
// factor of n is 1 mod F, so n is prime if F^2 > n. Returns `None` if none of the first MAX_BASES
// bases settles it. Also returns F and the cofactor R = (n - 1) / F. `prove` proves the primality
// of the cofactor of n - 1.
//
// This is fmpz_is_prime_pocklington from fmpz/is_prime_pocklington.c, FLINT 3.7.0-dev, except that
// FLINT tries bases until one settles it; see the u128 version in malachite-base.
crate_test_fn! {natural_is_prime_pocklington(
    n: &Natural,
    pm1: &[u64],
    prove: fn(&Natural) -> Option<bool>,
) -> (Option<bool>, Natural, Natural) {
    // start with n - 1
    let (f, r, fac) = factor_n_pm_1(n - Natural::ONE, pm1, prove);
    let mut c;
    let mut a = 2u64;
    loop {
        if a > const { MAX_BASES + 1 } {
            return (None, f, r);
        }
        // compute a^((n - 1) / F) mod n
        let pow = Natural::from(a).mod_pow(&r, n);
        // check a^(n - 1) = 1 mod n
        if (&pow).mod_pow(&f, n) != 1u32 {
            return (Some(false), f, r);
        }
        c = Natural::ONE;
        // find values a^((n - 1) / q) - 1 for each prime q dividing F
        let mut valid = true;
        for q in &fac {
            let pow2 = (&pow).mod_pow(&(&f / q), n).mod_sub(Natural::ONE, n);
            if pow2 == 0u32 {
                valid = false;
                break;
            }
            c.mod_mul_assign(pow2, n);
        }
        if valid {
            break;
        }
        a += 1;
    }
    // check for factors of n
    (Some(n.coprime_with(&c)), f, r)
}}

// x - y mod m, for x, y < m, all with the same number of limbs.
fn limbs_sub_mod(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> Vec<Limb> {
    let mut out = xs.to_vec();
    if limbs_sub_same_length_in_place_left(&mut out, ys) {
        limbs_slice_add_same_length_in_place_left(&mut out, ms);
    }
    out
}

// x + y mod m, for x, y < m, all with the same number of limbs.
fn limbs_add_mod(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> Vec<Limb> {
    let mut out = xs.to_vec();
    if limbs_slice_add_same_length_in_place_left(&mut out, ys)
        || limbs_cmp_same_length(&out, ms) != Less
    {
        limbs_sub_same_length_in_place_left(&mut out, ms);
    }
    out
}

// Arithmetic on the Lucas sequences U(A, B) and V(A, B) modulo an odd n, in Montgomery form. Every
// value is an n-limb slice reduced mod n: the REDC of a product of two reduced values is less than
// 2n, so one subtraction reduces it.
struct LucasContext<'a> {
    ms: &'a [Limb],
    redc: LimbsRedc<'a>,
    a: Vec<Limb>,
    b: Vec<Limb>,
    two: Vec<Limb>,
}

impl<'a> LucasContext<'a> {
    fn new(ms: &'a [Limb], a: u64, b: u64) -> Self {
        let redc = LimbsRedc::new(ms);
        let a = redc.to_redc(Natural::from(a).as_limbs_asc());
        let b = redc.to_redc(Natural::from(b).as_limbs_asc());
        let two = redc.to_redc(&[2]);
        Self {
            ms,
            redc,
            a,
            b,
            two,
        }
    }

    fn mul(&mut self, x: &[Limb], y: &[Limb]) -> Vec<Limb> {
        let mut out = vec![0; x.len()];
        self.redc.mul(&mut out, x, y);
        if limbs_cmp_same_length(&out, self.ms) != Less {
            limbs_sub_same_length_in_place_left(&mut out, self.ms);
        }
        out
    }

    fn mul_a(&mut self, x: &[Limb]) -> Vec<Limb> {
        let a = core::mem::take(&mut self.a);
        let out = self.mul(x, &a);
        self.a = a;
        out
    }

    fn mul_b(&mut self, x: &[Limb]) -> Vec<Limb> {
        let b = core::mem::take(&mut self.b);
        let out = self.mul(x, &b);
        self.b = b;
        out
    }

    // Computes V_m and V_{m + 1}, where Q_k = B^k.
    //
    // This is fmpz_lucas_chain_full from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_full(&mut self, m: &Natural) -> (Vec<Limb>, Vec<Limb>) {
        let ms = self.ms;
        let mut q = self.redc.to_redc(&[1]);
        let mut vm = self.two.clone();
        let mut vm1 = self.a.clone();
        for i in (0..m.significant_bits()).rev() {
            let qa = self.mul_a(&q);
            let q2 = limbs_add_mod(&q, &q, ms);
            let qq = self.mul(&q, &q);
            (vm, vm1, q) = if m.get_bit(i) {
                let t = self.mul(&vm1, &vm);
                let t1 = self.mul(&vm1, &vm1);
                (
                    limbs_sub_mod(&t, &qa, ms),
                    limbs_sub_mod(&t1, &self.mul_b(&q2), ms),
                    self.mul_b(&qq),
                )
            } else {
                let t = self.mul(&vm, &vm);
                let t1 = self.mul(&vm, &vm1);
                (limbs_sub_mod(&t, &q2, ms), limbs_sub_mod(&t1, &qa, ms), qq)
            };
        }
        (vm, vm1)
    }

    // Computes U_{2m} and U_{2m + 1} from U_m and U_{m + 1}.
    //
    // This is fmpz_lucas_chain_double from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_double(&mut self, um: &[Limb], um1: &[Limb]) -> (Vec<Limb>, Vec<Limb>) {
        let ms = self.ms;
        // U_m(2U_{m + 1} - AU_m)
        let t = limbs_sub_mod(&limbs_add_mod(um1, um1, ms), &self.mul_a(um), ms);
        let u2m = self.mul(&t, um);
        // U_{m + 1}^2 - BU_m^2
        let umum = self.mul(um, um);
        let t = self.mul(um1, um1);
        let u2m1 = limbs_sub_mod(&t, &self.mul_b(&umum), ms);
        (u2m, u2m1)
    }

    // Computes U_{m + n} and U_{m + n + 1} from U_m, U_{m + 1}, U_n, and U_{n + 1}.
    //
    // This is fmpz_lucas_chain_add from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_add(
        &mut self,
        um: &[Limb],
        um1: &[Limb],
        un: &[Limb],
        un1: &[Limb],
    ) -> (Vec<Limb>, Vec<Limb>) {
        let ms = self.ms;
        // U_n U_{m + 1} - BU_m(AU_n - U_{n + 1}) / B
        let t = limbs_sub_mod(un1, &self.mul_a(un), ms);
        let t = self.mul(&t, um);
        let umn = limbs_add_mod(&t, &self.mul(un, um1), ms);
        // U_{n + 1}U_{m + 1} - BU_mU_n
        let umun = self.mul(um, un);
        let t = self.mul(un1, um1);
        let umn1 = limbs_sub_mod(&t, &self.mul_b(&umun), ms);
        (umn, umn1)
    }

    // Computes U_{km} and U_{km + 1} from U_m and U_{m + 1}, for k > 0.
    //
    // This is fmpz_lucas_chain_mul from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn chain_mul(&mut self, um: &[Limb], um1: &[Limb], k: &Natural) -> (Vec<Limb>, Vec<Limb>) {
        let b = k.significant_bits();
        let mut ukm = um.to_vec();
        let mut ukm1 = um1.to_vec();
        let mut i = 0;
        while !k.get_bit(i) {
            (ukm, ukm1) = self.chain_double(&ukm, &ukm1);
            i += 1;
        }
        i += 1;
        let mut t = ukm.clone();
        let mut t1 = ukm1.clone();
        while i < b {
            (t, t1) = self.chain_double(&t, &t1);
            if k.get_bit(i) {
                (ukm, ukm1) = self.chain_add(&ukm, &ukm1, &t, &t1);
            }
            i += 1;
        }
        (ukm, ukm1)
    }

    // Computes U_m and U_{m + 1} from V_m and V_{m + 1}, given D^-1 mod n, where D = A^2 - 4B.
    //
    // This is fmpz_lucas_chain_VtoU from fmpz/lucas_chain.c, FLINT 3.7.0-dev.
    fn v_to_u(&mut self, vm: &[Limb], vm1: &[Limb], d_inv: &[Limb]) -> (Vec<Limb>, Vec<Limb>) {
        let ms = self.ms;
        // (2V_{m + 1} - AV_m) / D
        let t = limbs_sub_mod(&limbs_add_mod(vm1, vm1, ms), &self.mul_a(vm), ms);
        let um = self.mul(&t, d_inv);
        // (V_m + AU_m) / 2
        let mut t = limbs_add_mod(vm, &self.mul_a(&um), ms);
        let carry = t[0].odd() && limbs_slice_add_same_length_in_place_left(&mut t, ms);
        limbs_slice_shr_in_place(&mut t, 1);
        if carry {
            *t.last_mut().unwrap() |= LIMB_HIGH_BIT;
        }
        (um, t)
    }
}

// The Morrison test for an odd n, given the primes `pp1` dividing n + 1. Returns `Some(false)` if n
// is composite, and `Some(true)` if a Lucas sequence U has U_{n + 1} = 0 mod n and gcd(U_{(n + 1) /
// q}, n) = 1 for every prime q dividing the factored part F of n + 1. In that case every prime
// factor of n is ±1 mod F, so n is prime if F - 1 > sqrt(n). Returns `None` if none of the first
// MAX_BASES values of B settles it. Also returns F and the cofactor R = (n + 1) / F. `prove` proves
// the primality of the cofactor of n + 1. Requires n not to be a square.
//
// This is fmpz_is_prime_morrison from fmpz/is_prime_morrison.c, FLINT 3.7.0-dev, except that FLINT
// tries values of B until one settles it; see the u128 version in malachite-base.
crate_test_fn! {natural_is_prime_morrison(
    n: &Natural,
    pp1: &[u64],
    prove: fn(&Natural) -> Option<bool>,
) -> (Option<bool>, Natural, Natural) {
    // start with n + 1
    let (f, r, fac) = factor_n_pm_1(n + Natural::ONE, pp1, prove);
    let mut c;
    // Want D = A^2 - 4B where A = a, B = b, such that (D / n) = -1.
    let mut b = 1u64;
    loop {
        if b > MAX_BASES {
            return (None, f, r);
        }
        // need gcd(n, b) = 1. FLINT also skips any b with gcd(b, n) = n, but here b <= MAX_BASES <
        // n.
        let g = Natural::from(b).gcd(n);
        if g != 1u32 {
            // found a factor of n
            fail_on_untested_path("natural_is_prime_morrison, gcd(b, n) is a proper factor of n");
            return (Some(false), f, r);
        }
        let mut a = 2u64;
        // D mod n
        let d = loop {
            a += 1;
            let d = Integer::from(a * a) - Integer::from(b << 2);
            let d = Natural::exact_from(d.mod_op(Integer::from(n)));
            if (&d).jacobi_symbol(n) == -1 {
                break d;
            }
        };
        let d_inv = d.mod_inverse(n).unwrap();
        let mut lucas = LucasContext::new(n.as_limbs_asc(), a, b);
        let mut d_inv_limbs = d_inv.into_limbs_asc();
        d_inv_limbs.resize(n.as_limbs_asc().len(), 0);
        let d_inv = lucas.redc.to_redc(&d_inv_limbs);
        // compute U((n + 1) / F) mod n
        let (vm, vm1) = lucas.chain_full(&r);
        let (um, um1) = lucas.v_to_u(&vm, &vm1, &d_inv);
        // check U(n + 1) = 0 mod n
        if lucas.chain_mul(&um, &um1, &f).0.iter().any(|&x| x != 0) {
            return (Some(false), f, r);
        }
        c = lucas.redc.to_redc(&[1]);
        // find values U((n + 1) / q) for each prime q dividing F
        let mut valid = true;
        for q in &fac {
            let ukm = lucas.chain_mul(&um, &um1, &(&f / q)).0;
            if ukm.iter().all(|&x| x == 0) {
                valid = false;
                break;
            }
            c = lucas.mul(&c, &ukm);
        }
        if valid {
            break;
        }
        b += 1;
    }
    // check for factors of n; c is in Montgomery form, but R is coprime to n
    let g = n.gcd(Natural::from_owned_limbs_asc(c));
    if g != 1u32 {
        fail_on_untested_path("natural_is_prime_morrison, gcd(c, n) > 1");
    }
    (Some(g == 1u32), f, r)
}}

// Searches for a divisor of n congruent to r mod s, other than 1 and n, where gcd(r, s) = 1 and
// n^(1/3) < s <= n^(1/2), so that n has at most 11 such divisors. Returns the first one it finds.
//
// This is fmpz_divisor_in_residue_class_lenstra from fmpz/divisor_in_residue_class_lenstra.c, FLINT
// 3.7.0-dev.
crate_test_fn! {natural_divisor_in_residue_class_lenstra(
    n: &Natural,
    r: &Natural,
    s: &Natural,
) -> Option<Natural> {
    let is = Integer::from(s);
    let ir = Integer::from(r);
    let i_n = Integer::from(n);
    // ns2 = n / s^2
    let ns2 = Integer::from(n / s.square());
    // r1 = r^-1 mod s
    let r1 = r.mod_inverse(s).unwrap();
    // r2 = r1 * n mod s
    let r2 = (&r1).mod_mul(n % s, s);
    let ir2 = Integer::from(&r2);
    let s_data = <&Natural as DivModPrecomputed<&Natural>>::precompute_div_mod_data(&s);
    let mut a0 = s.clone();
    // a1 = r1 * r2 mod s, with 0 < a1 <= s
    let mut a1 = (&r1).mod_mul(&r2, s);
    if a1 == 0u32 {
        a1 = s.clone();
    }
    let mut b0 = Integer::ZERO;
    let mut b1 = Integer::ONE;
    let mut c0 = Natural::ZERO;
    // c1 = (n - r * r2) / s * r1 mod s
    let mut c1 = (n.sub_mul(r, &r2) / s % s).mod_mul(&r1, s);
    let is_divisor = |fac: &Natural| {
        *fac != 0u32 && *fac != 1u32 && fac != n && n.divisible_by(fac)
    };
    // deal with a0, b0, c0
    if *r != 1u32 && n != r && n.divisible_by(r) {
        return Some(r.clone());
    }
    for i in 1usize.. {
        let ia1 = Integer::from(&a1);
        let a1b1 = &ia1 * &b1;
        let (mut s1, s2) = if i.even() {
            // c1 is already reduced mod s
            (Integer::from(&c1), -&is)
        } else {
            let s2 = a1b1.clone();
            // s1 = a1 * b1 + n / s^2
            let mut s1 = &s2 + &ns2;
            // s1 is the largest integer less than a1 * b1 + n / s^2 congruent to c1 mod s
            let q = Natural::exact_from((&s1).mod_op(&is));
            if q < c1 {
                s1 -= &is;
            }
            s1 = s1 - Integer::from(q) + Integer::from(&c1);
            // s2 = 2 * a1 * b1 - 1
            (s1, (s2 << 1u32) - Integer::ONE)
        };
        // for each value s1 in range
        while s1 > s2 {
            // d = (s1 * s + a1 * r + b1 * r2)^2 - 4 * a1 * b1 * n
            let d1 = (&is).mul_add_mul(&s1, &ia1, &ir).add_mul(&b1, &ir2);
            let d = (&d1).square().sub_mul_shl(&a1b1, &i_n, 2);
            if d >= 0u32 && (&d).unsigned_abs().is_square() {
                // A divisor exists; the roots are (d1 ± sqrt(d)) / 2.
                let root = Integer::from(d.unsigned_abs().sqrt_rem().0);
                let d = (root + &d1).div_round(Integer::TWO, Down).0;
                // FLINT also handles b1 == 0, but b1 is never 0: the b's alternate in sign, and
                // |b_{k+1}| = |b_{k-1}| + q|b_k| with |b_1| = 1.
                if a1 == 0u32 {
                    // y * b1 = s1; check whether y * s + r2 is a factor
                    let fac = (&ir2).add_mul(&(&s1).div_round(&b1, Down).0, &is).unsigned_abs();
                    if is_divisor(&fac) {
                        return Some(fac);
                    }
                } else {
                    // either d / a1 or d / b1 is a divisor of n
                    let fac = (&d).div_round(&ia1, Down).0.unsigned_abs();
                    if is_divisor(&fac) {
                        return Some(fac);
                    }
                    let fac = (&d).div_round(&b1, Down).0.unsigned_abs();
                    if is_divisor(&fac) {
                        return Some(fac);
                    }
                }
            }
            s1 -= &is;
        }
        // the Euclidean chain has terminated
        if a1 == 0u32 {
            break;
        }
        // Euclidean chain: a1, a0 = a0 - q * a1, a1, where 0 <= a1 < a0 for i even and 0 < a1 <= a0
        // for i odd
        let (mut q, r0) = (&a0).div_mod(&a1);
        a0 = r0;
        if i.even() && a0 == 0u32 {
            q -= Natural::ONE;
            a0 += &a1;
        }
        core::mem::swap(&mut a0, &mut a1);
        // b1, b0 = b0 - q * b1, b1
        b0.sub_mul_assign(Integer::from(&q), &b1);
        core::mem::swap(&mut b0, &mut b1);
        // c1, c0 = c0 - q * c1, c1 mod s
        c0.mod_sub_assign((&q).div_mod_precomputed(s, &s_data).1.mod_mul(&c1, s), s);
        core::mem::swap(&mut c0, &mut c1);
    }
    None
}}

// The n - 1 and n + 1 stage of the primality proof for an odd n >= 2^64 with no small factors that
// is not a square and is a base-2 strong probable prime. Returns `Some(true)` if n is proven prime,
// `Some(false)` if n is proven composite, and `None` if neither test can settle it. `prove` proves
// the primality of the cofactors of n - 1 and n + 1.
//
// This is the proof stage of _fmpz_is_prime from fmpz/is_prime.c, FLINT 3.7.0-dev, except that it
// returns `None` where FLINT calls _fmpz_is_prime_proof. FLINT repeats the stage with ten times the
// trial division limit while the result is unknown, but every branch settles the result, so the
// stage runs once.
crate_test_fn! {natural_is_prime_n_pm_1(
    n: &Natural,
    prove: fn(&Natural) -> Option<bool>,
) -> Option<bool> {
    let logd = dlog(n);
    let mut limit = u64::rounding_from(logd * logd * logd / 100.0, Floor).0 + 20;
    // Below about 260 bits, a 10x larger trial factor base measurably increases the fraction of
    // inputs proven by the n ± 1 tests alone.
    if logd < 180.0 {
        limit *= 10;
    }
    let (pm1, pp1) = n_pm_1_trial_factors(n, limit);
    // Feasibility check: with F1 | n - 1 and F2 | n + 1 the fully factored parts, every proof below
    // requires lcm(F1, F2)^3 > n, except when a cofactor of n - 1 or n + 1 is itself prime.
    let mut fc = Natural::ONE;
    let mut feasible = false;
    for (pf, mut rc) in [(&pm1, n - Natural::ONE), (&pp1, n + Natural::ONE)] {
        for &p in pf {
            let p = Natural::from(p);
            let e = rc.remove_power_assign(&p);
            fc *= p.pow(e);
        }
        // A prime cofactor makes n ∓ 1 fully factorable.
        if rc == 1u32 || rc.is_strong_probable_prime(&Natural::TWO) {
            feasible = true;
            break;
        }
    }
    if !feasible {
        // Both 2-parts were included, so lcm(F1, F2) >= Fc / 2.
        fc >>= 1u32;
        feasible = (&fc).pow(3) > *n;
    }
    if !feasible {
        return None;
    }
    // p - 1 test
    let (res, f1, _) = natural_is_prime_pocklington(n, &pm1, prove);
    if res != Some(true) {
        return res;
    }
    let f1_sqr = (&f1).square();
    if f1_sqr >= *n {
        return Some(true);
    }
    if Natural::cmp_product(&f1_sqr, &f1, n) != Less {
        // Brillhart, Lehmer, Selfridge test. n = 1 mod F1, so let n = c2 * F1^2 + c1 * F1 + 1, and
        // check whether c1^2 - 4 * c2 is a square.
        let (c2, c1) = ((n - Natural::ONE) / &f1).div_mod(&f1);
        let d = Integer::from(c1.square()) - Integer::from(c2 << 2u32);
        return Some(d < 0u32 || !d.unsigned_abs().is_square());
    }
    // p + 1 test
    let (res, f2, r) = natural_is_prime_morrison(n, &pp1, prove);
    if res != Some(true) {
        return res;
    }
    // need F2 - 1 > sqrt(n)
    let f2_m1 = &f2 - Natural::ONE;
    let f2_m1_sqr = (&f2_m1).square();
    if f2_m1_sqr > *n {
        return Some(true);
    }
    if Natural::cmp_product(&f2_m1_sqr, &f2_m1, n) == Greater {
        // Improved n + 1 test. Let R = r1 * F2 + r0.
        let (r1, r0) = r.div_mod(&f2);
        let ir0 = Integer::from(&r0);
        // Check whether x^2 + r0 * x - r1 has a positive integral root.
        let (b, rem) = ((&r0).square() + (&r1 << 2u32)).sqrt_rem();
        let mut composite = rem == 0u32 && b > ir0;
        if composite {
            fail_on_untested_path("natural_is_prime_n_pm_1, improved n + 1 test, first root");
        }
        // Check whether x^2 + (r0 - F2) * x - r1 - 1 has a positive integral root.
        let ir0 = ir0 - Integer::from(&f2);
        let (b, rem) = ((&ir0).square().unsigned_abs() + ((r1 + Natural::ONE) << 2u32)).sqrt_rem();
        if rem == 0u32 && b > ir0 {
            fail_on_untested_path("natural_is_prime_n_pm_1, improved n + 1 test, second root");
            composite = true;
        }
        return Some(!composite);
    }
    // Brillhart, Lehmer, Selfridge combined p - 1, p + 1 test. F = lcm(F1, F2), where F1 | n - 1
    // and F2 | n + 1.
    let mut f = &f1 * &f2;
    if f1.even() && f2.even() {
        f >>= 1u32;
    }
    let f_sqr = (&f).square();
    if f_sqr > *n {
        // lcm(F1, F2) > sqrt(n); check that n mod F is not a factor of n
        let n_mod_f = n % &f;
        if n_mod_f != *n && n_mod_f != 1u32 && n.divisible_by(&n_mod_f) {
            fail_on_untested_path("natural_is_prime_n_pm_1, n mod F divides n");
            return Some(false);
        }
        return Some(true);
    }
    if Natural::cmp_product(&f_sqr, &f, n) == Greater {
        // Lenstra's divisors in residue classes
        let composite = natural_divisor_in_residue_class_lenstra(n, &Natural::ONE, &f).is_some()
            || natural_divisor_in_residue_class_lenstra(n, &(n % &f), &f).is_some();
        if composite {
            fail_on_untested_path("natural_is_prime_n_pm_1, Lenstra's search finds a divisor");
        }
        return Some(!composite);
    }
    None
}}

// Proves an n prime or composite, or returns `None` if the available tests cannot settle it.
//
// This is fmpz_is_prime from fmpz/is_prime.c, FLINT 3.7.0-dev, except that it returns `None` where
// FLINT calls _fmpz_is_prime_proof.
crate_test_fn! {natural_prove_prime(n: &Natural) -> Option<bool> {
    if let Ok(x) = u128::try_from(n) {
        return u128_prove_prime(x);
    }
    let xs = n.as_limbs_asc();
    if xs[0].even() {
        return Some(false);
    }
    let trial_primes = usize::exact_from(n.significant_bits() + 64);
    // Trial division, a square test, and a single base-2 test rule out most composites.
    if limbs_factor_trial(xs, 1, trial_primes) != 0
        || n.is_square()
        || !n.is_strong_probable_prime(&Natural::TWO)
    {
        return Some(false);
    }
    natural_is_prime_n_pm_1(n, natural_prove_prime)
}}
