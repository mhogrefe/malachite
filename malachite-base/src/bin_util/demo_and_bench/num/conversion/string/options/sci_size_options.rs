// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::sci_size_options_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_sci_size_options_to_debug_string);
}

fn demo_sci_size_options_to_debug_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for options in sci_size_options_gen().get(gm, config).take(limit) {
        println!("{options:?}");
    }
}
