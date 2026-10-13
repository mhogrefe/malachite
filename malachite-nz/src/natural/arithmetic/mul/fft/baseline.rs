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

// The vector types and primitive operations the FFT kernels in `body.rs` use, as `wide` provides
// them for the target features the crate is compiled with. `avx2.rs` is the same interface over
// AVX2 and FMA intrinsics, for x86 builds that detect those features at run time.

use wide::{f64x4, f64x8, u64x4};

// This is vec4d_reduce_pm1n_to_pmhn from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_pm1n_to_pmhn {
    ($a: expr, $n: expr) => {{
        let a = $a;
        let n = $n;
        // The two conditions exclude each other, so the selects may be applied in either order, and
        // this gives the same results as the branches of `f64_reduce_pm1n_to_pmhn`.
        let half_n = f64x4::splat(0.5) * n;
        let a_plus_n = a + n;
        let a = a.simd_gt(half_n).select(a - n, a);
        a_plus_n.simd_lt(half_n).select(a_plus_n, a)
    }};
}

// This is vec8d_reduce_pm1n_to_pmhn from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x8_reduce_pm1n_to_pmhn {
    ($a: expr, $n: expr) => {{
        let a = $a;
        let n = $n;
        let half_n = f64x8::splat(0.5) * n;
        let a_plus_n = a + n;
        let a = a.simd_gt(half_n).select(a - n, a);
        a_plus_n.simd_lt(half_n).select(a_plus_n, a)
    }};
}

// return a mod n in [0,n) assuming a in (-n,n)
//
// This is vec4d_reduce_pm1no_to_0n from machine_vectors.h, FLINT 3.3.0-dev.
macro_rules! f64x4_reduce_pm1no_to_0n {
    ($a: expr, $n: expr) => {{
        let a = $a;
        a.simd_lt(f64x4::ZERO).select(a + $n, a)
    }};
}

macro_rules! f64x4_round {
    ($x: expr) => {{
        let [x0, x1, x2, x3] = $x.to_array();
        f64x4::from([round_even!(x0), round_even!(x1), round_even!(x2), round_even!(x3)])
    }};
}

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

#[cfg(feature = "test_build")]
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

// The table of powers of two, as the kernels take it. The table is stored as `wide` vectors, which
// are this backend's vectors, so there is nothing to do.
macro_rules! two_pow_slice {
    ($s: expr) => {
        $s
    };
}

#[path = "body.rs"]
mod body;

pub(super) use body::{fmpz_poly_mul_mid_mpn_ctx, mpn_ctx_mpn_mul};
