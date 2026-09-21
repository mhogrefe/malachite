// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::Factor;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_factors_to_latex);
}

fn demo_factors_to_latex(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_1::<u64>().get(gm, config).take(limit) {
        println!("{}.factor().to_latex() = {}", n, n.factor().to_latex());
    }
}
