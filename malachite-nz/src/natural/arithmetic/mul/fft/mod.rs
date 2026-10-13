// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
#[cfg(not(feature = "32_bit_limbs"))]
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use crate::natural::arithmetic::add::add_with_carry_limb;
use crate::natural::arithmetic::mul::context::CONTEXT;
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::platform::Limb;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use core::fmt::Debug;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, DivRound, Mod, OverflowingAddAssign, OverflowingSubAssign, PowerOf2,
    RoundToMultiple, RoundToMultipleOfPowerOf2, WrappingAddAssign, XMulYToZZ, XXAddYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::JoinHalves;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_test_zero;
use wide::f64x4;

const VEC_SZ_2: usize = VEC_SZ << 1;

const VEC_SZ_3: usize = 3 * VEC_SZ;

// This is nmod_t from flint.h, FLINT 3.3.0-dev.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct ModData {
    pub(crate) n: u64,
    pub(crate) ninv: u64,
    pub(crate) norm: u64,
}

const SD_FFT_CTX_W2TAB_SIZE: usize = 40;

pub(crate) const SD_FFT_CTX_W2TAB_INIT: u64 = 12;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SerializedFFTContext {
    pub(crate) p: u64,
    pub(crate) pinv: u64,
    pub(crate) mod_data: ModData,
    pub(crate) primitive_root: u64,
    pub(crate) w2tab_depth: u64,
    pub(crate) w2tab_backing: [u64; 4096],
    pub(crate) w2tab_offsets: [usize; SD_FFT_CTX_W2TAB_SIZE],
}

// This is sd_fft_ctx_struct from fft_small.h, FLINT 3.3.0-dev.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FFTContext {
    pub(crate) p: f64,
    pub(crate) pinv: f64,
    pub(crate) mod_data: ModData,
    pub(crate) primitive_root: u64,
    pub(crate) w2tab_depth: u64,
    pub(crate) w2tab_backing: Cow<'static, [f64]>,
    pub(crate) w2tab_offsets: [usize; SD_FFT_CTX_W2TAB_SIZE],
}

impl Default for FFTContext {
    fn default() -> Self {
        Self {
            p: 0.0,
            pinv: 0.0,
            mod_data: ModData::default(),
            primitive_root: 0,
            w2tab_depth: 0,
            w2tab_backing: Cow::Borrowed(&[]),
            w2tab_offsets: [0; SD_FFT_CTX_W2TAB_SIZE],
        }
    }
}

macro_rules! w2tab {
    ($q: expr, $i: expr, $j: expr) => {
        $q.w2tab_backing[$q.w2tab_offsets[$i] + $j]
    };
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SerializedCRTData {
    pub(crate) prime: u64,
    pub(crate) coeff_len: usize,
    pub(crate) nprimes: usize,
}

// This is crt_data_struct from fft_small.h, FLINT 3.3.0-dev.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct CRTData {
    pub(crate) prime: u64,
    pub(crate) coeff_len: usize,
    pub(crate) nprimes: usize,
    pub(crate) data: Vec<u64>,
}

impl CRTData {
    // This is crt_data_co_prime_red from fft_small.h, FLINT 3.3.0-dev, read-only.
    #[inline]
    fn co_prime_red(&self, i: usize) -> u64 {
        assert!(i < self.nprimes);
        self.data[self.nprimes * self.coeff_len + self.coeff_len + i]
    }

    // return mpn of length C->coeff_len
    //
    // This is crt_data_co_prime from fft_small.h, FLINT 3.3.0-dev, read-only.
    #[inline]
    pub(crate) fn co_prime(&mut self, i: usize) -> &mut [u64] {
        assert!(i < self.nprimes);
        &mut self.data[i * self.coeff_len..]
    }

    #[inline]
    fn prod_primes_ref(&self) -> &[u64] {
        &self.data[self.nprimes * self.coeff_len..]
    }
}

// This is profile_entry_struct from fft_small.h, FLINT 3.3.0-dev.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct ProfileEntry {
    pub(crate) np: usize,
    pub(crate) bits: u64,
    pub(crate) bn_bound: usize,
    pub(crate) to_ffts: Option<MPNToFFTFunc>,
}

pub(crate) const MPN_CTX_NCRTS: usize = 8;

pub(crate) const VEC_SZ: usize = 4;

pub(crate) const MAX_NPROFILES: usize = 20;

crate_test_struct! {
    #[derive(Debug, Eq, PartialEq)]
    SerializedContext {
    pub(crate) ffts: [SerializedFFTContext; MPN_CTX_NCRTS],
    pub(crate) crts: [SerializedCRTData; MPN_CTX_NCRTS],
    pub(crate) crts_data_0: [u64; 3],
    pub(crate) crts_data_1: [u64; 8],
    pub(crate) crts_data_2: [u64; 15],
    pub(crate) crts_data_3: [u64; 24],
    pub(crate) crts_data_4: [u64; 29],
    pub(crate) crts_data_5: [u64; 41],
    pub(crate) crts_data_6: [u64; 55],
    pub(crate) crts_data_7: [u64; 71],
    pub(crate) vec_two_pow_tab_backing: [[u64; 4]; 768],
    pub(crate) vec_two_pow_tab_offsets: [usize; MPN_CTX_NCRTS.div_ceil(VEC_SZ)],
    pub(crate) slow_two_pow_backing: [u64; 1 << 11],
    pub(crate) slow_two_pow_offsets: [usize; MPN_CTX_NCRTS],
    pub(crate) profiles: [ProfileEntry; MAX_NPROFILES],
    pub(crate) profiles_size: usize,
    pub(crate) buffer_alloc: usize,
}}

// `CONTEXT`, which is a `const`, as a `static`, so that reading its fields at run time doesn't copy
// all of it.
static SERIALIZED_CONTEXT: SerializedContext = CONTEXT;

const fn f64s_from_bits<const N: usize>(xs: [u64; N]) -> [f64; N] {
    let mut out = [0.0; N];
    let mut i = 0;
    while i < N {
        out[i] = f64::from_bits(xs[i]);
        i += 1;
    }
    out
}

// The tables of `CONTEXT`, converted to floats at compile time. A default context borrows them, so
// that it costs almost nothing to create; only a transform deeper than the tables cover copies the
// one it extends (see `sd_fft_ctx_fit_depth_with_lock`).
static W2TABS: [[f64; 4096]; MPN_CTX_NCRTS] = {
    let mut tabs = [[0.0; 4096]; MPN_CTX_NCRTS];
    let mut i = 0;
    while i < MPN_CTX_NCRTS {
        tabs[i] = f64s_from_bits(CONTEXT.ffts[i].w2tab_backing);
        i += 1;
    }
    tabs
};

static VEC_TWO_POW_TAB: [f64x4; 768] = {
    let bits = CONTEXT.vec_two_pow_tab_backing;
    let mut tab = [f64x4::ZERO; 768];
    let mut i = 0;
    while i < 768 {
        tab[i] = f64x4::new(f64s_from_bits(bits[i]));
        i += 1;
    }
    tab
};

static SLOW_TWO_POW: [f64; 1 << 11] = f64s_from_bits(CONTEXT.slow_two_pow_backing);

// The default context, FLINT's `default_mpn_ctx`, whose tables are borrowed from the statics above.
crate_test_fn! {default_context() -> Context {
    let s = &SERIALIZED_CONTEXT;
    let crts_data: [&[u64]; MPN_CTX_NCRTS] = [
        &s.crts_data_0,
        &s.crts_data_1,
        &s.crts_data_2,
        &s.crts_data_3,
        &s.crts_data_4,
        &s.crts_data_5,
        &s.crts_data_6,
        &s.crts_data_7,
    ];
    Context {
        ffts: core::array::from_fn(|i| {
            let f = &s.ffts[i];
            FFTContext {
                p: f64::from_bits(f.p),
                pinv: f64::from_bits(f.pinv),
                mod_data: f.mod_data.clone(),
                primitive_root: f.primitive_root,
                w2tab_depth: f.w2tab_depth,
                w2tab_backing: Cow::Borrowed(&W2TABS[i]),
                w2tab_offsets: f.w2tab_offsets,
            }
        }),
        crts: core::array::from_fn(|i| {
            let c = &s.crts[i];
            CRTData {
                prime: c.prime,
                coeff_len: c.coeff_len,
                nprimes: c.nprimes,
                data: crts_data[i].to_vec(),
            }
        }),
        vec_two_pow_tab_backing: Cow::Borrowed(&VEC_TWO_POW_TAB),
        vec_two_pow_tab_offsets: s.vec_two_pow_tab_offsets,
        slow_two_pow_backing: Cow::Borrowed(&SLOW_TWO_POW),
        slow_two_pow_offsets: s.slow_two_pow_offsets,
        profiles: s.profiles.clone(),
        profiles_size: s.profiles_size,
        buffer: Vec::new(),
        buffer_alloc: s.buffer_alloc,
    }
}}

// This is mpn_ctx_struct from fft_small.h, FLINT 3.3.0-dev.
crate_test_struct! {
    #[derive(Debug, Default, Clone, PartialEq)]
Context {
    pub(crate) ffts: [FFTContext; MPN_CTX_NCRTS],
    pub(crate) crts: [CRTData; MPN_CTX_NCRTS],
    pub(crate) vec_two_pow_tab_backing: Cow<'static, [f64x4]>,
    pub(crate) vec_two_pow_tab_offsets: [usize; MPN_CTX_NCRTS.div_ceil(VEC_SZ)],
    pub(crate) slow_two_pow_backing: Cow<'static, [f64]>,
    pub(crate) slow_two_pow_offsets: [usize; MPN_CTX_NCRTS],
    pub(crate) profiles: [ProfileEntry; MAX_NPROFILES],
    pub(crate) profiles_size: usize,
    pub(crate) buffer: Vec<f64>,
    pub(crate) buffer_alloc: usize,
}}

