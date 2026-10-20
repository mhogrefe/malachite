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

use crate::integer_matrix::arithmetic::mul::ProductShape;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul::karatsuba::revbin;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mul::schonhage_strassen::combine_bits::fft_combine_bits;
use crate::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_truncate_sqrt2::fft_truncate_sqrt2;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_mfa_truncate_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_truncate_sqrt2::ifft_truncate_sqrt2;
use crate::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1_basecase::*;
use crate::natural::arithmetic::mul::schonhage_strassen::negmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::limbs_norm_mod_2expp1;
use crate::natural::arithmetic::mul::schonhage_strassen::split_bits::fft_split_bits;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::sub::limbs_sub_limb_to_out;
use crate::natural::{LIMB_HIGH_BIT, Natural};
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, DivisibleBy, PowerOf2, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;

// Sets `z` to the low `z.len()` limbs of the two's complement evaluation at $2^b$, where $b$ is
// `bits`, of the polynomial with coefficients `a`, each a normalized residue modulo $m =
// 2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, viewed in the signed range $(-m/2,
// m/2)$. Returns the lowest bit of the next limb of the two's complement evaluation. `t` is scratch
// of `limbs + 1` limbs.
//
// This is `fft_combine_bits_signed` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0, with the fix to the
// test for negative coefficients that FLINT made after 3.6.0.
fn fft_combine_bits_signed(
    z: &mut [Limb],
    a: &[Vec<Limb>],
    bits: u64,
    limbs: usize,
    t: &mut [Limb],
) -> Limb {
    let zn = z.len();
    let mut f: Limb = 0;
    let mut zout = 0;
    for (i, a) in a.iter().enumerate() {
        // add the ith coeffs a[i]
        let shift = bits * u64::exact_from(i);
        let q = usize::exact_from(shift >> Limb::LOG_WIDTH);
        let r = shift & Limb::WIDTH_MASK;
        // A residue above $2^{N-1}$, where $N$ is `limbs * Limb::WIDTH`, stands for a negative
        // coefficient. FLINT 3.6.0 tests only `a[limbs - 1] > LIMB_HIGH_BIT`, which misreads a
        // negative coefficient whose absolute value is within $2^{N-\text{W}}$ of $2^{N-1}$ as
        // positive; the choice of `bits` lets coefficients come that close, so its
        // `fmpz_mat_mul_fft` returns wrong results for some inputs. This is the test after FLINT's
        // fix, as for `_fmpz_vec_set_fft`.
        let top = a[limbs - 1];
        let s: Limb = if a[limbs] != 0
            || top > LIMB_HIGH_BIT
            || (top == LIMB_HIGH_BIT && !slice_test_zero(&a[..limbs - 1]))
        {
            limbs_sub_limb_to_out(t, &a[..limbs], 1);
            1
        } else {
            t[..limbs].copy_from_slice(&a[..limbs]);
            0
        };
        t[limbs] = s.wrapping_neg();
        // t[0] ... t[limbs] is now the twos complement signed version of a[i] and the shift occurs
        // without overflow
        if r != 0 {
            limbs_slice_shl_in_place(t, r);
        }
        if q < zn {
            let new_zout = min(zn, q + limbs + 1);
            while zout < new_zout {
                z[zout] = f.wrapping_neg();
                zout += 1;
            }
            f ^= s;
            f ^= Limb::from(limbs_slice_add_same_length_in_place_left(
                &mut z[q..new_zout],
                &t[..new_zout - q],
            ));
        } else {
            if q == zn {
                f ^= t[0] & 1;
            }
            break;
        }
    }
    while zout < zn {
        z[zout] = f.wrapping_neg();
        zout += 1;
    }
    f
}

// Splits `|x|` into coefficients of `bits` bits each, stored in `poly` as residues modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, negating each if `x` is negative,
// and returns the number of coefficients.
//
// This is `fft_split_bits_fmpz` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0.
fn fft_split_bits_fmpz<C: PolynomialCoefficient>(
    poly: &mut [Vec<Limb>],
    x: &C,
    bits: u64,
    limbs: usize,
) -> usize {
    let xs = x.unsigned_abs_ref().as_limbs_asc();
    if xs.is_empty() {
        return 0;
    }
    let len = fft_split_bits(poly, xs, xs.len(), bits, limbs);
    if x.is_negative() {
        for p in &mut poly[..len] {
            limbs_neg_mod_2expp1_in_place(p, limbs);
        }
    }
    len
}

