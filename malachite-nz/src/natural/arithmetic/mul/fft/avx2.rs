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

// The vector types and primitive operations the FFT kernels use, over AVX2 and FMA intrinsics. This
// is the same interface as `baseline.rs`, for x86 builds that lack those features at compile time:
// the kernels in `avx2_body.rs` (`body.rs` with `#[target_feature(enable = "avx2,fma")]` on every
// function) are compiled against it, and the parent module runs them only after `is_supported` has
// checked the CPU.
//
// Inside a function compiled with the features, the intrinsics are safe to call, so the kernels and
// the methods below are safe code. The exceptions are the operator impls: trait methods cannot
// carry `#[target_feature]`, so they call the intrinsics in `unsafe` blocks, whose SAFETY argument
// is the same for all of them. A value of these types can only be made by the `#[target_feature]`
// constructors, so one exists only in code that is already running with AVX2 and FMA available, and
// the operators run there too.

#![allow(unsafe_code)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Whether the running CPU supports the features `avx2_body.rs` is compiled with.
pub(super) fn is_supported() -> bool {
    std::is_x86_feature_detected!("avx2") && std::is_x86_feature_detected!("fma")
}

const ROUND_TO_NEAREST: i32 = _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC;

// Four `f64`s in one 256-bit register, with the operations the kernels use from `wide::f64x4`.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub(super) struct f64x4(__m256d);

// SAFETY: a transparent wrapper of `__m256d`, which is `Pod`.
unsafe impl bytemuck::Zeroable for f64x4 {}
unsafe impl bytemuck::Pod for f64x4 {}

impl f64x4 {
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn splat(x: f64) -> Self {
        Self(_mm256_set1_pd(x))
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn new(xs: [f64; 4]) -> Self {
        Self(bytemuck::cast(xs))
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn from(xs: [f64; 4]) -> Self {
        Self::new(xs)
    }

    // Reads a vector from 4 elements of a slice.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn from_slice(xs: &[f64]) -> Self {
        Self(bytemuck::pod_read_unaligned(bytemuck::cast_slice(xs)))
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn to_array(self) -> [f64; 4] {
        bytemuck::cast(self.0)
    }

    // Returns `self * m + a`, rounded once.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn mul_add(self, m: Self, a: Self) -> Self {
        Self(_mm256_fmadd_pd(self.0, m.0, a.0))
    }

    // Rounds each element to the nearest integer, ties to even.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn round(self) -> Self {
        Self(_mm256_round_pd::<ROUND_TO_NEAREST>(self.0))
    }