// This is vec1d_reduce_pm1n_to_pmhn from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_pm1n_to_pmhn {
    ($a: expr, $n: expr) => {{
        let a = $a;
        let n = $n;
        let halfn = 0.5 * n;
        if a > halfn {
            a - n
        } else {
            let t = a + n;
            if t < halfn { t } else { a }
        }
    }};
}

#[cfg(feature = "test_build")]
pub(crate) use f64_reduce_pm1n_to_pmhn;

// [0,n] -> [-n/2, n/2]
//
// This is f64_reduce_0n_to_pmhn from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_0n_to_pmhn {
    ($a: expr, $n: expr) => {{
        let a = $a;
        let n = $n;
        if a > 0.5 * n { a - n } else { a }
    }};
}

#[cfg(feature = "test_build")]
pub(crate) use f64_reduce_0n_to_pmhn;

// return a mod n in [0,n) assuming a in (-n,n)
//
// This is vec1d_reduce_pm1no_to_0n from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_pm1no_to_0n {
    ($a: expr, $n: expr) => {{
        let a = $a;
        if a >= 0.0 { a } else { a + $n }
    }};
}

// return a mod n in (-n,n)
//
// This is vec4d_reduce_to_pm1no from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_to_pm1no {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        f64x4_mul_add!(-f64x4_round!(a * $ninv), $n, a)
    }};
}

// return a mod n in (-n,n)
//
// This is vec1d_reduce_to_pm1no from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_to_pm1no {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        fma!(-round_even!(a * $ninv), $n, a)
    }};
}

// return a mod n in [0,n)
//
// This is vec4d_reduce_to_0n from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_to_0n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let n = $n;
        f64x4_reduce_pm1no_to_0n!(f64x4_reduce_to_pm1no!($a, n, $ninv), n)
    }};
}

// return a mod n in [0,n)
//
// This is vec1d_reduce_to_0n from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_to_0n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let n = $n;
        f64_reduce_pm1no_to_0n!(f64_reduce_to_pm1no!($a, n, $ninv), n)
    }};
}

// This is vec1d_mulmod from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_mulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        fma!(-round_even!(h * $ninv), $n, h) - fma!(-a, b, h)
    }};
}

#[cfg(feature = "test_build")]
pub(crate) use f64_mulmod;

// This is vec4d_mulmod from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_mulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x4_mul_add!(-f64x4_round!(h * $ninv), $n, h) - f64x4_mul_add!(-a, b, h)
    }};
}

// This is vec8d_mulmod from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x8_mulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x8_mul_add!(-f64x8_round!(h * $ninv), $n, h) - f64x8_mul_add!(-a, b, h)
    }};
}

// This is vec4d_nmulmod from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_nmulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x4_mul_add!(-a, b, h) - f64x4_mul_add!(-f64x4_round!(h * $ninv), $n, h)
    }};
}

// This is vec4d_nmulmod from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x8_nmulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x8_mul_add!(-a, b, h) - f64x8_mul_add!(-f64x8_round!(h * $ninv), $n, h)
    }};
}

// This is vec1d_reduce_to_pm1n from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_to_pm1n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        fma!(-round_even!(a * $ninv), $n, a)
    }};
}

// This is vec4d_reduce_to_pm1n from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_to_pm1n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        f64x4_mul_add!(-f64x4_round!(a * $ninv), $n, a)
    }};
}

// This is vec8d_reduce_to_pm1n from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x8_reduce_to_pm1n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        f64x8_mul_add!(-f64x8_round!(a * $ninv), $n, a)
    }};
}

// need  ceil(64 * bn / bits) <= prod_primes / 2 ^ (2 * bits) i.e. (64 * bn + bits - 1) / bits <=
// prod_primes / 2 ^ (2 * bits) 64 * bn <= bits * prod_primes / 2 ^ (2 * bits) - (bits - 1)
//
// This is crt_data_find_bn_bound from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
pub(crate) fn crt_data_find_bn_bound(c: &CRTData, bits: u64) -> usize {
    let two_bits = bits << 1;
    let q = usize::exact_from(two_bits >> u64::LOG_WIDTH);
    let r = two_bits & u64::WIDTH_MASK;
    let n = c.coeff_len;
    let n_p1 = n + 1;
    let mut xs = [0; 8];
    let xs = &mut xs[..n_p1];
    xs[n] = limbs_mul_limb_to_out::<u128, u64>(xs, &c.prod_primes_ref()[..n], bits);
    let mut bound = 0;
    if q < n_p1 {
        let xs_hi = &mut xs[q..];
        if r != 0 {
            limbs_slice_shr_in_place::<u64>(xs_hi, r);
        }
        if !limbs_sub_limb_in_place::<u64>(xs_hi, bits - 1) {
            limbs_slice_shr_in_place::<u64>(xs_hi, 6);
            bound = usize::exact_from(xs_hi[0]);
            if !slice_test_zero(&xs_hi[1..]) {
                return usize::ONE.wrapping_neg();
            }
        }
    }
    bound
}

// need ceil(64*bn/bits) <= prod_primes/2^(2*bits). first try bits = (nbits(prod_primes) -
// nbits(bn))/2 then adjust. also require bits > 64 for some applications below
//
// This is crt_data_find_bits from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn crt_data_find_bits(c: &CRTData, bn: usize) -> u64 {
    let p_nbits = limbs_significant_bits(&c.prod_primes_ref()[..c.coeff_len]);
    let mut bits = max(66, (p_nbits - bn.significant_bits()) >> 1);
    if bn > crt_data_find_bn_bound(c, bits) {
        bits -= 1;
        while bits > 65 && bn > crt_data_find_bn_bound(c, bits) {
            bits -= 1;
        }
    } else {
        while bn <= crt_data_find_bn_bound(c, bits + 1) {
            bits += 1;
        }
    }
    bits
}

#[cfg(feature = "32_bit_limbs")]
macro_rules! get {
    ($a: ident, $i: expr) => {
        $a[$i]
    };
}

#[cfg(not(feature = "32_bit_limbs"))]
macro_rules! get {
    ($a: ident, $i: expr) => {{
        let i = $i;
        let x = $a[i >> 1];
        if i & 1 == 0 {
            x.lower_half()
        } else {
            x.upper_half()
        }
    }};
}

// This is CODE from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
macro_rules! code {
    (
        $ir: expr,
        $nvs: expr,
        $np: expr,
        $ds: ident,
        $dstride: ident,
        $xs: ident,
        $ps: ident,
        $pinvs: ident,
        $two_pow: ident,
        $a: ident,
        $i: ident,
        $bits: expr
    ) => {
        let mut k = (($i + $ir) * $bits) / 32;
        let mut j = const { ($ir * $bits) % 32 };
        $xs[..$nvs].fill(f64x4::splat(f64::from(get!($a, k) >> j)));
        k += 1;
        j = const { 32 - ($ir * $bits) % 32 };
        let mut m = const { (32 - ($ir * $bits) % 32) * $nvs };
        while j <= const { $bits - 32 } {
            let ak = f64x4::splat(f64::from(get!($a, k)));
            let two_pow_hi = &$two_pow[m..];
            for l in 0..$nvs {
                $xs[l] += f64x4_mulmod!(ak, two_pow_hi[l], $ps[l], $pinvs[l]);
            }
            k += 1;
            j += 32;
            m += const { $nvs << 5 };
        }
        let bmj = $bits - j;
        if bmj != 0 {
            let ak = f64x4::splat(f64::from(get!($a, k) << (32 - bmj)));
            let two_pow_hi = &$two_pow[const { ($bits - 32) * $nvs }..];
            for l in 0..$nvs {
                $xs[l] += f64x4_mulmod!(ak, two_pow_hi[l], $ps[l], $pinvs[l]);
            }
        }
        for l in 0..$nvs {
            $xs[l] = f64x4_reduce_to_pm1n!($xs[l], $ps[l], $pinvs[l]);
        }
        let ds_hi = &mut $ds[$i + $ir..];
        let mut m = 0;
        for l in 0..$np {
            ds_hi[m] = $xs[l >> 2].to_array()[l & 3];
            m += $dstride;
        }
    };
}

#[cfg(feature = "32_bit_limbs")]
macro_rules! get_or_default {
    ($a: ident, $i: expr) => {
        $a.get($i).copied().unwrap_or_default()
    };
}

#[cfg(not(feature = "32_bit_limbs"))]
macro_rules! get_or_default {
    ($a: ident, $i: expr) => {{
        let i = $i;
        let j = i >> 1;
        if j < $a.len() {
            let x = $a[j];
            if i & 1 == 0 {
                x.lower_half()
            } else {
                x.upper_half()
            }
        } else {
            0
        }
    }};
}

// This is mpn_to_ffts_hard from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
macro_rules! mpn_to_ffts_hard {
    (
        $np: expr,
        $nvs: expr,
        $rffts: ident,
        $ds: ident,
        $dstride: ident,
        $a: ident,
        $atrunc: ident,
        $two_pow: ident,
        $start_hard: ident,
        $stop_hard: ident,
        $bits: expr
    ) => {{
        let mut xs = [f64x4::default(); $nvs];
        let mut ps = [f64x4::default(); $nvs];
        let mut pinvs = [f64x4::default(); $nvs];
        // Only the first `nvs` chunks; see the identical loop in apply_mpn_to_fft_func.
        for (i, r) in $rffts.chunks(4).take($nvs).enumerate() {
            ps[i] = f64x4::new([r[0].p, r[1].p, r[2].p, r[3].p]);
            pinvs[i] = f64x4::new([r[0].pinv, r[1].pinv, r[2].pinv, r[3].pinv]);
        }
        for i in $start_hard..$stop_hard {
            let ib = i * $bits;
            let mut k = ib >> 5;
            let mut j = ib & 31;
            xs[..$nvs].fill(f64x4::splat(f64::from(get_or_default!($a, k) >> j)));
            k += 1;
            j = 32 - j;
            let bm32 = $bits - 32;
            while j <= bm32 {
                let ak = f64x4::splat(f64::from(get_or_default!($a, k)));
                let two_pow_hi = &$two_pow[j * $nvs..];
                for l in 0..$nvs {
                    xs[l] += f64x4_mulmod!(ak, two_pow_hi[l], ps[l], pinvs[l]);
                }
                k += 1;
                j += 32;
            }
            let bmj = $bits - j;
            if bmj != 0 {
                let ak = f64x4::splat(f64::from(get_or_default!($a, k) << (32 - bmj)));
                let two_pow_hi = &$two_pow[bm32 * $nvs..];
                for l in 0..$nvs {
                    xs[l] += f64x4_mulmod!(ak, two_pow_hi[l], ps[l], pinvs[l]);
                }
            }
            for l in 0..$nvs {
                xs[l] = f64x4_reduce_to_pm1n!(xs[l], ps[l], pinvs[l]);
            }
            let ds_hi = &mut $ds[i..];
            let mut m = 0;
            for l in 0..$np {
                ds_hi[m] = xs[l / VEC_SZ].to_array()[l % VEC_SZ];
                m += $dstride;
            }
        }
        for l in 0..$np {
            $ds[l * $dstride..][$stop_hard..$atrunc].fill(0.0);
        }
    }};
}

