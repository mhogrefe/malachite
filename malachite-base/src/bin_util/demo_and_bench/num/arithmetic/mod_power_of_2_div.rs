// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_triple_gen_var_11;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_div);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_div);
}

fn demo_mod_power_of_2_div<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, pow) in unsigned_triple_gen_var_11::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{x}.mod_power_of_2_div({y}, {pow}) = {:?}",
            x.mod_power_of_2_div(y, pow)
        );
    }
}

fn benchmark_mod_power_of_2_div<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_div({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_11::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(x, y, pow)| {
            no_out!(x.mod_power_of_2_div(y, pow));
        })],
    );
}
