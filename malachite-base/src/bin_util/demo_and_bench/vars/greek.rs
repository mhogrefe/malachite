// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::char_gen;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::{GreekCapsVars, GreekVars};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_greek_vars_parse_var);
    register_demo!(runner, demo_greek_caps_vars_parse_var);
}

fn demo_greek_vars_parse_var(gm: GenMode, config: &GenConfig, limit: usize) {
    for c in char_gen().get(gm, config).take(limit) {
        let name = c.to_string();
        println!(
            "GreekVars.parse_var({:?}) = {:?}",
            name,
            GreekVars.parse_var(&name)
        );
    }
}

fn demo_greek_caps_vars_parse_var(gm: GenMode, config: &GenConfig, limit: usize) {
    for c in char_gen().get(gm, config).take(limit) {
        let name = c.to_string();
        println!(
            "GreekCapsVars.parse_var({:?}) = {:?}",
            name,
            GreekCapsVars.parse_var(&name)
        );
    }
}
