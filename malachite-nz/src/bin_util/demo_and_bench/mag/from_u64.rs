// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen;
use malachite_base::test_util::runner::Runner;
use malachite_nz::mag::Mag;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mag_from_u64);
    register_demo!(runner, demo_mag_from_u64_lower);
}

fn demo_mag_from_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen::<u64>().get(gm, config).take(limit) {
        println!("Mag::from({x}) = {}", Mag::from(x));
    }
}

fn demo_mag_from_u64_lower(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_gen::<u64>().get(gm, config).take(limit) {
        println!("Mag::from_u64_lower({x}) = {}", Mag::from_u64_lower(x));
    }
}
