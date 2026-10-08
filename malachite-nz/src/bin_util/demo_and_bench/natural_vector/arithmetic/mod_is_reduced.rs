// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_is_reduced);
    register_bench!(runner, benchmark_natural_vector_mod_is_reduced);
}

fn demo_natural_vector_mod_is_reduced(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({v}).mod_is_reduced(&{m}) = {}", v.mod_is_reduced(&m));
    }
}

fn benchmark_natural_vector_mod_is_reduced(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_is_reduced(&Natural)",
        BenchmarkType::Single,
        natural_vector_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [("Malachite", &mut |(v, m)| no_out!(v.mod_is_reduced(&m)))],
    );
}
