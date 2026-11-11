// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::mag_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mag_cmp);
    register_demo!(runner, demo_mag_max);
    register_demo!(runner, demo_mag_min);
}

fn demo_mag_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in mag_pair_gen().get(gm, config).take(limit) {
        println!("({x}).cmp(&{y}) = {:?}", x.cmp(&y));
    }
}

fn demo_mag_max(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in mag_pair_gen().get(gm, config).take(limit) {
        println!("({x}).max({y}) = {}", x.max(y));
    }
}

fn demo_mag_min(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in mag_pair_gen().get(gm, config).take(limit) {
        println!("({x}).min({y}) = {}", x.min(y));
    }
}
