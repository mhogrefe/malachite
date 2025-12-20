// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use crate::natural::arithmetic::div_exact::limbs_div_exact_limb_to_out;
use crate::natural::arithmetic::mod_op::limbs_mod_limb;
use crate::natural::arithmetic::mul::fft::{
    CRTData, Context, FFTContext, MAX_NPROFILES, MPN_CTX_NCRTS, MPN_CTX_TWO_POWER_TAB_SIZE,
    MPNToFFTFunc, ModData, SD_FFT_CTX_W2TAB_INIT, SerializedCRTData, SerializedContext,
    SerializedFFTContext, VEC_SZ, crt_data_find_bn_bound, f64_mulmod, f64_reduce_0n_to_pmhn,
    f64_reduce_pm1n_to_pmhn, f64x4_mul_add, mpn_mul_default_mpn_ctx,
};
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_6h, limbs_mul_greater_to_out_toom_8h,
    limbs_mul_greater_to_out_toom_22, limbs_mul_greater_to_out_toom_32,
    limbs_mul_greater_to_out_toom_33, limbs_mul_greater_to_out_toom_42,
    limbs_mul_greater_to_out_toom_43, limbs_mul_greater_to_out_toom_44,
    limbs_mul_greater_to_out_toom_53, limbs_mul_greater_to_out_toom_63,
};
use crate::natural::arithmetic::mul::{
    FFT_MUL_THRESHOLD, limbs_mul_greater_to_out_basecase, limbs_mul_greater_to_out_scratch_len,
    toom44_ok,
};
use crate::platform::{
    Limb, MUL_FFT_THRESHOLD, MUL_TOOM6H_THRESHOLD, MUL_TOOM8H_THRESHOLD, MUL_TOOM22_THRESHOLD,
    MUL_TOOM32_TO_TOOM43_THRESHOLD, MUL_TOOM32_TO_TOOM53_THRESHOLD, MUL_TOOM33_THRESHOLD,
    MUL_TOOM42_TO_TOOM53_THRESHOLD, MUL_TOOM42_TO_TOOM63_THRESHOLD, MUL_TOOM44_THRESHOLD,
};
use libm::scalbn;
use malachite_base::num::arithmetic::traits::{
    DivRound, ModPow, PowerOf2, XMulYToZZ, XXDivModYToQR,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::factorization::traits::{IsPrime, PrimitiveRootPrime};
use malachite_base::num::logic::traits::{LeadingZeros, SignificantBits, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode::*;
use wide::f64x4;

// In GMP this is hardcoded to 500
pub const MUL_BASECASE_MAX_UN: usize = 500;

// We must have 1 < ys.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < xs.len().
fn limbs_mul_greater_to_out_basecase_mem_opt_helper(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(ys_len > 1);
    assert!(ys_len < MUL_TOOM22_THRESHOLD);
    assert!(MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN);
    assert!(xs_len > MUL_BASECASE_MAX_UN);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in xs.chunks(MUL_BASECASE_MAX_UN) {
        let out = &mut out[offset..];
        if chunk.len() >= ys_len {
            limbs_mul_greater_to_out_basecase(out, chunk, ys);
        } else {
            limbs_mul_greater_to_out_basecase(out, ys, chunk);
        }
        if offset != 0 {
            limbs_slice_add_greater_in_place_left(out, &triangle_buffer[..ys_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < xs_len {
            triangle_buffer[..ys_len]
                .copy_from_slice(&out[MUL_BASECASE_MAX_UN..MUL_BASECASE_MAX_UN + ys_len]);
        }
    }
}

/// A version of `limbs_mul_greater_to_out_basecase` that attempts to be more efficient by
/// increasing cache locality. It is currently not measurably better than ordinary basecase.
pub fn limbs_mul_greater_to_out_basecase_mem_opt(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    if ys_len > 1 && ys_len < MUL_TOOM22_THRESHOLD && xs.len() > MUL_BASECASE_MAX_UN {
        limbs_mul_greater_to_out_basecase_mem_opt_helper(out, xs, ys);
    } else {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    }
}

pub fn limbs_product_naive(out: &mut [Limb], factors: &[Limb]) -> usize {
    let mut n = Natural::ONE;
    for &f in factors {
        n *= Natural::from(f);
    }
    let xs = n.into_limbs_asc();
    let size = xs.len();
    out[..size].copy_from_slice(&xs);
    size
}

pub fn natural_product_naive<I: Iterator<Item = Natural>>(xs: I) -> Natural {
    let mut p = Natural::ONE;
    for x in xs {
        if x == 0 {
            return Natural::ZERO;
        }
        p *= x;
    }
    p
}

impl CRTData {
    // This is crt_data_init from fft_small.h, FLINT 3.3.0-dev, returning CRTData.
    fn new(prime: u64, coeff_len: usize, nprimes: usize) -> Self {
        Self {
            prime,
            coeff_len,
            nprimes,
            data: vec![0; nprimes * coeff_len + coeff_len + nprimes],
        }
    }

    // This is crt_data_co_prime_red from fft_small.h, FLINT 3.3.0-dev, writing a value.
    #[inline]
    fn co_prime_red_write(&mut self, i: usize, val: u64) {
        assert!(i < self.nprimes);
        self.data[self.nprimes * self.coeff_len + self.coeff_len + i] = val;
    }

    // This is crt_data_co_prime from fft_small.h, FLINT 3.3.0-dev, writing a value.
    #[inline]
    fn co_prime_write(&mut self, i: usize, val: u64) {
        assert!(i < self.nprimes);
        self.data[i * self.coeff_len] = val;
    }

    // return mpn of length C->coeff_len
    //
    // This is crt_data_prod_primes from fft_small.h, FLINT 3.3.0-dev.
    #[inline]
    fn prod_primes(&mut self) -> &mut [u64] {
        &mut self.data[self.nprimes * self.coeff_len..]
    }

    fn serialize(self) -> SerializedCRTData {
        SerializedCRTData {
            prime: self.prime,
            coeff_len: self.coeff_len,
            nprimes: self.nprimes,
        }
    }
}

const D_BITS: i32 = 53;

// This is fft_small_mulmod_satisfies_bounds from fft_small/mulmod_statisfies_bounds.c, FLINT
// 3.3.0-dev.
fn fft_small_mulmod_satisfies_bounds(nn: u64) -> bool {
    let n = nn as f64;
    let ninv = 1.0 / n;
    let t1 = n.mul_add(ninv, -1.0).abs(); // epsilon ~= t1 / n  good enough
    let n1bits = i32::exact_from(nn.significant_bits());
    let n2hi = u64::x_mul_y_to_zz(nn, nn).0;
    assert_ne!(n2hi, 0);
    let n2bits = i32::exact_from(u64::WIDTH + n2hi.significant_bits());
    // for |a*b| < 2*n^2
    //
    // |h*n_inv| < 2*n, so rounding in mul(h, ninv) at least B bits after the .
    let b = D_BITS - n1bits - 1;
    assert!(b >= 2);
    let diff = n2bits - D_BITS;
    let p = 2.0 * n * t1;
    let limit2 = p + scalbn(ninv, diff) + 0.5 + scalbn(1.0, -b - 1);
    // for |a * b| < 4 * n ^ 2
    let limit4 = 2.0 * p + scalbn(ninv, diff + 1) + 0.5 + scalbn(1.0, -b);
    // fudge the limits 1 and 3/2 because the above is double arithmetic
    limit2 < 0.99 && limit4 < 1.49
}

// This is nmod_init from nmod.h, FLINT 3.3.0-dev, but returning `mod`.
fn nmod_init(n: u64) -> ModData {
    let norm = LeadingZeros::leading_zeros(n);
    let shifted = n << norm;
    let ninv = u64::xx_div_mod_y_to_qr(!shifted, u64::MAX, shifted).0;
    ModData { n, ninv, norm }
}

// This is next_fft_number from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn next_fft_number(p: u64) -> u64 {
    let bits = p.significant_bits();
    let l = TrailingZeros::trailing_zeros(p - 1);
    let q = p - u64::power_of_2(l + 1);
    if bits < 15 {
        panic!();
    } else if q.significant_bits() == bits {
        q
    } else {
        assert!(l >= 5);
        u64::power_of_2(bits) - u64::power_of_2(l - 1) + 1
    }
}

// This is sd_fft_ctx_init_prime from fft_small/sd_fft_ctx.c, FLINT 3.3.0-dev, returning q.
fn sd_fft_ctx_init_prime(pp: u64) -> FFTContext {
    assert!(
        fft_small_mulmod_satisfies_bounds(pp),
        "FFT prime {pp} does not satisfy bounds for arithmetic"
    );
    let mut q = FFTContext::default();
    q.p = pp as f64;
    q.pinv = 1.0 / q.p;
    q.mod_data = nmod_init(pp);
    q.primitive_root = pp.primitive_root_prime();
    let n = q.p;
    let ninv = q.pinv;
    //  fill wtab to a depth of SD_FFT_CTX_W2TAB_INIT: 2 ^ (SD_FFT_CTX_W2TAB_INIT - 1) entries: 1,
    //  e(1 / 4), e(1 / 8), e(3 / 8), ...
    //
    //  Q->w2tab[j] is itself a table of length 2 ^ (j - 1) containing 2 ^ (j + 1) st roots of
    //  unity.
    q.w2tab_backing = vec![0.0; 1 << 12];
    q.w2tab_backing[0] = 1.0;
    let mut l = 1;
    for k in 1..SD_FFT_CTX_W2TAB_INIT {
        let w = f64_reduce_0n_to_pmhn!(
            q.primitive_root
                .mod_pow((q.mod_data.n - 1) >> (k + 1), q.mod_data.n) as f64,
            n
        );
        q.w2tab_offsets[k as usize] = l;
        let (w_lo, w_hi) = q.w2tab_backing.split_at_mut(l);
        for (hi, &lo) in w_hi.iter_mut().zip(w_lo.iter()) {
            *hi = f64_reduce_pm1n_to_pmhn!(f64_mulmod!(lo, w, n, ninv), n);
        }
        l <<= 1;
    }
    q.w2tab_depth = SD_FFT_CTX_W2TAB_INIT;
    q
}

// fill x[i] = 2 ^ i mod p for 0 <= i < len
//
// This is fill_slow_two_pow_tab from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn fill_slow_two_pow_tab(xs: &mut [f64], p: f64, pinv: f64) {
    let mut t = 1.0;
    let (xs_head, xs_tail) = xs.split_first_mut().unwrap();
    *xs_head = t;
    for x in xs_tail {
        let q = (t * (2.0 * pinv)).round_ties_even();
        t = fma!(-q, p, t + t);
        *x = t;
    }
}

// fill in  d[i * nvs + k / VEC_SZ][k % VEC_SZ] = 2 ^ i mod Rffts[k].p for 0 <= k < VEC_SZ * nvs and
// 0 <= i < len.
//
// This is fill_vec_two_pow_tab from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn fill_vec_two_pow_tab(xs: &mut [f64x4], rffts: &mut [FFTContext], len: usize, nvs: usize) {
    let mut ps = vec![f64x4::ZERO; nvs << 1];
    for l in 0..nvs {
        let r = &rffts[l << 2..];
        let p = &mut ps[l << 1..];
        p[0] = f64x4::new([r[0].p, r[1].p, r[2].p, r[3].p]);
        p[1] = f64x4::new([r[0].pinv, r[1].pinv, r[2].pinv, r[3].pinv]) * f64x4::splat(2.0);
    }
    for x in xs.iter_mut().take(nvs) {
        *x = f64x4::ONE;
    }
    let mut k = 0;
    for _ in 1..len {
        for l in 0..nvs {
            let xs = &mut xs[k + l..];
            let t = xs[0];
            let p = &ps[l << 1..];
            xs[nvs] = f64x4_mul_add!(-(t * p[1]).round(), p[0], t * f64x4::splat(2.0));
        }
        k += nvs;
    }
}

// TODO make macro
//
// This is PUSH_PROFILE from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn push_profile(r: &mut Context, np: usize, bits: u64) {
    let i = r.profiles_size;
    r.profiles[i].np = np;
    r.profiles[i].bits = bits;
    r.profiles[i].bn_bound = crt_data_find_bn_bound(&r.crts[np - 1], bits);
    r.profiles[i].to_ffts = Some(MPNToFFTFunc { np, bits });
    r.profiles_size = i + 1;
}

const DEFAULT_PRIME: u64 = 0x0003f00000000001;

// This is mpn_ctx_init from fft_small/mpn_mul.c, FLINT 3.3.0-dev, returning the context.
pub fn initialize_context() -> Context {
    let mut p = DEFAULT_PRIME;
    let mut r = Context::default();
    for i in 0..MPN_CTX_NCRTS {
        if i > 0 {
            p = next_fft_number(p);
        }
        while !p.is_prime() {
            p = next_fft_number(p);
        }
        r.ffts[i] = sd_fft_ctx_init_prime(p);
        if i == 0 {
            r.crts[0] = CRTData::new(p, 1, 1);
            r.crts[0].co_prime_red_write(0, 1);
            r.crts[0].co_prime_write(0, 1);
            r.crts[0].prod_primes()[0] = p;
        } else {
            let mut len = r.crts[i - 1].coeff_len;
            let mut t = vec![0; (len + 2) << 1];
            let (t, tt) = t.split_at_mut(len + 2);
            t[len + 1] = 0;
            t[len] = limbs_mul_limb_to_out::<u128, u64>(t, &r.crts[i - 1].prod_primes()[..len], p);
            // leave enough room for (product of primes)*(number of primes)
            len += 2;
            limbs_mul_limb_to_out::<u128, u64>(tt, &t[..len], u64::wrapping_from(i) + 1);
            while tt[len - 1] == 0 {
                len -= 1;
            }
            r.crts[i] = CRTData::new(p, len, i + 1);
            // set product of primes
            r.crts[i].prod_primes()[..len].copy_from_slice(&t[..len]);
            // set cofactors
            for pi in 0..=i {
                let prime = r.crts[pi].prime;
                let cofac = r.crts[i].co_prime(pi);
                limbs_div_exact_limb_to_out::<u128, u64>(cofac, &t[..len], prime);
                let d = limbs_mod_limb::<u128, u64>(&cofac[..len], prime);
                r.crts[i].co_prime_red_write(pi, d);
            }
        }
    }
    // powers of two for slow mod
    {
        let len = MPN_CTX_TWO_POWER_TAB_SIZE;
        let x = vec![0.0; len * MPN_CTX_NCRTS];
        r.slow_two_pow_backing = x;
        let mut offset = 0;
        for i in 0..MPN_CTX_NCRTS {
            r.slow_two_pow_offsets[i] = offset;
            fill_slow_two_pow_tab(
                &mut r.slow_two_pow_backing[offset..offset + len],
                r.ffts[i].p,
                r.ffts[i].pinv,
            );
            offset += len;
        }
    }
    // powers of two for fast mod
    {
        let len = MPN_CTX_TWO_POWER_TAB_SIZE;
        let max_nvs = MPN_CTX_NCRTS.div_round(VEC_SZ, Ceiling).0;
        let x = vec![f64x4::ZERO; max_nvs * (max_nvs + 1) / 2 * len];
        r.vec_two_pow_tab_backing = x;
        let mut offset = 0;
        for nvs in 1..=max_nvs {
            r.vec_two_pow_tab_offsets[nvs - 1] = offset;
            fill_vec_two_pow_tab(
                &mut r.vec_two_pow_tab_backing[offset..],
                &mut r.ffts,
                len,
                nvs,
            );
            offset += nvs * len;
        }
    }
    r.profiles_size = 0;
    push_profile(&mut r, 4, 84);
    push_profile(&mut r, 4, 88);
    push_profile(&mut r, 4, 92);
    push_profile(&mut r, 5, 112);
    push_profile(&mut r, 5, 116);
    push_profile(&mut r, 5, 120);
    push_profile(&mut r, 6, 136);
    push_profile(&mut r, 6, 140);
    push_profile(&mut r, 6, 144);
    push_profile(&mut r, 7, 160);
    push_profile(&mut r, 7, 164);
    push_profile(&mut r, 7, 168);
    push_profile(&mut r, 8, 184);
    push_profile(&mut r, 8, 188);
    push_profile(&mut r, 8, 192);
    assert!(r.profiles_size <= MAX_NPROFILES);
    r
}

impl FFTContext {
    fn serialize(self) -> SerializedFFTContext {
        let mut w2tab_backing = [0; 4096];
        for (o, x) in w2tab_backing.iter_mut().zip(self.w2tab_backing.into_iter()) {
            *o = x.to_bits();
        }
        SerializedFFTContext {
            p: self.p.to_bits(),
            pinv: self.pinv.to_bits(),
            mod_data: self.mod_data,
            primitive_root: self.primitive_root,
            w2tab_depth: self.w2tab_depth,
            w2tab_backing,
            w2tab_offsets: self.w2tab_offsets,
        }
    }
}

impl Context {
    pub_crate_test! {serialize(self) -> SerializedContext {
        let mut crts_data_0 = [0; 3];
        let mut crts_data_1 = [0; 8];
        let mut crts_data_2 = [0; 15];
        let mut crts_data_3 = [0; 24];
        let mut crts_data_4 = [0; 29];
        let mut crts_data_5 = [0; 41];
        let mut crts_data_6 = [0; 55];
        let mut crts_data_7 = [0; 71];
        crts_data_0.copy_from_slice(&self.crts[0].data);
        crts_data_1.copy_from_slice(&self.crts[1].data);
        crts_data_2.copy_from_slice(&self.crts[2].data);
        crts_data_3.copy_from_slice(&self.crts[3].data);
        crts_data_4.copy_from_slice(&self.crts[4].data);
        crts_data_5.copy_from_slice(&self.crts[5].data);
        crts_data_6.copy_from_slice(&self.crts[6].data);
        crts_data_7.copy_from_slice(&self.crts[7].data);
        let mut vec_two_pow_tab_backing = [[0; 4]; 768];
        for (o, f) in vec_two_pow_tab_backing
            .iter_mut()
            .zip(self.vec_two_pow_tab_backing.into_iter())
        {
            let [f0, f1, f2, f3] = f.to_array();
            *o = [f0.to_bits(), f1.to_bits(), f2.to_bits(), f3.to_bits()];
        }
        let mut slow_two_pow_backing = [0; 1 << 11];
        for (o, x) in slow_two_pow_backing
            .iter_mut()
            .zip(self.slow_two_pow_backing.into_iter())
        {
            *o = x.to_bits();
        }
        let [f0, f1, f2, f3, f4, f5, f6, f7] = self.ffts;
        let [c0, c1, c2, c3, c4, c5, c6, c7] = self.crts;
        SerializedContext {
            ffts: [
                f0.serialize(),
                f1.serialize(),
                f2.serialize(),
                f3.serialize(),
                f4.serialize(),
                f5.serialize(),
                f6.serialize(),
                f7.serialize(),
            ],
            crts: [
                c0.serialize(),
                c1.serialize(),
                c2.serialize(),
                c3.serialize(),
                c4.serialize(),
                c5.serialize(),
                c6.serialize(),
                c7.serialize(),
            ],
            crts_data_0,
            crts_data_1,
            crts_data_2,
            crts_data_3,
            crts_data_4,
            crts_data_5,
            crts_data_6,
            crts_data_7,
            vec_two_pow_tab_backing,
            vec_two_pow_tab_offsets: self.vec_two_pow_tab_offsets,
            slow_two_pow_backing,
            slow_two_pow_offsets: self.slow_two_pow_offsets,
            profiles: self.profiles,
            profiles_size: self.profiles_size,
            buffer_alloc: self.buffer_alloc,
        }
    }}
}

fn limbs_mul_same_length_to_out_slow(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    assert_ne!(len, 0);
    if len < MUL_TOOM22_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if len < MUL_TOOM33_THRESHOLD {
        limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
    } else if len < MUL_TOOM44_THRESHOLD {
        limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
    } else if len < MUL_TOOM6H_THRESHOLD {
        limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch);
    } else if len < MUL_TOOM8H_THRESHOLD {
        limbs_mul_greater_to_out_toom_6h(out, xs, ys, scratch);
    } else if len < FFT_MUL_THRESHOLD {
        limbs_mul_greater_to_out_toom_8h(out, xs, ys, scratch);
    } else {
        mpn_mul_default_mpn_ctx(out, xs, ys, true);
    }
}

