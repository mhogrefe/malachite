// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::split_bits::*;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{large_type_gen_var_51, large_type_gen_var_52};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_fft_split_limbs);
    register_demo!(runner, demo_fft_split_bits);
}

fn demo_fft_split_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (limbs, coeff_limbs, output_limbs) in large_type_gen_var_51().get(gm, config).take(limit) {
        let total_limbs = limbs.len();
        let length = (total_limbs - 1) / coeff_limbs + 1;
        let mut poly = vec![vec![0; output_limbs + 1]; length];
        let result = fft_split_limbs(&mut poly, &limbs, total_limbs, coeff_limbs, output_limbs);
        println!(
            "fft_split_limbs(_, {limbs:?}, {total_limbs}, {coeff_limbs}, {output_limbs}) = \
            ({result}, {poly:?})"
        );
    }
}

fn demo_fft_split_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (limbs, bits, output_limbs) in large_type_gen_var_52().get(gm, config).take(limit) {
        let total_limbs = limbs.len();
        let length =
            usize::exact_from(((u64::exact_from(total_limbs) << Limb::LOG_WIDTH) - 1) / bits + 1);
        let mut poly = vec![vec![0; output_limbs + 1]; length];
        let result = fft_split_bits(&mut poly, &limbs, total_limbs, bits, output_limbs);
        println!(
            "fft_split_bits(_, {limbs:?}, {total_limbs}, {bits}, {output_limbs}) = \
            ({result}, {poly:?})"
        );
    }
}
