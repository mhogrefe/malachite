// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::bool_gen;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_bool_to_typst);
}

fn demo_bool_to_typst(gm: GenMode, config: &GenConfig, limit: usize) {
    for b in bool_gen().get(gm, config).take(limit) {
        println!("{}.to_typst() = {}", b, b.to_typst());
    }
}
