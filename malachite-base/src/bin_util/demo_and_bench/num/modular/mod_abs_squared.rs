// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_16;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_abs_squared);
    register_unsigned_demos!(runner, demo_mod_abs_squared_assign);
}

fn demo_mod_abs_squared<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, m) in unsigned_pair_gen_var_16::<T>().get(gm, config).take(limit) {
        println!("{x}.mod_abs_squared({m}) = {}", x.mod_abs_squared(m));
    }
}

fn demo_mod_abs_squared_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, m) in unsigned_pair_gen_var_16::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.mod_abs_squared_assign(m);
        println!("x := {old_x}; x.mod_abs_squared_assign({m}); x = {x}");
    }
}
