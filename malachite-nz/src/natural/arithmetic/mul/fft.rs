// Copyright © 2025 Mikhail Hogrefe
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

#[cfg(feature = "32_bit_limbs")]
use crate::natural::arithmetic::add::add_with_carry_limb;
#[cfg(not(feature = "32_bit_limbs"))]
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mul::context::CONTEXT;
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::natural::logic::significant_bits::limbs_significant_bits;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use core::fmt::Debug;
use malachite_base::fail_on_untested_path;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, DivRound, ModInverse, ModPow, OverflowingAddAssign, OverflowingSubAssign,
    Parity, PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2, WrappingAddAssign,
    WrappingSubAssign, XMulYToZZ, XXAddYYToZZ, XXXAddYYYToZZZ, XXXXAddYYYYToZZZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf};
use malachite_base::num::logic::traits::{LeadingZeros, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::*;
use wide::{f64x4, f64x8, u64x4};

// This is nmod_t from src/flint.h, FLINT 3.3.0-dev.
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

impl SerializedFFTContext {
    fn deserialize(self) -> FFTContext {
        FFTContext {
            p: f64::from_bits(self.p),
            pinv: f64::from_bits(self.pinv),
            mod_data: self.mod_data,
            primitive_root: self.primitive_root,
            w2tab_depth: self.w2tab_depth,
            w2tab_backing: self.w2tab_backing.into_iter().map(f64::from_bits).collect(),
            w2tab_offsets: self.w2tab_offsets,
        }
    }
}

// This is sd_fft_ctx_struct from src/fft_small.h, FLINT 3.3.0-dev.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FFTContext {
    pub(crate) p: f64,
    pub(crate) pinv: f64,
    pub(crate) mod_data: ModData,
    pub(crate) primitive_root: u64,
    pub(crate) w2tab_depth: u64,
    pub(crate) w2tab_backing: Vec<f64>,
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
            w2tab_backing: Vec::new(),
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

impl SerializedCRTData {
    fn deserialize(self, data: &[u64]) -> CRTData {
        CRTData {
            prime: self.prime,
            coeff_len: self.coeff_len,
            nprimes: self.nprimes,
            data: data.to_vec(),
        }
    }
}

// This is crt_data_struct from src/fft_small.h, FLINT 3.3.0-dev.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub(crate) struct CRTData {
    pub(crate) prime: u64,
    pub(crate) coeff_len: usize,
    pub(crate) nprimes: usize,
    pub(crate) data: Vec<u64>,
}

impl CRTData {
    // This is crt_data_co_prime_red from src/fft_small.h, FLINT 3.3.0-dev, read-only.
    #[inline]
    fn co_prime_red(&self, i: usize) -> u64 {
        assert!(i < self.nprimes);
        self.data[self.nprimes * self.coeff_len + self.coeff_len + i]
    }

    // return mpn of length C->coeff_len
    //
    // This is crt_data_co_prime from src/fft_small.h, FLINT 3.3.0-dev, read-only.
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

// This is profile_entry_struct from src/fft_small.h, FLINT 3.3.0-dev.
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

pub_test_struct! {
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

impl SerializedContext {
    pub_test! {deserialize(self) -> Context {
        let [f0, f1, f2, f3, f4, f5, f6, f7] = self.ffts;
        let [c0, c1, c2, c3, c4, c5, c6, c7] = self.crts;
        Context {
            ffts: [
                f0.deserialize(),
                f1.deserialize(),
                f2.deserialize(),
                f3.deserialize(),
                f4.deserialize(),
                f5.deserialize(),
                f6.deserialize(),
                f7.deserialize(),
            ],
            crts: [
                c0.deserialize(&self.crts_data_0),
                c1.deserialize(&self.crts_data_1),
                c2.deserialize(&self.crts_data_2),
                c3.deserialize(&self.crts_data_3),
                c4.deserialize(&self.crts_data_4),
                c5.deserialize(&self.crts_data_5),
                c6.deserialize(&self.crts_data_6),
                c7.deserialize(&self.crts_data_7),
            ],
            vec_two_pow_tab_backing: self
                .vec_two_pow_tab_backing
                .into_iter()
                .map(|[u0, u1, u2, u3]| {
                    f64x4::from([
                        f64::from_bits(u0),
                        f64::from_bits(u1),
                        f64::from_bits(u2),
                        f64::from_bits(u3),
                    ])
                })
                .collect(),
            vec_two_pow_tab_offsets: self.vec_two_pow_tab_offsets,
            slow_two_pow_backing: self
                .slow_two_pow_backing
                .into_iter()
                .map(f64::from_bits)
                .collect(),
            slow_two_pow_offsets: self.slow_two_pow_offsets,
            profiles: self.profiles,
            profiles_size: self.profiles_size,
            buffer: Vec::new(),
            buffer_alloc: self.buffer_alloc,
        }
    }}
}

// This is mpn_ctx_struct from src/fft_small.h, FLINT 3.3.0-dev.
pub_test_struct! {
    #[derive(Debug, Default, Clone, PartialEq)]
Context {
    pub(crate) ffts: [FFTContext; MPN_CTX_NCRTS],
    pub(crate) crts: [CRTData; MPN_CTX_NCRTS],
    pub(crate) vec_two_pow_tab_backing: Vec<f64x4>,
    pub(crate) vec_two_pow_tab_offsets: [usize; MPN_CTX_NCRTS.div_ceil(VEC_SZ)],
    pub(crate) slow_two_pow_backing: Vec<f64>,
    pub(crate) slow_two_pow_offsets: [usize; MPN_CTX_NCRTS],
    pub(crate) profiles: [ProfileEntry; MAX_NPROFILES],
    pub(crate) profiles_size: usize,
    pub(crate) buffer: Vec<f64>,
    pub(crate) buffer_alloc: usize,
}}

// This is vec1d_reduce_pm1n_to_pmhn from src/machine_vectors.h, FLINT 3.3.0-dev.
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
pub(crate) use f64_reduce_pm1n_to_pmhn;

// This is vec4d_reduce_pm1n_to_pmhn from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_pm1n_to_pmhn {
    ($a: expr, $n: expr) => {{
        let [a0, a1, a2, a3] = $a.to_array();
        let [n0, n1, n2, n3] = $n.to_array();
        f64x4::from([
            f64_reduce_pm1n_to_pmhn!(a0, n0),
            f64_reduce_pm1n_to_pmhn!(a1, n1),
            f64_reduce_pm1n_to_pmhn!(a2, n2),
            f64_reduce_pm1n_to_pmhn!(a3, n3),
        ])
    }};
}

// This is vec8d_reduce_pm1n_to_pmhn from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x8_reduce_pm1n_to_pmhn {
    ($a: expr, $n: expr) => {{
        let [a0, a1, a2, a3, a4, a5, a6, a7] = $a.to_array();
        let [n0, n1, n2, n3, n4, n5, n6, n7] = $n.to_array();
        f64x8::from([
            f64_reduce_pm1n_to_pmhn!(a0, n0),
            f64_reduce_pm1n_to_pmhn!(a1, n1),
            f64_reduce_pm1n_to_pmhn!(a2, n2),
            f64_reduce_pm1n_to_pmhn!(a3, n3),
            f64_reduce_pm1n_to_pmhn!(a4, n4),
            f64_reduce_pm1n_to_pmhn!(a5, n5),
            f64_reduce_pm1n_to_pmhn!(a6, n6),
            f64_reduce_pm1n_to_pmhn!(a7, n7),
        ])
    }};
}

// [0,n] -> [-n/2, n/2]
//
// This is f64_reduce_0n_to_pmhn from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_0n_to_pmhn {
    ($a: expr, $n: expr) => {{
        let a = $a;
        let n = $n;
        if a > 0.5 * n { a - n } else { a }
    }};
}
pub(crate) use f64_reduce_0n_to_pmhn;

// return a mod n in [0,n) assuming a in (-n,n)
//
// This is vec1d_reduce_pm1no_to_0n from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_pm1no_to_0n {
    ($a: expr, $n: expr) => {{
        let a = $a;
        if a >= 0.0 { a } else { a + $n }
    }};
}

// return a mod n in [0,n) assuming a in (-n,n)
//
// This is vec4d_reduce_pm1no_to_0n from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_pm1no_to_0n {
    ($a: expr, $n: expr) => {{
        let [a0, a1, a2, a3] = $a.to_array();
        let [n0, n1, n2, n3] = $n.to_array();
        f64x4::from([
            f64_reduce_pm1no_to_0n!(a0, n0),
            f64_reduce_pm1no_to_0n!(a1, n1),
            f64_reduce_pm1no_to_0n!(a2, n2),
            f64_reduce_pm1no_to_0n!(a3, n3),
        ])
    }};
}

macro_rules! f64x4_round {
    ($x: expr) => {{
        let [x0, x1, x2, x3] = $x.to_array();
        f64x4::from([round_even!(x0), round_even!(x1), round_even!(x2), round_even!(x3)])
    }};
}
pub(crate) use f64x4_round;

macro_rules! f64x8_round {
    ($x: expr) => {{
        let [x0, x1, x2, x3, x4, x5, x6, x7] = $x.to_array();
        f64x8::from([
            round_even!(x0),
            round_even!(x1),
            round_even!(x2),
            round_even!(x3),
            round_even!(x4),
            round_even!(x5),
            round_even!(x6),
            round_even!(x7),
        ])
    }};
}

// In this case f64x4::mul_add is not perfectly accurate, so we must fall back to a slower, more
// accurate implementation
#[cfg(not(any(
    all(
        target_feature = "fma",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    all(target_feature = "neon", target_arch = "aarch64")
)))]
macro_rules! f64x4_mul_add {
    ($a: expr, $b: expr, $c: expr) => {{
        let [a0, a1, a2, a3] = $a.to_array();
        let [b0, b1, b2, b3] = $b.to_array();
        let [c0, c1, c2, c3] = $c.to_array();
        f64x4::from([fma!(a0, b0, c0), fma!(a1, b1, c1), fma!(a2, b2, c2), fma!(a3, b3, c3)])
    }};
}

#[cfg(any(
    all(
        target_feature = "fma",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    all(target_feature = "neon", target_arch = "aarch64")
))]
macro_rules! f64x4_mul_add {
    ($a: expr, $b: expr, $c: expr) => {{ $a.mul_add($b, $c) }};
}
pub(crate) use f64x4_mul_add;

// In this case f64x8::mul_add is not perfectly accurate, so we must fall back to a slower, more
// accurate implementation
#[cfg(not(any(
    all(
        target_feature = "fma",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    all(target_feature = "neon", target_arch = "aarch64")
)))]
macro_rules! f64x8_mul_add {
    ($a: expr, $b: expr, $c: expr) => {{
        let [a0, a1, a2, a3, a4, a5, a6, a7] = $a.to_array();
        let [b0, b1, b2, b3, b4, b5, b6, b7] = $b.to_array();
        let [c0, c1, c2, c3, c4, c5, c6, c7] = $c.to_array();
        f64x8::from([
            fma!(a0, b0, c0),
            fma!(a1, b1, c1),
            fma!(a2, b2, c2),
            fma!(a3, b3, c3),
            fma!(a4, b4, c4),
            fma!(a5, b5, c5),
            fma!(a6, b6, c6),
            fma!(a7, b7, c7),
        ])
    }};
}

#[cfg(any(
    all(
        target_feature = "fma",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    all(target_feature = "neon", target_arch = "aarch64")
))]
macro_rules! f64x8_mul_add {
    ($a: expr, $b: expr, $c: expr) => {{ $a.mul_add($b, $c) }};
}

// return a mod n in (-n,n)
//
// This is vec4d_reduce_to_pm1no from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_to_pm1no {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        f64x4_mul_add!(-f64x4_round!(a * $ninv), $n, a)
    }};
}

// return a mod n in (-n,n)
//
// This is vec1d_reduce_to_pm1no from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_to_pm1no {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        fma!(-round_even!(a * $ninv), $n, a)
    }};
}

// return a mod n in [0,n)
//
// This is vec4d_reduce_to_0n from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_to_0n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let n = $n;
        f64x4_reduce_pm1no_to_0n!(f64x4_reduce_to_pm1no!($a, n, $ninv), n)
    }};
}

// return a mod n in [0,n)
//
// This is vec1d_reduce_to_0n from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_to_0n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let n = $n;
        f64_reduce_pm1no_to_0n!(f64_reduce_to_pm1no!($a, n, $ninv), n)
    }};
}

// This is vec1d_mulmod from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_mulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        fma!(-round_even!(h * $ninv), $n, h) - fma!(-a, b, h)
    }};
}
pub(crate) use f64_mulmod;

// This is vec4d_mulmod from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_mulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x4_mul_add!(-f64x4_round!(h * $ninv), $n, h) - f64x4_mul_add!(-a, b, h)
    }};
}

// This is vec8d_mulmod from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x8_mulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x8_mul_add!(-f64x8_round!(h * $ninv), $n, h) - f64x8_mul_add!(-a, b, h)
    }};
}

// This is vec4d_nmulmod from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_nmulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x4_mul_add!(-a, b, h) - f64x4_mul_add!(-f64x4_round!(h * $ninv), $n, h)
    }};
}

// This is vec4d_nmulmod from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x8_nmulmod {
    ($a: expr, $b: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        let b = $b;
        let h = a * b;
        f64x8_mul_add!(-a, b, h) - f64x8_mul_add!(-f64x8_round!(h * $ninv), $n, h)
    }};
}

// This is vec1d_reduce_to_pm1n from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64_reduce_to_pm1n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        fma!(-round_even!(a * $ninv), $n, a)
    }};
}

// This is vec4d_reduce_to_pm1n from src/machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_to_pm1n {
    ($a: expr, $n: expr, $ninv: expr) => {{
        let a = $a;
        f64x4_mul_add!(-f64x4_round!(a * $ninv), $n, a)
    }};
}

// This is vec8d_reduce_to_pm1n from src/machine_vectors.h, FLINT 3.3.0-dev.
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
            if xs_hi[1..].iter().any(|&x| x != 0) {
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
        for (i, r) in $rffts.chunks(4).enumerate() {
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

// The the l^th fft ctx Rffts[l] is expected to have data at d + l*dstride
//
// This is mpn_to_ffts from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
macro_rules! apply_mpn_to_fft_func {
    ($f: ident, $np: expr, $bits: expr, $nvs: expr) => {
        fn $f(
            rffts: &[FFTContext],
            ds: &mut [f64],
            dstride: usize,
            a: &[Limb],
            atrunc: usize,
            two_pow: &[f64x4],
            stop_easy: usize,
            start_hard: usize,
            stop_hard: usize,
        ) {
            let mut xs = [f64x4::default(); $nvs];
            let mut ps = [f64x4::default(); $nvs];
            let mut pinvs = [f64x4::default(); $nvs];
            for (i, r) in rffts.chunks(4).enumerate() {
                ps[i] = f64x4::from([r[0].p, r[1].p, r[2].p, r[3].p]);
                pinvs[i] = f64x4::from([r[0].pinv, r[1].pinv, r[2].pinv, r[3].pinv]);
            }
            if const { $bits & 7 == 0 } {
                assert_eq!(stop_easy & 3, 0);
                for i in (0..stop_easy).step_by(4) {
                    code!(
                        0, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        1, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        2, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        3, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                }
            } else {
                assert_eq!(stop_easy & 7, 0);
                for i in (0..stop_easy).step_by(8) {
                    code!(
                        0, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        1, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        2, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        3, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        4, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        5, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        6, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                    code!(
                        7, $nvs, $np, ds, dstride, xs, ps, pinvs, two_pow, a, i, $bits
                    );
                }
            }
            mpn_to_ffts_hard!(
                $np, $nvs, rffts, ds, dstride, a, atrunc, two_pow, start_hard, stop_hard, $bits
            );
        }
    };
}

// nvs == np.div_ceil(4)
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_4_84, 4, 84, 1);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_4_88, 4, 88, 1);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_4_92, 4, 92, 1);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_5_112, 5, 112, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_5_116, 5, 116, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_5_120, 5, 120, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_6_136, 6, 136, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_6_140, 6, 140, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_6_144, 6, 144, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_7_160, 7, 160, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_7_164, 7, 164, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_7_168, 7, 168, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_8_184, 8, 184, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_8_188, 8, 188, 2);
apply_mpn_to_fft_func!(apply_mpn_to_fft_func_8_192, 8, 192, 2);

const LG_BLK_SZ: u64 = 8;
const BLK_SZ: usize = 256;

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

// This is slow_mpn_to_fft_easy from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn slow_mpn_to_fft_easy(
    q: &FFTContext,
    zs: &mut [f64],
    a: &[Limb],
    iq_stop_easy: usize,
    bits: usize,
    two_pow: &[f64],
) {
    let p = f64x8::splat(q.p);
    let pinv = f64x8::splat(q.pinv);
    let b2 = bits << 1;
    let b3 = b2 + bits;
    let b4 = b3 + bits;
    let b5 = b4 + bits;
    let b6 = b5 + bits;
    let b7 = b6 + bits;
    for iq in 0..iq_stop_easy {
        let zi = &mut zs[iq << LG_BLK_SZ..];
        let mut m = 0;
        for ir in 0..const { BLK_SZ >> 3 } {
            let mut k = iq * const { BLK_SZ >> 5 } * bits + (m >> 5);
            let mut j = m & 31;
            let mut ak = f64x8::from([
                f64::from(get!(a, k) >> j),
                f64::from(get!(a, k + bits) >> j),
                f64::from(get!(a, k + b2) >> j),
                f64::from(get!(a, k + b3) >> j),
                f64::from(get!(a, k + b4) >> j),
                f64::from(get!(a, k + b5) >> j),
                f64::from(get!(a, k + b6) >> j),
                f64::from(get!(a, k + b7) >> j),
            ]);
            let mut x = ak;
            k += 1;
            j = 32 - j;
            let bm32 = bits - 32;
            while j <= bm32 {
                ak = f64x8::from([
                    f64::from(get!(a, k)),
                    f64::from(get!(a, k + bits)),
                    f64::from(get!(a, k + b2)),
                    f64::from(get!(a, k + b3)),
                    f64::from(get!(a, k + b4)),
                    f64::from(get!(a, k + b5)),
                    f64::from(get!(a, k + b6)),
                    f64::from(get!(a, k + b7)),
                ]);
                x += f64x8_mulmod!(ak, f64x8::splat(two_pow[j]), p, pinv);
                k += 1;
                j += 32;
            }
            let bmj = bits - j;
            if bmj != 0 {
                let shift = 32 - bmj;
                ak = f64x8::from([
                    f64::from(get!(a, k) << shift),
                    f64::from(get!(a, k + bits) << shift),
                    f64::from(get!(a, k + b2) << shift),
                    f64::from(get!(a, k + b3) << shift),
                    f64::from(get!(a, k + b4) << shift),
                    f64::from(get!(a, k + b5) << shift),
                    f64::from(get!(a, k + b6) << shift),
                    f64::from(get!(a, k + b7) << shift),
                ]);
                x += f64x8_mulmod!(ak, f64x8::splat(two_pow[bm32]), p, pinv);
            }
            x = f64x8_reduce_to_pm1n!(x, p, pinv);
            let [x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7] = x.to_array();
            let zi_hi = &mut zi[ir..];
            const B: usize = BLK_SZ >> 3;
            zi_hi[0] = x_0;
            zi_hi[B] = x_1;
            zi_hi[const { 2 * B }] = x_2;
            zi_hi[const { 3 * B }] = x_3;
            zi_hi[const { 4 * B }] = x_4;
            zi_hi[const { 5 * B }] = x_5;
            zi_hi[const { 6 * B }] = x_6;
            zi_hi[const { 7 * B }] = x_7;
            m += bits;
        }
    }
}

