// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::combine_bits::*;
use malachite_nz::test_util::generators::{large_type_gen_var_53, large_type_gen_var_54};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_fft_combine_limbs);
    register_demo!(runner, demo_fft_combine_bits);
}

fn demo_fft_combine_limbs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut res, poly, coeff_limbs, output_limbs) in
        large_type_gen_var_53().get(gm, config).take(limit)
    {
        let res_old = res.clone();
        let total_limbs = res.len();
        let coeff_limbs = usize::exact_from(coeff_limbs);
        fft_combine_limbs(
            &mut res,
            &poly,
            poly.len(),
            coeff_limbs,
            output_limbs,
            total_limbs,
        );
        println!(
            "fft_combine_limbs({res_old:?}, {poly:?}, {}, {coeff_limbs}, {output_limbs}, \
            {total_limbs}) = {res:?}",
            poly.len()
        );
    }
}

fn demo_fft_combine_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut res, poly, bits, output_limbs) in large_type_gen_var_54().get(gm, config).take(limit) {
        let res_old = res.clone();
        let total_limbs = res.len();
        fft_combine_bits(&mut res, &poly, poly.len(), bits, output_limbs, total_limbs);
        println!(
            "fft_combine_bits({res_old:?}, {poly:?}, {}, {bits}, {output_limbs}, {total_limbs}) = \
            {res:?}",
            poly.len()
        );
    }
}
