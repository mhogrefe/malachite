// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModDiv;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_3_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_triple_gen_var_12;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_div);
    register_unsigned_benches!(runner, benchmark_mod_div);
}

fn demo_mod_div<T: ModDiv<T, T, Output = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, m) in unsigned_triple_gen_var_12::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{x}.mod_div({y}, {m}) = {:?}", x.mod_div(y, m));
    }
}

fn benchmark_mod_div<T: ModDiv<T, T, Output = T> + PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_div({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_12::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [("Malachite", &mut |(x, y, m)| {
            no_out!(x.mod_div(y, m));
        })],
    );
}
