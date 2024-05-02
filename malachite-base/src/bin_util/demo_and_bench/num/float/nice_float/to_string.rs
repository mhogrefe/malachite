// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_nice_float_to_string);
    register_primitive_float_benches!(runner, benchmark_nice_float_to_string_algorithms);
}

fn demo_nice_float_to_string<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!("{}", NiceFloat(f));
    }
}

fn benchmark_nice_float_to_string_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("NiceFloat::<{}>.to_string()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("Malachite", &mut |f| no_out!(NiceFloat(f).to_string())),
            ("Rust default", &mut |f| no_out!(f.to_string())),
        ],
    );
}
