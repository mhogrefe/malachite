// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::bench::bucketers::primitive_int_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_9;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_int_demos!(runner, demo_low_mask);
    register_primitive_int_benches!(runner, benchmark_low_mask);
}

fn demo_low_mask<T: PrimitiveInt>(gm: GenMode, config: &GenConfig, limit: usize) {
    for bits in unsigned_gen_var_9::<T>().get(gm, config).take(limit) {
        println!("{}::low_mask({}) = {}", T::NAME, bits, T::low_mask(bits));
    }
}

fn benchmark_low_mask<T: PrimitiveInt>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.low_mask(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_9::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |bits| no_out!(T::low_mask(bits)))],
    );
}