// This is slow_mpn_to_fft from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn slow_mpn_to_fft(
    q: &FFTContext,
    zs: &mut [f64],
    ztrunc: usize,
    a: &[Limb],
    bits: usize,
    two_pow: &[f64],
) {
    let an = a.len();
    #[cfg(not(feature = "32_bit_limbs"))]
    let an = an << 1;
    let bm32 = bits - 32;
    // the highest index read from two_pow is bits - 32
    assert!(bm32 < MPN_CTX_TWO_POWER_TAB_SIZE);
    // if i*bits + 32 < 32*an, then the index into a is always in bounds
    let i_stop_easy = min(ztrunc, ((an << 5) - 33) / bits);
    let iq_stop_easy = i_stop_easy >> LG_BLK_SZ;
    slow_mpn_to_fft_easy(q, zs, a, iq_stop_easy, bits, two_pow);
    // now the hard ones
    let p = q.p;
    let pinv = q.pinv;
    for iq in iq_stop_easy..ztrunc >> LG_BLK_SZ {
        let big_i = iq << LG_BLK_SZ;
        for (i, z) in zs[big_i..].iter_mut().enumerate().take(BLK_SZ) {
            let n = (big_i + i) * bits;
            let mut k = n >> 5;
            let mut j = n & 31;
            let mut x = f64::from(get_or_default!(a, k) >> j);
            k += 1;
            j = 32 - j;
            while j <= bm32 {
                x += f64_mulmod!(f64::from(get_or_default!(a, k)), two_pow[j], p, pinv);
                k += 1;
                j += 32;
            }
            let bmj = bits - j;
            if bmj != 0 {
                x += f64_mulmod!(
                    f64::from(get_or_default!(a, k) << (32 - bmj)),
                    two_pow[bm32],
                    p,
                    pinv
                );
            }
            *z = f64_reduce_to_pm1n!(x, p, pinv);
        }
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

macro_rules! read_f64x4 {
    ($xs: expr, $i: expr) => {{
        let i = $i;
        f64x4::from(&$xs[i..i + 4])
    }};
}

macro_rules! read_f64x8 {
    ($xs: expr, $i: expr) => {{
        let i = $i;
        f64x8::from(&$xs[i..i + 8])
    }};
}

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

macro_rules! write_f64x4 {
    ($xs: ident, $i: expr, $f: expr) => {{
        let i = $i;
        $xs[i..i + 4].copy_from_slice(&$f.to_array())
    }};
}

macro_rules! write_f64x8 {
    ($xs: ident, $i: expr, $f: expr) => {{
        let i = $i;
        $xs[i..i + 8].copy_from_slice(&$f.to_array())
    }};
}

macro_rules! write_f64x8_w2tab {
    ($q: expr, $i: expr, $j: expr, $f: expr) => {
        let start = $q.w2tab_offsets[$i] + $j;
        $q.w2tab_backing[start..start + 8].copy_from_slice(&$f.to_array());
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

// S is stride; BLK_SZ transforms each of length 2^k
//
// This is sd_fft_no_trunc_block from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_no_trunc_block(q: &FFTContext, xs: &mut [f64], s: usize, k: u64, j: usize) {
    let big_s = s << LG_BLK_SZ;
    if k > 4 {
        let k1 = k >> 1;
        let k2 = k - k1;
        let shifted = s << k2;
        let mut m = 0;
        for _ in 0..usize::power_of_2(k2) {
            sd_fft_no_trunc_block(q, &mut xs[m..], shifted, k1, j);
            m += big_s;
        }
        // row ffts
        let big_s = s << (k2 + LG_BLK_SZ);
        let shifted = j << k1;
        let mut m = 0;
        for b in 0..usize::power_of_2(k1) {
            sd_fft_no_trunc_block(q, &mut xs[m..], s, k2, shifted + b);
            m += big_s;
        }
        return;
    }
    set_j_bits_and_j_r!(j, j_bits, j_r);
    if k >= 2 {
        let k1 = 2;
        let k2 = k - k1;
        let l2 = usize::power_of_2(k2);
        let big_s_2 = s << (k2 + LG_BLK_SZ);
        // column ffts
        if j_bits == 0 {
            radix_4_forward_param_j_is_z!(q, iw, n, ninv);
            for a in 0..l2 {
                split_into_chunks_mut!(&mut xs[a * big_s..], big_s_2, [x0, x1, x2], x3);
                process_4_3!(
                    radix_4_forward_moth_j_is_z,
                    BLK_SZ,
                    x0,
                    x1,
                    x2,
                    x3,
                    iw,
                    n,
                    ninv
                );
            }
        } else {
            radix_4_forward_param_j_is_nz!(q, j_r, j_bits, w, w2, iw, n, ninv);
            for a in 0..l2 {
                split_into_chunks_mut!(&mut xs[a * big_s..], big_s_2, [x0, x1, x2], x3);
                process_4_5!(
                    radix_4_forward_moth_j_is_nz,
                    BLK_SZ,
                    x0,
                    x1,
                    x2,
                    x3,
                    w,
                    w2,
                    iw,
                    n,
                    ninv
                );
            }
        }
        if l2 == 1 {
            return;
        }
        // row ffts
        let shifted = j << k1;
        let mut m = 0;
        for b in 0..usize::power_of_2(k1) {
            sd_fft_no_trunc_block(q, &mut xs[m..], s, k2, shifted + b);
            m += big_s_2;
        }
    } else if k == 1 {
        let (x0, x1) = xs.split_at_mut(big_s);
        if j_bits == 0 {
            radix_2_forward_param_j_is_z!(q, n, ninv);
            process_2_2!(radix_2_forward_moth_j_is_z, BLK_SZ, x0, x1, n, ninv);
        } else {
            radix_2_forward_param_j_is_nz!(q, j_r, j_bits, w, n, ninv);
            process_2_3!(radix_2_forward_moth_j_is_nz, BLK_SZ, x0, x1, w, n, ninv);
        }
    }
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

// This is vec4d_unpack_lo_permute_0_2_1_3 from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_unpack_lo_permute_0_2_1_3 {
    ($u: ident, $v: ident) => {{
        let [u0, _, u2, _] = $u.to_array();
        let [v0, _, v2, _] = $v.to_array();
        f64x4::from([u0, u2, v0, v2])
    }};
}

// This is vec4d_unpack_hi_permute_0_2_1_3 from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_unpack_hi_permute_0_2_1_3 {
    ($u: ident, $v: ident) => {{
        let [_, u1, _, u3] = $u.to_array();
        let [_, v1, _, v3] = $v.to_array();
        f64x4::from([u1, u3, v1, v3])
    }};
}

// view the 4 vectors as the rows of a 4x4 matrix
//
// This is VEC4D_TRANSPOSE from machine_vectors.h, FLINT 3.3.0-dev, modifying the vectors in place.
macro_rules! f64x4_transpose {
    ($a0: ident, $a1: ident, $a2: ident, $a3: ident) => {{
        let [a00, a01, a02, a03] = $a0.to_array();
        let [a10, a11, a12, a13] = $a1.to_array();
        let [a20, a21, a22, a23] = $a2.to_array();
        let [a30, a31, a32, a33] = $a3.to_array();
        $a0 = f64x4::from([a00, a10, a20, a30]);
        $a1 = f64x4::from([a01, a11, a21, a31]);
        $a2 = f64x4::from([a02, a12, a22, a32]);
        $a3 = f64x4::from([a03, a13, a23, a33]);
    }};
}

// This is sd_fft_basecase_0_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
const fn sd_fft_basecase_0_1(_q: &FFTContext, _xs: &mut [f64]) {}

// This is sd_fft_basecase_1_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_basecase_1_1(q: &FFTContext, xs: &mut [f64]) {
    let n = q.p;
    let ninv = q.pinv;
    let x0 = f64_reduce_to_pm1n!(xs[0], n, ninv);
    let x1 = f64_reduce_to_pm1n!(xs[1], n, ninv);
    xs[0] = x0 + x1;
    xs[1] = x0 - x1;
}

// This is sd_fft_basecase_2_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_basecase_2_1(q: &FFTContext, xs: &mut [f64]) {
    if let &mut [ref mut x0, ref mut x1, ref mut x2, ref mut x3, ..] = xs {
        f64_length4_zero_j!(x0, x1, x2, x3, q.p, q.pinv, w2tab!(q, 1, 0));
    }
}

// This is sd_fft_basecase_3_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_basecase_3_1(q: &FFTContext, xs: &mut [f64]) {
    if let &mut [
        ref mut x0,
        ref mut x1,
        ref mut x2,
        ref mut x3,
        ref mut x4,
        ref mut x5,
        ref mut x6,
        ref mut x7,
        ..,
    ] = xs
    {
        f64_length8_zero_j!(
            x0,
            x1,
            x2,
            x3,
            x4,
            x5,
            x6,
            x7,
            q.p,
            q.pinv,
            w2tab!(q, 1, 0),
            w2tab!(q, 2, 0),
            w2tab!(q, 2, 1)
        );
    }
}

// This is sd_fft_basecase_4_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_basecase_4_1(q: &FFTContext, xs: &mut [f64]) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    // q.w2tab[0] points to consecutive entries
    const _: () = assert!(SD_FFT_CTX_W2TAB_INIT >= 4);
    let iw = f64x4::splat(w2tab!(q, 0, 1));
    f64x4_length4_zero_j!(x0, x1, x2, x3, n, ninv, iw);
    let u = read_f64x4_w2tab!(q, 0, 0);
    let v = read_f64x4_w2tab!(q, 0, 4);
    let w = f64x4_unpack_lo_permute_0_2_1_3!(u, v);
    let iw = f64x4_unpack_hi_permute_0_2_1_3!(u, v);
    f64x4_transpose!(x0, x1, x2, x3);
    length4_any_j!(x0, x1, x2, x3, n, ninv, u, w, iw);
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
}

// This is sd_fft_basecase_4_0 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_basecase_4_0(q: &FFTContext, xs: &mut [f64], j_r: usize, j_bits: u64) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    let j_bits = usize::exact_from(j_bits);
    let w2 = f64x4::splat(w2tab!(q, j_bits, j_r));
    let jp1 = j_bits + 1;
    let j_2 = j_r << 1;
    let w = f64x4::splat(w2tab!(q, jp1, j_2));
    let iw = f64x4::splat(w2tab!(q, jp1, j_2 + 1));
    length4_any_j!(x0, x1, x2, x3, n, ninv, w2, w, iw);
    let jp3 = j_bits + 3;
    let j_8 = j_r << 3;
    let u = read_f64x4_w2tab!(q, jp3, j_8);
    let v = read_f64x4_w2tab!(q, jp3, j_8 + 4);
    let w2 = read_f64x4_w2tab!(q, j_bits + 2, j_r << 2);
    let w = f64x4_unpack_lo_permute_0_2_1_3!(u, v);
    let iw = f64x4_unpack_hi_permute_0_2_1_3!(u, v);
    f64x4_transpose!(x0, x1, x2, x3);
    length4_any_j!(x0, x1, x2, x3, n, ninv, w2, w, iw);
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
}

// The length 32 transform can be broken up as
// - 8 transforms of length 4 within columns, followed by 4 transforms of length 8 in the rows, or
// - 4 transforms of length 8 within columns, followed by 8 transforms of length 4 in the rows
// Since the length 16 basecase is missing the final 4x4 transpose, so the output is worse than
// bit-reversed. If the length 32 transform used a order different from 16's, then we will have a
// problem at a higher level since it would be difficult to keep track of what basecase happened to
// have be used. Therefore, the length 16 and 32 basecases should produce the same order, and this
// is easier with (b).
//
// This is sd_fft_basecase_5_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_basecase_5_1(q: &FFTContext, xs: &mut [f64]) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    let mut x4 = read_f64x4!(xs, 16);
    let mut x5 = read_f64x4!(xs, 20);
    let mut x6 = read_f64x4!(xs, 24);
    let mut x7 = read_f64x4!(xs, 28);
    let q00 = w2tab!(q, 0, 0);
    let q10 = w2tab!(q, 1, 0);
    let q20 = w2tab!(q, 2, 0);
    let q21 = w2tab!(q, 2, 1);
    let ww1 = f64x4::splat(q10);
    let www2 = f64x4::splat(q20);
    let www3 = f64x4::splat(q21);
    f64x4_length8_zero_j!(x0, x1, x2, x3, x4, x5, x6, x7, n, ninv, ww1, www2, www3);
    f64x4_transpose!(x0, x1, x2, x3);
    f64x4_transpose!(x4, x5, x6, x7);
    // j = 0, 1, 2, 3
    let mut w0 = f64x4::from([q00, q10, q20, q21]);
    let mut ww0 = f64x4::from([q00, q20, w2tab!(q, 3, 0), w2tab!(q, 3, 2)]);
    let mut ww1 = f64x4::from([q10, q21, w2tab!(q, 3, 1), w2tab!(q, 3, 3)]);
    length4_any_j!(x0, x1, x2, x3, n, ninv, w0, ww0, ww1);
    // j = 4, 5, 6, 7
    w0 = f64x4::from([w2tab!(q, 3, 0), w2tab!(q, 3, 1), w2tab!(q, 3, 2), w2tab!(q, 3, 3)]);
    let u = read_f64x4_w2tab!(q, 4, 0);
    let v = read_f64x4_w2tab!(q, 4, 4);
    ww0 = f64x4_unpack_lo_permute_0_2_1_3!(u, v);
    ww1 = f64x4_unpack_hi_permute_0_2_1_3!(u, v);
    length4_any_j!(x4, x5, x6, x7, n, ninv, w0, ww0, ww1);
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
    write_f64x4!(xs, 16, x4);
    write_f64x4!(xs, 20, x5);
    write_f64x4!(xs, 24, x6);
    write_f64x4!(xs, 28, x7);
}

// This is sd_fft_basecase_5_0 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_basecase_5_0(q: &FFTContext, xs: &mut [f64], j_r: usize, j_bits: u64) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    let mut x4 = read_f64x4!(xs, 16);
    let mut x5 = read_f64x4!(xs, 20);
    let mut x6 = read_f64x4!(xs, 24);
    let mut x7 = read_f64x4!(xs, 28);
    let j_bits = usize::exact_from(j_bits);
    let w0 = f64x4::splat(w2tab!(q, j_bits, j_r));
    let jp1 = j_bits + 1;
    let j_2 = j_r << 1;
    let ww0 = f64x4::splat(w2tab!(q, jp1, j_2));
    let ww1 = f64x4::splat(w2tab!(q, jp1, j_2 + 1));
    let jp2 = j_bits + 2;
    let j_4 = j_r << 2;
    let www0 = f64x4::splat(w2tab!(q, jp2, j_4));
    let www1 = f64x4::splat(w2tab!(q, jp2, j_4 + 1));
    let www2 = f64x4::splat(w2tab!(q, jp2, j_4 + 2));
    let www3 = f64x4::splat(w2tab!(q, jp2, j_4 + 3));
    length8_any_j!(
        x0, x1, x2, x3, x4, x5, x6, x7, n, ninv, w0, ww0, ww1, www0, www1, www2, www3
    );
    f64x4_transpose!(x0, x1, x2, x3);
    f64x4_transpose!(x4, x5, x6, x7);
    // j = 8*j+0, 8*j+1, 8*j+2, 8*j+3
    let jp3 = j_bits + 3;
    let jp4 = j_bits + 4;
    let j_8 = j_r << 3;
    let w0 = read_f64x4_w2tab!(q, jp3, j_8);
    let sixteen_j = j_r << 4;
    let u = read_f64x4_w2tab!(q, jp4, sixteen_j);
    let v = read_f64x4_w2tab!(q, jp4, sixteen_j + 4);
    let ww0 = f64x4_unpack_lo_permute_0_2_1_3!(u, v);
    let ww1 = f64x4_unpack_hi_permute_0_2_1_3!(u, v);
    length4_any_j!(x0, x1, x2, x3, n, ninv, w0, ww0, ww1);
    // j = 8*j+4, 8*j+5, 8*j+6, 8*j+7
    let w0 = read_f64x4_w2tab!(q, jp3, j_8 + 4);
    let u = read_f64x4_w2tab!(q, jp4, sixteen_j + 8);
    let v = read_f64x4_w2tab!(q, jp4, sixteen_j + 12);
    let ww0 = f64x4_unpack_lo_permute_0_2_1_3!(u, v);
    let ww1 = f64x4_unpack_hi_permute_0_2_1_3!(u, v);
    length4_any_j!(x4, x5, x6, x7, n, ninv, w0, ww0, ww1);
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
    write_f64x4!(xs, 16, x4);
    write_f64x4!(xs, 20, x5);
    write_f64x4!(xs, 24, x6);
    write_f64x4!(xs, 28, x7);
}

macro_rules! sd_fft_basecase {
    ($n: expr, $f0: ident, $f1: ident, $fs0: ident, $fs1: ident) => {
        // This is sd_fft_basecase_n_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
        fn $f1(q: &FFTContext, xs: &mut [f64]) {
            const LIMIT: usize = 1 << ($n - 2);
            radix_4_forward_param_j_is_z!(q, iw, n, ninv);
            split_into_chunks_mut!(xs, LIMIT, [xs0, xs1, xs2], xs3);
            process_4_3!(
                radix_4_forward_moth_j_is_z,
                LIMIT,
                xs0,
                xs1,
                xs2,
                xs3,
                iw,
                n,
                ninv
            );
            $fs1(q, xs0);
            $fs0(q, xs1, 0, 1);
            $fs0(q, xs2, 0, 2);
            $fs0(q, xs3, 1, 2);
        }

        // This is sd_fft_basecase_n_0 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
        fn $f0(q: &FFTContext, xs: &mut [f64], j_r: usize, j_bits: u64) {
            const LIMIT: usize = 1 << ($n - 2);
            radix_4_forward_param_j_is_nz!(q, j_r, j_bits, w, w2, iw, n, ninv);
            split_into_chunks_mut!(xs, LIMIT, [xs0, xs1, xs2], xs3);
            process_4_5!(
                radix_4_forward_moth_j_is_nz,
                LIMIT,
                xs0,
                xs1,
                xs2,
                xs3,
                w,
                w2,
                iw,
                n,
                ninv
            );
            let j_r_4 = j_r << 2;
            let j_bits_p_2 = j_bits + 2;
            $fs0(q, xs0, j_r_4, j_bits_p_2);
            $fs0(q, xs1, j_r_4 + 1, j_bits_p_2);
            $fs0(q, xs2, j_r_4 + 2, j_bits_p_2);
            $fs0(q, xs3, j_r_4 + 3, j_bits_p_2);
        }
    };
}