fn limbs_mul_greater_to_out_old_slow(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) -> Limb {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert_ne!(ys_len, 0);
    assert!(out.len() >= xs_len + ys_len);
    if ys_len < MUL_TOOM22_THRESHOLD {
        // Plain schoolbook multiplication. Unless xs_len is very large, or else if
        // `limbs_mul_same_length_to_out` applies, perform basecase multiply directly.
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if ys_len < MUL_TOOM33_THRESHOLD {
        if xs_len >= 3 * ys_len {
            let two_ys_len = ys_len << 1;
            let three_ys_len = two_ys_len + ys_len;
            let four_ys_len = two_ys_len << 1;
            let (scratch, mul_scratch) = scratch.split_at_mut(four_ys_len);
            limbs_mul_greater_to_out_toom_42(out, &xs[..two_ys_len], ys, mul_scratch);
            let mut xs = &xs[two_ys_len..];
            let mut out_offset = two_ys_len;
            while xs.len() >= three_ys_len {
                let out = &mut out[out_offset..];
                let (xs_lo, xs_hi) = xs.split_at(two_ys_len);
                limbs_mul_greater_to_out_toom_42(scratch, xs_lo, ys, mul_scratch);
                let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
                out[ys_len..three_ys_len].copy_from_slice(&scratch_hi[..two_ys_len]);
                assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
                xs = xs_hi;
                out_offset += two_ys_len;
            }
            let xs_len = xs.len();
            let out = &mut out[out_offset..];
            // ys_len <= xs_len < 3 * ys_len
            let four_xs_len = xs_len << 2;
            if four_xs_len < 5 * ys_len {
                limbs_mul_greater_to_out_toom_22(scratch, xs, ys, mul_scratch);
            } else if four_xs_len < 7 * ys_len {
                limbs_mul_greater_to_out_toom_32(scratch, xs, ys, mul_scratch);
            } else {
                limbs_mul_greater_to_out_toom_42(scratch, xs, ys, mul_scratch);
            }
            let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
            out[ys_len..ys_len + xs_len].copy_from_slice(&scratch_hi[..xs_len]);
            assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
        } else if 4 * xs_len < 5 * ys_len {
            limbs_mul_greater_to_out_toom_22(out, xs, ys, scratch);
        } else if 4 * xs_len < 7 * ys_len {
            limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
        } else {
            limbs_mul_greater_to_out_toom_42(out, xs, ys, scratch);
        }
    } else if (xs_len + ys_len) >> 1 < MUL_FFT_THRESHOLD || 3 * ys_len < MUL_FFT_THRESHOLD {
        // Handle the largest operands that are not in the FFT range. The 2nd condition makes very
        // unbalanced operands avoid the FFT code (except perhaps as coefficient products of the
        // Toom code).
        if ys_len < MUL_TOOM44_THRESHOLD || !toom44_ok(xs_len, ys_len) {
            // Use ToomX3 variants
            if xs_len << 1 >= 5 * ys_len {
                let two_ys_len = ys_len << 1;
                let four_ys_len = two_ys_len << 1;
                let (scratch, mul_scratch) = scratch.split_at_mut(four_ys_len);
                let (xs_lo, mut xs) = xs.split_at(two_ys_len);
                if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                    limbs_mul_greater_to_out_toom_42(out, xs_lo, ys, mul_scratch);
                } else {
                    limbs_mul_greater_to_out_toom_63(out, xs_lo, ys, mul_scratch);
                }
                let mut out_offset = two_ys_len;
                // xs_len >= 2.5 * ys_len
                while xs.len() << 1 >= 5 * ys_len {
                    let out = &mut out[out_offset..];
                    let (xs_lo, xs_hi) = xs.split_at(two_ys_len);
                    if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                        limbs_mul_greater_to_out_toom_42(scratch, xs_lo, ys, mul_scratch);
                    } else {
                        limbs_mul_greater_to_out_toom_63(scratch, xs_lo, ys, mul_scratch);
                    }
                    let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
                    out[ys_len..ys_len + two_ys_len].copy_from_slice(&scratch_hi[..two_ys_len]);
                    assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
                    xs = xs_hi;
                    out_offset += two_ys_len;
                }
                let xs_len = xs.len();
                let out = &mut out[out_offset..];
                // ys_len / 2 <= xs_len < 2.5 * ys_len
                limbs_mul_to_out_slow(scratch, xs, ys, mul_scratch);
                let (scratch_lo, scratch_hi) = scratch.split_at(ys_len);
                out[ys_len..xs_len + ys_len].copy_from_slice(&scratch_hi[..xs_len]);
                assert!(!limbs_slice_add_greater_in_place_left(out, scratch_lo));
            } else if 6 * xs_len < 7 * ys_len {
                limbs_mul_greater_to_out_toom_33(out, xs, ys, scratch);
            } else if xs_len << 1 < 3 * ys_len {
                if ys_len < MUL_TOOM32_TO_TOOM43_THRESHOLD {
                    limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
                } else {
                    limbs_mul_greater_to_out_toom_43(out, xs, ys, scratch);
                }
            } else if 6 * xs_len < 11 * ys_len {
                if xs_len << 2 < 7 * ys_len {
                    if ys_len < MUL_TOOM32_TO_TOOM53_THRESHOLD {
                        limbs_mul_greater_to_out_toom_32(out, xs, ys, scratch);
                    } else {
                        limbs_mul_greater_to_out_toom_53(out, xs, ys, scratch);
                    }
                } else if ys_len < MUL_TOOM42_TO_TOOM53_THRESHOLD {
                    limbs_mul_greater_to_out_toom_42(out, xs, ys, scratch);
                } else {
                    limbs_mul_greater_to_out_toom_53(out, xs, ys, scratch);
                }
            } else if ys_len < MUL_TOOM42_TO_TOOM63_THRESHOLD {
                limbs_mul_greater_to_out_toom_42(out, xs, ys, scratch);
            } else {
                limbs_mul_greater_to_out_toom_63(out, xs, ys, scratch);
            }
        } else if ys_len < MUL_TOOM6H_THRESHOLD {
            limbs_mul_greater_to_out_toom_44(out, xs, ys, scratch);
        } else if ys_len < MUL_TOOM8H_THRESHOLD {
            limbs_mul_greater_to_out_toom_6h(out, xs, ys, scratch);
        } else {
            limbs_mul_greater_to_out_toom_8h(out, xs, ys, scratch);
        }
    } else {
        mpn_mul_default_mpn_ctx(out, xs, ys, true);
    }
    out[xs_len + ys_len - 1]
}

