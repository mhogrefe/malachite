// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_from_elements);
    register_demo!(runner, demo_unsigned_vector_from_owned_elements);
    register_bench!(
        runner,
        benchmark_unsigned_vector_from_elements_evaluation_strategy
    );
}

fn demo_unsigned_vector_from_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "Vector::from_elements({}) = {}",
            xs.to_debug_string(),
            Vector::from_elements(&xs)
        );
    }
}

fn demo_unsigned_vector_from_owned_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u64>().get(gm, config).take(limit) {
        println!(
            "Vector::from_owned_elements({}) = {}",
            xs.to_debug_string(),
            Vector::from_owned_elements(xs.clone())
        );
    }
}

fn benchmark_unsigned_vector_from_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Vector::from_elements(&[u64])",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("Vector::from_elements(&[u64])", &mut |xs| {
                no_out!(Vector::from_elements(&xs));
            }),
            ("Vector::from_owned_elements(Vec<u64>)", &mut |xs| {
                no_out!(Vector::from_owned_elements(xs));
            }),
        ],
    );
}
