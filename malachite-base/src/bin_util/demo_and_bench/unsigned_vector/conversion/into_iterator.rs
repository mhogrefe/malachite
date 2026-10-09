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
    register_demo!(runner, demo_unsigned_vector_into_iter);
    register_demo!(runner, demo_unsigned_vector_into_iter_ref);
    register_bench!(
        runner,
        benchmark_unsigned_vector_into_iter_evaluation_strategy
    );
}

fn demo_unsigned_vector_into_iter(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!(
            "{v_old}.into_iter() = {}",
            v.into_iter().collect::<Vec<_>>().to_debug_string()
        );
    }
}

fn demo_unsigned_vector_into_iter_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in unsigned_vector_gen().get(gm, config).take(limit) {
        println!(
            "(&{v}).into_iter() = {}",
            IntoIterator::into_iter(&v)
                .collect::<Vec<_>>()
                .to_debug_string()
        );
    }
}

fn benchmark_unsigned_vector_into_iter_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.into_iter()",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64>.into_iter()", &mut |v| {
                no_out!(v.into_iter().count());
            }),
            ("(&UnsignedVector<u64>).into_iter()", &mut |v| {
                no_out!(IntoIterator::into_iter(&v).count());
            }),
        ],
    );
}
