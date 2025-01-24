// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_12;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_is_reduced);
    register_unsigned_benches!(runner, benchmark_mod_is_reduced);
}

fn demo_mod_is_reduced<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        if n.mod_is_reduced(&m) {
            println!("{n} is reduced mod {m}");
        } else {
            println!("{n} is not reduced mod {m}");
        }
    }
}

fn benchmark_mod_is_reduced<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_is_reduced(&{})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [("Malachite", &mut |(n, m)| no_out!(n.mod_is_reduced(&m)))],
    );
}
