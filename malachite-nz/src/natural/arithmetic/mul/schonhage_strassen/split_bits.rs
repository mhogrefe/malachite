// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2011, 2020 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;

// Splits `limbs[..total_limbs]` into coefficients of `coeff_limbs` limbs each, the last possibly
// shorter, storing them, zero-extended to `output_limbs + 1` limbs, in `poly`, and returns the
// number of coefficients.
//
// This is fft_split_limbs from fft/split_bits.c, FLINT 3.6.0, without the threading.
crate_test_fn! {fft_split_limbs(
    poly: &mut [Vec<Limb>],
    limbs: &[Limb],
    total_limbs: usize,
    coeff_limbs: usize,
    output_limbs: usize,
) -> usize {
    let length = (total_limbs - 1) / coeff_limbs + 1;
    let num = total_limbs / coeff_limbs;
    for (i, p) in poly[..num].iter_mut().enumerate() {
        let skip = i * coeff_limbs;
        p[..=output_limbs].fill(0);
        p[..coeff_limbs].copy_from_slice(&limbs[skip..skip + coeff_limbs]);
    }
    let i = num;
    let skip = i * coeff_limbs;
    if i < length {
        poly[i][..=output_limbs].fill(0);
    }
    if total_limbs > skip {
        poly[i][..total_limbs - skip].copy_from_slice(&limbs[skip..total_limbs]);
    }
    length
}}

// Splits `limbs[..total_limbs]` into coefficients of `bits` bits each, the last possibly shorter,
// storing them, zero-extended to `output_limbs + 1` limbs, in `poly`, and returns the number of
// coefficients.
//
// This is fft_split_bits from fft/split_bits.c, FLINT 3.6.0, without the threading.
crate_test_fn! {fft_split_bits(
    poly: &mut [Vec<Limb>],
    limbs: &[Limb],
    total_limbs: usize,
    bits: u64,
    output_limbs: usize,
) -> usize {
    let length = usize::exact_from(
        ((u64::exact_from(total_limbs) << Limb::LOG_WIDTH) - 1) / bits + 1,
    );
    let top_bits = bits & Limb::WIDTH_MASK;
    if top_bits == 0 {
        return fft_split_limbs(
            poly,
            limbs,
            total_limbs,
            usize::exact_from(bits >> Limb::LOG_WIDTH),
            output_limbs,
        );
    }
    let coeff_limbs = usize::exact_from(bits >> Limb::LOG_WIDTH) + 1;
    let mask = Limb::low_mask(top_bits);
    for (i, p) in poly[..length - 1].iter_mut().enumerate() {
        p[..=output_limbs].fill(0);
        let i_top_bits = u64::exact_from(i) * top_bits;
        let mut limb_ptr = i * (coeff_limbs - 1) + usize::exact_from(i_top_bits >> Limb::LOG_WIDTH);
        let mut shift_bits = i_top_bits & Limb::WIDTH_MASK;
        if shift_bits == 0 {
            p[..coeff_limbs].copy_from_slice(&limbs[limb_ptr..limb_ptr + coeff_limbs]);
            p[coeff_limbs - 1] &= mask;
        } else {
            limbs_shr_to_out(
                &mut p[..coeff_limbs],
                &limbs[limb_ptr..limb_ptr + coeff_limbs],
                shift_bits,
            );
            limb_ptr += coeff_limbs - 1;
            shift_bits += top_bits;
            if shift_bits >= Limb::WIDTH {
                limb_ptr += 1;
                p[coeff_limbs - 1] += limbs[limb_ptr] << (Limb::WIDTH - (shift_bits - top_bits));
            }
            p[coeff_limbs - 1] &= mask;
        }
    }
    let i = length - 1;
    let i_top_bits = u64::exact_from(i) * top_bits;
    let limb_ptr = i * (coeff_limbs - 1) + usize::exact_from(i_top_bits >> Limb::LOG_WIDTH);
    let shift_bits = i_top_bits & Limb::WIDTH_MASK;
    let p = &mut poly[i];
    p[..=output_limbs].fill(0);
    let limbs_left = total_limbs - limb_ptr;
    if shift_bits == 0 {
        p[..limbs_left].copy_from_slice(&limbs[limb_ptr..total_limbs]);
    } else {
        limbs_shr_to_out(&mut p[..limbs_left], &limbs[limb_ptr..total_limbs], shift_bits);
    }
    length
}}
