// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_option_to_latex_unsigned);
}

fn demo_option_to_latex_unsigned<T: PrimitiveUnsigned + ToLatex>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    println!("None.to_latex() = {}", None::<T>.to_latex());
    for x in unsigned_gen::<T>().get(gm, config).take(limit) {
        println!("Some({}).to_latex() = {}", x, Some(x).to_latex());
    }
}