    // [-n, n] -> [-n/2, n/2]. The two conditions exclude each other, so the blends may be applied
    // in either order, and this gives the same results as the branches of
    // `f64_reduce_pm1n_to_pmhn`.
    //
    // This is vec4d_reduce_pm1n_to_pmhn from machine_vectors.h, FLINT 3.3.0-dev.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn reduce_pm1n_to_pmhn(self, n: Self) -> Self {
        let a = self.0;
        let half_n = _mm256_mul_pd(_mm256_set1_pd(0.5), n.0);
        let a_minus_n = _mm256_sub_pd(a, n.0);
        let a_plus_n = _mm256_add_pd(a, n.0);
        let a = _mm256_blendv_pd(a, a_minus_n, _mm256_cmp_pd::<_CMP_GT_OQ>(a, half_n));
        Self(_mm256_blendv_pd(
            a,
            a_plus_n,
            _mm256_cmp_pd::<_CMP_LT_OQ>(a_plus_n, half_n),
        ))
    }

    // (-n, n) -> [0, n), as the branch of `f64_reduce_pm1no_to_0n` does.
    //
    // This is vec4d_reduce_pm1no_to_0n from machine_vectors.h, FLINT 3.3.0-dev.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn reduce_pm1no_to_0n(self, n: Self) -> Self {
        let a = self.0;
        let a_plus_n = _mm256_add_pd(a, n.0);
        Self(_mm256_blendv_pd(
            a,
            a_plus_n,
            _mm256_cmp_pd::<_CMP_LT_OQ>(a, _mm256_setzero_pd()),
        ))
    }

    // (u0, u2, v0, v2)
    //
    // This is vec4d_unpack_lo_permute_0_2_1_3 from machine_vectors.h, FLINT 3.3.0-dev.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn unpack_lo_permute_0_2_1_3(u: Self, v: Self) -> Self {
        Self(_mm256_permute4x64_pd::<0xd8>(_mm256_unpacklo_pd(u.0, v.0)))
    }

    // (u1, u3, v1, v3)
    //
    // This is vec4d_unpack_hi_permute_0_2_1_3 from machine_vectors.h, FLINT 3.3.0-dev.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn unpack_hi_permute_0_2_1_3(u: Self, v: Self) -> Self {
        Self(_mm256_permute4x64_pd::<0xd8>(_mm256_unpackhi_pd(u.0, v.0)))
    }

    // Transposes the 4x4 matrix whose rows are the four vectors, in place.
    //
    // This is VEC4D_TRANSPOSE from machine_vectors.h, FLINT 3.3.0-dev.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn transpose(a0: &mut Self, a1: &mut Self, a2: &mut Self, a3: &mut Self) {
        // (a00, a10, a02, a12), (a01, a11, a03, a13), and the same for the other two rows
        let t0 = _mm256_unpacklo_pd(a0.0, a1.0);
        let t1 = _mm256_unpackhi_pd(a0.0, a1.0);
        let t2 = _mm256_unpacklo_pd(a2.0, a3.0);
        let t3 = _mm256_unpackhi_pd(a2.0, a3.0);
        // 0x20 joins the low halves and 0x31 the high halves.
        *a0 = Self(_mm256_permute2f128_pd::<0x20>(t0, t2));
        *a1 = Self(_mm256_permute2f128_pd::<0x20>(t1, t3));
        *a2 = Self(_mm256_permute2f128_pd::<0x31>(t0, t2));
        *a3 = Self(_mm256_permute2f128_pd::<0x31>(t1, t3));
    }
}

impl Default for f64x4 {
    #[inline]
    fn default() -> Self {
        Self(bytemuck::Zeroable::zeroed())
    }
}

impl Add for f64x4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        // SAFETY: see the module comment.
        Self(unsafe { _mm256_add_pd(self.0, other.0) })
    }
}

impl Sub for f64x4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        // SAFETY: see the module comment.
        Self(unsafe { _mm256_sub_pd(self.0, other.0) })
    }
}

impl Mul for f64x4 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        // SAFETY: see the module comment.
        Self(unsafe { _mm256_mul_pd(self.0, other.0) })
    }
}

impl Neg for f64x4 {
    type Output = Self;

    // Flips the sign bits, as negating an `f64` does.
    #[inline(always)]
    fn neg(self) -> Self {
        // SAFETY: see the module comment.
        Self(unsafe { _mm256_xor_pd(self.0, _mm256_set1_pd(-0.0)) })
    }
}

impl AddAssign for f64x4 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for f64x4 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for f64x4 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

// Eight `f64`s in two 256-bit registers, with the operations the kernels use from `wide::f64x8`.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct f64x8(f64x4, f64x4);