sd_fft_basecase!(
    6,
    sd_fft_basecase_6_0,
    sd_fft_basecase_6_1,
    sd_fft_basecase_4_0,
    sd_fft_basecase_4_1
);
sd_fft_basecase!(
    7,
    sd_fft_basecase_7_0,
    sd_fft_basecase_7_1,
    sd_fft_basecase_5_0,
    sd_fft_basecase_5_1
);
sd_fft_basecase!(
    8,
    sd_fft_basecase_8_0,
    sd_fft_basecase_8_1,
    sd_fft_basecase_6_0,
    sd_fft_basecase_6_1
);
sd_fft_basecase!(
    9,
    sd_fft_basecase_9_0,
    sd_fft_basecase_9_1,
    sd_fft_basecase_7_0,
    sd_fft_basecase_7_1
);

// parameter 1: j can be zero
//
// This is sd_fft_base_8_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_base_8_1(q: &FFTContext, xs: &mut [f64], j: usize) {
    set_j_bits_and_j_r!(j, j_bits, j_r);
    if j == 0 {
        sd_fft_basecase_8_1(q, xs);
    } else {
        sd_fft_basecase_8_0(q, xs, j_r, j_bits);
    }
}

// parameter 0: j cannot be zero
//
// This is sd_fft_base_8_0 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_base_8_0(q: &FFTContext, xs: &mut [f64], j: usize) {
    assert_ne!(j, 0);
    set_j_bits_and_j_r!(j, j_bits, j_r);
    sd_fft_basecase_8_0(q, xs, j_r, j_bits);
}

// This is sd_fft_base_9_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_base_9_1(q: &FFTContext, xs: &mut [f64], j: usize) {
    set_j_bits_and_j_r!(j, j_bits, j_r);
    if j == 0 {
        sd_fft_basecase_9_1(q, xs);
    } else {
        sd_fft_basecase_9_0(q, xs, j_r, j_bits);
    }
}

// This is sd_fft_no_trunc_internal from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_no_trunc_internal(
    q: &FFTContext,
    xs: &mut [f64],
    // stride
    s: usize,
    // 1 transform of length BLK_SZ*2^k
    k: u64,
    // twist param
    j: usize,
) {
    if k > 2 {
        let k1 = k >> 1;
        let k2 = k - k1;
        // column ffts
        let shift = s << k2;
        let big_s = s << LG_BLK_SZ;
        let mut m = 0;
        for _ in 0..usize::power_of_2(k2) {
            sd_fft_no_trunc_block(q, &mut xs[m..], shift, k1, j);
            m += big_s;
        }
        // row ffts
        let shift = k2 + LG_BLK_SZ;
        let shifted = j << k1;
        let big_s = s << shift;
        let mut m = 0;
        for b in 0..usize::power_of_2(k1) {
            sd_fft_no_trunc_internal(q, &mut xs[m..], s, k2, shifted + b);
            m += big_s;
        }
        return;
    }
    if k == 2 {
        // k1 = 2; k2 = 0
        sd_fft_no_trunc_block(q, xs, s, 2, j);
        split_into_chunks_mut!(xs, s << LG_BLK_SZ, [xs0, xs1, xs2], xs3);
        let j_4 = j << 2;
        sd_fft_base_8_1(q, xs0, j_4);
        sd_fft_base_8_0(q, xs1, j_4 + 1);
        sd_fft_base_8_0(q, xs2, j_4 + 2);
        sd_fft_base_8_0(q, xs3, j_4 + 3);
    } else if k == 1 {
        sd_fft_base_9_1(q, xs, j);
    } else {
        // currently unreachable because all ffts are called with k > 0
        sd_fft_base_8_1(q, xs, j);
    }
}

// This is sd_fft_moth_trunc_block_i_j_1 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_moth_trunc_block_1<const ITRUNC: usize, const OTRUNC: usize>(
    q: &FFTContext,
    _j_r: usize,
    _j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    radix_4_forward_param_j_is_z!(q, iw, n, ninv);
    let gap_2 = gap << 1;
    let gap_3 = gap_2 + gap;
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let mut x0 = f64x8::default();
        let mut x1 = f64x8::default();
        let mut x2 = f64x8::default();
        let mut x3 = f64x8::default();
        if 0 < ITRUNC {
            x0 = read_f64x8!(xs, i);
        }
        if 0 < ITRUNC {
            x0 = f64x8_reduce_to_pm1n!(x0, n, ninv);
        }
        if 1 < ITRUNC {
            x1 = read_f64x8!(xs, gap + i);
        }
        if 2 < ITRUNC {
            x2 = read_f64x8!(xs, gap_2 + i);
        }
        if 2 < ITRUNC {
            x2 = f64x8_reduce_to_pm1n!(x2, n, ninv);
        }
        if 3 < ITRUNC {
            x3 = read_f64x8!(xs, gap_3 + i);
        }
        if 3 < ITRUNC {
            x3 = f64x8_reduce_to_pm1n!(x3, n, ninv);
        }
        let y0 = if 2 < ITRUNC { x0 + x2 } else { x0 };
        let mut y1 = if 3 < ITRUNC { x1 + x3 } else { x1 };
        let y2 = if 2 < ITRUNC { x0 - x2 } else { x0 };
        let mut y3 = if 3 < ITRUNC { x1 - x3 } else { x1 };
        y1 = f64x8_reduce_to_pm1n!(y1, n, ninv);
        y3 = f64x8_mulmod!(y3, iw, n, ninv);
        x0 = y0 + y1;
        x1 = y0 - y1;
        x2 = y2 + y3;
        x3 = y2 - y3;
        if 0 < OTRUNC {
            write_f64x8!(xs, i, x0);
        }
        if 1 < OTRUNC {
            write_f64x8!(xs, gap + i, x1);
        }
        if 2 < OTRUNC {
            write_f64x8!(xs, gap_2 + i, x2);
        }
        if 3 < OTRUNC {
            write_f64x8!(xs, gap_3 + i, x3);
        }
    }
}

// This is sd_fft_moth_trunc_block_i_j_0 from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_moth_trunc_block_0<const ITRUNC: usize, const OTRUNC: usize>(
    q: &FFTContext,
    j_r: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    radix_4_forward_param_j_is_nz!(q, j_r, j_bits, w, w2, iw, n, ninv);
    let gap_2 = gap << 1;
    let gap_3 = gap_2 + gap;
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let mut x0 = f64x8::default();
        let mut x1 = f64x8::default();
        let mut x2 = f64x8::default();
        let mut x3 = f64x8::default();
        if 0 < ITRUNC {
            x0 = read_f64x8!(xs, i);
        }
        if 0 < ITRUNC {
            x0 = f64x8_reduce_to_pm1n!(x0, n, ninv);
        }
        if 1 < ITRUNC {
            x1 = read_f64x8!(xs, gap + i);
        }
        if 2 < ITRUNC {
            x2 = read_f64x8!(xs, gap_2 + i);
        }
        if 2 < ITRUNC {
            x2 = f64x8_mulmod!(x2, w2, n, ninv);
        }
        if 3 < ITRUNC {
            x3 = read_f64x8!(xs, gap_3 + i);
        }
        if 3 < ITRUNC {
            x3 = f64x8_mulmod!(x3, w2, n, ninv);
        }
        let y0 = if 2 < ITRUNC { x0 + x2 } else { x0 };
        let mut y1 = if 3 < ITRUNC { x1 + x3 } else { x1 };
        let y2 = if 2 < ITRUNC { x0 - x2 } else { x0 };
        let mut y3 = if 3 < ITRUNC { x1 - x3 } else { x1 };
        y1 = f64x8_mulmod!(y1, w, n, ninv);
        y3 = f64x8_mulmod!(y3, iw, n, ninv);
        x0 = y0 + y1;
        x1 = y0 - y1;
        x2 = y2 + y3;
        x3 = y2 - y3;
        if 0 < OTRUNC {
            write_f64x8!(xs, i, x0);
        }
        if 1 < OTRUNC {
            write_f64x8!(xs, gap + i, x1);
        }
        if 2 < OTRUNC {
            write_f64x8!(xs, gap_2 + i, x2);
        }
        if 3 < OTRUNC {
            write_f64x8!(xs, gap_3 + i, x3);
        }
    }
}

type Sd2MothTruncBlockFn =
    for<'a, 'b, 'c, 'd> fn(&'a FFTContext, usize, &'b mut [f64], &'c mut [f64]);

type Sd4MothTruncBlockFn = for<'a, 'b, 'c> fn(&'a FFTContext, usize, u64, &'b mut [f64], usize);

const SD_FFT_4_MOTH_TRUNC_BLOCK_TABLE: [Sd4MothTruncBlockFn; 24] = [
    sd_fft_moth_trunc_block_0::<2, 1>,
    sd_fft_moth_trunc_block_1::<2, 1>,
    sd_fft_moth_trunc_block_0::<2, 2>,
    sd_fft_moth_trunc_block_1::<2, 2>,
    sd_fft_moth_trunc_block_0::<2, 3>,
    sd_fft_moth_trunc_block_1::<2, 3>,
    sd_fft_moth_trunc_block_0::<2, 4>,
    sd_fft_moth_trunc_block_1::<2, 4>,
    sd_fft_moth_trunc_block_0::<3, 1>,
    sd_fft_moth_trunc_block_1::<3, 1>,
    sd_fft_moth_trunc_block_0::<3, 2>,
    sd_fft_moth_trunc_block_1::<3, 2>,
    sd_fft_moth_trunc_block_0::<3, 3>,
    sd_fft_moth_trunc_block_1::<3, 3>,
    sd_fft_moth_trunc_block_0::<3, 4>,
    sd_fft_moth_trunc_block_1::<3, 4>,
    sd_fft_moth_trunc_block_0::<4, 1>,
    sd_fft_moth_trunc_block_1::<4, 1>,
    sd_fft_moth_trunc_block_0::<4, 2>,
    sd_fft_moth_trunc_block_1::<4, 2>,
    sd_fft_moth_trunc_block_0::<4, 3>,
    sd_fft_moth_trunc_block_1::<4, 3>,
    sd_fft_moth_trunc_block_0::<4, 4>,
    sd_fft_moth_trunc_block_1::<4, 4>,
];

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

// This is sd_fft_trunc_block from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_trunc_block(
    q: &FFTContext,
    xs: &mut [f64],
    s: usize,
    // transform length 2^k
    k: u64,
    j: usize,
    itrunc: usize,
    otrunc: usize,
) {
    let pow = usize::power_of_2(k);
    assert!(itrunc <= pow);
    assert!(otrunc <= pow);
    if otrunc < 1 {
        fail_on_untested_path("sd_fft_trunc_block, otrunc < 1");
        return;
    }
    if itrunc <= 1 {
        let big_s = s << LG_BLK_SZ;
        if itrunc < 1 {
            fail_on_untested_path("sd_fft_trunc_block, itrunc < 1");
            for c in xs[..big_s * otrunc].chunks_mut(big_s) {
                c[..BLK_SZ].fill(0.0);
            }
        } else {
            let mut m = big_s;
            for _ in 1..otrunc {
                let (xs_lo, xs_hi) = xs.split_at_mut(m);
                xs_hi[..BLK_SZ].copy_from_slice(&xs_lo[..BLK_SZ]);
                m += big_s;
            }
        }
        return;
    }
    if itrunc == otrunc && otrunc == pow {
        sd_fft_no_trunc_block(q, xs, s, k, j);
        return;
    }
    let big_s = s << LG_BLK_SZ;
    if k > 2 {
        let k1 = k >> 1;
        let k2 = k - k1;
        let l2 = usize::power_of_2(k2);
        let n1 = otrunc >> k2;
        let mask = l2 - 1;
        let n2 = otrunc & mask;
        let z1 = itrunc >> k2;
        let z2 = itrunc & mask;
        let n1p = n1 + usize::from(n2 != 0);
        let z2p = min(l2, itrunc);
        // columns
        let shifted = s << k2;
        let mut m = 0;
        for a in 0..z2p {
            sd_fft_trunc_block(
                q,
                &mut xs[m..],
                shifted,
                k1,
                j,
                z1 + usize::from(a < z2),
                n1p,
            );
            m += big_s;
        }
        // full rows
        let big_s_2 = s << (k2 + LG_BLK_SZ);
        let shifted = j << k1;
        let mut m = 0;
        for b in 0..n1 {
            sd_fft_trunc_block(q, &mut xs[m..], s, k2, shifted + b, z2p, l2);
            m += big_s_2;
        }
        // last partial row
        if n2 != 0 {
            sd_fft_trunc_block(q, &mut xs[n1 * big_s_2..], s, k2, shifted + n1, z2p, n2);
        }
        return;
    }
    set_j_bits_and_j_r!(j, j_bits, j_r);
    if k == 2 {
        let index = usize::from(j == 0) + ((otrunc - 1 + ((itrunc - 2) << 2)) << 1);
        SD_FFT_4_MOTH_TRUNC_BLOCK_TABLE[index](q, j_r, j_bits, xs, big_s);
    } else if k == 1 {
        let (xs0, xs1) = xs.split_at_mut(big_s);
        assert_eq!(itrunc, 2);
        assert_eq!(otrunc, 1);
        if j_bits == 0 {
            fail_on_untested_path("sd_fft_trunc_block, j_bits == 0");
            radix_2_forward_param_j_is_z!(q, n, ninv);
            process_2_2!(
                radix_2_forward_moth_trunc_2_1_j_is_z,
                BLK_SZ,
                xs0,
                xs1,
                n,
                ninv
            );
        } else {
            radix_2_forward_param_j_is_nz!(q, j_r, j_bits, w, n, ninv);
            process_2_3!(
                radix_2_forward_moth_trunc_2_1_j_is_nz,
                BLK_SZ,
                xs0,
                xs1,
                w,
                n,
                ninv
            );
        }
    }
}

// This is sd_fft_trunc_internal from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_trunc_internal(
    q: &FFTContext,
    // x = data + BLK_SZ*I  where I = starting index
    xs: &mut [f64],
    // stride
    s: usize,
    // transform length BLK_SZ*2^k
    k: u64,
    j: usize,
    // actual trunc is BLK_SZ*itrunc
    itrunc: usize,
    // actual trunc is BLK_SZ*otrunc
    otrunc: usize,
) {
    if otrunc < 1 {
        fail_on_untested_path("sd_fft_trunc_internal, otrunc < 1");
        return;
    }
    if itrunc < 1 {
        fail_on_untested_path("sd_fft_trunc_internal, itrunc < 1");
        let big_s = s << LG_BLK_SZ;
        for c in xs[..big_s * otrunc].chunks_mut(big_s) {
            c[..BLK_SZ].fill(0.0);
        }
        return;
    }
    if itrunc == otrunc && otrunc == usize::power_of_2(k) {
        sd_fft_no_trunc_internal(q, xs, s, k, j);
        return;
    }
    let big_s = s << LG_BLK_SZ;
    if k > 2 {
        let k1 = k >> 1;
        let k2 = k - k1;
        let l2 = usize::power_of_2(k2);
        let n1 = otrunc >> k2;
        let mask = l2 - 1;
        let n2 = otrunc & mask;
        let z1 = itrunc >> k2;
        let z2 = itrunc & mask;
        let n1p = n1 + usize::from(n2 != 0);
        let z2p = min(l2, itrunc);
        // columns
        let shifted = s << k2;
        let mut m = 0;
        for a in 0..z2p {
            sd_fft_trunc_block(
                q,
                &mut xs[m..],
                shifted,
                k1,
                j,
                z1 + usize::from(a < z2),
                n1p,
            );
            m += big_s;
        }
        // full rows
        let big_s_2 = s << (k2 + LG_BLK_SZ);
        let shifted = j << k1;
        let mut m = 0;
        for b in 0..n1 {
            sd_fft_trunc_internal(q, &mut xs[m..], s, k2, shifted + b, z2p, l2);
            m += big_s_2;
        }
        // last partial row
        if n2 > 0 {
            sd_fft_trunc_internal(q, &mut xs[n1 * big_s_2..], s, k2, shifted + n1, z2p, n2);
        }
        return;
    }
    if k == 2 {
        sd_fft_trunc_block(q, xs, s, 2, j, itrunc, otrunc);
        let j_4 = j << 2;
        sd_fft_base_8_1(q, xs, j_4);
        if otrunc > 1 {
            sd_fft_base_8_0(q, &mut xs[big_s..], j_4 + 1);
        }
        if otrunc > 2 {
            sd_fft_base_8_0(q, &mut xs[big_s << 1..], j_4 + 2);
        }
        if otrunc > 3 {
            sd_fft_base_8_0(q, &mut xs[big_s * 3..], j_4 + 3);
        }
    } else if k == 1 {
        sd_fft_trunc_block(q, xs, s, 1, j, itrunc, otrunc);
        let j_2 = j << 1;
        sd_fft_base_8_1(q, xs, j_2);
        if otrunc > 1 {
            sd_fft_base_8_0(q, &mut xs[big_s..], j_2 + 1);
        }
    } else {
        // currently unreachable
        sd_fft_base_8_1(q, xs, j);
    }
}

// This is sd_fft_ctx_fit_depth_with_lock from fft_small/sd_fft_ctx.c, FLINT 3.3.0-dev.
fn sd_fft_ctx_fit_depth_with_lock(q: &mut FFTContext, depth: u64) {
    for k in q.w2tab_depth..depth {
        let ww = q
            .primitive_root
            .mod_pow((q.mod_data.n - 1) >> (k + 1), q.mod_data.n);
        let w = f64x8::splat(f64_reduce_0n_to_pmhn!(ww as f64, q.p));
        let n = f64x8::splat(q.p);
        let ninv = f64x8::splat(q.pinv);
        let big_n = usize::power_of_2(k - 1);
        let old_len = q.w2tab_backing.len();
        q.w2tab_backing.resize(
            old_len + big_n.round_to_multiple_of_power_of_2(12, Ceiling).0,
            0.0,
        );
        let ku = k as usize;
        q.w2tab_offsets[ku] = old_len;
        // The first few tables are stored consecutively, so vec16 is ok.
        let mut off = 0;
        let mut l = const { 1 << (SD_FFT_CTX_W2TAB_INIT - 1) };
        let mut kk = 0;
        for j in SD_FFT_CTX_W2TAB_INIT - 1..k {
            for i in (0..l).step_by(16) {
                let x0 = read_f64x8_w2tab!(q, kk, i);
                let x1 = read_f64x8_w2tab!(q, kk, i + 8);
                let y0 = f64x8_reduce_pm1n_to_pmhn!(f64x8_mulmod!(x0, w, n, ninv), n);
                let y1 = f64x8_reduce_pm1n_to_pmhn!(f64x8_mulmod!(x1, w, n, ninv), n);
                write_f64x8_w2tab!(q, ku, off + i, y0);
                write_f64x8_w2tab!(q, ku, off + i + 8, y1);
            }
            kk = j as usize + 1;
            l += off;
            off = l;
        }
        q.w2tab_depth = k;
    }
}