// The tables for powers of two each have this fixed length. This has to go up linearly with the max
// number of primes MPN_CTX_NCRTS involved in chinese remaindering. This length is checked with
// asserts in the code.
pub(crate) const MPN_CTX_TWO_POWER_TAB_SIZE: usize = 256;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct MPNToFFTFunc {
    pub(crate) np: usize,
    pub(crate) bits: u64,
}

const LG_BLK_SZ: u64 = 8;

const BLK_SZ: usize = 256;

const BLK_SZ_DIV_8: usize = BLK_SZ >> 3;

// This is mpn_ctx_best_profile from fft_small/mpn_mul.c, FLINT 3.3.0-dev, returning the context.
fn mpn_ctx_best_profile(r: &Context, p: &mut ProfileEntry, an: usize, bn: usize) {
    // The first profile is supposed to have the biggest bn_bound. If the given bn is too large, we
    // must fill in p.to_ffts = None because we don't have a fast mod function.
    //
    // We can also fill in p.to_ffts = None any time to not use the fast mod function and use the
    // slow generic one instead.
    const BIGGEST_BOUND: usize = CONTEXT.profiles[0].bn_bound;
    const PROFILES_SIZE: usize = CONTEXT.profiles_size;
    if bn > BIGGEST_BOUND {
        p.np = 4;
        p.bits = crt_data_find_bits(&r.crts[p.np - 1], bn);
        p.to_ffts = None;
        return;
    }
    let mut i = 0;
    let mut best_i = 0;
    let mut best_score = 100000000.0 * (an.checked_add(bn).unwrap() as f64);
    loop {
        // maximize r.profiles[i].bits
        assert!(i < PROFILES_SIZE);
        assert!(bn <= r.profiles[i].bn_bound);
        while i + 1 < PROFILES_SIZE
            && bn <= r.profiles[i + 1].bn_bound
            && r.profiles[i + 1].np == r.profiles[i].np
        {
            i += 1;
        }
        let np = r.profiles[i].np;
        let bits = r.profiles[i].bits as usize;
        let alen = (an << 6).div_round(bits, Ceiling).0;
        let blen = (bn << 6).div_round(bits, Ceiling).0;
        let zlen = alen + blen - 1;
        let ztrunc = zlen.round_to_multiple(BLK_SZ, Ceiling).0;
        let depth = max(LG_BLK_SZ, ztrunc.ceiling_log_base_2());
        let ratio = (ztrunc as f64) / (f64::power_of_2(depth));
        let mut score = (1.0 - 0.25 * ratio) * const { 1.0 / 1000000.0 };
        score *= (np * usize::exact_from(depth)) as f64;
        score *= ztrunc as f64;
        if score < best_score {
            best_i = i;
            best_score = score;
        }
        loop {
            i += 1;
            if i >= PROFILES_SIZE {
                p.np = r.profiles[best_i].np;
                p.bits = r.profiles[best_i].bits;
                p.to_ffts = r.profiles[best_i].to_ffts;
                return;
            }
            if bn <= r.profiles[i].bn_bound {
                break;
            }
        }
    }
}

// This is mpn_ctx_fit_buffer from fft_small/mpn_mul.c, FLINT 3.3.0-dev, not returning the buffer.
fn mpn_ctx_fit_buffer(r: &mut Context, n: usize) {
    if n > r.buffer_alloc {
        let n = max(n, (r.buffer_alloc * 17) >> 4)
            .round_to_multiple_of_power_of_2(12, Ceiling)
            .0;
        r.buffer.resize(n >> 3, 0.0);
        r.buffer_alloc = n;
    }
}

// This is n_nbits_nz from fft_small.h, FLINT 3.3.0-dev.
macro_rules! n_nbits_nz {
    ($x: expr) => {{ (LeadingZeros::leading_zeros($x) ^ const { usize::WIDTH - 1 }) + 1 }};
}

// for the fft look up of powers of w
//
// This is SET_J_BITS_AND_J_R from fft_small.h, FLINT 3.3.0-dev, returning j_bits and j_r.
macro_rules! set_j_bits_and_j_r {
    ($j: ident, $j_bits: ident, $j_r: ident) => {
        let ($j_bits, $j_r) = if $j == 0 {
            (0, 0)
        } else {
            let j_bits = n_nbits_nz!($j);
            (j_bits, $j - usize::power_of_2(j_bits - 1))
        };
    };
}

