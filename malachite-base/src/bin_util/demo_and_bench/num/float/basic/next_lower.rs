// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen_var_10;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_next_lower);
    register_primitive_float_benches!(runner, benchmark_next_lower);
}

fn demo_next_lower<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in primitive_float_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "next_lower({}) = {}",
            NiceFloat(x),
            NiceFloat(x.next_lower())
        );
    }
}

fn benchmark_next_lower<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.next_lower()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |x| no_out!(x.next_lower()))],
    );
}
