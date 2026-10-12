// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2AbsSquared, ModPowerOf2AbsSquaredAssign};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_11;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_abs_squared);
    register_demo!(runner, demo_natural_mod_power_of_2_abs_squared_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_abs_squared_assign);
}

fn demo_natural_mod_power_of_2_abs_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "{x_old}.mod_power_of_2_abs_squared({pow}) = {}",
            x.mod_power_of_2_abs_squared(pow)
        );
    }
}

fn demo_natural_mod_power_of_2_abs_squared_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mod_power_of_2_abs_squared({pow}) = {}",
            (&x).mod_power_of_2_abs_squared(pow)
        );
    }
}

fn demo_natural_mod_power_of_2_abs_squared_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, pow) in natural_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_power_of_2_abs_squared_assign(pow);
        println!("x := {x_old}; x.mod_power_of_2_abs_squared_assign({pow}); x = {x}");
    }
}
