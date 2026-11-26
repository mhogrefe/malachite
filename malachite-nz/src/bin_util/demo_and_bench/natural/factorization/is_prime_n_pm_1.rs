// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::factorization::is_prime_n_pm_1::*;
use malachite_nz::test_util::generators::natural_triple_gen_var_11;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_divisor_in_residue_class_lenstra);
}

fn demo_natural_divisor_in_residue_class_lenstra(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, r, s) in natural_triple_gen_var_11().get(gm, config).take(limit) {
        match natural_divisor_in_residue_class_lenstra(&n, &r, &s) {
            Some(d) => println!("lenstra({n}, {r}, {s}) = {d}"),
            None => println!("lenstra({n}, {r}, {s}) = none"),
        }
    }
}