// This is  RADIX_4_FORWARD_PARAM_J_IS_Z from fft_small/sd_fft.c, FLINT 3.3.0-dev, returning iw, n,
// and ninv.
macro_rules! radix_4_forward_param_j_is_z {
    ($q: ident, $iw: ident, $n: ident, $ninv: ident) => {
        let $iw = f64x8::splat(w2tab!($q, 1, 0));
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_4_FORWARD_MOTH_J_IS_Z from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! radix_4_forward_moth_j_is_z {
    (
    $x0: ident,
    $x1: ident,
    $x2: ident,
    $x3: ident,
    $iw: ident,
    $n: ident,
    $ninv: ident
) => {
        $x0 = f64x8_reduce_to_pm1n!($x0, $n, $ninv);
        $x2 = f64x8_reduce_to_pm1n!($x2, $n, $ninv);
        $x3 = f64x8_reduce_to_pm1n!($x3, $n, $ninv);
        let y0 = $x0 + $x2;
        let y1 = f64x8_reduce_to_pm1n!($x1 + $x3, $n, $ninv);
        let y2 = $x0 - $x2;
        let y3 = f64x8_mulmod!($x1 - $x3, $iw, $n, $ninv);
        $x0 = y0 + y1;
        $x1 = y0 - y1;
        $x2 = y2 + y3;
        $x3 = y2 - y3;
    };
}

// This is RADIX_4_FORWARD_PARAM_J_IS_NZ from fft_small/sd_fft.c, FLINT 3.3.0-dev, returning w, w2,
// iw, n, and ninv.
macro_rules! radix_4_forward_param_j_is_nz {
    (
    $q: ident,
    $j_r: ident,
    $j_bits: ident,
    $w: ident,
    $w2: ident,
    $iw: ident,
    $n: ident,
    $ninv: ident
) => {
        assert_ne!($j_bits, 0);
        let j_bits = usize::exact_from($j_bits);
        let j_bits_p_1 = j_bits + 1;
        let j_2 = $j_r << 1;
        let $w = f64x8::splat(w2tab!($q, j_bits_p_1, j_2));
        let $w2 = f64x8::splat(w2tab!($q, j_bits, $j_r));
        let $iw = f64x8::splat(w2tab!($q, j_bits_p_1, j_2 + 1));
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_4_FORWARD_MOTH_J_IS_NZ from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! radix_4_forward_moth_j_is_nz {
    (
    $x0: ident,
    $x1: ident,
    $x2: ident,
    $x3: ident,
    $w: ident,
    $w2: ident,
    $iw: ident,
    $n: ident,
    $ninv: ident
) => {
        $x0 = f64x8_reduce_to_pm1n!($x0, $n, $ninv);
        $x2 = f64x8_mulmod!($x2, $w2, $n, $ninv);
        $x3 = f64x8_mulmod!($x3, $w2, $n, $ninv);
        let y0 = $x0 + $x2;
        let y1 = f64x8_mulmod!($x1 + $x3, $w, $n, $ninv);
        let y2 = $x0 - $x2;
        let y3 = f64x8_mulmod!($x1 - $x3, $iw, $n, $ninv);
        $x0 = y0 + y1;
        $x1 = y0 - y1;
        $x2 = y2 + y3;
        $x3 = y2 - y3;
    };
}

// This is RADIX_2_FORWARD_PARAM_J_IS_Z from fft_small/sd_fft.c, FLINT 3.3.0-dev, returning n and
// ninv.
macro_rules! radix_2_forward_param_j_is_z {
    ($q: ident, $n: ident, $ninv: ident) => {
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_2_FORWARD_MOTH_J_IS_Z from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! radix_2_forward_moth_j_is_z {
    ($x0: ident, $x1: ident, $n: ident, $ninv: ident) => {
        $x0 = f64x8_reduce_to_pm1n!($x0, $n, $ninv);
        $x1 = f64x8_reduce_to_pm1n!($x1, $n, $ninv);
        ($x0, $x1) = ($x0 + $x1, $x0 - $x1);
    };
}

// This is RADIX_2_FORWARD_PARAM_J_IS_NZ from fft_small/sd_fft.c, FLINT 3.3.0-dev, returning w, n,
// and ninv.
macro_rules! radix_2_forward_param_j_is_nz {
    ($q: ident, $j_r: ident, $j_bits: ident, $w: ident, $n: ident, $ninv: ident) => {
        let $w = f64x8::splat(w2tab!($q, usize::exact_from($j_bits), $j_r));
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_2_FORWARD_MOTH_J_IS_NZ from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! radix_2_forward_moth_j_is_nz {
    ($x0: ident, $x1: ident, $w: ident, $n: ident, $ninv: ident) => {
        $x0 = f64x8_reduce_to_pm1n!($x0, $n, $ninv);
        $x1 = f64x8_mulmod!($x1, $w, $n, $ninv);
        ($x0, $x1) = ($x0 + $x1, $x0 - $x1);
    };
}

const BIG_N: usize = 8;

macro_rules! read_f64x4_w2tab {
    ($q: expr, $i: expr, $j: expr) => {
        read_f64x4!($q.w2tab_backing, $q.w2tab_offsets[$i] + $j)
    };
}

macro_rules! read_f64x8_w2tab {
    ($q: expr, $i: expr, $j: expr) => {
        read_f64x8!($q.w2tab_backing, $q.w2tab_offsets[$i] + $j)
    };
}

macro_rules! write_f64x8_w2tab {
    ($q: expr, $i: expr, $j: expr, $f: expr) => {
        let start = $q.w2tab_offsets[$i] + $j;
        $q.w2tab_backing.to_mut()[start..start + 8].copy_from_slice(&$f.to_array());
    };
}

macro_rules! process_2_2 {
    ($f: ident, $limit: expr, $x0: ident, $x1: ident, $p0: ident, $p1: ident) => {
        for i in (0..$limit).step_by(BIG_N) {
            let mut f0 = read_f64x8!($x0, i);
            let mut f1 = read_f64x8!($x1, i);
            $f!(f0, f1, $p0, $p1);
            write_f64x8!($x0, i, f0);
            write_f64x8!($x1, i, f1);
        }
    };
}

macro_rules! process_2_3 {
    ($f: ident, $limit: expr, $x0: ident, $x1: ident, $p0: ident, $p1: ident, $p2: ident) => {
        for i in (0..$limit).step_by(BIG_N) {
            let mut f0 = read_f64x8!($x0, i);
            let mut f1 = read_f64x8!($x1, i);
            $f!(f0, f1, $p0, $p1, $p2);
            write_f64x8!($x0, i, f0);
            write_f64x8!($x1, i, f1);
        }
    };
}

macro_rules! process_4_3 {
    (
        $f: ident,
        $limit: expr,
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $p0: ident,
        $p1: ident,
        $p2: ident
    ) => {
        for i in (0..$limit).step_by(BIG_N) {
            let mut f0 = read_f64x8!($x0, i);
            let mut f1 = read_f64x8!($x1, i);
            let mut f2 = read_f64x8!($x2, i);
            let mut f3 = read_f64x8!($x3, i);
            $f!(f0, f1, f2, f3, $p0, $p1, $p2);
            write_f64x8!($x0, i, f0);
            write_f64x8!($x1, i, f1);
            write_f64x8!($x2, i, f2);
            write_f64x8!($x3, i, f3);
        }
    };
}

macro_rules! process_4_5 {
    (
        $f: ident,
        $limit: expr,
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $p0: ident,
        $p1: ident,
        $p2: ident,
        $p3: ident,
        $p4: ident
    ) => {
        for i in (0..$limit).step_by(BIG_N) {
            let mut f0 = read_f64x8!($x0, i);
            let mut f1 = read_f64x8!($x1, i);
            let mut f2 = read_f64x8!($x2, i);
            let mut f3 = read_f64x8!($x3, i);
            $f!(f0, f1, f2, f3, $p0, $p1, $p2, $p3, $p4);
            write_f64x8!($x0, i, f0);
            write_f64x8!($x1, i, f1);
            write_f64x8!($x2, i, f2);
            write_f64x8!($x3, i, f3);
        }
    };
}

// This is LENGTH4_ZERO_J from fft_small/sd_fft.c, FLINT 3.3.0-dev, for vec1d.
macro_rules! f64_length4_zero_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $n: expr,
        $ninv: expr,
        $e14: expr
    ) => {
        let n = $n;
        let ninv = $ninv;
        *$x0 = f64_reduce_to_pm1n!(*$x0, n, ninv);
        *$x2 = f64_reduce_to_pm1n!(*$x2, n, ninv);
        *$x3 = f64_reduce_to_pm1n!(*$x3, n, ninv);
        let y0 = *$x0 + *$x2;
        let y1 = f64_reduce_to_pm1n!(*$x1 + *$x3, n, ninv);
        let y2 = *$x0 - *$x2;
        let y3 = f64_mulmod!(*$x1 - *$x3, $e14, n, ninv);
        *$x0 = y0 + y1;
        *$x1 = y0 - y1;
        *$x2 = y2 + y3;
        *$x3 = y2 - y3;
    };
}

// This is LENGTH4_ZERO_J from fft_small/sd_fft.c, FLINT 3.3.0-dev, for vec4d.
macro_rules! f64x4_length4_zero_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $n: ident,
        $ninv: ident,
        $e14: ident
    ) => {
        $x0 = f64x4_reduce_to_pm1n!($x0, $n, $ninv);
        $x2 = f64x4_reduce_to_pm1n!($x2, $n, $ninv);
        $x3 = f64x4_reduce_to_pm1n!($x3, $n, $ninv);
        let y0 = $x0 + $x2;
        let y1 = f64x4_reduce_to_pm1n!($x1 + $x3, $n, $ninv);
        let y2 = $x0 - $x2;
        let y3 = f64x4_mulmod!($x1 - $x3, $e14, $n, $ninv);
        $x0 = y0 + y1;
        $x1 = y0 - y1;
        $x2 = y2 + y3;
        $x3 = y2 - y3;
    };
}

// This is LENGTH4_ANY_J from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! length4_any_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $n: ident,
        $ninv: ident,
        $w2: ident,
        $w: ident,
        $iw: ident
    ) => {
        $x0 = f64x4_reduce_to_pm1n!($x0, $n, $ninv);
        $x2 = f64x4_mulmod!($x2, $w2, $n, $ninv);
        $x3 = f64x4_mulmod!($x3, $w2, $n, $ninv);
        let y0 = $x0 + $x2;
        let y1 = f64x4_mulmod!($x1 + $x3, $w, $n, $ninv);
        let y2 = $x0 - $x2;
        let y3 = f64x4_mulmod!($x1 - $x3, $iw, $n, $ninv);
        $x0 = y0 + y1;
        $x1 = y0 - y1;
        $x2 = y2 + y3;
        $x3 = y2 - y3;
    };
}

// This is LENGTH8_ZERO_J from fft_small/sd_fft.c, FLINT 3.3.0-dev, for vec1d.
macro_rules! f64_length8_zero_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $x4: ident,
        $x5: ident,
        $x6: ident,
        $x7: ident,
        $n: expr,
        $ninv: expr,
        $e14: expr,
        $e18: expr,
        $e38: expr
    ) => {
        let n = $n;
        let ninv = $ninv;
        let e14 = $e14;
        let y0 = f64_reduce_to_pm1n!(*$x0 + *$x4, n, ninv);
        let y1 = f64_reduce_to_pm1n!(*$x1 + *$x5, n, ninv);
        let y2 = f64_reduce_to_pm1n!(*$x2 + *$x6, n, ninv);
        let y3 = f64_reduce_to_pm1n!(*$x3 + *$x7, n, ninv);
        let y4 = f64_reduce_to_pm1n!(*$x0 - *$x4, n, ninv);
        let y5 = f64_reduce_to_pm1n!(*$x1 - *$x5, n, ninv);
        let y6 = f64_mulmod!(e14, f64_reduce_to_pm1n!(*$x2 - *$x6, n, ninv), n, ninv);
        let y7 = f64_mulmod!(e14, f64_reduce_to_pm1n!(*$x3 - *$x7, n, ninv), n, ninv);
        let z0 = y0 + y2;
        let z1 = y1 + y3;
        let z2 = y0 - y2;
        let z3 = f64_mulmod!(e14, y1 - y3, n, ninv);
        let z4 = y4 + y6;
        let z5 = f64_mulmod!($e18, y5 + y7, n, ninv);
        let z6 = y4 - y6;
        let z7 = f64_mulmod!($e38, y5 - y7, n, ninv);
        *$x0 = z0 + z1;
        *$x1 = z0 - z1;
        *$x2 = z2 + z3;
        *$x3 = z2 - z3;
        *$x4 = z4 + z5;
        *$x5 = z4 - z5;
        *$x6 = z6 + z7;
        *$x7 = z6 - z7;
    };
}

// This is LENGTH8_ZERO_J from fft_small/sd_fft.c, FLINT 3.3.0-dev, for vec4d.
macro_rules! f64x4_length8_zero_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $x4: ident,
        $x5: ident,
        $x6: ident,
        $x7: ident,
        $n: ident,
        $ninv: ident,
        $e14: ident,
        $e18: ident,
        $e38: ident
    ) => {
        let y0 = f64x4_reduce_to_pm1n!($x0 + $x4, $n, $ninv);
        let y1 = f64x4_reduce_to_pm1n!($x1 + $x5, $n, $ninv);
        let y2 = f64x4_reduce_to_pm1n!($x2 + $x6, $n, $ninv);
        let y3 = f64x4_reduce_to_pm1n!($x3 + $x7, $n, $ninv);
        let y4 = f64x4_reduce_to_pm1n!($x0 - $x4, $n, $ninv);
        let y5 = f64x4_reduce_to_pm1n!($x1 - $x5, $n, $ninv);
        let y6 = f64x4_mulmod!($e14, f64x4_reduce_to_pm1n!($x2 - $x6, $n, $ninv), $n, $ninv);
        let y7 = f64x4_mulmod!($e14, f64x4_reduce_to_pm1n!($x3 - $x7, $n, $ninv), $n, $ninv);
        let z0 = y0 + y2;
        let z1 = y1 + y3;
        let z2 = y0 - y2;
        let z3 = f64x4_mulmod!($e14, y1 - y3, $n, $ninv);
        let z4 = y4 + y6;
        let z5 = f64x4_mulmod!($e18, y5 + y7, $n, $ninv);
        let z6 = y4 - y6;
        let z7 = f64x4_mulmod!($e38, y5 - y7, $n, $ninv);
        $x0 = z0 + z1;
        $x1 = z0 - z1;
        $x2 = z2 + z3;
        $x3 = z2 - z3;
        $x4 = z4 + z5;
        $x5 = z4 - z5;
        $x6 = z6 + z7;
        $x7 = z6 - z7;
    };
}