// This is sd_fft_ctx_fit_depth from src/fft_small.h, FLINT 3.3.0-dev.
fn sd_fft_ctx_fit_depth(q: &mut FFTContext, depth: u64) {
    if q.w2tab_depth < depth {
        sd_fft_ctx_fit_depth_with_lock(q, depth);
    }
}

// This is sd_fft_trunc from fft_small/sd_fft.c, FLINT 3.3.0-dev.
fn sd_fft_trunc(
    q: &mut FFTContext,
    ds: &mut [f64],
    // convolution length 2^L
    l: u64,
    itrunc: usize,
    otrunc: usize,
) {
    let pow = usize::power_of_2(l);
    assert!(itrunc <= pow);
    assert!(otrunc <= pow);
    if l > LG_BLK_SZ {
        sd_fft_ctx_fit_depth(q, l);
        let new_itrunc = itrunc.div_round(BLK_SZ, Ceiling).0;
        let new_otrunc = otrunc.div_round(BLK_SZ, Ceiling).0;
        ds[itrunc..][..itrunc.wrapping_neg() & const { BLK_SZ - 1 }].fill(0.0);
        sd_fft_trunc_internal(q, ds, 1, l - LG_BLK_SZ, 0, new_itrunc, new_otrunc);
        return;
    }
    fail_on_untested_path("sd_fft_trunc, l <= LG_BLK_SZ");
    ds[itrunc..][..usize::power_of_2(l)].fill(0.0);
    // L=8 reads from w2tab[7]
    const _: () = assert!(LG_BLK_SZ <= SD_FFT_CTX_W2TAB_INIT);
    match l {
        0 => sd_fft_basecase_0_1(q, ds),
        1 => sd_fft_basecase_1_1(q, ds),
        2 => sd_fft_basecase_2_1(q, ds),
        3 => sd_fft_basecase_3_1(q, ds),
        4 => sd_fft_basecase_4_1(q, ds),
        5 => sd_fft_basecase_5_1(q, ds),
        6 => sd_fft_basecase_6_1(q, ds),
        7 => sd_fft_basecase_7_1(q, ds),
        8 => sd_fft_basecase_8_1(q, ds),
        _ => unreachable!(),
    }
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

// This is sd_ifft_no_trunc_block from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_no_trunc_block(
    q: &FFTContext,
    xs: &mut [f64],
    // stride
    s: usize,
    // BLK_SZ transforms each of length 2^k
    k: u64,
    j: usize,
) {
    let big_s = s << LG_BLK_SZ;
    if k > 2 {
        let k1 = k >> 1;
        let k2 = k - k1;
        // row ffts
        let big_s_2 = s << (k2 + LG_BLK_SZ);
        let shifted = j << k1;
        let mut m = 0;
        for b in 0..usize::power_of_2(k1) {
            sd_ifft_no_trunc_block(q, &mut xs[m..], s, k2, shifted + b);
            m += big_s_2;
        }
        // column ffts
        let shifted = s << k2;
        let mut m = 0;
        for _ in 0..usize::power_of_2(k2) {
            sd_ifft_no_trunc_block(q, &mut xs[m..], shifted, k1, j);
            m += big_s;
        }
        return;
    }
    set_j_bits_and_j_mr!(j, j_bits, j_mr);
    if k == 2 {
        split_into_chunks_mut!(xs, big_s, [xs0, xs1, xs2], xs3);
        if j_bits == 0 {
            radix_4_reverse_param_j_is_z!(q, iw, n, ninv);
            process_4_3!(
                radix_4_reverse_moth_j_is_z,
                BLK_SZ,
                xs0,
                xs1,
                xs2,
                xs3,
                iw,
                n,
                ninv
            );
        } else {
            radix_4_reverse_param_j_is_nz!(q, j_mr, j_bits, w, w2, iw, n, ninv);
            process_4_5!(
                radix_4_reverse_moth_j_is_nz,
                BLK_SZ,
                xs0,
                xs1,
                xs2,
                xs3,
                w,
                w2,
                iw,
                n,
                ninv
            );
        }
    } else if k == 1 {
        let (xs0, xs1) = xs.split_at_mut(big_s);
        if j_bits == 0 {
            radix_2_reverse_param_j_is_z!(q, n, ninv);
            process_2_2!(radix_2_reverse_moth_j_is_z, BLK_SZ, xs0, xs1, n, ninv);
        } else {
            radix_2_reverse_param_j_is_nz!(q, j_mr, j_bits, w, n, ninv);
            process_2_3!(radix_2_reverse_moth_j_is_nz, BLK_SZ, xs0, xs1, w, n, ninv);
        }
    }
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

// This is sd_ifft_basecase_0_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
const fn sd_ifft_basecase_0_1(_q: &FFTContext, _xs: &mut [f64]) {}

// This is sd_ifft_basecase_1_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_basecase_1_1(q: &FFTContext, xs: &mut [f64]) {
    if let &mut [ref mut x0, ref mut x1, ..] = xs {
        f64_length2inv_zero_j_mut!(x0, x1, q.p, q.pinv);
    }
}

// This is sd_ifft_basecase_2_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_basecase_2_1(q: &FFTContext, xs: &mut [f64]) {
    if let &mut [ref mut x0, ref mut x1, ref mut x2, ref mut x3, ..] = xs {
        f64_length4inv_zero_j_mut!(x0, x1, x2, x3, q.p, q.pinv, w2tab!(q, 1, 0));
    }
}

// This is sd_ifft_basecase_3_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_basecase_3_1(q: &FFTContext, xs: &mut [f64]) {
    if let &mut [
        ref mut x0,
        ref mut x1,
        ref mut x2,
        ref mut x3,
        ref mut x4,
        ref mut x5,
        ref mut x6,
        ref mut x7,
        ..,
    ] = xs
    {
        f64_length8inv_zero_j!(
            x0,
            x1,
            x2,
            x3,
            x4,
            x5,
            x6,
            x7,
            q.p,
            q.pinv,
            w2tab!(q, 1, 0),
            w2tab!(q, 2, 1),
            w2tab!(q, 2, 0)
        );
    }
}

// This is sd_ifft_basecase_4_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_basecase_4_1(q: &FFTContext, xs: &mut [f64]) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    let q00 = w2tab!(q, 0, 0);
    let q01 = w2tab!(q, 0, 1);
    let q02 = w2tab!(q, 0, 2);
    let q03 = w2tab!(q, 0, 3);
    let w = f64x4::from([-q00, q03, w2tab!(q, 0, 7), w2tab!(q, 0, 5)]);
    let iw = f64x4::from([q01, q02, w2tab!(q, 0, 6), w2tab!(q, 0, 4)]);
    let w2 = f64x4::from([-q00, q01, q03, q02]);
    f64x4_length4inv_any_j!(x0, x1, x2, x3, n, ninv, w2, w, iw);
    f64x4_transpose!(x0, x1, x2, x3);
    let iw = f64x4::splat(q01);
    f64x4_length4inv_zero_j!(x0, x1, x2, x3, n, ninv, iw);
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
}

// This is sd_ifft_basecase_4_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_basecase_4_0(q: &FFTContext, xs: &mut [f64], j_mr: usize, j_bits: u64) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    let j_bits = usize::exact_from(j_bits);
    let w2 = read_f64x4_w2tab!(q, j_bits + 2, j_mr << 2);
    let [w20, w21, w22, w23] = w2.to_array();
    let w2 = f64x4::from([w23, w22, w21, w20]);
    let jp3 = j_bits + 3;
    let j_8 = j_mr << 3;
    let [u0, u1, u2, u3] = read_f64x4_w2tab!(q, jp3, j_8).to_array();
    let [v0, v1, v2, v3] = read_f64x4_w2tab!(q, jp3, j_8 + 4).to_array();
    let mut w = f64x4::from([v3, v1, u3, u1]);
    let mut iw = f64x4::from([v2, v0, u2, u0]);
    f64x4_length4inv_any_j!(x0, x1, x2, x3, n, ninv, w2, w, iw);
    f64x4_transpose!(x0, x1, x2, x3);
    let jp1 = j_bits + 1;
    let j_2 = j_mr << 1;
    w = f64x4::splat(w2tab!(q, jp1, j_2 + 1));
    iw = f64x4::splat(w2tab!(q, jp1, j_2));
    let w2 = f64x4::splat(w2tab!(q, j_bits, j_mr));
    f64x4_length4inv_any_j!(x0, x1, x2, x3, n, ninv, w2, w, iw);
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
}

// This is sd_ifft_basecase_5_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_basecase_5_1(q: &FFTContext, xs: &mut [f64]) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    let mut x4 = read_f64x4!(xs, 16);
    let mut x5 = read_f64x4!(xs, 20);
    let mut x6 = read_f64x4!(xs, 24);
    let mut x7 = read_f64x4!(xs, 28);
    let q00 = w2tab!(q, 0, 0);
    let q10 = w2tab!(q, 1, 0);
    let q20 = w2tab!(q, 2, 0);
    let q21 = w2tab!(q, 2, 1);
    let q30 = w2tab!(q, 3, 0);
    let q31 = w2tab!(q, 3, 1);
    let q32 = w2tab!(q, 3, 2);
    let q33 = w2tab!(q, 3, 3);
    // j = 0, 1, 2, 3  then {j,2j+0,2j+1}^-1 in each column
    let mut w0 = f64x4::from([-q00, q10, q21, q20]);
    let mut ww0 = f64x4::from([-q00, q21, q33, q31]);
    let mut ww1 = f64x4::from([q10, q20, q32, q30]);
    f64x4_length4inv_any_j!(x0, x1, x2, x3, n, ninv, w0, ww0, ww1);
    // j = 4, 5, 6, 7  then {j,2j+0,2j+1}^-1 in each column
    w0 = f64x4::from([q33, q32, q31, q30]);
    ww0 = f64x4::from([w2tab!(q, 4, 7), w2tab!(q, 4, 5), w2tab!(q, 4, 3), w2tab!(q, 4, 1)]);
    ww1 = f64x4::from([w2tab!(q, 4, 6), w2tab!(q, 4, 4), w2tab!(q, 4, 2), w2tab!(q, 4, 0)]);
    f64x4_length4inv_any_j!(x4, x5, x6, x7, n, ninv, w0, ww0, ww1);
    f64x4_transpose!(x0, x1, x2, x3);
    f64x4_transpose!(x4, x5, x6, x7);
    ww1 = f64x4::splat(q10);
    let www2 = f64x4::splat(q21);
    let www3 = f64x4::splat(q20);
    f64x4_length8inv_zero_j!(x0, x1, x2, x3, x4, x5, x6, x7, n, ninv, ww1, www2, www3);
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
    write_f64x4!(xs, 16, x4);
    write_f64x4!(xs, 20, x5);
    write_f64x4!(xs, 24, x6);
    write_f64x4!(xs, 28, x7);
}

// This is sd_ifft_basecase_5_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_basecase_5_0(q: &FFTContext, xs: &mut [f64], j_mr: usize, j_bits: u64) {
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let mut x0 = read_f64x4!(xs, 0);
    let mut x1 = read_f64x4!(xs, 4);
    let mut x2 = read_f64x4!(xs, 8);
    let mut x3 = read_f64x4!(xs, 12);
    let mut x4 = read_f64x4!(xs, 16);
    let mut x5 = read_f64x4!(xs, 20);
    let mut x6 = read_f64x4!(xs, 24);
    let mut x7 = read_f64x4!(xs, 28);
    let j_bits = usize::exact_from(j_bits);
    let j_8 = j_mr << 3;
    let jp3 = j_bits + 3;
    let [w00, w01, w02, w03] = read_f64x4_w2tab!(q, jp3, j_8 + 4).to_array();
    let w0 = f64x4::from([w03, w02, w01, w00]);
    let jp4 = j_bits + 4;
    let j_16 = j_mr << 4;
    let [u0, u1, u2, u3] = read_f64x4_w2tab!(q, jp4, j_16 + 8).to_array();
    let [v0, v1, v2, v3] = read_f64x4_w2tab!(q, jp4, j_16 + 12).to_array();
    let mut ww0 = f64x4::from([v3, v1, u3, u1]);
    let mut ww1 = f64x4::from([v2, v0, u2, u0]);
    f64x4_length4inv_any_j!(x0, x1, x2, x3, n, ninv, w0, ww0, ww1);
    let [w00, w01, w02, w03] = read_f64x4_w2tab!(q, jp3, j_8).to_array();
    let w0 = f64x4::from([w03, w02, w01, w00]);
    let [u0, u1, u2, u3] = read_f64x4_w2tab!(q, jp4, j_16).to_array();
    let [v0, v1, v2, v3] = read_f64x4_w2tab!(q, jp4, j_16 + 4).to_array();
    ww0 = f64x4::from([v3, v1, u3, u1]);
    ww1 = f64x4::from([v2, v0, u2, u0]);
    f64x4_length4inv_any_j!(x4, x5, x6, x7, n, ninv, w0, ww0, ww1);
    f64x4_transpose!(x0, x1, x2, x3);
    f64x4_transpose!(x4, x5, x6, x7);
    let w0 = f64x4::splat(w2tab!(q, j_bits, j_mr));
    let jp1 = j_bits + 1;
    let j_2 = j_mr << 1;
    ww0 = f64x4::splat(w2tab!(q, jp1, j_2 + 1));
    ww1 = f64x4::splat(w2tab!(q, jp1, j_2));
    let jp2 = j_bits + 2;
    let j_4 = j_mr << 2;
    let www0 = f64x4::splat(w2tab!(q, jp2, j_4 + 3));
    let www1 = f64x4::splat(w2tab!(q, jp2, j_4 + 2));
    let www2 = f64x4::splat(w2tab!(q, jp2, j_4 + 1));
    let www3 = f64x4::splat(w2tab!(q, jp2, j_4));
    f64x4_length8inv_any_j!(
        x0, x1, x2, x3, x4, x5, x6, x7, n, ninv, w0, ww0, ww1, www0, www1, www2, www3
    );
    write_f64x4!(xs, 0, x0);
    write_f64x4!(xs, 4, x1);
    write_f64x4!(xs, 8, x2);
    write_f64x4!(xs, 12, x3);
    write_f64x4!(xs, 16, x4);
    write_f64x4!(xs, 20, x5);
    write_f64x4!(xs, 24, x6);
    write_f64x4!(xs, 28, x7);
}

macro_rules! sd_ifft_basecase {
    ($n: expr, $f0: ident, $f1: ident, $fs0: ident, $fs1: ident) => {
        // This is sd_ifft_basecase_n_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
        fn $f1(q: &FFTContext, xs: &mut [f64]) {
            const LIMIT: usize = 1 << ($n - 2);
            split_into_chunks_mut!(xs, LIMIT, [xs0, xs1, xs2], xs3);
            $fs1(q, xs0);
            $fs0(q, xs1, 0, 1);
            $fs0(q, xs2, 1, 2);
            $fs0(q, xs3, 0, 2);
            radix_4_reverse_param_j_is_z!(q, iw, n, ninv);
            process_4_3!(
                radix_4_reverse_moth_j_is_z,
                LIMIT,
                xs0,
                xs1,
                xs2,
                xs3,
                iw,
                n,
                ninv
            );
        }

        // This is sd_ifft_basecase_n_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
        fn $f0(q: &FFTContext, xs: &mut [f64], j_mr: usize, j_bits: u64) {
            const LIMIT: usize = 1 << ($n - 2);
            assert_ne!(j_bits, 0);
            split_into_chunks_mut!(xs, LIMIT, [xs0, xs1, xs2], xs3);
            let j_mr_4 = j_mr << 2;
            let j_bits_p_2 = j_bits + 2;
            $fs0(q, xs0, j_mr_4 + 3, j_bits_p_2);
            $fs0(q, xs1, j_mr_4 + 2, j_bits_p_2);
            $fs0(q, xs2, j_mr_4 + 1, j_bits_p_2);
            $fs0(q, xs3, j_mr_4, j_bits_p_2);
            radix_4_reverse_param_j_is_nz!(q, j_mr, j_bits, w, w2, iw, n, ninv);
            process_4_5!(
                radix_4_reverse_moth_j_is_nz,
                LIMIT,
                xs0,
                xs1,
                xs2,
                xs3,
                w,
                w2,
                iw,
                n,
                ninv
            );
        }
    };
}
sd_ifft_basecase!(
    6,
    sd_ifft_basecase_6_0,
    sd_ifft_basecase_6_1,
    sd_ifft_basecase_4_0,
    sd_ifft_basecase_4_1
);
sd_ifft_basecase!(
    7,
    sd_ifft_basecase_7_0,
    sd_ifft_basecase_7_1,
    sd_ifft_basecase_5_0,
    sd_ifft_basecase_5_1
);
sd_ifft_basecase!(
    8,
    sd_ifft_basecase_8_0,
    sd_ifft_basecase_8_1,
    sd_ifft_basecase_6_0,
    sd_ifft_basecase_6_1
);
sd_ifft_basecase!(
    9,
    sd_ifft_basecase_9_0,
    sd_ifft_basecase_9_1,
    sd_ifft_basecase_7_0,
    sd_ifft_basecase_7_1
);

// parameter 1: j can be zero
//
// This is sd_ifft_base_8_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_base_8_1(q: &FFTContext, x: &mut [f64], j: usize) {
    set_j_bits_and_j_mr!(j, j_bits, j_mr);
    if j == 0 {
        sd_ifft_basecase_8_1(q, x);
    } else {
        sd_ifft_basecase_8_0(q, x, j_mr, j_bits);
    }
}

// parameter 1: j cannot be zero
//
// This is sd_ifft_base_8_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_base_8_0(q: &FFTContext, x: &mut [f64], j: usize) {
    set_j_bits_and_j_mr!(j, j_bits, j_mr);
    sd_ifft_basecase_8_0(q, x, j_mr, j_bits);
}

// parameter 1: j can be zero
//
// This is sd_ifft_base_9_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_base_9_1(q: &FFTContext, x: &mut [f64], j: usize) {
    set_j_bits_and_j_mr!(j, j_bits, j_mr);
    if j == 0 {
        sd_ifft_basecase_9_1(q, x);
    } else {
        sd_ifft_basecase_9_0(q, x, j_mr, j_bits);
    }
}

