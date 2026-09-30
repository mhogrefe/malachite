// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 Jason Moxham
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::mul::{
    limbs_mul_same_length_to_out, limbs_mul_same_length_to_out_scratch_len,
};
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::arithmetic::sub::limbs_sub_same_length_to_out;
use crate::platform::Limb;
use alloc::vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `xs[..n]` plus the returned carry times $2^b$ to `xs[..n]` times `ys[..n]` (or `xs[..n]`
// squared, if `ys` is `None`) modulo $2^b + 1$, where `n` is $\lceil b/\text{W} \rceil$ and W is
// `Limb::WIDTH`. The inputs and outputs are fully reduced, and `tp` needs `2 * n` limbs.
//
// This is flint_mpn_mulmod_2expp1_internal from mpn_extras/mulmod_2expp1_basecase.c, FLINT 3.6.0,
// where `xp` and `yp` are the same.
fn limbs_mul_mod_2expp1_internal(
    xs: &mut [Limb],
    ys: Option<&[Limb]>,
    b: u64,
    tp: &mut [Limb],
) -> Limb {
    let n = usize::exact_from(b.div_ceil(Limb::WIDTH));
    let k = (u64::exact_from(n) << Limb::LOG_WIDTH) - b;
    let tp = &mut tp[..n << 1];
    if let Some(ys) = ys {
        let mut scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(n)];
        limbs_mul_same_length_to_out(tp, &xs[..n], &ys[..n], &mut scratch);
    } else {
        let mut scratch = vec![0; limbs_square_to_out_scratch_len(n)];
        limbs_square_to_out(tp, &xs[..n], &mut scratch);
    }
    let (tp_lo, tp_hi) = tp.split_at_mut(n);
    if k == 0 {
        let c = limbs_sub_same_length_to_out(&mut xs[..n], tp_lo, tp_hi);
        return Limb::from(limbs_slice_add_limb_in_place(&mut xs[..n], Limb::from(c)));
    }
    let c = tp_lo[n - 1];
    tp_lo[n - 1] &= Limb::MAX >> k;
    let c1 = limbs_slice_shl_in_place(tp_hi, k);
    tp_hi[0] |= c >> (Limb::WIDTH - k);
    let c = Limb::from(limbs_sub_same_length_to_out(&mut xs[..n], tp_lo, tp_hi)) + c1;
    let c = Limb::from(limbs_slice_add_limb_in_place(&mut xs[..n], c));
    xs[n - 1] &= Limb::MAX >> k;
    c
}

// Sets `xs[..n]` plus the returned carry times $2^b$ to `xs` times `ys` (or `xs` squared, if `ys`
// is `None`) modulo $2^b + 1$, where `n` is $\lceil b/\text{W} \rceil$ and W is `Limb::WIDTH`. Each
// input is given by its low `n` limbs and a top bit: bit 1 of `c` is the top bit of `xs`, and bit 0
// is the top bit of `ys`. The inputs must be fully reduced, and `tp` needs `2 * n` limbs.
//
// This is flint_mpn_mulmod_2expp1_basecase from mpn_extras/mulmod_2expp1_basecase.c, FLINT 3.6.0,
// where `xp` and `yp` are the same.
crate_test_fn! {limbs_mul_mod_2expp1_basecase(
    xs: &mut [Limb],
    ys: Option<&[Limb]>,
    c: Limb,
    b: u64,
    tp: &mut [Limb],
) -> Limb {
    let cy = c & 2;
    let cz = c & 1;
    let n = usize::exact_from(b.div_ceil(Limb::WIDTH));
    let k = (u64::exact_from(n) << Limb::LOG_WIDTH) - b;
    if cy == 0 {
        if cz == 0 {
            limbs_mul_mod_2expp1_internal(xs, ys, b, tp)
        } else {
            let c = Limb::from(limbs_neg_in_place(&mut xs[..n]));
            let c = Limb::from(limbs_slice_add_limb_in_place(&mut xs[..n], c));
            xs[n - 1] &= Limb::MAX >> k;
            c
        }
    } else if cz == 0 {
        let c = if let Some(ys) = ys {
            xs[..n].copy_from_slice(&ys[..n]);
            Limb::from(limbs_neg_in_place(&mut xs[..n]))
        } else {
            Limb::from(limbs_neg_in_place(&mut xs[..n]))
        };
        let c = Limb::from(limbs_slice_add_limb_in_place(&mut xs[..n], c));
        xs[n - 1] &= Limb::MAX >> k;
        c
    } else {
        xs[0] = 1;
        xs[1..n].fill(0);
        0
    }
}}