// This is LENGTH8_ANY_J from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! length8_any_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $x4: ident,
        $x5: ident,
        $x6: ident,
        $x7: ident,
        $n: ident,
        $ninv: ident,
        $w2: ident,
        $w: ident,
        $iw: ident,
        $ww0: ident,
        $ww1: ident,
        $ww2: ident,
        $ww3: ident
    ) => {
        $x0 = f64x4_reduce_to_pm1n!($x0, $n, $ninv);
        $x1 = f64x4_reduce_to_pm1n!($x1, $n, $ninv);
        $x4 = f64x4_mulmod!($x4, $w2, $n, $ninv);
        $x5 = f64x4_mulmod!($x5, $w2, $n, $ninv);
        $x6 = f64x4_mulmod!($x6, $w2, $n, $ninv);
        $x7 = f64x4_mulmod!($x7, $w2, $n, $ninv);
        let y0 = $x0 + $x4;
        let y1 = $x1 + $x5;
        let y2 = f64x4_mulmod!($x2 + $x6, $w, $n, $ninv);
        let y3 = f64x4_mulmod!($x3 + $x7, $w, $n, $ninv);
        let y4 = $x0 - $x4;
        let y5 = $x1 - $x5;
        let y6 = f64x4_mulmod!($x2 - $x6, $iw, $n, $ninv);
        let y7 = f64x4_mulmod!($x3 - $x7, $iw, $n, $ninv);
        let z0 = f64x4_reduce_to_pm1n!(y0 + y2, $n, $ninv);
        let z1 = f64x4_mulmod!(y1 + y3, $ww0, $n, $ninv);
        let z2 = f64x4_reduce_to_pm1n!(y0 - y2, $n, $ninv);
        let z3 = f64x4_mulmod!(y1 - y3, $ww1, $n, $ninv);
        let z4 = f64x4_reduce_to_pm1n!(y4 + y6, $n, $ninv);
        let z5 = f64x4_mulmod!(y5 + y7, $ww2, $n, $ninv);
        let z6 = f64x4_reduce_to_pm1n!(y4 - y6, $n, $ninv);
        let z7 = f64x4_mulmod!(y5 - y7, $ww3, $n, $ninv);
        $x0 = z0 + z1;
        $x1 = z0 - z1;
        $x2 = z2 + z3;
        $x3 = z2 - z3;
        $x4 = z4 + z5;
        $x5 = z4 - z5;
        $x6 = z6 + z7;
        $x7 = z6 - z7;
    };
}

// This is RADIX_2_FORWARD_MOTH_TRUNC_2_1_J_IS_Z from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! radix_2_forward_moth_trunc_2_1_j_is_z {
    ($x0: ident, $x1: ident, $n: ident, $ninv: ident) => {
        $x0 = f64x8_reduce_to_pm1n!($x0, $n, $ninv);
        $x1 = f64x8_reduce_to_pm1n!($x1, $n, $ninv);
        $x0 += $x1;
    };
}

// This is RADIX_2_FORWARD_MOTH_TRUNC_2_1_J_IS_NZ from fft_small/sd_fft.c, FLINT 3.3.0-dev.
macro_rules! radix_2_forward_moth_trunc_2_1_j_is_nz {
    ($x0: ident, $x1: ident, $w: ident, $n: ident, $ninv: ident) => {
        $x0 = f64x8_reduce_to_pm1n!($x0, $n, $ninv);
        $x1 = f64x8_mulmod!($x1, $w, $n, $ninv);
        $x0 += $x1;
    };
}

// for the ifft look up of powers of w^-1: the remainder has to be flipped
//
// This is SET_J_BITS_AND_J_MR from fft_small.h, FLINT 3.3.0-dev, returning j_bits and j_mr.
macro_rules! set_j_bits_and_j_mr {
    ($j: ident, $j_bits: ident, $j_mr: ident) => {
        let ($j_bits, $j_mr) = if $j == 0 {
            (0, 0)
        } else {
            let j_bits = n_nbits_nz!($j);
            (j_bits, usize::power_of_2(j_bits) - 1 - $j)
        };
    };
}

// This is RADIX_4_REVERSE_PARAM_J_IS_Z from fft_small/sd_ifft.c, FLINT 3.3.0-dev, returning iw, n,
// and ninv.
macro_rules! radix_4_reverse_param_j_is_z {
    ($q: ident, $iw: ident, $n: ident, $ninv: ident) => {
        let $iw = f64x8::splat(w2tab!($q, 0, 1));
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_4_REVERSE_PARAM_J_IS_NZ from fft_small/sd_ifft.c, FLINT 3.3.0-dev, returning w, w2,
// iw, n, and ninv.
macro_rules! radix_4_reverse_param_j_is_nz {
    (
        $q: ident,
        $j_mr: ident,
        $j_bits: ident,
        $w: ident,
        $w2: ident,
        $iw: ident,
        $n: ident,
        $ninv: ident
    ) => {
        let j_bits = usize::exact_from($j_bits);
        let jp1 = j_bits + 1;
        let j_2 = $j_mr << 1;
        let $w = f64x8::splat(w2tab!($q, jp1, j_2 + 1));
        let $w2 = f64x8::splat(w2tab!($q, j_bits, $j_mr));
        let $iw = f64x8::splat(w2tab!($q, jp1, j_2));
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_4_REVERSE_MOTH_J_IS_Z from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! radix_4_reverse_moth_j_is_z {
    ($x0: ident,
     $x1: ident,
     $x2: ident,
     $x3: ident,
     $iw: ident,
     $n: ident,
     $ninv: ident
) => {
        let mut y0 = $x0 + $x1;
        let mut y1 = $x2 + $x3;
        let mut y2 = $x0 - $x1;
        let mut y3 = $x2 - $x3;
        y0 = f64x8_reduce_to_pm1n!(y0, $n, $ninv);
        y1 = f64x8_reduce_to_pm1n!(y1, $n, $ninv);
        y2 = f64x8_reduce_to_pm1n!(y2, $n, $ninv);
        y3 = f64x8_mulmod!(y3, $iw, $n, $ninv);
        $x0 = y0 + y1;
        $x2 = y0 - y1;
        $x1 = y2 - y3;
        $x3 = y2 + y3;
    };
}

// This is RADIX_4_REVERSE_MOTH_J_IS_NZ from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! radix_4_reverse_moth_j_is_nz {
    (
    $x0: ident,
    $x1: ident,
    $x2: ident,
    $x3: ident,
    $w: ident,
    $w2: ident,
    $iw: ident,
    $n: ident,
    $ninv: ident
) => {
        let y0 = $x0 + $x1;
        let y1 = $x2 + $x3;
        let y2 = f64x8_mulmod!($x0 - $x1, $w, $n, $ninv);
        let y3 = f64x8_mulmod!($x3 - $x2, $iw, $n, $ninv);
        $x0 = y0 + y1;
        $x1 = y3 - y2;
        $x2 = y1 - y0;
        $x3 = y3 + y2;
        $x0 = f64x8_reduce_to_pm1n!($x0, $n, $ninv);
        $x2 = f64x8_mulmod!($x2, $w2, $n, $ninv);
        $x3 = f64x8_mulmod!($x3, $w2, $n, $ninv);
    };
}

// This is RADIX_2_REVERSE_PARAM_J_IS_Z from fft_small/sd_ifft.c, FLINT 3.3.0-dev, returning n and
// ninv.
macro_rules! radix_2_reverse_param_j_is_z {
    ($q: ident, $n: ident, $ninv: ident) => {
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_2_REVERSE_PARAM_J_IS_NZ from fft_small/sd_ifft.c, FLINT 3.3.0-dev, returning w, n,
// and ninv.
macro_rules! radix_2_reverse_param_j_is_nz {
    (
        $q: ident,
        $j_mr: ident,
        $j_bits: ident,
        $w: ident,
        $n: ident,
        $ninv: ident
    ) => {
        let $w = f64x8::splat(w2tab!($q, usize::exact_from($j_bits), $j_mr));
        let $n = f64x8::splat($q.p);
        let $ninv = f64x8::splat($q.pinv);
    };
}

// This is RADIX_2_REVERSE_MOTH_J_IS_Z from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! radix_2_reverse_moth_j_is_z {
    ($x0: ident, $x1: ident, $n: ident, $ninv: ident) => {
        let y0 = f64x8_reduce_to_pm1n!($x0 + $x1, $n, $ninv);
        let y1 = f64x8_reduce_to_pm1n!($x0 - $x1, $n, $ninv);
        $x0 = y0;
        $x1 = y1;
    };
}

// This is RADIX_2_REVERSE_MOTH_J_IS_NZ from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! radix_2_reverse_moth_j_is_nz {
    ($x0: ident, $x1: ident, $w: ident, $n: ident, $ninv: ident) => {
        let y0 = f64x8_reduce_to_pm1n!($x0 + $x1, $n, $ninv);
        let y1 = f64x8_mulmod!($x1 - $x0, $w, $n, $ninv);
        $x0 = y0;
        $x1 = y1;
    };
}

// This is LENGTH2INV_ZERO_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64_length2inv_zero_j_mut {
    ($x0: ident, $x1: ident, $n: expr, $ninv: expr) => {
        (*$x0, *$x1) = (
            f64_reduce_to_pm1n!(*$x0 + *$x1, $n, $ninv),
            f64_reduce_to_pm1n!(*$x0 - *$x1, $n, $ninv),
        );
    };
}

// This is LENGTH2INV_ZERO_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64x4_length2inv_zero_j {
    ($x0: ident, $x1: ident, $n: ident, $ninv: ident) => {
        ($x0, $x1) = (
            f64x4_reduce_to_pm1n!($x0 + $x1, $n, $ninv),
            f64x4_reduce_to_pm1n!($x0 - $x1, $n, $ninv),
        );
    };
}

// This is LENGTH2INV_ANY_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64_length2inv_any_j_mut {
    ($x0: ident, $x1: ident, $n: ident, $ninv: ident, $w0: expr) => {
        (*$x0, *$x1) = (
            f64_reduce_to_pm1n!(*$x0 + *$x1, $n, $ninv),
            f64_mulmod!(*$x1 - *$x0, $w0, $n, $ninv),
        );
    };
}