impl f64x8 {
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn splat(x: f64) -> Self {
        let half = f64x4::splat(x);
        Self(half, half)
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn from(xs: [f64; 8]) -> Self {
        let [lo, hi]: [[f64; 4]; 2] = bytemuck::cast(xs);
        Self(f64x4::new(lo), f64x4::new(hi))
    }

    // Reads a vector from 8 elements of a slice.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn from_slice(xs: &[f64]) -> Self {
        Self(f64x4::from_slice(&xs[..4]), f64x4::from_slice(&xs[4..8]))
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn to_array(self) -> [f64; 8] {
        bytemuck::cast([self.0.to_array(), self.1.to_array()])
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn mul_add(self, m: Self, a: Self) -> Self {
        Self(self.0.mul_add(m.0, a.0), self.1.mul_add(m.1, a.1))
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn round(self) -> Self {
        Self(self.0.round(), self.1.round())
    }

    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn reduce_pm1n_to_pmhn(self, n: Self) -> Self {
        Self(
            self.0.reduce_pm1n_to_pmhn(n.0),
            self.1.reduce_pm1n_to_pmhn(n.1),
        )
    }
}

impl Add for f64x8 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for f64x8 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul for f64x8 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl Neg for f64x8 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl AddAssign for f64x8 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for f64x8 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for f64x8 {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

// Four `u64`s, as `f64x4_to_u64x4` produces them. Only written out, so an array is enough.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct u64x4([u64; 4]);

impl u64x4 {
    // Rounds each element to the nearest integer and converts it to a `u64`, as `f64x4_to_u64x4`
    // does for the baseline.
    #[inline]
    #[target_feature(enable = "avx2,fma")]
    pub(super) fn from_f64x4(x: f64x4) -> Self {
        let [a, b, c, d] = x.round().to_array();
        Self([a as u64, b as u64, c as u64, d as u64])
    }

    #[inline]
    pub(super) const fn to_array(self) -> [u64; 4] {
        self.0
    }
}

macro_rules! f64x4_reduce_pm1n_to_pmhn {
    ($a: expr, $n: expr) => {
        f64x4::reduce_pm1n_to_pmhn($a, $n)
    };
}

macro_rules! f64x8_reduce_pm1n_to_pmhn {
    ($a: expr, $n: expr) => {
        f64x8::reduce_pm1n_to_pmhn($a, $n)
    };
}

macro_rules! f64x4_reduce_pm1no_to_0n {
    ($a: expr, $n: expr) => {
        f64x4::reduce_pm1no_to_0n($a, $n)
    };
}

macro_rules! f64x4_round {
    ($x: expr) => {
        f64x4::round($x)
    };
}

macro_rules! f64x8_round {
    ($x: expr) => {
        f64x8::round($x)
    };
}

macro_rules! f64x4_mul_add {
    ($a: expr, $b: expr, $c: expr) => {
        f64x4::mul_add($a, $b, $c)
    };
}

macro_rules! f64x8_mul_add {
    ($a: expr, $b: expr, $c: expr) => {
        f64x8::mul_add($a, $b, $c)
    };
}

macro_rules! read_f64x4 {
    ($xs: expr, $i: expr) => {{
        let i = $i;
        f64x4::from_slice(&$xs[i..i + 4])
    }};
}

macro_rules! read_f64x8 {
    ($xs: expr, $i: expr) => {{
        let i = $i;
        f64x8::from_slice(&$xs[i..i + 8])
    }};
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

macro_rules! f64x4_unpack_lo_permute_0_2_1_3 {
    ($u: ident, $v: ident) => {
        f64x4::unpack_lo_permute_0_2_1_3($u, $v)
    };
}

macro_rules! f64x4_unpack_hi_permute_0_2_1_3 {
    ($u: ident, $v: ident) => {
        f64x4::unpack_hi_permute_0_2_1_3($u, $v)
    };
}

macro_rules! f64x4_transpose {
    ($a0: ident, $a1: ident, $a2: ident, $a3: ident) => {
        f64x4::transpose(&mut $a0, &mut $a1, &mut $a2, &mut $a3)
    };
}

macro_rules! f64x4_to_u64x4 {
    ($x: ident) => {
        u64x4::from_f64x4($x)
    };
}

// The table of powers of two, as the kernels take it. The table is stored as `wide` vectors, which
// have the same layout as this backend's.
macro_rules! two_pow_slice {
    ($s: expr) => {
        bytemuck::cast_slice::<wide::f64x4, f64x4>($s)
    };
}

#[path = "avx2_body.rs"]
mod body;

pub(super) use body::{fmpz_poly_mul_mid_mpn_ctx, mpn_ctx_mpn_mul};
