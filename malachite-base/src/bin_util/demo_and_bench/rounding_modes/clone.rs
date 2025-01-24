// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{rounding_mode_gen, rounding_mode_pair_gen};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_clone);
    register_demo!(runner, demo_rounding_mode_clone_from);
}

fn demo_rounding_mode_clone(gm: GenMode, config: &GenConfig, limit: usize) {
    for rm in rounding_mode_gen().get(gm, config).take(limit) {
        println!("clone({}) = {}", rm, rm.clone());
    }
}

fn demo_rounding_mode_clone_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rounding_mode_pair_gen().get(gm, config).take(limit) {
        let x_old = x;
        x.clone_from(&y);
        println!("x := {x_old}; x.clone_from({y}); x = {x}");
    }
}