// This is LENGTH2INV_ANY_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64x4_length2inv_any_j {
    ($x0: ident, $x1: ident, $n: ident, $ninv: ident, $w0: ident) => {
        ($x0, $x1) = (
            f64x4_reduce_to_pm1n!($x0 + $x1, $n, $ninv),
            f64x4_mulmod!($x1 - $x0, $w0, $n, $ninv),
        );
    };
}

// This is LENGTH4INV_ZERO_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64_length4inv_zero_j_mut {
    ($x0: ident, $x1: ident, $x2: ident, $x3: ident, $n: expr, $ninv: expr, $ww1: expr) => {
        let n = $n;
        let ninv = $ninv;
        let y0 = f64_reduce_to_pm1n!(*$x0 + *$x1, n, ninv);
        let y1 = f64_reduce_to_pm1n!(*$x2 + *$x3, n, ninv);
        let y2 = f64_reduce_to_pm1n!(*$x0 - *$x1, n, ninv);
        let y3 = f64_mulmod!(*$x2 - *$x3, $ww1, n, ninv);
        *$x0 = y0 + y1;
        *$x2 = y0 - y1;
        *$x1 = y2 - y3;
        *$x3 = y2 + y3;
    };
}

// This is LENGTH4INV_ZERO_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64x4_length4inv_zero_j {
    ($x0: ident, $x1: ident, $x2: ident, $x3: ident, $n: ident, $ninv: ident, $ww1: ident) => {
        let y0 = f64x4_reduce_to_pm1n!($x0 + $x1, $n, $ninv);
        let y1 = f64x4_reduce_to_pm1n!($x2 + $x3, $n, $ninv);
        let y2 = f64x4_reduce_to_pm1n!($x0 - $x1, $n, $ninv);
        let y3 = f64x4_mulmod!($x2 - $x3, $ww1, $n, $ninv);
        $x0 = y0 + y1;
        $x2 = y0 - y1;
        $x1 = y2 - y3;
        $x3 = y2 + y3;
    };
}

// This is LENGTH4INV_ZERO_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64x4_length4inv_any_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $n: expr,
        $ninv: ident,
        $w0: ident,
        $ww0: ident,
        $ww1: ident
    ) => {
        let y0 = $x0 + $x1;
        let y1 = $x2 + $x3;
        let y2 = f64x4_mulmod!($x0 - $x1, $ww0, $n, $ninv);
        let y3 = f64x4_mulmod!($x3 - $x2, $ww1, $n, $ninv);
        $x0 = f64x4_reduce_to_pm1n!(y0 + y1, $n, $ninv);
        $x1 = y3 - y2;
        $x2 = f64x4_mulmod!(y1 - y0, $w0, $n, $ninv);
        $x3 = f64x4_mulmod!(y3 + y2, $w0, $n, $ninv);
    };
}

// This is LENGTH8INV_ZERO_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64_length8inv_zero_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $x4: ident,
        $x5: ident,
        $x6: ident,
        $x7: ident,
        $n: expr,
        $ninv: expr,
        $ww1: expr,
        $www2: expr,
        $www3: expr
    ) => {
        let n = $n;
        let ninv = $ninv;
        let ww1 = $ww1;
        f64_length2inv_zero_j_mut!($x0, $x1, n, ninv);
        f64_length2inv_any_j_mut!($x2, $x3, n, ninv, ww1);
        f64_length2inv_any_j_mut!($x4, $x5, n, ninv, $www2);
        f64_length2inv_any_j_mut!($x6, $x7, n, ninv, $www3);
        f64_length4inv_zero_j_mut!($x0, $x2, $x4, $x6, n, ninv, ww1);
        f64_length4inv_zero_j_mut!($x1, $x3, $x5, $x7, n, ninv, ww1);
    };
}

// This is LENGTH8INV_ZERO_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64x4_length8inv_zero_j {
    (
    $x0: ident,
    $x1: ident,
    $x2: ident,
    $x3: ident,
    $x4: ident,
    $x5: ident,
    $x6: ident,
    $x7: ident,
    $n: ident,
    $ninv: ident,
    $ww1: ident,
    $www2: ident,
    $www3: ident
) => {
        f64x4_length2inv_zero_j!($x0, $x1, $n, $ninv);
        f64x4_length2inv_any_j!($x2, $x3, $n, $ninv, $ww1);
        f64x4_length2inv_any_j!($x4, $x5, $n, $ninv, $www2);
        f64x4_length2inv_any_j!($x6, $x7, $n, $ninv, $www3);
        f64x4_length4inv_zero_j!($x0, $x2, $x4, $x6, $n, $ninv, $ww1);
        f64x4_length4inv_zero_j!($x1, $x3, $x5, $x7, $n, $ninv, $ww1);
    };
}

// This is LENGTH8INV_ANY_J from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
macro_rules! f64x4_length8inv_any_j {
    (
        $x0: ident,
        $x1: ident,
        $x2: ident,
        $x3: ident,
        $x4: ident,
        $x5: ident,
        $x6: ident,
        $x7: ident,
        $n: ident,
        $ninv: ident,
        $w0: ident,
        $ww0: ident,
        $ww1: ident,
        $www0: ident,
        $www1: ident,
        $www2: ident,
        $www3: ident
    ) => {
        f64x4_length2inv_any_j!($x0, $x1, $n, $ninv, $www0);
        f64x4_length2inv_any_j!($x2, $x3, $n, $ninv, $www1);
        f64x4_length2inv_any_j!($x4, $x5, $n, $ninv, $www2);
        f64x4_length2inv_any_j!($x6, $x7, $n, $ninv, $www3);
        f64x4_length4inv_any_j!($x0, $x2, $x4, $x6, $n, $ninv, $w0, $ww0, $ww1);
        f64x4_length4inv_any_j!($x1, $x3, $x5, $x7, $n, $ninv, $w0, $ww0, $ww1);
    };
}

const BIG_M: usize = 4;

// cmp(a, b*2^e), a does not have to be normalized
//
// This is flint_mpn_cmp_ui_2exp from fft_small/mul_helpers.c, FLINT 3.3.0-dev.
fn flint_mpn_cmp_ui_2exp(a: &[u64], b: u64, e: u64) -> Ordering {
    let mut an = a.len();
    let mut q = usize::exact_from(e >> u64::LOG_WIDTH);
    let r = e & u64::WIDTH_MASK;
    while an != 0 && a[an - 1] == 0 {
        an -= 1;
    }
    if an == 0 {
        return if b == 0 { Equal } else { Greater };
    }
    // ```
    // b*2^e = (b*2^r       )*2^(64*q)
    //       = (b0 + b1*2^64)*2^(64*q)
    // ```
    let (b0, b1) = if r == 0 {
        (b, 0)
    } else {
        (b << r, b >> (u64::WIDTH - r))
    };
    // ```
    //      check words [q+2,infty)
    // then check words [q+1, 64*q+128) against b1
    // then check words [q, q+1) against b0
    // then check words [0, q)
    // ```
    if an > q + 2 {
        return Greater;
    }
    let mut x = if q + 1 < an { a[q + 1] } else { 0 };
    if x != b1 {
        return x.cmp(&b1);
    }
    x = if q < an { a[q] } else { 0 };
    if x != b0 {
        return x.cmp(&b0);
    }
    q = min(q, an);
    if !slice_test_zero(&a[..q]) {
        return Greater;
    }
    Less
}

// This is NMOD_RED2 from nmod.h, FLINT 3.3.0-dev.
macro_rules! nmod_red2 {
    ($a_hi: expr, $a_lo: expr, $mod_data: expr) => {{
        let a_lo = $a_lo;
        let mod_data = $mod_data;
        let u1xx = ($a_hi << mod_data.norm)
            + (if mod_data.norm == 0 {
                0
            } else {
                a_lo >> (u64::WIDTH - mod_data.norm)
            });
        let u0xx = a_lo << mod_data.norm;
        let nxx = mod_data.n << mod_data.norm;
        let (mut q1xx, mut q0xx) = u64::x_mul_y_to_zz(mod_data.ninv, u1xx);
        (q1xx, q0xx) = u64::xx_add_yy_to_zz(q1xx, q0xx, u1xx, u0xx);
        let mut r1xx = u0xx.wrapping_sub((q1xx + 1).wrapping_mul(nxx));
        if r1xx > q0xx {
            r1xx.wrapping_add_assign(nxx);
        }
        if r1xx < nxx {
            r1xx >> mod_data.norm
        } else {
            (r1xx - nxx) >> mod_data.norm
        }
    }};
}

// This is _madd from crt_helpers.h, FLINT 3.3.0-dev, modifying hi and lo.
macro_rules! madd {
    ($hi: expr, $lo: expr, $y: expr, $x: expr) => {
        let (r1, r0) = u64::x_mul_y_to_zz($x, $y);
        ($hi, $lo) = u64::xx_add_yy_to_zz(r1, r0, $hi, $lo);
    };
}

// This is big_mul from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! big_mul {
    ($n: expr, $m: expr, $r: ident, $t: ident, $c: expr, $y: expr) => {
        let y = $y;
        let c = $c;
        for k in (0..$n).step_by(2) {
            if k < const { $n - 1 } {
                assert!(k < $m);
                ($r[k + 1], $r[k]) = u64::x_mul_y_to_zz(c[k], y);
            } else {
                assert_eq!(k, const { $n - 1 });
                $r[k] = if k < $m { c[k] * y } else { 0 };
            }
            if k < const { $n - 2 } {
                assert!(k < const { $m - 1 });
                let kp1 = k + 1;
                ($t[k + 2], $t[kp1]) = u64::x_mul_y_to_zz(c[kp1], y);
            } else if k < const { $n - 1 } {
                let kp1 = k + 1;
                $t[kp1] = if k < const { $m - 1 } { c[kp1] * y } else { 0 }
            }
        }
    };
}

