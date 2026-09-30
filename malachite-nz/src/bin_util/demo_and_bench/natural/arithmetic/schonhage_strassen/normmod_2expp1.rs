// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::*;
use malachite_nz::test_util::generators::large_type_gen_var_29;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_norm_mod_2expp1);
}

fn demo_limbs_norm_mod_2expp1(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut t, limbs) in large_type_gen_var_29().get(gm, config).take(limit) {
        let t_old = t.clone();
        limbs_norm_mod_2expp1(&mut t, limbs);
        println!("limbs_norm_mod_2expp1({t_old:?}, {limbs}) = {t:?}");
    }
}
