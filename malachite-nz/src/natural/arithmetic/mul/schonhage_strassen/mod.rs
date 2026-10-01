// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2011 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{add_with_carry, limbs_slice_add_limb_in_place};
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::sub::{limbs_sub_limb_in_place, sub_with_carry};
use crate::platform::{Limb, SignedLimb};
use malachite_base::num::conversion::traits::WrappingFrom;

// Schönhage–Strassen multiplication, as FLINT implements it: transforms of vectors of residues
// modulo $2^{wn} + 1$, where each residue is `limbs + 1` limbs, `limbs = wn / Limb::WIDTH`, and the
// top limb holds a small signed overflow. Multiplication by powers of 2 in this ring is a shift and
// a negation, so the transforms need no multiplications.

pub mod adjust;
pub mod adjust_sqrt2;
pub mod butterfly_lsh_b;
pub mod butterfly_rsh_b;
pub mod combine_bits;
pub mod convolution;
pub mod div_2expmod_2expp1;
pub mod fft_mfa_truncate_sqrt2;
pub mod fft_mfa_truncate_sqrt2_inner;
pub mod fft_negacyclic;
pub mod fft_radix2;
pub mod fft_truncate;
pub mod fft_truncate_sqrt2;
pub mod ifft_mfa_truncate_sqrt2;
pub mod ifft_negacyclic;
pub mod ifft_radix2;
pub mod ifft_truncate;
pub mod ifft_truncate_sqrt2;
pub mod mul_2expmod_2expp1;
pub mod mulmod_2expp1;
pub mod mulmod_2expp1_basecase;
pub mod negmod_2expp1;
pub mod normmod_2expp1;
pub mod split_bits;

// Adds `c`, a signed number in two's complement, to the residue `r[..=limbs]` modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`.
//
// This is mpn_addmod_2expp1_1 from fft.h, FLINT 3.6.0.
crate_test_fn! {limbs_add_signed_limb_mod_2expp1(r: &mut [Limb], limbs: usize, c: Limb) {
    let sum = r[0].wrapping_add(c);
    // Check whether adding c would cause a carry to propagate.
    if SignedLimb::wrapping_from(sum ^ r[0]) >= 0 {
        r[0] = sum;
    } else if SignedLimb::wrapping_from(c) >= 0 {
        limbs_slice_add_limb_in_place(&mut r[..=limbs], c);
    } else {
        limbs_sub_limb_in_place(&mut r[..=limbs], c.wrapping_neg());
    }
}}

// Sets the first `n` limbs of `s` to the sum, and of `d` to the difference, of the first `n` limbs
// of `x` and `y`, and returns twice the carry plus the borrow.
//
// This is flint_mpn_sumdiff_n from mpn_extras/sumdiff_n.c, FLINT 3.6.0, where the outputs are
// separate from the inputs, and fft_sumdiff from fft.h.
crate_test_fn! {limbs_sum_diff(
    s: &mut [Limb],
    d: &mut [Limb],
    x: &[Limb],
    y: &[Limb],
    n: usize,
) -> Limb {
    let (s, d, x, y) = (&mut s[..n], &mut d[..n], &x[..n], &y[..n]);
    // One pass over the inputs. Within each 4-limb block the sum's carry chain and then the
    // difference's run over the same loaded limbs, so that LLVM keeps each chain in the flags for
    // the length of a block. The difference is computed in carry form (see `sub_with_carry`): its
    // carry is the negated borrow.
    let mut carry = false;
    let mut diff_carry = true;
    let (s_blocks, s_rem) = s.as_chunks_mut::<4>();
    let (d_blocks, d_rem) = d.as_chunks_mut::<4>();
    let (x_blocks, x_rem) = x.as_chunks::<4>();
    let (y_blocks, y_rem) = y.as_chunks::<4>();
    for (((s, d), x), y) in s_blocks.iter_mut().zip(d_blocks).zip(x_blocks).zip(y_blocks) {
        for i in 0..4 {
            (s[i], carry) = add_with_carry(x[i], y[i], carry);
        }
        for i in 0..4 {
            (d[i], diff_carry) = sub_with_carry(x[i], y[i], diff_carry);
        }
    }
    for (((s, d), &x), &y) in s_rem.iter_mut().zip(d_rem).zip(x_rem).zip(y_rem) {
        (*s, carry) = add_with_carry(x, y, carry);
        (*d, diff_carry) = sub_with_carry(x, y, diff_carry);
    }
    (Limb::from(carry) << 1) | Limb::from(!diff_carry)
}}

// Sets the first `xs.len()` limbs of `out` to the negation of `xs` modulo $2^{\text{W}\cdot n}$,
// and returns 1 if `xs` is nonzero and 0 otherwise.
//
// This is mpn_neg from gmp.h, GMP 6.2.1, where the output is separate from the input.
pub(crate) fn limbs_neg_to_out(out: &mut [Limb], xs: &[Limb]) -> Limb {
    let out = &mut out[..xs.len()];
    out.copy_from_slice(xs);
    Limb::from(limbs_neg_in_place(out))
}