// Returns the evaluation at $2^b$, where $b$ is `bits`, of the polynomial with the coefficients
// `poly`, which are normalized residues modulo $2^{\text{limbs}\cdot\text{W}} + 1$, where W is
// `Limb::WIDTH`, viewed as signed if `sign`. The result must fit in `total_limbs` limbs. `t` is
// scratch of `limbs + 1` limbs.
//
// This is `fft_combine_bits_fmpz` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0.
fn fft_combine_bits_fmpz<C: PolynomialCoefficient>(
    poly: &[Vec<Limb>],
    bits: u64,
    limbs: usize,
    total_limbs: usize,
    sign: bool,
    t: &mut [Limb],
) -> C {
    let mut d = vec![0; total_limbs];
    if sign {
        if fft_combine_bits_signed(&mut d, poly, bits, limbs, t) != 0 {
            limbs_neg_in_place(&mut d);
            return C::from_sign_and_abs(false, Natural::from_owned_limbs_asc(d));
        }
    } else {
        fft_combine_bits(&mut d, poly, poly.len(), bits, limbs, total_limbs);
    }
    C::from_sign_and_abs(true, Natural::from_owned_limbs_asc(d))
}

// Scratch residues for the transforms: two that they swap with the residues they transform, and one
// more.
struct Temporaries {
    t1: Vec<Limb>,
    t2: Vec<Limb>,
    t3: Vec<Limb>,
}

// Applies the forward transform to `coeffs`, with the matrix Fourier algorithm if `use_mfa`, and
// normalizes the outputs that the pointwise products read.
//
// This is `_either_fft_or_mfa` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0.
fn either_fft_or_mfa(
    coeffs: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    temps: &mut Temporaries,
    n1: usize,
    depth: u64,
    trunc: usize,
    limbs: usize,
    use_mfa: bool,
) {
    let Temporaries { t1, t2, t3 } = temps;
    if use_mfa {
        fft_mfa_truncate_sqrt2(coeffs, n, w, t1, t2, t3, n1, trunc);
        for c in &mut coeffs[..n << 1] {
            limbs_norm_mod_2expp1(c, limbs);
        }
        // the second half is out of order
        let trunc2 = (trunc - (n << 1)) / n1;
        for s in 0..trunc2 {
            let rs = revbin(s, depth - (depth >> 1) + 1);
            for u in 0..n1 {
                limbs_norm_mod_2expp1(&mut coeffs[(n << 1) + rs * n1 + u], limbs);
            }
        }
    } else {
        fft_truncate_sqrt2(coeffs, n, w, t1, t2, t3, trunc);
        for c in &mut coeffs[..trunc] {
            limbs_norm_mod_2expp1(c, limbs);
        }
    }
}

// Sets `c` to the sum of the products of the residues `a(i)` and `b(i)`, for `i` from 0 to `len`,
// modulo $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`. `t` must have `limbs + 1`
// limbs, and `scratch` must have `limbs_mul_mod_2expp1_basecase_scratch_len(limbs)`.
//
// This is `_dot` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0.
fn dot<'a>(
    c: &mut [Limb],
    a: impl Fn(usize) -> &'a [Limb],
    b: impl Fn(usize) -> &'a [Limb],
    len: usize,
    limbs: usize,
    t: &mut [Limb],
    scratch: &mut [Limb],
) {
    assert_ne!(len, 0);
    // n * w
    let nw = u64::exact_from(limbs) << Limb::LOG_WIDTH;
    for i in 0..len {
        let (x, y) = (a(i), b(i));
        let carry = (x[limbs] << 1) + y[limbs];
        if i == 0 {
            c[..limbs].copy_from_slice(&x[..limbs]);
            c[limbs] = limbs_mul_mod_2expp1_basecase(c, Some(y), carry, nw, scratch);
        } else {
            t[..limbs].copy_from_slice(&x[..limbs]);
            t[limbs] = limbs_mul_mod_2expp1_basecase(t, Some(y), carry, nw, scratch);
            c[limbs].wrapping_add_assign(t[limbs]);
            let carry = limbs_slice_add_same_length_in_place_left(&mut c[..limbs], &t[..limbs]);
            c[limbs].wrapping_add_assign(Limb::from(carry));
            limbs_norm_mod_2expp1(c, limbs);
        }
    }
}

