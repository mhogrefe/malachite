// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModSqrt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_16;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_sqrt);
    register_unsigned_benches!(runner, benchmark_mod_sqrt);
}

fn demo_mod_sqrt<T: ModSqrt<T, Output = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, m) in unsigned_pair_gen_var_16::<T>().get(gm, config).take(limit) {
        println!("{x}.mod_sqrt({m}) = {:?}", x.mod_sqrt(m));
    }
}

fn benchmark_mod_sqrt<T: ModSqrt<T, Output = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_sqrt({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_16::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [("Malachite", &mut |(x, m)| {
            no_out!(x.mod_sqrt(m));
        })],
    );
}