fn limbs_mul_greater_to_out_slow(
    r: &mut [Limb],
    x: &[Limb],
    y: &[Limb],
    scratch: &mut [Limb],
) -> Limb {
    let xs_len = x.len();
    let ys_len = y.len();
    assert!(xs_len >= ys_len);
    if xs_len == ys_len {
        limbs_mul_same_length_to_out_slow(r, x, y, scratch);
    } else if ys_len < FFT_MUL_THRESHOLD {
        let mut scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)];
        limbs_mul_greater_to_out_old_slow(r, x, y, &mut scratch);
    } else {
        mpn_mul_default_mpn_ctx(r, x, y, true);
    }
    r[xs_len + ys_len - 1]
}

fn limbs_mul_greater_slow_fft(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let out_len = xs_len + ys_len;
    let mut scratch = vec![0; out_len + limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)];
    let (out, mul_scratch) = scratch.split_at_mut(out_len);
    limbs_mul_greater_to_out_slow(out, xs, ys, mul_scratch);
    scratch.truncate(out_len);
    scratch.shrink_to_fit();
    scratch
}

fn limbs_mul_to_out_slow(out: &mut [Limb], xs: &[Limb], ys: &[Limb], scratch: &mut [Limb]) -> Limb {
    if xs.len() >= ys.len() {
        limbs_mul_greater_to_out_slow(out, xs, ys, scratch)
    } else {
        limbs_mul_greater_to_out_slow(out, ys, xs, scratch)
    }
}

pub fn mul_slow_fft(x: &Natural, y: &Natural) -> Natural {
    match (x, y) {
        (Natural(Small(x)), y) => y.mul_limb_ref(*x),
        (x, Natural(Small(y))) => x.mul_limb_ref(*y),
        (Natural(Large(xs)), Natural(Large(ys))) => {
            let big_limbs = if xs.len() >= ys.len() {
                limbs_mul_greater_slow_fft(xs, ys)
            } else {
                limbs_mul_greater_slow_fft(ys, xs)
            };
            Natural::from_owned_limbs_asc(big_limbs)
        }
    }
}