// Returns the residues of the elements `xs` of a matrix, `4 * n` per element: each element is split
// into coefficients of `bits` bits, padded with zeros, and transformed.
fn transformed_elements<C: PolynomialCoefficient>(
    xs: &[C],
    n: usize,
    w: u64,
    bits: u64,
    temps: &mut Temporaries,
    sqrt: usize,
    depth: u64,
    trunc: usize,
    limbs: usize,
    use_mfa: bool,
) -> Vec<Vec<Limb>> {
    let size = limbs + 1;
    let mut coeffs = vec![vec![0; size]; xs.len() * (n << 2)];
    for (x, coeffs) in xs.iter().zip(coeffs.chunks_exact_mut(n << 2)) {
        // chop inputs into bits
        let h = fft_split_bits_fmpz(coeffs, x, bits, limbs);
        for c in &mut coeffs[h..] {
            c.fill(0);
        }
        either_fft_or_mfa(coeffs, n, w, temps, sqrt, depth, trunc, limbs, use_mfa);
    }
    coeffs
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, whose elements
// have at most `a_bits` and `b_bits` significant bits, by viewing each element as a polynomial
// evaluated at $2^b$ and multiplying the matrices of transforms of these polynomials pointwise. The
// transforms have length `4 * 2^depth` and use the root of unity $\sqrt{2}^w$. If `sign`, some
// element is negative.
//
// This is `_fmpz_mat_mul_truncate_sqrt2` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0.
fn matrix_mul_truncate_sqrt2<C: PolynomialCoefficient>(
    xs: &[C],
    a_bits: u64,
    ys: &[C],
    b_bits: u64,
    shape: ProductShape,
    depth: u64,
    w: u64,
    j1: usize,
    j2: usize,
    use_mfa: bool,
    sign: bool,
) -> Vec<C> {
    let ProductShape {
        ar: m,
        br: k,
        bc: n_cols,
    } = shape;
    let clg_k = u64::exact_from(k).ceiling_log_base_2() + u64::from(sign);
    let n = usize::power_of_2(depth);
    let nw = u64::exact_from(n) * w;
    let bits1 = (nw - (depth + 1 + clg_k)) >> 1;
    // This bound on c_limbs is also ok with the original clg_k = ceil(log_2(k))
    let c_limbs = usize::exact_from((a_bits + b_bits + clg_k).div_ceil(Limb::WIDTH));
    let limbs = usize::exact_from(nw >> Limb::LOG_WIDTH);
    let size = limbs + 1;
    assert_ne!(limbs, 0);
    assert_eq!(u64::exact_from(limbs) << Limb::LOG_WIDTH, nw);
    // should have k * min(j1, j2) * 2^bits1 <= 2^(n * w)
    assert!(j1 <= n << 1 || j2 <= n << 1);
    assert!(j1 > 0);
    assert!(j2 > 0);
    assert!(j1 + j2 - 1 <= n << 2);
    let mut trunc = max(j1 + j2 - 1, (n << 1) + 1);
    let sqrt = if use_mfa {
        let sqrt = usize::power_of_2(depth >> 1);
        trunc = (trunc + (sqrt << 1) - 1) & (sqrt << 1).wrapping_neg();
        sqrt
    } else {
        trunc = (trunc + 1) & 2usize.wrapping_neg();
        1
    };
    assert!(trunc > n << 1);
    assert!(trunc.divisible_by(sqrt << 1));
    let mut temps = Temporaries {
        t1: vec![0; size],
        t2: vec![0; size],
        t3: vec![0; size],
    };
    let a_coeffs = transformed_elements(
        xs, n, w, bits1, &mut temps, sqrt, depth, trunc, limbs, use_mfa,
    );
    let b_coeffs = transformed_elements(
        ys, n, w, bits1, &mut temps, sqrt, depth, trunc, limbs, use_mfa,
    );
    let mut c_coeffs = vec![vec![0; size]; n << 2];
    let mut scratch = vec![0; size + limbs_mul_mod_2expp1_basecase_scratch_len(limbs)];
    let (t, scratch) = scratch.split_at_mut(size);
    let four_n = n << 2;
    let mut out = Vec::with_capacity(m * n_cols);
    // pointwise dot products for C[i, j]
    for i in 0..m {
        for j in 0..n_cols {
            let mut dot_at = |l: usize, c: &mut [Limb]| {
                dot(
                    c,
                    |h| &a_coeffs[(i * k + h) * four_n + l],
                    |h| &b_coeffs[(h * n_cols + j) * four_n + l],
                    k,
                    limbs,
                    t,
                    scratch,
                );
            };
            let Temporaries { t1, t2, t3 } = &mut temps;
            if use_mfa {
                for (l, c) in c_coeffs[..n << 1].iter_mut().enumerate() {
                    dot_at(l, c);
                }
                // second half is out of order
                let trunc2 = (trunc - (n << 1)) / sqrt;
                for s in 0..trunc2 {
                    let rs = revbin(s, depth - (depth >> 1) + 1);
                    for u in 0..sqrt {
                        let l = (n << 1) + rs * sqrt + u;
                        dot_at(l, &mut c_coeffs[l]);
                    }
                }
                ifft_mfa_truncate_sqrt2(&mut c_coeffs, n, w, t1, t2, t3, sqrt, trunc);
            } else {
                for (l, c) in c_coeffs[..trunc].iter_mut().enumerate() {
                    dot_at(l, c);
                }
                ifft_truncate_sqrt2(&mut c_coeffs, n, w, t1, t2, t3, trunc);
            }
            // pointwise scalar division by 4 * n
            for c in &mut c_coeffs[..trunc] {
                limbs_div_2exp_mod_2expp1_in_place(c, limbs, depth + 2);
                limbs_norm_mod_2expp1(c, limbs);
            }
            out.push(fft_combine_bits_fmpz(
                &c_coeffs[..j1 + j2 - 1],
                bits1,
                limbs,
                c_limbs,
                sign,
                t,
            ));
        }
    }
    out
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, whose elements
// have at most `a_bits` and `b_bits` significant bits, both positive, choosing the parameters of
// the transforms. If `sign`, some element is negative.
//
// The inputs are viewed as polynomials evaluated at $2^b$, with all coefficients in $[0, 2^b)$ or
// all in $(-2^b, 0]$. The product of the matrices of these polynomials has coefficients bounded in
// absolute value by $k 2^{d+1} 2^{2b}$, where $k$ is the inner dimension and $d$ the depth, which
// must be less than $2^{nw}$ in the unsigned case and less than $2^{nw}/2$ in the signed case.
//
// This is `_fmpz_mat_mul_fft` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0.
fn matrix_mul_fft_with_bits<C: PolynomialCoefficient>(
    xs: &[C],
    a_bits: u64,
    ys: &[C],
    b_bits: u64,
    shape: ProductShape,
    sign: bool,
) -> Vec<C> {
    let clg_k = u64::exact_from(shape.br).ceiling_log_base_2() + u64::from(sign);
    let mut depth = 6;
    let mut w = 1;
    let mut n = u64::power_of_2(depth);
    let bits_for = |n: u64, w: u64, depth: u64| (n * w - (depth + 1 + clg_k)) >> 1;
    let mut bits = bits_for(n, w, depth);
    let bits1 = max(a_bits, 2000);
    let bits2 = max(b_bits, 2000);
    // when viewed as eval of poly at x = 2^bits, the max length of the entries
    let mut j1 = bits1.div_ceil(bits);
    // ditto for B
    let mut j2 = bits2.div_ceil(bits);
    assert!(j1 + j2 - 1 > n << 1);
    // find initial n, w
    while j1 + j2 - 1 > n << 2 {
        if w == 1 {
            w = 2;
        } else {
            depth += 1;
            w = 1;
            n <<= 1;
        }
        bits = bits_for(n, w, depth);
        j1 = bits1.div_ceil(bits);
        j2 = bits2.div_ceil(bits);
    }
    // FLINT then tries smaller values of `w` when the depth is less than 11, in steps of $2^{6 -
    // d}$ for depths $d$ below 6 and of 1 otherwise, and recomputes `bits`, `j1`, and `j2`. The
    // depth is never below 6, and the loop above leaves `w` at 2 only when `w = 1` failed at the
    // same depth, so the adjustment never changes anything and is omitted.
    let use_mfa = depth >= 11;
    matrix_mul_truncate_sqrt2(
        xs,
        a_bits,
        ys,
        b_bits,
        shape,
        depth,
        w,
        usize::exact_from(j1),
        usize::exact_from(j2),
        use_mfa,
        sign,
    )
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, both in
// row-major order, by multiplying matrices of Schönhage–Strassen transforms of the elements
// pointwise.
//
// This is equivalent to `fmpz_mat_mul_fft` from `fmpz_mat/mul_fft.c`, FLINT 3.6.0, with the fix to
// the test for negative coefficients that FLINT made after 3.6.0; see `fft_combine_bits_signed`.
// `Mul` does not use it: FLINT's `fmpz_mat_mul` does not either, and it was slower than `Mul` at
// every size measured, so it is kept here as a reference algorithm.
pub fn matrix_mul_fft<C: PolynomialCoefficient>(xs: &[C], ys: &[C], shape: ProductShape) -> Vec<C> {
    let ProductShape { ar, br, bc } = shape;
    if ar == 0 || br == 0 || bc == 0 {
        return vec![C::ZERO; ar * bc];
    }
    let (a_bits, a_negative) = vec_max_bits(xs);
    let (b_bits, b_negative) = vec_max_bits(ys);
    if a_bits == 0 || b_bits == 0 {
        return vec![C::ZERO; ar * bc];
    }
    matrix_mul_fft_with_bits(xs, a_bits, ys, b_bits, shape, a_negative || b_negative)
}
