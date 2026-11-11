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
use malachite_nz::test_util::generators::natural_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mag_from_natural);
}

fn demo_mag_from_natural(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in natural_gen().get(gm, config).take(limit) {
        println!("Mag::from(&{x}) = {}", Mag::from(&x));
    }
}
