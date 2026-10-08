// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::bucketers::unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_to_elements);
    register_demo!(runner, demo_unsigned_vector_into_elements);
    register_demo!(runner, demo_unsigned_vector_elements_ref);
    register_bench!(
        runner,
        benchmark_unsigned_vector_to_elements_evaluation_strategy
    );
}

fn demo_unsigned_vector_to_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!("{v}.to_elements() = {}", v.to_elements().to_debug_string());
    }
}

fn demo_unsigned_vector_into_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!(
            "{v}.into_elements() = {}",
            v.clone().into_elements().to_debug_string()
        );
    }
}

fn demo_unsigned_vector_elements_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!(
            "{v}.elements_ref() = {}",
            v.elements_ref().to_debug_string()
        );
    }
}

fn benchmark_unsigned_vector_to_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.to_elements()",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64>.to_elements()", &mut |v| {
                no_out!(v.to_elements());
            }),
            ("UnsignedVector<u64>.into_elements()", &mut |v| {
                no_out!(v.into_elements());
            }),
            ("UnsignedVector<u64>.elements_ref()", &mut |v| {
                no_out!(v.elements_ref().len());
            }),
        ],
    );
}
