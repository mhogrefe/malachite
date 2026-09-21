// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_foer_sequence_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_foer_sequence_to_latex);
}

fn demo_foer_sequence_to_latex(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_foer_sequence_gen::<u8>()
        .get(gm, config)
        .take(limit)
    {
        println!("{:?}.to_latex() = {}", xs, xs.to_latex());
    }
}