// This is sd_ifft_no_trunc_internal from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_no_trunc_internal(
    q: &FFTContext,
    xs: &mut [f64],
    // stride
    s: usize,
    // transform length BLK_SZ*2^k
    k: u64,
    j: usize,
) {
    if k > 2 {
        let k1 = k >> 1;
        let k2 = k - k1;
        let big_s_2 = s << (k2 + LG_BLK_SZ);
        let shifted = j << k1;
        let mut m = 0;
        for b in 0..usize::power_of_2(k1) {
            sd_ifft_no_trunc_internal(q, &mut xs[m..], s, k2, shifted + b);
            m += big_s_2;
        }
        let big_s = s << LG_BLK_SZ;
        let shifted = s << k2;
        let mut m = 0;
        for _ in 0..usize::power_of_2(k2) {
            sd_ifft_no_trunc_block(q, &mut xs[m..], shifted, k1, j);
            m += big_s;
        }
        return;
    }
    if k == 2 {
        let big_s = s << LG_BLK_SZ;
        // k1 = 2; k2 = 0
        split_into_chunks_mut!(xs, big_s, [xs0, xs1, xs2], xs3);
        let j_4 = j << 2;
        sd_ifft_base_8_1(q, xs0, j_4);
        sd_ifft_base_8_0(q, xs1, j_4 + 1);
        sd_ifft_base_8_0(q, xs2, j_4 + 2);
        sd_ifft_base_8_0(q, xs3, j_4 + 3);
        sd_ifft_no_trunc_block(q, xs, s, 2, j);
    } else if k == 1 {
        sd_ifft_base_9_1(q, xs, j);
    } else {
        sd_ifft_base_8_1(q, xs, j);
    }
}

const BIG_M: usize = 4;

// ```
// k = 2, n = 3, z = 4, f = true
// [      -r + 1           r + 1         2   r*w^3]
// [        2//w           -2//w         0    -w^2]
// [(r + 1)//w^2   (-r + 1)//w^2   -2//w^2    -r*w]
// [          -r               r         1   r*w^3]
// ```
//
// This is radix_4_moth_inv_trunc_block_3_4_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_3_4_1(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let pow = usize::power_of_2(j_bits);
    let j_mr = pow - 1 - j;
    let j_r = j & ((pow >> 1) - 1);
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let w = if j == 0 {
        -1.0
    } else {
        w2tab!(q, 1 + j_bits, (j_mr << 1) + 1)
    };
    let w2 = if j == 0 {
        -1.0
    } else {
        w2tab!(q, j_bits, j_mr)
    };
    let two_w = f64_reduce_pm1n_to_pmhn!(-2.0 * w, q.p);
    let f0 = f64x4::splat(w2tab!(q, 0, 1)); // r
    let f1 = f64x4::splat(two_w); // 2*w^-1
    let f2 = f64x4::splat(2.0);
    let f3 = f64x4::splat(w2); // -w^-2
    let rw = if j == 0 {
        w2tab!(q, 0, 1)
    } else {
        w2tab!(q, 1 + j_bits, (j_r << 1) + 1)
    };
    let fr = f64x4::splat(rw); // r*w
    let fq = f64x4::splat(w2tab!(q, j_bits, j_r)); // w^2
    let fp_alt = f64x4_mulmod!(fr, fq, n, ninv);
    let fp = f64x4_reduce_pm1n_to_pmhn!(fp_alt, n); // r*w^3
    for i in (0..BLK_SZ).step_by(BIG_M) {
        let a = read_f64x4!(xs0, i);
        let mut b = read_f64x4!(xs1, i);
        let mut c = read_f64x4!(xs2, i);
        let mut d = read_f64x4!(xs3, i);
        let mut v = a - b;
        let p = f64x4_mulmod!(d, fp, n, ninv);
        let q = f64x4_mulmod!(d, fq, n, ninv);
        let r = f64x4_mulmod!(d, fr, n, ninv);
        c = f64x4_reduce_to_pm1n!(c, n, ninv);
        let u = f64x4_reduce_to_pm1n!(a + b, n, ninv);
        b = f64x4_mulmod!(v, f1, n, ninv);
        v = f64x4_mulmod!(v, f0, n, ninv);
        d = c - v;
        c = f64x4_mul_add!(f2, c, -v);
        write_f64x4!(xs0, i, c + u + p);
        write_f64x4!(xs1, i, b - q);
        write_f64x4!(xs2, i, f64x4_mulmod!(c - u, f3, n, ninv) - r);
        write_f64x4!(xs3, i, d + p);
    }
}

// ```
// k = 2, n = 3, z = 4, f = false
// [      -r + 1           r + 1         2   r*w^3]
// [        2//w           -2//w         0    -w^2]
// [(r + 1)//w^2   (-r + 1)//w^2   -2//w^2    -r*w]
// ```
//
// This is radix_4_moth_inv_trunc_block_3_4_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_3_4_0(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let pow = usize::power_of_2(j_bits);
    let j_mr = pow - 1 - j;
    let j_r = j & ((pow >> 1) - 1);
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let jp1 = j_bits + 1;
    let w = if j == 0 {
        -1.0
    } else {
        w2tab!(q, jp1, (j_mr << 1) + 1)
    };
    let w2 = if j == 0 {
        -1.0
    } else {
        w2tab!(q, j_bits, j_mr)
    };
    let two_w = f64_reduce_pm1n_to_pmhn!(-2.0 * w, q.p);
    let f0 = f64x4::splat(w2tab!(q, 0, 1)); // r
    let f1 = f64x4::splat(two_w); // 2*w^-1
    let f2 = f64x4::splat(2.0);
    let f3 = f64x4::splat(w2); // -w^-2
    let rw = if j == 0 {
        w2tab!(q, 0, 1)
    } else {
        w2tab!(q, jp1, (j_r << 1) + 1)
    };
    let fr = f64x4::splat(rw); // r*w
    let fq = f64x4::splat(w2tab!(q, j_bits, j_r)); // w^2
    let fp_alt = f64x4_mulmod!(fr, fq, n, ninv);
    let fp = f64x4_reduce_pm1n_to_pmhn!(fp_alt, n); // r*w^3
    for i in (0..BLK_SZ).step_by(BIG_M) {
        let a = read_f64x4!(xs0, i);
        let b = read_f64x4!(xs1, i);
        let mut c = read_f64x4!(xs2, i);
        let d = read_f64x4!(xs3, i);
        let u = f64x4_reduce_to_pm1n!(a + b, n, ninv);
        let v = a - b;
        c = f64x4_mul_add!(
            f2,
            f64x4_reduce_to_pm1n!(c, n, ninv),
            -f64x4_mulmod!(v, f0, n, ninv)
        );
        write_f64x4!(xs0, i, c + u + f64x4_mulmod!(d, fp, n, ninv));
        write_f64x4!(
            xs1,
            i,
            f64x4_mulmod!(v, f1, n, ninv) - f64x4_mulmod!(d, fq, n, ninv)
        );
        write_f64x4!(
            xs2,
            i,
            f64x4_mulmod!(c - u, f3, n, ninv) - f64x4_mulmod!(d, fr, n, ninv)
        );
    }
}

// ```
// k = 2, n = 3, z = 3, f = true
// [      -r + 1           r + 1         2]
// [        2//w           -2//w         0]
// [(r + 1)//w^2   (-r + 1)//w^2   -2//w^2]
// [          -r               r         1]
//
//     {x0, x1, x3, x4} = {        -r*(x0 - x1) + (x0 + x1) + 2*x2,
//                         2*w^-1*    (x0 - x1),
//                          -w^-2*(-r*(x0 - x1) - (x0 + x1) + 2*x2),
//                                 -r*(x0 - x1)             +   x2  }
// ```
//
// This is radix_4_moth_inv_trunc_block_3_3_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_3_3_1(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let j_mr = usize::power_of_2(j_bits) - 1 - j;
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let w = if j == 0 {
        -1.0
    } else {
        w2tab!(q, 1 + j_bits, (j_mr << 1) + 1)
    };
    let w2 = if j == 0 {
        -1.0
    } else {
        w2tab!(q, j_bits, j_mr)
    };
    let two_w = f64_reduce_pm1n_to_pmhn!(-2.0 * w, q.p);
    let f0 = f64x8::splat(w2tab!(q, 0, 1)); // r
    let f1 = f64x8::splat(two_w); // 2*w^-1
    let f2 = f64x8::splat(2.0);
    let f3 = f64x8::splat(w2); // -w^-2
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let b = read_f64x8!(xs1, i);
        let mut c = read_f64x8!(xs2, i);
        let mut v = a - b;
        write_f64x8!(xs1, i, f64x8_mulmod!(v, f1, n, ninv));
        c = f64x8_reduce_to_pm1n!(c, n, ninv);
        v = f64x8_mulmod!(v, f0, n, ninv);
        write_f64x8!(xs3, i, c - v);
        let u = f64x8_reduce_to_pm1n!(a + b, n, ninv);
        c = f64x8_mul_add!(-f2, c, v);
        write_f64x8!(xs0, i, u - c);
        write_f64x8!(xs2, i, f64x8_nmulmod!(u + c, f3, n, ninv));
    }
}

// ```
// k = 2, n = 3, z = 3, f = false
// [      -r + 1           r + 1         2]
// [        2//w           -2//w         0]
// [(r + 1)//w^2   (-r + 1)//w^2   -2//w^2]
//
//     {x0, x1, x3} = {        -r*(x0 - x1) + (x0 + x1) + 2*x2,
//                     2*w^-1*(x0 - x1),
//                      -w^-2*(-r*(x0 - x1) - (x0 + x1) + 2*x2)}
//
//                  = {        2*x2 - r*v + u,
//                     2*w^-1*v,
//                      -w^-2*(2*x2 - r*v - u)}
// ```
//
// This is radix_4_moth_inv_trunc_block_3_3_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_3_3_0(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1], xs2);
    let j_mr = usize::power_of_2(j_bits) - 1 - j;
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let w = if j == 0 {
        -1.0
    } else {
        w2tab!(q, 1 + j_bits, (j_mr << 1) + 1)
    };
    let w2 = if j == 0 {
        -1.0
    } else {
        w2tab!(q, j_bits, j_mr)
    };
    let two_w = f64_reduce_pm1n_to_pmhn!(-2.0 * w, q.p);
    let f0 = f64x8::splat(w2tab!(q, 0, 1)); // r
    let f1 = f64x8::splat(two_w); // 2*w^-1
    let f2 = f64x8::splat(2.0);
    let f3 = f64x8::splat(w2); // -w^-2
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let b = read_f64x8!(xs1, i);
        let mut c = read_f64x8!(xs2, i);
        let v = a - b;
        write_f64x8!(xs1, i, f64x8_mulmod!(v, f1, n, ninv));
        c = f64x8_reduce_to_pm1n!(c, n, ninv);
        let mut u = a + b;
        u = f64x8_reduce_to_pm1n!(u, n, ninv);
        c = f64x8_mul_add!(-f2, c, f64x8_mulmod!(v, f0, n, ninv));
        write_f64x8!(xs0, i, u - c);
        write_f64x8!(xs2, i, f64x8_nmulmod!(u + c, f3, n, ninv));
    }
}

// ```
// k = 2, n = 2, z = 4, f = true
// [            2                2        -w^2             0]
// [         2//w            -2//w           0          -w^2]
// [1//2*r + 1//2   -1//2*r + 1//2   -1//2*w^2   -1//2*r*w^3]
// ```
//
// This is radix_4_moth_inv_trunc_block_2_4_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_2_4_1(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let pow = usize::power_of_2(j_bits);
    let j_mr = pow - 1 - j;
    let j_r = j & ((pow >> 1) - 1);
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let jp1 = j_bits + 1;
    let big_w = if j == 0 {
        -1.0
    } else {
        w2tab!(q, jp1, (j_mr << 1) + 1)
    };
    let rw = if j == 0 {
        w2tab!(q, 0, 1)
    } else {
        w2tab!(q, jp1, (j_r << 1) + 1)
    };
    let w = w2tab!(q, j_bits, j_r);
    let two_w = f64_reduce_pm1n_to_pmhn!(-2.0 * big_w, q.p);
    let rw3 = f64_mulmod!(w, rw, q.p, q.pinv);
    let f0 = f64x4::splat(2.0);
    let f1 = f64x4::splat(two_w); // 2*w^-1
    let f2 = f64x4::splat(fma!(-0.5f64, q.p, 0.5)); // 1/2
    let f3 = f64x4::splat(w2tab!(q, 0, 1)); // r
    let f4 = f64x4::splat(w); // w^2
    let f5 = f64x4::splat(f64_reduce_pm1n_to_pmhn!(rw3, q.p)); // r*w^3
    for i in (0..BLK_SZ).step_by(BIG_M) {
        let u = read_f64x4!(xs0, i);
        let v = read_f64x4!(xs1, i);
        let a = read_f64x4!(xs2, i);
        let b = read_f64x4!(xs3, i);
        let p = f64x4_mulmod!(a, f4, n, ninv);
        let q = f64x4_mulmod!(b, f4, n, ninv);
        let r = f64x4_mulmod!(b, f5, n, ninv);
        let s = f64x4_reduce_to_pm1n!(u + v, n, ninv);
        let t = u - v;
        write_f64x4!(xs0, i, f64x4_mulmod!(s, f0, n, ninv) - p);
        write_f64x4!(xs1, i, f64x4_mulmod!(t, f1, n, ninv) - q);
        write_f64x4!(
            xs2,
            i,
            f64x4_mulmod!((s + f64x4_mulmod!(t, f3, n, ninv)) - (p + r), f2, n, ninv)
        );
    }
}

// ```
// k = 2, n = 2, z = 4, f = false
// [   2       2   -w^2      0]
// [2//w   -2//w      0   -w^2]
// ```
//
// This is radix_4_moth_inv_trunc_block_2_4_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_2_4_0(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let pow = usize::power_of_2(j_bits);
    let j_mr = pow - 1 - j;
    let j_r = j & ((pow >> 1) - 1);
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let wi = if j == 0 {
        -1.0
    } else {
        w2tab!(q, 1 + j_bits, (j_mr << 1) + 1)
    };
    let w2 = f64x8::splat(w2tab!(q, j_bits, j_r));
    let twowi = f64x8::splat(f64_reduce_pm1n_to_pmhn!(-2.0 * wi, q.p));
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let b = read_f64x8!(xs1, i);
        let c = read_f64x8!(xs2, i);
        let d = read_f64x8!(xs3, i);
        let mut u = a + b;
        u += u;
        write_f64x8!(
            xs0,
            i,
            f64x8_reduce_to_pm1n!(u, n, ninv) - f64x8_mulmod!(c, w2, n, ninv)
        );
        write_f64x8!(
            xs1,
            i,
            f64x8_mulmod!(a - b, twowi, n, ninv) - f64x8_mulmod!(d, w2, n, ninv)
        );
    }
}

// ```
// k = 2, n = 2, z = 2, f = true
// [            2                2]
// [         2//w            -2//w]
// [1//2*r + 1//2   -1//2*r + 1//2]
//
// {x0, x1, x2} = {2*(x0 + x1), 2*w^-1*(x0 - x1), (x0+x1)/2 + (x0-x1)*i/2}
// ```
//
// This is radix_4_moth_inv_trunc_block_2_2_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_2_2_1(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1], xs2);
    let j_mr = usize::power_of_2(j_bits) - 1 - j;
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let w = if j == 0 {
        -1.0
    } else {
        w2tab!(q, 1 + j_bits, (j_mr << 1) + 1)
    };
    let c1 = f64x8::splat(f64_reduce_pm1n_to_pmhn!(-2.0 * w, q.p)); // 2/w
    let c2 = f64x8::splat(fma!(-0.5f64, q.p, 0.5)); // 1/2
    let c3 = f64x8::splat(w2tab!(q, 1, 0)); // r
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let mut u = read_f64x8!(xs0, i);
        let v = read_f64x8!(xs1, i);
        let s = u + v;
        let t = u - v;
        u = s + s;
        write_f64x8!(xs0, i, f64x8_reduce_to_pm1n!(u, n, ninv));
        write_f64x8!(xs1, i, f64x8_mulmod!(t, c1, n, ninv));
        write_f64x8!(
            xs2,
            i,
            f64x8_mulmod!(s + f64x8_mulmod!(t, c3, n, ninv), c2, n, ninv)
        );
    }
}

// ```
// k = 2, n = 2, z = 2, f = 0
//
// {x0, x1} = {2*(x0 + x1), 2*w^-1*(x0 - x1)}
//
// ```
//
// This is radix_4_moth_inv_trunc_block_2_2_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_2_2_0(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    fail_on_untested_path("radix_4_moth_inv_trunc_block_2_2_0");
    let (xs0, xs1) = xs.split_at_mut(gap);
    let j_mr = usize::power_of_2(j_bits) - 1 - j;
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let j_bits = usize::exact_from(j_bits);
    let w = if j == 0 {
        -1.0
    } else {
        w2tab!(q, 1 + j_bits, (j_mr << 1) + 1)
    };
    let c0 = f64x8::splat(2.0);
    let c1 = f64x8::splat(f64_reduce_pm1n_to_pmhn!(-2.0 * w, q.p));
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let u = read_f64x8!(xs0, i);
        let v = read_f64x8!(xs1, i);
        write_f64x8!(xs0, i, f64x8_mulmod!(u + v, c0, n, ninv));
        write_f64x8!(xs1, i, f64x8_mulmod!(u - v, c1, n, ninv));
    }
}

// ```
// k = 2, n = 1, z = 4, f = true
// [4        -w   -w^2        -w^3]
// [1   -1//2*w      0   -1//2*w^3]
// ```
//
// This is radix_4_moth_inv_trunc_block_1_4_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_1_4_1(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let j_r = j & ((usize::power_of_2(j_bits) >> 1) - 1);
    let j_bits = usize::exact_from(j_bits);
    let big_w2 = w2tab!(q, j_bits, j_r);
    let w = if j == 0 {
        1.0
    } else {
        w2tab!(q, 1 + j_bits, (j_r << 1))
    };
    let n = f64x4::splat(q.p);
    let ninv = f64x4::splat(q.pinv);
    let f2 = f64x4::splat(2.0);
    let w2 = f64x4::splat(big_w2);
    let ha = fma!(-0.5f64, q.p, 0.5);
    let ha_w = f64_mulmod!(w, ha, q.p, q.pinv);
    let wo2 = f64x4::splat(f64_reduce_pm1n_to_pmhn!(ha_w, q.p));
    for i in (0..BLK_SZ).step_by(BIG_M) {
        let mut a = read_f64x4!(xs0, i);
        a = f64x4_reduce_to_pm1n!(a, n, ninv);
        let mut b = read_f64x4!(xs1, i);
        let mut c = read_f64x4!(xs2, i);
        let mut d = read_f64x4!(xs3, i);
        c = f64x4_nmulmod!(c, w2, n, ninv);
        d = f64x4_mulmod!(d, w2, n, ninv);
        b = f64x4_mulmod!(b + d, wo2, n, ninv);
        let u = f64x4_mul_add!(-f2, a, b);
        write_f64x4!(xs0, i, c - f64x4_reduce_to_pm1n!(u + u, n, ninv));
        write_f64x4!(xs1, i, a - b);
    }
}

