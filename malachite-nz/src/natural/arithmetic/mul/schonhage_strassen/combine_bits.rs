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

use crate::natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::shl::limbs_shl_to_out;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Adds the first `length` coefficients of `poly`, each of `output_limbs` limbs, to
// `res[..total_limbs]`, with coefficient `i` shifted left by `i * coeff_limbs` limbs, and
// discarding anything past `total_limbs`.
//
// This is fft_combine_limbs from fft/combine_bits.c, FLINT 3.6.0.
crate_test_fn! {fft_combine_limbs(
    res: &mut [Limb],
    poly: &[Vec<Limb>],
    length: usize,
    coeff_limbs: usize,
    output_limbs: usize,
    total_limbs: usize,
) {
    let mut skip = 0;
    let mut i = 0;
    while i < length && skip + output_limbs < total_limbs {
        limbs_slice_add_greater_in_place_left(
            &mut res[skip..=skip + output_limbs],
            &poly[i][..output_limbs],
        );
        i += 1;
        skip += coeff_limbs;
    }
    while skip < total_limbs && i < length {
        limbs_slice_add_greater_in_place_left(
            &mut res[skip..total_limbs],
            &poly[i][..(total_limbs - skip).min(output_limbs)],
        );
        i += 1;
        skip += coeff_limbs;
    }
}}

// Adds the first `length` coefficients of `poly`, each of `output_limbs` limbs, to
// `res[..total_limbs]`, with coefficient `i` shifted left by `i * bits` bits, and discarding
// anything past `total_limbs`.
//
// This is fft_combine_bits from fft/combine_bits.c, FLINT 3.6.0.
crate_test_fn! {fft_combine_bits(
    res: &mut [Limb],
    poly: &[Vec<Limb>],
    length: usize,
    bits: u64,
    output_limbs: usize,
    total_limbs: usize,
) {
    let top_bits = bits & Limb::WIDTH_MASK;
    if top_bits == 0 {
        fft_combine_limbs(
            res,
            poly,
            length,
            usize::exact_from(bits >> Limb::LOG_WIDTH),
            output_limbs,
            total_limbs,
        );
        return;
    }
    let coeff_limbs = usize::exact_from(bits >> Limb::LOG_WIDTH) + 1;
    let mut temp = vec![0; output_limbs + 1];
    let mut shift_bits = 0;
    let mut limb_ptr = 0;
    let end = total_limbs;
    let mut i = 0;
    while i < length && limb_ptr + output_limbs + 1 < end {
        if shift_bits != 0 {
            limbs_shl_to_out(&mut temp, &poly[i][..=output_limbs], shift_bits);
            limbs_slice_add_same_length_in_place_left(
                &mut res[limb_ptr..=limb_ptr + output_limbs],
                &temp,
            );
        } else {
            limbs_slice_add_greater_in_place_left(
                &mut res[limb_ptr..=limb_ptr + output_limbs],
                &poly[i][..output_limbs],
            );
        }
        shift_bits += top_bits;
        limb_ptr += coeff_limbs - 1;
        if shift_bits >= Limb::WIDTH {
            limb_ptr += 1;
            shift_bits -= Limb::WIDTH;
        }
        i += 1;
    }
    while limb_ptr < end && i < length {
        if shift_bits != 0 {
            limbs_shl_to_out(&mut temp, &poly[i][..=output_limbs], shift_bits);
            limbs_slice_add_same_length_in_place_left(
                &mut res[limb_ptr..end],
                &temp[..end - limb_ptr],
            );
        } else {
            limbs_slice_add_same_length_in_place_left(
                &mut res[limb_ptr..end],
                &poly[i][..end - limb_ptr],
            );
        }
        shift_bits += top_bits;
        limb_ptr += coeff_limbs - 1;
        if shift_bits >= Limb::WIDTH {
            limb_ptr += 1;
            shift_bits -= Limb::WIDTH;
        }
        i += 1;
    }
}}
