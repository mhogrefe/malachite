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
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vec_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_from_elements);
    register_demo!(runner, demo_integer_vector_from_owned_elements);
    register_bench!(
        runner,
        benchmark_integer_vector_from_elements_evaluation_strategy
    );
}

fn demo_integer_vector_from_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen().get(gm, config).take(limit) {
        println!(
            "IntegerVector::from_elements({}) = {}",
            xs.to_debug_string(),
            IntegerVector::from_elements(&xs)
        );
    }
}

fn demo_integer_vector_from_owned_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in integer_vec_gen().get(gm, config).take(limit) {
        println!(
            "IntegerVector::from_owned_elements({}) = {}",
            xs.to_debug_string(),
            IntegerVector::from_owned_elements(xs.clone())
        );
    }
}

fn benchmark_integer_vector_from_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector::from_elements(&[Integer])",
        BenchmarkType::EvaluationStrategy,
        integer_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("IntegerVector::from_elements(&[Integer])", &mut |xs| {
                no_out!(IntegerVector::from_elements(&xs));
            }),
            (
                "IntegerVector::from_owned_elements(Vec<Integer>)",
                &mut |xs| {
                    no_out!(IntegerVector::from_owned_elements(xs));
                },
            ),
        ],
    );
}