// ```
// k = 2, n = 1, z = 4, f = false
// [4   -w   -w^2   -w^3]
// ```
//
// This is radix_4_moth_inv_trunc_block_1_4_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_1_4_0(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let j_r = j & ((usize::power_of_2(j_bits) >> 1) - 1);
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let f1 = f64x8::splat(4.0);
    let j_bits = usize::exact_from(j_bits);
    let w2 = f64x8::splat(w2tab!(q, j_bits, j_r));
    let w = f64x8::splat(if j == 0 {
        1.0
    } else {
        w2tab!(q, 1 + j_bits, j_r << 1)
    });
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let b = read_f64x8!(xs1, i);
        let c = read_f64x8!(xs2, i);
        let d = read_f64x8!(xs3, i);
        write_f64x8!(
            xs0,
            i,
            f64x8_reduce_to_pm1n!(a * f1, n, ninv)
                - f64x8_mulmod!(b, w, n, ninv)
                - f64x8_mulmod!(c + f64x8_mulmod!(d, w, n, ninv), w2, n, ninv)
        );
    }
}

// ```
// k = 2, n = 1, z = 1, f = true
// [4]
// [1]
// ```
//
// This is radix_4_moth_inv_trunc_block_1_1_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_1_1_1(
    q: &FFTContext,
    _j: usize,
    _j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    fail_on_untested_path("radix_4_moth_inv_trunc_block_1_1_1");
    let (xs0, xs1) = xs.split_at_mut(gap);
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let f = f64x8::splat(4.0);
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        write_f64x8!(xs0, i, f64x8_reduce_to_pm1n!(f * a, n, ninv));
        write_f64x8!(xs1, i, f64x8_reduce_to_pm1n!(a, n, ninv));
    }
}

// ```
// k = 2, n = 1, z = 1, f = false
// [4]
// ```
//
// This is radix_4_moth_inv_trunc_block_1_1_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_1_1_0(
    q: &FFTContext,
    _j: usize,
    _j_bits: u64,
    xs: &mut [f64],
    _gap: usize,
) {
    fail_on_untested_path("radix_4_moth_inv_trunc_block_1_1_0");
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let f = f64x8::splat(4.0);
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs, i);
        write_f64x8!(xs, i, f64x8_reduce_to_pm1n!(f * a, n, ninv));
    }
}

// ```
// k = 2, n = 0, z = 4, f = true
// [1//4   1//4*w   1//4*w^2   1//4*w^3]
// ```
//
// This is radix_4_moth_inv_trunc_block_0_4_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_4_moth_inv_trunc_block_0_4_1(
    q: &FFTContext,
    j: usize,
    j_bits: u64,
    xs: &mut [f64],
    gap: usize,
) {
    split_into_chunks_mut!(xs, gap, [xs0, xs1, xs2], xs3);
    let j_r = j & ((usize::power_of_2(j_bits) >> 1) - 1);
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let one4th = f64x8::splat(fma!(-0.25f64, q.p, 0.25));
    let j_bits = usize::exact_from(j_bits);
    let w2 = f64x8::splat(w2tab!(q, j_bits, j_r));
    let w = f64x8::splat(if j == 0 {
        1.0
    } else {
        w2tab!(q, 1 + j_bits, j_r << 1)
    });
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let b = read_f64x8!(xs1, i);
        let c = read_f64x8!(xs2, i);
        let d = read_f64x8!(xs3, i);
        write_f64x8!(
            xs0,
            i,
            f64x8_mulmod!(
                a + f64x8_mulmod!(b, w, n, ninv)
                    + f64x8_mulmod!(c + f64x8_mulmod!(d, w, n, ninv), w2, n, ninv),
                one4th,
                n,
                ninv
            )
        );
    }
}

// This is sd_fft_ctx_w2 from fft_small.h, FLINT 3.3.0-dev.
fn sd_fft_ctx_w2(q: &FFTContext, j: usize) -> f64 {
    set_j_bits_and_j_r!(j, j_bits, j_r);
    w2tab!(q, usize::exact_from(j_bits), j_r)
}

// {x0, x1} = {2*x0 - w*x1, x0 - w*x1}
//
// This is radix_2_moth_inv_trunc_block_1_2_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_2_moth_inv_trunc_block_1_2_1(q: &FFTContext, j: usize, xs0: &mut [f64], xs1: &mut [f64]) {
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let w = f64x8::splat(sd_fft_ctx_w2(q, j));
    let c = f64x8::splat(2.0);
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let mut b = read_f64x8!(xs1, i);
        b = f64x8_nmulmod!(b, w, n, ninv);
        write_f64x8!(
            xs0,
            i,
            f64x8_reduce_to_pm1n!(f64x8_mul_add!(c, a, b), n, ninv)
        );
        write_f64x8!(xs1, i, f64x8_reduce_to_pm1n!(a + b, n, ninv));
    }
}

// {x0} = {2*x0 - w*x1}
//
// This is radix_2_moth_inv_trunc_block_1_2_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_2_moth_inv_trunc_block_1_2_0(q: &FFTContext, j: usize, xs0: &mut [f64], xs1: &mut [f64]) {
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let w = f64x8::splat(sd_fft_ctx_w2(q, j));
    let c = f64x8::splat(2.0);
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let b = read_f64x8!(xs1, i);
        write_f64x8!(
            xs0,
            i,
            f64x8_reduce_to_pm1n!(f64x8_mul_add!(c, a, -f64x8_mulmod!(b, w, n, ninv)), n, ninv)
        );
    }
}

// {x0, x1} = {2*x0, x0}
//
// This is radix_2_moth_inv_trunc_block_1_1_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_2_moth_inv_trunc_block_1_1_1(q: &FFTContext, _j: usize, xs0: &mut [f64], xs1: &mut [f64]) {
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let u = a + a;
        write_f64x8!(xs0, i, f64x8_reduce_to_pm1n!(u, n, ninv));
        write_f64x8!(xs1, i, a);
    }
}

// {x0} = {2*x0}
//
// This is radix_2_moth_inv_trunc_block_1_1_0 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_2_moth_inv_trunc_block_1_1_0(
    q: &FFTContext,
    _j: usize,
    xs0: &mut [f64],
    _xs1: &mut [f64],
) {
    fail_on_untested_path("radix_2_moth_inv_trunc_block_1_1_0");
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let mut a = read_f64x8!(xs0, i);
        a = a + a;
        write_f64x8!(xs0, i, f64x8_reduce_to_pm1n!(a, n, ninv));
    }
}

// {x0} = {(x0 + w*x1)/2}
//
// This is radix_2_moth_inv_trunc_block_0_2_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_2_moth_inv_trunc_block_0_2_1(q: &FFTContext, j: usize, xs0: &mut [f64], xs1: &mut [f64]) {
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let w = f64x8::splat(sd_fft_ctx_w2(q, j));
    let c = f64x8::splat(fma!(-0.5f64, q.p, 0.5));
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        let b = read_f64x8!(xs1, i);
        write_f64x8!(
            xs0,
            i,
            f64x8_mulmod!(a + f64x8_mulmod!(b, w, n, ninv), c, n, ninv)
        );
    }
}

// {x0} = {(x0)/2}
//
// This is radix_2_moth_inv_trunc_block_0_1_1 from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn radix_2_moth_inv_trunc_block_0_1_1(
    q: &FFTContext,
    _j: usize,
    xs0: &mut [f64],
    _xs1: &mut [f64],
) {
    fail_on_untested_path("radix_2_moth_inv_trunc_block_0_1_1");
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    let c = f64x8::splat(fma!(-0.5f64, q.p, 0.5));
    for i in (0..BLK_SZ).step_by(BIG_N) {
        let a = read_f64x8!(xs0, i);
        write_f64x8!(xs0, i, f64x8_mulmod!(a, c, n, ninv));
    }
}

const SD_IFFT_4_MOTH_TRUNC_BLOCK_TABLE: [Option<Sd4MothTruncBlockFn>; 40] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(radix_4_moth_inv_trunc_block_0_4_1),
    Some(radix_4_moth_inv_trunc_block_1_1_0),
    Some(radix_4_moth_inv_trunc_block_1_1_1),
    None,
    None,
    None,
    None,
    Some(radix_4_moth_inv_trunc_block_1_4_0),
    Some(radix_4_moth_inv_trunc_block_1_4_1),
    None,
    None,
    Some(radix_4_moth_inv_trunc_block_2_2_0),
    Some(radix_4_moth_inv_trunc_block_2_2_1),
    None,
    None,
    Some(radix_4_moth_inv_trunc_block_2_4_0),
    Some(radix_4_moth_inv_trunc_block_2_4_1),
    None,
    None,
    None,
    None,
    Some(radix_4_moth_inv_trunc_block_3_3_0),
    Some(radix_4_moth_inv_trunc_block_3_3_1),
    Some(radix_4_moth_inv_trunc_block_3_4_0),
    Some(radix_4_moth_inv_trunc_block_3_4_1),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

const SD_IFFT_2_MOTH_TRUNC_BLOCK_TABLE: [Option<Sd2MothTruncBlockFn>; 12] = [
    None,
    Some(radix_2_moth_inv_trunc_block_0_1_1),
    None,
    Some(radix_2_moth_inv_trunc_block_0_2_1),
    Some(radix_2_moth_inv_trunc_block_1_1_0),
    Some(radix_2_moth_inv_trunc_block_1_1_1),
    Some(radix_2_moth_inv_trunc_block_1_2_0),
    Some(radix_2_moth_inv_trunc_block_1_2_1),
    None,
    None,
    None,
    None,
];

// This is sd_ifft_trunc_block from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_trunc_block(
    q: &FFTContext,
    // data + BLK_SZ*I
    xs: &mut [f64],
    // stride
    s: usize,
    // BLK_SZ transforms each of length 2^k
    k: u64,
    j: usize,
    // actual trunc is z
    z: usize,
    // actual trunc is n
    n: usize,
    f: bool,
) {
    assert!(n <= z);
    let pow = usize::power_of_2(k);
    assert!(1 <= z && z <= pow);
    let sum = n + usize::from(f);
    assert!(1 <= sum && sum <= pow);
    if !f && z == n && n == pow {
        sd_ifft_no_trunc_block(q, xs, s, k, j);
        return;
    }
    let big_s = s << LG_BLK_SZ;
    if k == 2 {
        if let Some(fxn) =
            SD_IFFT_4_MOTH_TRUNC_BLOCK_TABLE[usize::from(f) + ((z - 1 + (n << 2)) << 1)]
        {
            fxn(q, j, j.significant_bits(), xs, big_s);
            return;
        }
    }
    if k > 1 {
        let k1 = k >> 1;
        let k2 = k - k1;
        let l2 = usize::power_of_2(k2);
        let n1 = n >> k2;
        let mask = l2 - 1;
        let n2 = n & mask;
        let z1 = z >> k2;
        let z2 = z & mask;
        let fp = f || n2 != 0;
        let z2p = min(l2, z);
        let m = min(n2, z2);
        let mp = max(n2, z2);
        // complete rows
        let big_s_2 = s << (k2 + LG_BLK_SZ);
        let shifted = j << k1;
        let mut p = 0;
        for b in 0..n1 {
            sd_ifft_no_trunc_block(q, &mut xs[p..], s, k2, shifted + b);
            p += big_s_2;
        }
        // rightmost columns
        let shifted2 = s << k2;
        let mut p = n2 * big_s;
        for a in n2..z2p {
            sd_ifft_trunc_block(
                q,
                &mut xs[p..],
                shifted2,
                k1,
                j,
                z1 + usize::from(a < mp),
                n1,
                fp,
            );
            p += big_s;
        }
        // last partial row
        if fp {
            sd_ifft_trunc_block(q, &mut xs[n1 * big_s_2..], s, k2, shifted + n1, z2p, n2, f);
        }
        let sum = n1 + 1;
        let mut p = 0;
        // leftmost columns
        for a in 0..n2 {
            sd_ifft_trunc_block(
                q,
                &mut xs[p..],
                shifted2,
                k1,
                j,
                z1 + usize::from(a < m),
                sum,
                false,
            );
            p += big_s;
        }
        return;
    }
    if k == 1 {
        if let Some(fxn) =
            SD_IFFT_2_MOTH_TRUNC_BLOCK_TABLE[usize::from(f) + ((z - 1 + (n << 1)) << 1)]
        {
            let (xs0, xs1) = xs.split_at_mut(big_s);
            fxn(q, j, xs0, xs1);
        }
    }
}

// This is sd_ifft_trunc_internal from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_trunc_internal(
    q: &FFTContext,
    // x = data + BLK_SZ*I  where I = starting index
    xs: &mut [f64],
    // stride
    s: usize,
    // transform length 2^(k + LG_BLK_SZ)
    k: u64,
    j: usize,
    // actual trunc is z*BLK_SZ
    z: usize,
    // actual trunc is n*BLK_SZ
    n: usize,
    f: bool,
) {
    assert!(n <= z);
    let big_z = z << LG_BLK_SZ;
    let pow = usize::power_of_2(k + LG_BLK_SZ);
    assert!(1 <= big_z && big_z <= pow);
    let uf = usize::from(f);
    let big_n = n << LG_BLK_SZ;
    assert!(1 <= big_n + uf && big_n + uf <= pow);
    if !f && z == n && n == usize::power_of_2(k) {
        sd_ifft_no_trunc_internal(q, xs, s, k, j);
        return;
    }
    let big_s = s << LG_BLK_SZ;
    if k > 2 {
        let k1 = k >> 1;
        let k2 = k - k1;
        let l2 = usize::power_of_2(k2);
        let n1 = n >> k2;
        let mask = l2 - 1;
        let n2 = n & mask;
        let z1 = z >> k2;
        let z2 = z & mask;
        let fp = f || n2 != 0;
        let z2p = min(l2, z);
        let m = min(n2, z2);
        let mp = max(n2, z2);
        // complete rows
        let shifted = j << k1;
        let big_s_2 = s << (k2 + LG_BLK_SZ);
        let mut p = 0;
        for b in 0..n1 {
            sd_ifft_no_trunc_internal(q, &mut xs[p..], s, k2, shifted + b);
            p += big_s_2;
        }
        // rightmost columns
        let shifted_2 = s << k2;
        let mut p = n2 * big_s;
        for a in n2..z2p {
            sd_ifft_trunc_block(
                q,
                &mut xs[p..],
                shifted_2,
                k1,
                j,
                z1 + usize::from(a < mp),
                n1,
                fp,
            );
            p += big_s;
        }
        // last partial row
        if fp {
            sd_ifft_trunc_internal(q, &mut xs[n1 * big_s_2..], s, k2, shifted + n1, z2p, n2, f);
        }
        // leftmost columns
        let sum = n1 + 1;
        let mut p = 0;
        for a in 0..n2 {
            sd_ifft_trunc_block(
                q,
                &mut xs[p..],
                shifted_2,
                k1,
                j,
                z1 + usize::from(a < m),
                sum,
                false,
            );
            p += big_s;
        }
        return;
    }
    if k == 2 {
        let four_j = j << 2;
        sd_ifft_base_8_1(q, xs, four_j);
        if n > 1 {
            sd_ifft_base_8_0(q, &mut xs[big_s..], four_j + 1);
        }
        if n > 2 {
            sd_ifft_base_8_0(q, &mut xs[big_s << 1..], four_j + 2);
        }
        if n > 3 {
            sd_ifft_base_8_0(q, &mut xs[big_s * 3..], four_j + 3);
        }
        sd_ifft_trunc_block(q, xs, s, 2, j, z, n, f);
        if f {
            sd_ifft_trunc_internal(q, &mut xs[big_s * n..], s, 0, four_j + n, 1, 0, f);
        }
    } else if k == 1 {
        fail_on_untested_path("sd_ifft_trunc_internal, k == 1");
        let two_j = j << 1;
        sd_ifft_base_8_1(q, xs, two_j);
        if n > 1 {
            sd_ifft_base_8_0(q, &mut xs[big_s..], two_j + 1);
        }
        sd_ifft_trunc_block(q, xs, s, 1, j, z, n, f);
        if f {
            sd_ifft_trunc_internal(q, &mut xs[big_s * n..], s, 0, two_j + n, 1, 0, f);
        }
    } else {
        assert!(!f);
        sd_ifft_base_8_1(q, xs, j);
    }
}

// This is sd_ifft_trunc from fft_small/sd_ifft.c, FLINT 3.3.0-dev.
fn sd_ifft_trunc(
    q: &mut FFTContext,
    d: &mut [f64],
    // convolution length 2^L
    l: u64,
    trunc: usize,
) {
    assert!(trunc <= usize::power_of_2(l));
    if l > LG_BLK_SZ {
        let new_trunc = trunc.div_round(BLK_SZ, Ceiling).0;
        sd_fft_ctx_fit_depth(q, l);
        sd_ifft_trunc_internal(q, d, 1, l - LG_BLK_SZ, 0, new_trunc, new_trunc, false);
        return;
    }
    fail_on_untested_path("sd_ifft_trunc, l <= LG_BLK_SZ");
    match l {
        0 => sd_ifft_basecase_0_1(q, d),
        1 => sd_ifft_basecase_1_1(q, d),
        2 => sd_ifft_basecase_2_1(q, d),
        3 => sd_ifft_basecase_3_1(q, d),
        4 => sd_ifft_basecase_4_1(q, d),
        5 => sd_ifft_basecase_5_1(q, d),
        6 => sd_ifft_basecase_6_1(q, d),
        7 => sd_ifft_basecase_7_1(q, d),
        8 => sd_ifft_basecase_8_1(q, d),
        _ => unreachable!(),
    }
}

// pointwise mul of a with b and m
//
// This is sd_fft_ctx_point_mul from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn sd_fft_ctx_point_mul(q: &FFTContext, a: &mut [f64], b: &[f64], m_orig: u64, depth: u64) {
    let m = f64x8::splat(f64_reduce_0n_to_pmhn!(m_orig as f64, q.p));
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    assert!(depth >= LG_BLK_SZ);
    let mut k = 0;
    for _ in 0..usize::power_of_2(depth - LG_BLK_SZ) {
        let ax = &mut a[k..];
        let bx = &b[k..];
        for j in (0..BLK_SZ).step_by(16) {
            let mut x0 = read_f64x8!(ax, j);
            let mut x1 = read_f64x8!(ax, j + 8);
            let b0 = read_f64x8!(bx, j);
            let b1 = read_f64x8!(bx, j + 8);
            x0 = f64x8_mulmod!(x0, m, n, ninv);
            x1 = f64x8_mulmod!(x1, m, n, ninv);
            x0 = f64x8_mulmod!(x0, b0, n, ninv);
            x1 = f64x8_mulmod!(x1, b1, n, ninv);
            write_f64x8!(ax, j, x0);
            write_f64x8!(ax, j + 8, x1);
        }
        k += BLK_SZ;
    }
}

