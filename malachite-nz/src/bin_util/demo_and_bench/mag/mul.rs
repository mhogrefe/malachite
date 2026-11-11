// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::mag::Mag;
use malachite_nz::test_util::generators::{mag_pair_gen, mag_unsigned_pair_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mag_mul);
    register_demo!(runner, demo_mag_mul_u64);
}

fn demo_mag_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in mag_pair_gen().get(gm, config).take(limit) {
        println!("Mag::mul({x}, {y}) = {}", Mag::mul(x, y));
    }
}

fn demo_mag_mul_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in mag_unsigned_pair_gen().get(gm, config).take(limit) {
        println!("({x}).mul_u64({y}) = {}", x.mul_u64(y));
    }
}
