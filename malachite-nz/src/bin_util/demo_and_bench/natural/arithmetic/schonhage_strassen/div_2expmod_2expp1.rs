// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use malachite_nz::test_util::generators::large_type_gen_var_31;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_div_2exp_mod_2expp1_in_place);
    register_demo!(runner, demo_limbs_div_2exp_mod_2expp1_to_out);
}

fn demo_limbs_div_2exp_mod_2expp1_in_place(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut t, limbs, d) in large_type_gen_var_31().get(gm, config).take(limit) {
        let t_old = t.clone();
        limbs_div_2exp_mod_2expp1_in_place(&mut t, limbs, d);
        println!("limbs_div_2exp_mod_2expp1_in_place({t_old:?}, {limbs}, {d}) = {t:?}");
    }
}

fn demo_limbs_div_2exp_mod_2expp1_to_out(gm: GenMode, config: &GenConfig, limit: usize) {
    for (i1, limbs, d) in large_type_gen_var_31().get(gm, config).take(limit) {
        let mut t = vec![0; limbs + 1];
        limbs_div_2exp_mod_2expp1_to_out(&mut t, &i1, limbs, d);
        println!("limbs_div_2exp_mod_2expp1_to_out(_, {i1:?}, {limbs}, {d}) = {t:?}");
    }
}
