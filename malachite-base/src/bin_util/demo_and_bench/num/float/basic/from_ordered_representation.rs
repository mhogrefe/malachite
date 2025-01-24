// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_13;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_from_ordered_representation);
    register_primitive_float_benches!(runner, benchmark_from_ordered_representation);
}

fn demo_from_ordered_representation<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in unsigned_gen_var_13::<T>().get(gm, config).take(limit) {
        println!(
            "from_ordered_representation({}) = {}",
            x,
            NiceFloat(T::from_ordered_representation(x))
        );
    }
}

fn benchmark_from_ordered_representation<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_ordered_representation(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_13::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |x| {
            no_out!(T::from_ordered_representation(x))
        })],
    );
}