// This is big_addmul from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! big_add_mul {
    ($n: expr, $m: expr, $r: ident, $t: ident, $c: expr, $y: expr) => {
        let y = $y;
        let c = $c;
        for k in (0..$n).step_by(2) {
            if k < const { $n - 1 } {
                assert!(k < $m);
                madd!($r[k + 1], $r[k], c[k], y);
            } else {
                assert_eq!(k, const { $n - 1 });
                if k < $m {
                    $r[k] += c[k] * y;
                }
            }
            #[allow(clippy::redundant_comparisons)]
            if k < const { $n - 2 } {
                assert!(k < const { $m - 1 });
                let kp1 = k + 1;
                madd!($t[k + 2], $t[kp1], c[kp1], y);
            } else if k < const { $n - 1 } && k < const { $m - 1 } {
                let kp1 = k + 1;
                $t[kp1] += c[kp1] * y;
            }
        }
    };
}

// These are multi_add_3 through multi_add_8 from crt_helpers.h, FLINT 3.3.0-dev, as one function
// generic over the word count. Adds the low N words of `a` to `z`; the top word wraps (no carry
// out). The folded carry chain ((carry_1 | carry_2), with the previous carry added before the next
// word) is the shape LLVM fuses into branchless adds/adcs on aarch64; the fixed trip count unrolls
// fully.
#[inline(always)]
fn multi_add<const N: usize>(z: &mut [u64], a: &[u64]) {
    let mut carry = false;
    for i in 0..N - 1 {
        let (mut s, carry_1) = z[i].overflowing_add(a[i]);
        let carry_2 = s.overflowing_add_assign(u64::from(carry));
        z[i] = s;
        carry = carry_1 | carry_2;
    }
    z[N - 1] = z[N - 1]
        .wrapping_add(a[N - 1])
        .wrapping_add(u64::from(carry));
}

// These are multi_add_4_alt through multi_add_8_alt from crt_helpers.h, FLINT 3.3.0-dev, as one
// function generic over the source word count. Splits the low N u64 words of `a` into 2N half words
// and adds them to `z`, whose limbs are half-width; the final half word (index 2N - 1) may fall
// past the end of `z`, in which case it (and the carry into it) is dropped, and otherwise wraps.
// See `multi_add` for the carry-chain shape.
#[cfg(feature = "32_bit_limbs")]
#[inline(always)]
fn multi_add_alt<const N: usize>(z: &mut [Limb], a: &[u64]) {
    let mut carry = false;
    let mut w = 0;
    for ak in &a[..N] {
        let (hi, lo) = ak.split_in_half();
        let (s, carry_1) = z[w].overflowing_add(lo);
        let (s, carry_2) = s.overflowing_add(Limb::from(carry));
        z[w] = s;
        carry = carry_1 | carry_2;
        if w + 1 == (N << 1) - 1 {
            if w + 1 < z.len() {
                z[w + 1] = z[w + 1].wrapping_add(hi).wrapping_add(Limb::from(carry));
            }
        } else {
            let (s, carry_1) = z[w + 1].overflowing_add(hi);
            let (s, carry_2) = s.overflowing_add(Limb::from(carry));
            z[w + 1] = s;
            carry = carry_1 | carry_2;
        }
        w += 2;
    }
}

// These are multi_sub_4 through multi_sub_7 from crt_helpers.h, FLINT 3.3.0-dev, as one function
// generic over the word count. Subtracts the low N words of `a` from `z`; the top word wraps (no
// borrow out). See `multi_add` for the (borrow) chain shape.
#[inline(always)]
fn multi_sub<const N: usize>(z: &mut [u64], a: &[u64]) {
    let mut borrow = false;
    for i in 0..N - 1 {
        let (mut d, borrow_1) = z[i].overflowing_sub(a[i]);
        let borrow_2 = d.overflowing_sub_assign(u64::from(borrow));
        z[i] = d;
        borrow = borrow_1 | borrow_2;
    }
    z[N - 1] = z[N - 1]
        .wrapping_sub(a[N - 1])
        .wrapping_sub(u64::from(borrow));
}

// This is multi_add_7 from crt_helpers.h, FLINT 3.3.0-dev. This is _reduce_big_sum from
// crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! reduce_big_sum {
    ($n: expr, $r: ident, $t: ident, $limit: expr) => {
        let limit = $limit;
        multi_add::<{ $n - 1 }>(&mut $r[1..], &$t[1..]);
        'outer: loop {
            let mut goto_sub = false;
            for k in (2..=$n).rev() {
                let km1 = k - 1;
                if $r[km1] > limit[km1] {
                    goto_sub = true;
                    break;
                }
                if $r[km1] < limit[km1] {
                    break 'outer;
                }
            }
            if !goto_sub && $r[0] < limit[0] {
                break 'outer;
            }
            multi_sub::<{ $n }>(&mut $r, limit);
        }
    };
}

// This is _add_to_answer_easy from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
macro_rules! add_to_answer_easy {
    ($n: expr, $z: ident, $r: ident, $toff: ident, $tshift: ident) => {{
        #[cfg(feature = "32_bit_limbs")]
        let z_len = $z.len().shr_round(1, Ceiling).0;
        #[cfg(not(feature = "32_bit_limbs"))]
        let z_len = $z.len();
        assert!(z_len > $toff);
        if $tshift == 0 {
            #[cfg(feature = "32_bit_limbs")]
            multi_add_alt::<{ $n }>(&mut $z[$toff << 1..], &$r);
            #[cfg(not(feature = "32_bit_limbs"))]
            multi_add::<{ $n }>(&mut $z[$toff..], &$r);
        } else {
            let comp_shift = 64 - $tshift;
            $r[$n] = $r[$n - 1] >> comp_shift;
            for k in (2..=$n).rev() {
                $r[k - 1] = ($r[k - 1] << $tshift) | ($r[k - 2] >> comp_shift);
            }
            $r[0] <<= $tshift;
            #[cfg(feature = "32_bit_limbs")]
            multi_add_alt::<{ $n + 1 }>(&mut $z[$toff << 1..], &$r);
            #[cfg(not(feature = "32_bit_limbs"))]
            multi_add::<{ $n + 1 }>(&mut $z[$toff..], &$r);
        }
    }};
}

#[cfg(feature = "32_bit_limbs")]
fn limbs_slice_add_same_length_in_place_left_alt(xs: &mut [u32], ys: &[u64]) -> bool {
    let mut carry = 0;
    let mut xi = xs.iter_mut();
    for &y in ys {
        let (y_hi, y_lo) = y.split_in_half();
        let x = xi.next().unwrap();
        (*x, carry) = add_with_carry_limb(*x, y_lo, carry);
        if let Some(x) = xi.next() {
            (*x, carry) = add_with_carry_limb(*x, y_hi, carry);
        }
    }
    carry != 0
}

// This is _add_to_answer_hard from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
macro_rules! add_to_answer_hard {
    ($n: expr, $z: ident, $r: ident, $toff: ident, $tshift: ident) => {{
        #[cfg(feature = "32_bit_limbs")]
        let z_len = $z.len().shr_round(1, Ceiling).0;
        #[cfg(not(feature = "32_bit_limbs"))]
        let z_len = $z.len();
        assert!(z_len > $toff);
        let mut do_add = true;
        if $tshift == 0 {
            if z_len - $toff >= const { $n as usize } {
                #[cfg(feature = "32_bit_limbs")]
                multi_add_alt::<{ $n }>(&mut $z[$toff << 1..], &$r);
                #[cfg(not(feature = "32_bit_limbs"))]
                multi_add::<{ $n }>(&mut $z[$toff..], &$r);
                do_add = false;
            }
        } else {
            let comp_shift = 64 - $tshift;
            $r[$n] = $r[const { $n - 1 }] >> comp_shift;
            for k in (2..=$n).rev() {
                $r[k - 1] = ($r[k - 1] << $tshift) | ($r[k - 2] >> comp_shift);
            }
            $r[0] <<= $tshift;
            if z_len - $toff > const { $n as usize } {
                #[cfg(feature = "32_bit_limbs")]
                multi_add_alt::<{ $n + 1 }>(&mut $z[$toff << 1..], &$r);
                #[cfg(not(feature = "32_bit_limbs"))]
                multi_add::<{ $n + 1 }>(&mut $z[$toff..], &$r);
                do_add = false;
            }
        }
        if do_add {
            let diff = z_len - $toff;
            assert!(diff <= const { $n as usize });
            #[cfg(feature = "32_bit_limbs")]
            limbs_slice_add_same_length_in_place_left_alt(&mut $z[$toff << 1..], &$r[..diff]);
            #[cfg(not(feature = "32_bit_limbs"))]
            limbs_slice_add_same_length_in_place_left(&mut $z[$toff..], &$r[..diff]);
        }
    }};
}