// This is sd_fft_ctx_point_sqr from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn sd_fft_ctx_point_sqr(q: &FFTContext, a: &mut [f64], m_orig: u64, depth: u64) {
    let m = f64x8::splat(f64_reduce_0n_to_pmhn!(m_orig as f64, q.p));
    let n = f64x8::splat(q.p);
    let ninv = f64x8::splat(q.pinv);
    assert!(depth >= LG_BLK_SZ);
    let mut k = 0;
    for _ in 0..usize::power_of_2(depth - LG_BLK_SZ) {
        let ax = &mut a[k..];
        for j in (0..BLK_SZ).step_by(16) {
            let mut x0 = read_f64x8!(ax, j);
            let mut x1 = read_f64x8!(ax, j + 8);
            x0 = f64x8_mulmod!(x0, x0, n, ninv);
            x1 = f64x8_mulmod!(x1, x1, n, ninv);
            x0 = f64x8_mulmod!(x0, m, n, ninv);
            x1 = f64x8_mulmod!(x1, m, n, ninv);
            write_f64x8!(ax, j, x0);
            write_f64x8!(ax, j + 8, x1);
        }
        k += BLK_SZ;
    }
}

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
    if a[..q].iter().any(|&x| x != 0) {
        return Greater;
    }
    Less
}

// This is mod_worker_func from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn process_mod(
    to_ffts: MPNToFFTFunc,
    ffts: &[FFTContext],
    stride: usize,
    two_pow_tab: &[f64x4],
    abuf: &mut [f64],
    a: &[Limb],
    atrunc: usize,
    a_stop_easy: usize,
    a_start_hard: usize,
    a_stop_hard: usize,
    bbuf: &mut [f64],
    b: &[Limb],
    btrunc: usize,
    b_stop_easy: usize,
    b_start_hard: usize,
    b_stop_hard: usize,
    squaring: bool,
) {
    let f = match to_ffts {
        MPNToFFTFunc { np: 4, bits: 84 } => apply_mpn_to_fft_func_4_84,
        MPNToFFTFunc { np: 4, bits: 88 } => apply_mpn_to_fft_func_4_88,
        MPNToFFTFunc { np: 4, bits: 92 } => apply_mpn_to_fft_func_4_92,
        MPNToFFTFunc { np: 5, bits: 112 } => apply_mpn_to_fft_func_5_112,
        MPNToFFTFunc { np: 5, bits: 116 } => apply_mpn_to_fft_func_5_116,
        MPNToFFTFunc { np: 5, bits: 120 } => apply_mpn_to_fft_func_5_120,
        MPNToFFTFunc { np: 6, bits: 136 } => apply_mpn_to_fft_func_6_136,
        MPNToFFTFunc { np: 6, bits: 140 } => apply_mpn_to_fft_func_6_140,
        MPNToFFTFunc { np: 6, bits: 144 } => apply_mpn_to_fft_func_6_144,
        MPNToFFTFunc { np: 7, bits: 160 } => apply_mpn_to_fft_func_7_160,
        MPNToFFTFunc { np: 7, bits: 164 } => apply_mpn_to_fft_func_7_164,
        MPNToFFTFunc { np: 7, bits: 168 } => apply_mpn_to_fft_func_7_168,
        MPNToFFTFunc { np: 8, bits: 184 } => apply_mpn_to_fft_func_8_184,
        MPNToFFTFunc { np: 8, bits: 188 } => apply_mpn_to_fft_func_8_188,
        MPNToFFTFunc { np: 8, bits: 192 } => apply_mpn_to_fft_func_8_192,
        _ => unreachable!(),
    };
    f(
        ffts,
        abuf,
        stride,
        a,
        atrunc,
        two_pow_tab,
        a_stop_easy,
        a_start_hard,
        a_stop_hard,
    );
    if !squaring {
        f(
            ffts,
            bbuf,
            stride,
            b,
            btrunc,
            two_pow_tab,
            b_stop_easy,
            b_start_hard,
            b_stop_hard,
        );
    }
}

// This is NMOD_RED2 from src/nmod.h, FLINT 3.3.0-dev.
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

// This is fft_worker_func from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn process_fft(
    fctx: &mut FFTContext,
    cop: u64,
    depth: u64,
    ztrunc: usize,
    abuf: &mut [f64],
    atrunc: usize,
    bbuf: &mut [f64],
    btrunc: usize,
    squaring: bool,
) {
    let q = &mut *fctx;
    if !squaring {
        sd_fft_trunc(q, bbuf, depth, btrunc, ztrunc);
    }
    sd_fft_trunc(q, abuf, depth, atrunc, ztrunc);
    let m = nmod_red2!(cop >> (u64::WIDTH - depth), cop << depth, &q.mod_data)
        .mod_inverse(q.mod_data.n)
        .unwrap();
    if squaring {
        sd_fft_ctx_point_sqr(q, abuf, m, depth);
    } else {
        sd_fft_ctx_point_mul(q, abuf, bbuf, m, depth);
    }
    sd_ifft_trunc(q, abuf, depth, ztrunc);
}

// This is mod_fft_worker_func from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn process_mod_fft(
    bits: usize,
    fctx: &mut FFTContext,
    two_pow_tab: &[f64],
    cop: u64,
    depth: u64,
    ztrunc: usize,
    a: &[Limb],
    abuf: &mut [f64],
    atrunc: usize,
    b: &[Limb],
    bbuf: &mut [f64],
    btrunc: usize,
    squaring: bool,
) {
    let q = fctx;
    if !squaring {
        slow_mpn_to_fft(q, bbuf, btrunc, b, bits, two_pow_tab);
        sd_fft_trunc(q, bbuf, depth, btrunc, ztrunc);
    }
    slow_mpn_to_fft(q, abuf, atrunc, a, bits, two_pow_tab);
    sd_fft_trunc(q, abuf, depth, atrunc, ztrunc);
    let m = nmod_red2!(cop >> (u64::WIDTH - depth), cop << depth, &q.mod_data)
        .mod_inverse(q.mod_data.n)
        .unwrap();
    if squaring {
        sd_fft_ctx_point_sqr(q, abuf, m, depth);
    } else {
        sd_fft_ctx_point_mul(q, abuf, bbuf, m, depth);
    }
    sd_ifft_trunc(q, abuf, depth, ztrunc);
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

// This is multi_add_3 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_add_3{($z: expr, $a: expr) => {{
    let z = $z;
    let a = $a;
    (z[2], z[1], z[0]) = u64::xxx_add_yyy_to_zzz(z[2], z[1], z[0], a[2], a[1], a[0]);
}}}

// This is multi_add_4 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_add_4{($z: expr, $a: expr) => {{
    let z = $z;
    let a = $a;
    (z[3], z[2], z[1], z[0]) =
        u64::xxxx_add_yyyy_to_zzzz(z[3], z[2], z[1], z[0], a[3], a[2], a[1], a[0]);
}}}

// This is multi_add_5 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_add_5 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let carry_1 = z[0].overflowing_add_assign(a[0]);
        let mut carry_2 = z[1].overflowing_add_assign(a[1]);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a[2]);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a[3]);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        z[4].wrapping_add_assign(a[4]);
        if carry_4 {
            z[4].wrapping_add_assign(1);
        }
    }};
}

// This is multi_add_6 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_add_6 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let carry_1 = z[0].overflowing_add_assign(a[0]);
        let mut carry_2 = z[1].overflowing_add_assign(a[1]);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a[2]);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a[3]);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a[4]);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        z[5].wrapping_add_assign(a[5]);
        if carry_5 {
            z[5].wrapping_add_assign(1);
        }
    }};
}

// This is multi_add_7 from crt_helpers.h, FLINT 3.3.0-dev.
#[cfg(not(feature = "32_bit_limbs"))]
macro_rules! multi_add_7 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let carry_1 = z[0].overflowing_add_assign(a[0]);
        let mut carry_2 = z[1].overflowing_add_assign(a[1]);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a[2]);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a[3]);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a[4]);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        let mut carry_6 = z[5].overflowing_add_assign(a[5]);
        if carry_5 {
            carry_6 |= z[5].overflowing_add_assign(1);
        }
        z[6].wrapping_add_assign(a[6]);
        if carry_6 {
            z[6].wrapping_add_assign(1);
        }
    }};
}

// This is multi_add_8 from crt_helpers.h, FLINT 3.3.0-dev.
#[cfg(not(feature = "32_bit_limbs"))]
macro_rules! multi_add_8 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let carry_1 = z[0].overflowing_add_assign(a[0]);
        let mut carry_2 = z[1].overflowing_add_assign(a[1]);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a[2]);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a[3]);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a[4]);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        let mut carry_6 = z[5].overflowing_add_assign(a[5]);
        if carry_5 {
            carry_6 |= z[5].overflowing_add_assign(1);
        }
        let mut carry_7 = z[6].overflowing_add_assign(a[6]);
        if carry_6 {
            carry_7 |= z[6].overflowing_add_assign(1);
        }
        z[7].wrapping_add_assign(a[7]);
        if carry_7 {
            z[7].wrapping_add_assign(1);
        }
    }};
}

// This is multi_add_4 from crt_helpers.h, FLINT 3.3.0-dev.
#[cfg(feature = "32_bit_limbs")]
macro_rules! multi_add_4_alt {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let (a_1, a_0) = a[0].split_in_half();
        let (a_3, a_2) = a[1].split_in_half();
        let (a_5, a_4) = a[2].split_in_half();
        let (a_7, a_6) = a[3].split_in_half();
        let carry_1 = z[0].overflowing_add_assign(a_0);
        let mut carry_2 = z[1].overflowing_add_assign(a_1);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a_2);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a_3);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a_4);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        let mut carry_6 = z[5].overflowing_add_assign(a_5);
        if carry_5 {
            carry_6 |= z[5].overflowing_add_assign(1);
        }
        let mut carry_7 = z[6].overflowing_add_assign(a_6);
        if carry_6 {
            carry_7 |= z[6].overflowing_add_assign(1);
        }
        if 7 < z.len() {
            z[7].wrapping_add_assign(a_7);
            if carry_7 {
                z[7].wrapping_add_assign(1);
            }
        }
    }};
}

// This is multi_add_5 from crt_helpers.h, FLINT 3.3.0-dev.
#[cfg(feature = "32_bit_limbs")]
macro_rules! multi_add_5_alt {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let (a_1, a_0) = a[0].split_in_half();
        let (a_3, a_2) = a[1].split_in_half();
        let (a_5, a_4) = a[2].split_in_half();
        let (a_7, a_6) = a[3].split_in_half();
        let (a_9, a_8) = a[4].split_in_half();
        let carry_1 = z[0].overflowing_add_assign(a_0);
        let mut carry_2 = z[1].overflowing_add_assign(a_1);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a_2);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a_3);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a_4);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        let mut carry_6 = z[5].overflowing_add_assign(a_5);
        if carry_5 {
            carry_6 |= z[5].overflowing_add_assign(1);
        }
        let mut carry_7 = z[6].overflowing_add_assign(a_6);
        if carry_6 {
            carry_7 |= z[6].overflowing_add_assign(1);
        }
        let mut carry_8 = z[7].overflowing_add_assign(a_7);
        if carry_7 {
            carry_8 |= z[7].overflowing_add_assign(1);
        }
        let mut carry_9 = z[8].overflowing_add_assign(a_8);
        if carry_8 {
            carry_9 |= z[8].overflowing_add_assign(1);
        }
        if 9 < z.len() {
            z[9].wrapping_add_assign(a_9);
            if carry_9 {
                z[9].wrapping_add_assign(1);
            }
        }
    }};
}

// This is multi_add_6 from crt_helpers.h, FLINT 3.3.0-dev.
#[cfg(feature = "32_bit_limbs")]
macro_rules! multi_add_6_alt {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let (a_1, a_0) = a[0].split_in_half();
        let (a_3, a_2) = a[1].split_in_half();
        let (a_5, a_4) = a[2].split_in_half();
        let (a_7, a_6) = a[3].split_in_half();
        let (a_9, a_8) = a[4].split_in_half();
        let (a_11, a_10) = a[5].split_in_half();
        let carry_1 = z[0].overflowing_add_assign(a_0);
        let mut carry_2 = z[1].overflowing_add_assign(a_1);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a_2);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a_3);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a_4);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        let mut carry_6 = z[5].overflowing_add_assign(a_5);
        if carry_5 {
            carry_6 |= z[5].overflowing_add_assign(1);
        }
        let mut carry_7 = z[6].overflowing_add_assign(a_6);
        if carry_6 {
            carry_7 |= z[6].overflowing_add_assign(1);
        }
        let mut carry_8 = z[7].overflowing_add_assign(a_7);
        if carry_7 {
            carry_8 |= z[7].overflowing_add_assign(1);
        }
        let mut carry_9 = z[8].overflowing_add_assign(a_8);
        if carry_8 {
            carry_9 |= z[8].overflowing_add_assign(1);
        }
        let mut carry_10 = z[9].overflowing_add_assign(a_9);
        if carry_9 {
            carry_10 |= z[9].overflowing_add_assign(1);
        }
        let mut carry_11 = z[10].overflowing_add_assign(a_10);
        if carry_10 {
            carry_11 |= z[10].overflowing_add_assign(1);
        }
        if 11 < z.len() {
            z[11].wrapping_add_assign(a_11);
            if carry_11 {
                z[11].wrapping_add_assign(1);
            }
        }
    }};
}

// This is multi_add_7 from crt_helpers.h, FLINT 3.3.0-dev.
#[cfg(feature = "32_bit_limbs")]
macro_rules! multi_add_7_alt {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let (a_1, a_0) = a[0].split_in_half();
        let (a_3, a_2) = a[1].split_in_half();
        let (a_5, a_4) = a[2].split_in_half();
        let (a_7, a_6) = a[3].split_in_half();
        let (a_9, a_8) = a[4].split_in_half();
        let (a_11, a_10) = a[5].split_in_half();
        let (a_13, a_12) = a[6].split_in_half();
        let carry_1 = z[0].overflowing_add_assign(a_0);
        let mut carry_2 = z[1].overflowing_add_assign(a_1);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a_2);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a_3);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a_4);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        let mut carry_6 = z[5].overflowing_add_assign(a_5);
        if carry_5 {
            carry_6 |= z[5].overflowing_add_assign(1);
        }
        let mut carry_7 = z[6].overflowing_add_assign(a_6);
        if carry_6 {
            carry_7 |= z[6].overflowing_add_assign(1);
        }
        let mut carry_8 = z[7].overflowing_add_assign(a_7);
        if carry_7 {
            carry_8 |= z[7].overflowing_add_assign(1);
        }
        let mut carry_9 = z[8].overflowing_add_assign(a_8);
        if carry_8 {
            carry_9 |= z[8].overflowing_add_assign(1);
        }
        let mut carry_10 = z[9].overflowing_add_assign(a_9);
        if carry_9 {
            carry_10 |= z[9].overflowing_add_assign(1);
        }
        let mut carry_11 = z[10].overflowing_add_assign(a_10);
        if carry_10 {
            carry_11 |= z[10].overflowing_add_assign(1);
        }
        let mut carry_12 = z[11].overflowing_add_assign(a_11);
        if carry_11 {
            carry_12 |= z[11].overflowing_add_assign(1);
        }
        let mut carry_13 = z[12].overflowing_add_assign(a_12);
        if carry_12 {
            carry_13 |= z[12].overflowing_add_assign(1);
        }
        if 13 < z.len() {
            z[13].wrapping_add_assign(a_13);
            if carry_13 {
                z[13].wrapping_add_assign(1);
            }
        }
    }};
}

// This is multi_add_8 from crt_helpers.h, FLINT 3.3.0-dev.
#[cfg(feature = "32_bit_limbs")]
macro_rules! multi_add_8_alt {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let (a_1, a_0) = a[0].split_in_half();
        let (a_3, a_2) = a[1].split_in_half();
        let (a_5, a_4) = a[2].split_in_half();
        let (a_7, a_6) = a[3].split_in_half();
        let (a_9, a_8) = a[4].split_in_half();
        let (a_11, a_10) = a[5].split_in_half();
        let (a_13, a_12) = a[6].split_in_half();
        let (a_15, a_14) = a[7].split_in_half();
        let carry_1 = z[0].overflowing_add_assign(a_0);
        let mut carry_2 = z[1].overflowing_add_assign(a_1);
        if carry_1 {
            carry_2 |= z[1].overflowing_add_assign(1);
        }
        let mut carry_3 = z[2].overflowing_add_assign(a_2);
        if carry_2 {
            carry_3 |= z[2].overflowing_add_assign(1);
        }
        let mut carry_4 = z[3].overflowing_add_assign(a_3);
        if carry_3 {
            carry_4 |= z[3].overflowing_add_assign(1);
        }
        let mut carry_5 = z[4].overflowing_add_assign(a_4);
        if carry_4 {
            carry_5 |= z[4].overflowing_add_assign(1);
        }
        let mut carry_6 = z[5].overflowing_add_assign(a_5);
        if carry_5 {
            carry_6 |= z[5].overflowing_add_assign(1);
        }
        let mut carry_7 = z[6].overflowing_add_assign(a_6);
        if carry_6 {
            carry_7 |= z[6].overflowing_add_assign(1);
        }
        let mut carry_8 = z[7].overflowing_add_assign(a_7);
        if carry_7 {
            carry_8 |= z[7].overflowing_add_assign(1);
        }
        let mut carry_9 = z[8].overflowing_add_assign(a_8);
        if carry_8 {
            carry_9 |= z[8].overflowing_add_assign(1);
        }
        let mut carry_10 = z[9].overflowing_add_assign(a_9);
        if carry_9 {
            carry_10 |= z[9].overflowing_add_assign(1);
        }
        let mut carry_11 = z[10].overflowing_add_assign(a_10);
        if carry_10 {
            carry_11 |= z[10].overflowing_add_assign(1);
        }
        let mut carry_12 = z[11].overflowing_add_assign(a_11);
        if carry_11 {
            carry_12 |= z[11].overflowing_add_assign(1);
        }
        let mut carry_13 = z[12].overflowing_add_assign(a_12);
        if carry_12 {
            carry_13 |= z[12].overflowing_add_assign(1);
        }
        let mut carry_14 = z[13].overflowing_add_assign(a_13);
        if carry_13 {
            carry_14 |= z[13].overflowing_add_assign(1);
        }
        let mut carry_15 = z[14].overflowing_add_assign(a_14);
        if carry_14 {
            carry_15 |= z[14].overflowing_add_assign(1);
        }
        if 15 < z.len() {
            z[15].wrapping_add_assign(a_15);
            if carry_15 {
                z[15].wrapping_add_assign(1);
            }
        }
    }};
}

