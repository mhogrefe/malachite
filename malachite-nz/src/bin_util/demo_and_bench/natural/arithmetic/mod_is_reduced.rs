// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen_var_5;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_is_reduced);

    register_bench!(runner, benchmark_natural_mod_is_reduced);
}

fn demo_natural_mod_is_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_5().get(gm, config).take(limit) {
        if n.mod_is_reduced(&m) {
            println!("{n} is reduced mod {m}");
        } else {
            println!("{n} is not reduced mod {m}");
        }
    }
}

fn benchmark_natural_mod_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_is_reduced(&Natural)",
        BenchmarkType::Single,
        natural_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, m)| no_out!(n.mod_is_reduced(&m)))],
    );
}