// transpose a block
//
// This is _convert_block from fft_small/mpn_helpers.c, FLINT 3.3.0-dev.
macro_rules! convert_block {
    (
    $xs: ident,
    $rffts: ident,
    $d: ident,
    $dstride: ident,
    $np: expr,
    $i: expr
) => {
        let d_hi = &$d[$i << LG_BLK_SZ..];
        let mut m = 0;
        for l in 0..$np {
            let xs_hi = &mut $xs[l << LG_BLK_SZ..];
            let ds = &d_hi[m..];
            let p = f64x4::splat($rffts[l].p);
            let pinv = f64x4::splat($rffts[l].pinv);
            for j in (0..BLK_SZ).step_by(VEC_SZ << 2) {
                let mut x0 = read_f64x4!(ds, j);
                let mut x1 = read_f64x4!(ds, j + VEC_SZ);
                let mut x2 = read_f64x4!(ds, j + VEC_SZ_2);
                let mut x3 = read_f64x4!(ds, j + VEC_SZ_3);
                x0 = f64x4_reduce_to_0n!(x0, p, pinv);
                x1 = f64x4_reduce_to_0n!(x1, p, pinv);
                x2 = f64x4_reduce_to_0n!(x2, p, pinv);
                x3 = f64x4_reduce_to_0n!(x3, p, pinv);
                let y0 = f64x4_to_u64x4!(x0);
                let y1 = f64x4_to_u64x4!(x1);
                let y2 = f64x4_to_u64x4!(x2);
                let y3 = f64x4_to_u64x4!(x3);
                write_f64x4!(xs_hi, j, y0);
                write_f64x4!(xs_hi, j + VEC_SZ, y1);
                write_f64x4!(xs_hi, j + VEC_SZ_2, y2);
                write_f64x4!(xs_hi, j + VEC_SZ_3, y3);
            }
            m += $dstride;
        }
    };
}

// The `Natural` whose 64-bit words, least significant first, are `xs`.
fn natural_from_u64s(xs: &[u64]) -> Natural {
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        Natural::from_limbs_asc(xs)
    }
    #[cfg(feature = "32_bit_limbs")]
    {
        let mut limbs = Vec::with_capacity(xs.len() << 1);
        for &x in xs {
            let (hi, lo) = x.split_in_half();
            limbs.push(lo);
            limbs.push(hi);
        }
        Natural::from_owned_limbs_asc(limbs)
    }
}

// Returns `x` modulo the prime of `mod_data`, reducing the 64-bit words of `x` from the most
// significant down.
//
// This is fmpz_get_nmod from fmpz/get.c, FLINT 3.6.0, for a nonnegative `fmpz`, using NMOD_RED2 for
// every word.
fn natural_mod_prime(x: &Natural, mod_data: &ModData) -> u64 {
    let mut r = 0;
    #[cfg(not(feature = "32_bit_limbs"))]
    for word in x.limbs().rev() {
        r = nmod_red2!(r, word, mod_data);
    }
    #[cfg(feature = "32_bit_limbs")]
    for pair in x.to_limbs_asc().chunks(2).rev() {
        let word = if let &[lo, hi] = pair {
            u64::join_halves(hi, lo)
        } else {
            u64::from(pair[0])
        };
        r = nmod_red2!(r, word, mod_data);
    }
    r
}

// The value of `x`, which must fit in an `i64`. With 64-bit limbs, any such `x` is stored in a
// single limb, which is read directly. The value of a coefficient whose absolute value is less than
// $2^{63}$, as an `i64`.
#[inline]
fn coefficient_to_i64<C: PolynomialCoefficient>(x: &C) -> i64 {
    let abs = x.unsigned_abs_ref();
    #[cfg(not(feature = "32_bit_limbs"))]
    let value = if let Natural(Small(small)) = *abs {
        i64::wrapping_from(small)
    } else {
        i64::exact_from(abs)
    };
    #[cfg(feature = "32_bit_limbs")]
    let value = i64::exact_from(abs);
    if x.is_negative() { -value } else { value }
}

// Reduces the coefficients `a` modulo the prime of `fft` into the first `atrunc` entries of `abuf`,
// zero-padding past `a.len()`. `abits` is the largest number of significant bits of any element of
// `a`, and `negative` is whether any of them is negative.
//
// This is _mod from fft_small/fmpz_poly_mul.c, FLINT 3.6.0, where `abits` is negated when
// `negative` is true.
fn coefficients_to_fft<C: PolynomialCoefficient>(
    abuf: &mut [f64],
    atrunc: usize,
    a: &[C],
    abits: u64,
    negative: bool,
    fft: &FFTContext,
) {
    let p = fft.mod_data.n;
    assert!(atrunc >= a.len());
    if abits < p.significant_bits() {
        if negative {
            for (x, c) in abuf.iter_mut().zip(a) {
                let c = coefficient_to_i64(c);
                *x = if c >= 0 {
                    c as f64
                } else {
                    (c + i64::wrapping_from(p)) as f64
                };
            }
        } else {
            for (x, c) in abuf.iter_mut().zip(a) {
                *x = coefficient_to_i64(c) as f64;
            }
        }
    } else if abits <= const { u64::WIDTH - 2 } {
        for (x, c) in abuf.iter_mut().zip(a) {
            *x = coefficient_to_i64(c).mod_op(i64::wrapping_from(p)) as f64;
        }
    } else {
        for (x, c) in abuf.iter_mut().zip(a) {
            let r = natural_mod_prime(c.unsigned_abs_ref(), &fft.mod_data);
            *x = (if c.is_negative() && r != 0 { p - r } else { r }) as f64;
        }
    }
    abuf[a.len()..atrunc].fill(0.0);
}

// Shifts the 64-bit words `xs` right by `bits` bits, where `0 < bits < 64`.
fn limbs_slice_shr_in_place_u64(xs: &mut [u64], bits: u64) {
    let mut high = 0;
    for x in xs.iter_mut().rev() {
        let new_high = *x << (u64::WIDTH - bits);
        *x = (*x >> bits) | high;
        high = new_high;
    }
}

// The FFT kernels, compiled for the features the whole crate is built with.
mod baseline;
// The same kernels compiled for AVX2 and FMA, in x86 builds that lack those features at compile
// time (see build.rs). They are used after checking the CPU, so that a default `cargo build` on x86
// gets hardware fused multiply-adds and rounding in the FFT rather than library calls.
#[cfg(fft_avx2)]
mod avx2;

#[cfg(feature = "test_build")]
pub(crate) use baseline::f64x4_mul_add;

// Multiplies with the FFT, using the AVX2 kernels when the CPU supports them.
//
// This is mpn_ctx_mpn_mul from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
#[cfg(fft_avx2)]
#[allow(unsafe_code)]
fn mpn_ctx_mpn_mul(r: &mut Context, r1: &mut [Limb], i1: &[Limb], i2: &[Limb], test_slow: bool) {
    if avx2::is_supported() {
        // SAFETY: `is_supported` has just checked that the CPU supports AVX2 and FMA, the features
        // the function is compiled with.
        unsafe { avx2::mpn_ctx_mpn_mul(r, r1, i1, i2, test_slow) }
    } else {
        baseline::mpn_ctx_mpn_mul(r, r1, i1, i2, test_slow);
    }
}

#[cfg(not(fft_avx2))]
#[inline]
fn mpn_ctx_mpn_mul(r: &mut Context, r1: &mut [Limb], i1: &[Limb], i2: &[Limb], test_slow: bool) {
    baseline::mpn_ctx_mpn_mul(r, r1, i1, i2, test_slow);
}

// Multiplies polynomials with the FFT, using the AVX2 kernels when the CPU supports them.
//
// This is _fmpz_poly_mul_mid_mpn_ctx from fft_small/mpn_mul.c, FLINT 3.6.0.
#[cfg(fft_avx2)]
#[allow(unsafe_code)]
fn fmpz_poly_mul_mid_mpn_ctx<C: PolynomialCoefficient>(
    r: &mut Context,
    z: &mut [C],
    zl: usize,
    zh: usize,
    a: &[C],
    b: &[C],
) -> bool {
    if avx2::is_supported() {
        // SAFETY: `is_supported` has just checked that the CPU supports AVX2 and FMA, the features
        // the function is compiled with.
        unsafe { avx2::fmpz_poly_mul_mid_mpn_ctx(r, z, zl, zh, a, b) }
    } else {
        baseline::fmpz_poly_mul_mid_mpn_ctx(r, z, zl, zh, a, b)
    }
}

#[cfg(not(fft_avx2))]
#[inline]
fn fmpz_poly_mul_mid_mpn_ctx<C: PolynomialCoefficient>(
    r: &mut Context,
    z: &mut [C],
    zl: usize,
    zh: usize,
    a: &[C],
    b: &[C],
) -> bool {
    baseline::fmpz_poly_mul_mid_mpn_ctx(r, z, zl, zh, a, b)
}

// # Worst-case complexity
// $T(n, m) = O(n(m + \log n))$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `a.len() + b.len()`, and $m$ is the largest
// number of significant bits of any element of `a` or `b`.
//
// This is _fmpz_poly_mul_mid_default_mpn_ctx from fft_small/default_ctx.c, FLINT 3.6.0.
pub(crate) fn fmpz_poly_mul_mid_default_mpn_ctx<C: PolynomialCoefficient>(
    z: &mut [C],
    zl: usize,
    zh: usize,
    a: &[C],
    b: &[C],
) -> bool {
    let mut context = default_context();
    fmpz_poly_mul_mid_mpn_ctx(&mut context, z, zl, zh, a, b)
}

// This is mpn_mul_default_mpn_ctx from fft_small/default_ctx.c, FLINT 3.3.0-dev.
pub(crate) fn mpn_mul_default_mpn_ctx(r1: &mut [Limb], i1: &[Limb], i2: &[Limb], test_slow: bool) {
    let mut context = default_context();
    mpn_ctx_mpn_mul(&mut context, r1, i1, i2, test_slow);
}

// Tuner entry points (see bin_util/tune.rs and PORTING.md's note on `_for_tuning` wrappers).
// Creating the default context is part of every production call, so it is correct for the threshold
// measurements to include it.
#[cfg(feature = "test_build")]
pub fn mpn_mul_fft_for_tuning(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    mpn_mul_default_mpn_ctx(out, xs, ys, false);
}

#[cfg(feature = "test_build")]
pub fn mpn_square_fft_for_tuning(out: &mut [Limb], xs: &[Limb]) {
    mpn_square_default_mpn_ctx(out, xs);
}

pub(crate) fn mpn_square_default_mpn_ctx(r1: &mut [Limb], i1: &[Limb]) {
    let mut context = default_context();
    mpn_ctx_mpn_mul(&mut context, r1, i1, i1, false);
}
