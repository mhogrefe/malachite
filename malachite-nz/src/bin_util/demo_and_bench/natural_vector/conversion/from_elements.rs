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
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vec_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_from_elements);
    register_demo!(runner, demo_natural_vector_from_owned_elements);
    register_bench!(
        runner,
        benchmark_natural_vector_from_elements_evaluation_strategy
    );
}

fn demo_natural_vector_from_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!(
            "NaturalVector::from_elements({}) = {}",
            xs.to_debug_string(),
            NaturalVector::from_elements(&xs)
        );
    }
}

fn demo_natural_vector_from_owned_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in natural_vec_gen().get(gm, config).take(limit) {
        println!(
            "NaturalVector::from_owned_elements({}) = {}",
            xs.to_debug_string(),
            NaturalVector::from_owned_elements(xs.clone())
        );
    }
}

fn benchmark_natural_vector_from_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::from_elements(&[Natural])",
        BenchmarkType::EvaluationStrategy,
        natural_vec_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("NaturalVector::from_elements(&[Natural])", &mut |xs| {
                no_out!(NaturalVector::from_elements(&xs));
            }),
            (
                "NaturalVector::from_owned_elements(Vec<Natural>)",
                &mut |xs| {
                    no_out!(NaturalVector::from_owned_elements(xs));
                },
            ),
        ],
    );
}