// This is multi_sub_4 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_sub_4 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let borrow_1 = z[0].overflowing_sub_assign(a[0]);
        let mut borrow_2 = z[1].overflowing_sub_assign(a[1]);
        if borrow_1 {
            borrow_2 |= z[1].overflowing_sub_assign(1);
        }
        let mut borrow_3 = z[2].overflowing_sub_assign(a[2]);
        if borrow_2 {
            borrow_3 |= z[2].overflowing_sub_assign(1);
        }
        z[3].wrapping_sub_assign(a[3]);
        if borrow_3 {
            z[3].wrapping_sub_assign(1);
        }
    }};
}

// This is multi_sub_5 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_sub_5 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let borrow_1 = z[0].overflowing_sub_assign(a[0]);
        let mut borrow_2 = z[1].overflowing_sub_assign(a[1]);
        if borrow_1 {
            borrow_2 |= z[1].overflowing_sub_assign(1);
        }
        let mut borrow_3 = z[2].overflowing_sub_assign(a[2]);
        if borrow_2 {
            borrow_3 |= z[2].overflowing_sub_assign(1);
        }
        let mut borrow_4 = z[3].overflowing_sub_assign(a[3]);
        if borrow_3 {
            borrow_4 |= z[3].overflowing_sub_assign(1);
        }
        z[4].wrapping_sub_assign(a[4]);
        if borrow_4 {
            z[4].wrapping_sub_assign(1);
        }
    }};
}

// This is multi_sub_6 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_sub_6 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let borrow_1 = z[0].overflowing_sub_assign(a[0]);
        let mut borrow_2 = z[1].overflowing_sub_assign(a[1]);
        if borrow_1 {
            borrow_2 |= z[1].overflowing_sub_assign(1);
        }
        let mut borrow_3 = z[2].overflowing_sub_assign(a[2]);
        if borrow_2 {
            borrow_3 |= z[2].overflowing_sub_assign(1);
        }
        let mut borrow_4 = z[3].overflowing_sub_assign(a[3]);
        if borrow_3 {
            borrow_4 |= z[3].overflowing_sub_assign(1);
        }
        let mut borrow_5 = z[4].overflowing_sub_assign(a[4]);
        if borrow_4 {
            borrow_5 |= z[4].overflowing_sub_assign(1);
        }
        z[5].wrapping_sub_assign(a[5]);
        if borrow_5 {
            z[5].wrapping_sub_assign(1);
        }
    }};
}

// This is multi_sub_7 from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! multi_sub_7 {
    ($z: expr, $a: expr) => {{
        let z = $z;
        let a = $a;
        let borrow_1 = z[0].overflowing_sub_assign(a[0]);
        let mut borrow_2 = z[1].overflowing_sub_assign(a[1]);
        if borrow_1 {
            borrow_2 |= z[1].overflowing_sub_assign(1);
        }
        let mut borrow_3 = z[2].overflowing_sub_assign(a[2]);
        if borrow_2 {
            borrow_3 |= z[2].overflowing_sub_assign(1);
        }
        let mut borrow_4 = z[3].overflowing_sub_assign(a[3]);
        if borrow_3 {
            borrow_4 |= z[3].overflowing_sub_assign(1);
        }
        let mut borrow_5 = z[4].overflowing_sub_assign(a[4]);
        if borrow_4 {
            borrow_5 |= z[4].overflowing_sub_assign(1);
        }
        let mut borrow_6 = z[5].overflowing_sub_assign(a[5]);
        if borrow_5 {
            borrow_6 |= z[5].overflowing_sub_assign(1);
        }
        z[6].wrapping_sub_assign(a[6]);
        if borrow_6 {
            z[6].wrapping_sub_assign(1);
        }
    }};
}

// This is _reduce_big_sum from crt_helpers.h, FLINT 3.3.0-dev.
macro_rules! reduce_big_sum {
    ($n: expr, $faddm1: ident, $fsub: ident, $r: ident, $t: ident, $limit: expr) => {
        let limit = $limit;
        $faddm1!(&mut $r[1..], &$t[1..]);
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
            $fsub!(&mut $r, limit);
        }
    };
}

// This is _add_to_answer_easy from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
macro_rules! add_to_answer_easy {
    (
        $n: expr,
        $fadd: ident,
        $fadd_alt: ident,
        $faddp1: ident,
        $fadd_altp1: ident,
        $z: ident,
        $r: ident,
        $toff: ident,
        $tshift: ident
    ) => {{
        #[cfg(feature = "32_bit_limbs")]
        let z_len = $z.len().shr_round(1, Ceiling).0;
        #[cfg(not(feature = "32_bit_limbs"))]
        let z_len = $z.len();
        assert!(z_len > $toff);
        if $tshift == 0 {
            #[cfg(feature = "32_bit_limbs")]
            $fadd_alt!(&mut $z[$toff << 1..], $r);
            #[cfg(not(feature = "32_bit_limbs"))]
            $fadd!(&mut $z[$toff..], $r);
        } else {
            let comp_shift = 64 - $tshift;
            $r[$n] = $r[$n - 1] >> comp_shift;
            for k in (2..=$n).rev() {
                $r[k - 1] = ($r[k - 1] << $tshift) | ($r[k - 2] >> comp_shift);
            }
            $r[0] <<= $tshift;
            #[cfg(feature = "32_bit_limbs")]
            $fadd_altp1!(&mut $z[$toff << 1..], $r);
            #[cfg(not(feature = "32_bit_limbs"))]
            $faddp1!(&mut $z[$toff..], $r);
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
    (
        $n: expr,
        $fadd: ident,
        $fadd_alt: ident,
        $faddp1: ident,
        $fadd_altp1: ident,
        $z: ident,
        $r: ident,
        $toff: ident,
        $tshift: ident
    ) => {{
        #[cfg(feature = "32_bit_limbs")]
        let z_len = $z.len().shr_round(1, Ceiling).0;
        #[cfg(not(feature = "32_bit_limbs"))]
        let z_len = $z.len();
        assert!(z_len > $toff);
        let mut do_add = true;
        if $tshift == 0 {
            if z_len - $toff >= const { $n as usize } {
                #[cfg(feature = "32_bit_limbs")]
                $fadd_alt!(&mut $z[$toff << 1..], $r);
                #[cfg(not(feature = "32_bit_limbs"))]
                $fadd!(&mut $z[$toff..], $r);
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
                $fadd_altp1!(&mut $z[$toff << 1..], $r);
                #[cfg(not(feature = "32_bit_limbs"))]
                $faddp1!(&mut $z[$toff..], $r);
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

macro_rules! f64x4_to_u64x4 {
    ($x: ident) => {{
        let [a, b, c, d] = $x.to_array();
        u64x4::from([
            round_even!(a) as u64,
            round_even!(b) as u64,
            round_even!(c) as u64,
            round_even!(d) as u64,
        ])
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
                let mut x2 = read_f64x4!(ds, j + const { 2 * VEC_SZ });
                let mut x3 = read_f64x4!(ds, j + const { 3 * VEC_SZ });
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
                write_f64x4!(xs_hi, j + const { 2 * VEC_SZ }, y2);
                write_f64x4!(xs_hi, j + const { 3 * VEC_SZ }, y3);
            }
            m += $dstride;
        }
    };
}

//  The "n" here is the limb count Rcrts[np-1].coeff_len, which is big enough to hold (product of
//  primes)*(number of primes), so it can hold the intermediate dot products f[0]*x[0] + ... +
//  f[np-1]*x[np-1]. The x[i] are single limb and the f[i] are of length "m". The number of primes
//  is "np".
//
//  The coefficient of X^i, 0 <= i < zlen needs to be reconstructed and added to the answer mpn (z,
//  zn). This involves the limbs
//
//  z[floor(i*bits/64)] ... z[floor(i*bits/64)+n]
//
//  so is easy if floor(i*bits/64)+n < zn.
//
//  The the l^th fft ctx Rffts[l] is expected to have data at d + l*dstride
//
//  handle output coefficients from [start_easy, zlen) end_easy is still expected to be valid
//
// This is _mpn_from_ffts and crt_worker_func from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
macro_rules! process_crt {
    (
        $f: ident,
        $np: expr,
        $n: expr,
        $m: expr,
        $faddm1: ident,
        $fadd: ident,
        $faddp1: ident,
        $fadd_alt: ident,
        $faddp1_alt: ident,
        $fsub: ident
    ) => {
        fn $f(
            zs: &mut [Limb],
            zlen: usize,
            rffts: &[FFTContext],
            ds: &[f64],
            dstride: usize,
            rcrts: &mut [CRTData],
            bits: usize,
            stop_easy: usize,
        ) {
            #[cfg(feature = "32_bit_limbs")]
            let zn_stop = zs.len().shr_round(1, Ceiling).0;
            #[cfg(not(feature = "32_bit_limbs"))]
            let zn_stop = zs.len();
            let rcrt = &mut rcrts[const {$np - 1}];
            assert_eq!($n, rcrt.coeff_len);
            if const{$n != $m} {
                for l in 0..$np {
                    assert_eq!(rcrt.co_prime(l)[$m], 0);
                }
            }
            zs.fill(0);
            let mut xs = [0; $np << LG_BLK_SZ];
            let mut r = [0; $n + 1];
            let mut t = [0; $n + 1];
            let mut k = 0;
            for i in (0..stop_easy).step_by(BLK_SZ) {
                convert_block!(xs, rffts, ds, dstride, $np, i >> LG_BLK_SZ);
                for j in 0..BLK_SZ {
                    big_mul!($n, $m, r, t, rcrt.co_prime(0), xs[j]);
                    let xs_hi = &xs[j..];
                    let mut m = BLK_SZ;
                    for l in 1..$np {
                        big_add_mul!($n, $m, r, t, rcrt.co_prime(l), xs_hi[m]);
                        m += BLK_SZ;
                    }
                    reduce_big_sum!($n, $faddm1, $fsub, r, t, rcrt.prod_primes_ref());
                    let toff = k >> u64::LOG_WIDTH;
                    let tshift = k & const {u64::WIDTH_MASK as usize};
                    assert!(zn_stop > $n + toff);
                    add_to_answer_easy!(
                        $n,
                        $fadd,
                        $fadd_alt,
                        $faddp1,
                        $faddp1_alt,
                        zs,
                        r,
                        toff,
                        tshift
                    );
                    k += bits;
                }
            }
            let mut j = stop_easy * bits;
            for i in stop_easy..zlen {
                let ds_hi = &ds[i..];
                let mut xx = ds_hi[0];
                let rfft = &rffts[0];
                let mut x = f64_reduce_to_0n!(xx, rfft.p, rfft.pinv) as u64;
                big_mul!($n, $m, r, t, rcrt.co_prime(0), x);
                let mut m = dstride;
                for l in 1..$np {
                    xx = ds_hi[m];
                    let rfft = &rffts[l];
                    x = f64_reduce_to_0n!(xx, rfft.p, rfft.pinv) as u64;
                    big_add_mul!($n, $m, r, t, rcrt.co_prime(l), x);
                    m += dstride;
                }
                reduce_big_sum!($n, $faddm1, $fsub, r, t, rcrt.prod_primes_ref());
                let toff = j >> u64::LOG_WIDTH;
                let tshift = j & const {u64::WIDTH_MASK as usize};
                if toff >= zn_stop {
                    break;
                }
                add_to_answer_hard!(
                    $n,
                    $fadd,
                    $fadd_alt,
                    $faddp1,
                    $faddp1_alt,
                    zs,
                    r,
                    toff,
                    tshift
                );
                j += bits;
            }
        }
    }
}
process_crt!(
    process_crt_4_4_3,
    4,
    4,
    3,
    multi_add_3,
    multi_add_4,
    multi_add_5,
    multi_add_4_alt,
    multi_add_5_alt,
    multi_sub_4
);
process_crt!(
    process_crt_5_4_4,
    5,
    4,
    4,
    multi_add_3,
    multi_add_4,
    multi_add_5,
    multi_add_4_alt,
    multi_add_5_alt,
    multi_sub_4
);
process_crt!(
    process_crt_6_5_4,
    6,
    5,
    4,
    multi_add_4,
    multi_add_5,
    multi_add_6,
    multi_add_5_alt,
    multi_add_6_alt,
    multi_sub_5
);
process_crt!(
    process_crt_7_6_5,
    7,
    6,
    5,
    multi_add_5,
    multi_add_6,
    multi_add_7,
    multi_add_6_alt,
    multi_add_7_alt,
    multi_sub_6
);
process_crt!(
    process_crt_8_7_6,
    8,
    7,
    6,
    multi_add_6,
    multi_add_7,
    multi_add_8,
    multi_add_7_alt,
    multi_add_8_alt,
    multi_sub_7
);

// This is mpn_ctx_mpn_mul from fft_small/mpn_mul.c, FLINT 3.3.0-dev.
fn mpn_ctx_mpn_mul(r: &mut Context, z: &mut [Limb], a: &[Limb], b: &[Limb], test_slow: bool) {
    let squaring = core::ptr::addr_eq(a as *const [Limb], b as *const [Limb]) && a.len() == b.len();
    let an = a.len();
    let bn = b.len();
    #[cfg(feature = "32_bit_limbs")]
    let an = an.shr_round(1, Ceiling).0;
    #[cfg(feature = "32_bit_limbs")]
    let bn = bn.shr_round(1, Ceiling).0;
    let mut p = ProfileEntry::default();
    mpn_ctx_best_profile(r, &mut p, an, bn);
    let zn = an + bn;
    let alen = usize::exact_from(
        u64::exact_from(an << u64::LOG_WIDTH)
            .div_round(p.bits, Ceiling)
            .0,
    );
    let blen = usize::exact_from(
        u64::exact_from(bn << u64::LOG_WIDTH)
            .div_round(p.bits, Ceiling)
            .0,
    );
    let zlen = alen + blen - 1;
    let atrunc = alen.round_to_multiple(BLK_SZ, Ceiling).0;
    let btrunc = blen.round_to_multiple(BLK_SZ, Ceiling).0;
    let ztrunc = zlen.round_to_multiple(BLK_SZ, Ceiling).0;
    let depth = max(LG_BLK_SZ, ztrunc.ceiling_log_base_2());
    let stride = usize::power_of_2(depth)
        .round_to_multiple_of_power_of_2(7, Ceiling)
        .0;
    assert_ne!(an, 0);
    assert_ne!(bn, 0);
    let coeff_len = CONTEXT.crts[p.np - 1].coeff_len;
    assert_ne!(
        flint_mpn_cmp_ui_2exp(
            &r.crts[p.np - 1].prod_primes_ref()[..coeff_len],
            blen as u64,
            p.bits << 1,
        ),
        Less
    );
    let abuf;
    // Normally, p.to_ffts is None only for extremely large arguments, which are impractical to
    // test, so we set a test_slow flag instead
    if p.to_ffts.is_none() || test_slow {
        mpn_ctx_fit_buffer(r, ((p.np + 1) * stride) << 3);
        let crt = &r.crts[p.np - 1];
        let bbuf;
        (abuf, bbuf) = r.buffer.split_at_mut(p.np * stride);
        let mut m = 0;
        for l in 0..p.np {
            process_mod_fft(
                p.bits as usize,
                &mut r.ffts[l],
                &r.slow_two_pow_backing[r.slow_two_pow_offsets[l]..],
                crt.co_prime_red(l),
                depth,
                ztrunc,
                a,
                &mut abuf[m..],
                atrunc,
                b,
                bbuf,
                btrunc,
                squaring,
            );
            m += stride;
        }
    } else {
        let bits = p.bits as usize;
        let an_64 = an << 6;
        let bn_64 = bn << 6;
        // if i*bits + 32 < 64*an, then the index into a is always in bounds
        let mut a_stop_easy = min(atrunc, (an_64 - 33) / bits);
        // if i*bits >= 64*an, then the index into a is always out of bounds
        let a_stop_hard = min(atrunc, an_64.div_ceil(bits));
        // ditto
        let mut b_stop_easy = min(btrunc, (bn_64 - 33) / bits);
        let b_stop_hard = min(btrunc, bn_64.div_ceil(bits));
        let rounding: usize = if bits & 7 == 0 {
            4
        } else if bits & 3 == 0 {
            8
        } else {
            16
        };
        let prod = p.np * stride;
        mpn_ctx_fit_buffer(r, prod << 4);
        let bbuf;
        (abuf, bbuf) = r.buffer.split_at_mut(prod);
        // some fixups for loop unrollings: round down the easy stops
        assert!(bits.even());
        a_stop_easy &= rounding.wrapping_neg();
        b_stop_easy &= rounding.wrapping_neg();
        process_mod(
            p.to_ffts.unwrap(),
            &r.ffts,
            stride,
            &r.vec_two_pow_tab_backing
                [r.vec_two_pow_tab_offsets[p.np.div_round(VEC_SZ, Ceiling).0 - 1]..],
            abuf,
            a,
            atrunc,
            a_stop_easy.round_to_multiple(rounding, Ceiling).0,
            a_stop_easy,
            a_stop_hard,
            bbuf,
            b,
            btrunc,
            b_stop_easy.round_to_multiple(rounding, Ceiling).0,
            b_stop_easy,
            b_stop_hard,
            squaring,
        );
        let crt = &r.crts[p.np - 1];
        let mut m = 0;
        for l in 0..p.np {
            process_fft(
                &mut r.ffts[l],
                crt.co_prime_red(l),
                depth,
                ztrunc,
                &mut abuf[m..],
                atrunc,
                &mut bbuf[m..],
                btrunc,
                squaring,
            );
            m += stride;
        }
    }
    let n = r.crts[p.np - 1].coeff_len;
    let mut end_easy = ((if zn > n { zn - (n + 1) } else { 0 }) << 6) / (p.bits as usize);
    // this is how much space was statically allocated in each struct
    assert!(n <= MPN_CTX_NCRTS);
    end_easy &= BLK_SZ.wrapping_neg();
    assert!(4 <= p.np && p.np <= 8);
    let process_crt = match p.np {
        4 => process_crt_4_4_3,
        5 => process_crt_5_4_4,
        6 => process_crt_6_5_4,
        7 => process_crt_7_6_5,
        8 => process_crt_8_7_6,
        _ => unreachable!(),
    };
    process_crt(
        z,
        zlen,
        &r.ffts,
        abuf,
        stride,
        &mut r.crts,
        p.bits as usize,
        end_easy.round_to_multiple(BLK_SZ, Ceiling).0,
    );
}

// This is mpn_mul_default_mpn_ctx from fft_small/default_ctx.c, FLINT 3.3.0-dev.
pub(crate) fn mpn_mul_default_mpn_ctx(r1: &mut [Limb], i1: &[Limb], i2: &[Limb], test_slow: bool) {
    let mut context = CONTEXT.deserialize();
    mpn_ctx_mpn_mul(&mut context, r1, i1, i2, test_slow);
}

pub(crate) fn mpn_square_default_mpn_ctx(r1: &mut [Limb], i1: &[Limb]) {
    let mut context = CONTEXT.deserialize();
    mpn_ctx_mpn_mul(&mut context, r1, i1, i1, false);
}
