// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::rounding_mode_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rounding_mode_neg_assign);
    register_demo!(runner, demo_rounding_mode_neg);
}

fn demo_rounding_mode_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut rm in rounding_mode_gen().get(gm, config).take(limit) {
        let rm_old = rm;
        rm.neg_assign();
        println!("rm := {rm_old}; r.neg_assign(); rm = {rm}");
    }
}

fn demo_rounding_mode_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for rm in rounding_mode_gen().get(gm, config).take(limit) {
        println!("-{} = {}", rm, -rm);
    }
}
