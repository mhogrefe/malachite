// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_to_elements);
    register_demo!(runner, demo_rational_vector_into_elements);
    register_demo!(runner, demo_rational_vector_elements_ref);
    register_bench!(
        runner,
        benchmark_rational_vector_to_elements_evaluation_strategy
    );
}

fn demo_rational_vector_to_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("{v}.to_elements() = {}", v.to_elements().to_debug_string());
    }
}

fn demo_rational_vector_into_elements(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!(
            "{v}.into_elements() = {}",
            v.clone().into_elements().to_debug_string()
        );
    }
}

fn demo_rational_vector_elements_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!(
            "{v}.elements_ref() = {}",
            v.elements_ref().to_debug_string()
        );
    }
}

fn benchmark_rational_vector_to_elements_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.to_elements()",
        BenchmarkType::EvaluationStrategy,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector.to_elements()", &mut |v| {
                no_out!(v.to_elements());
            }),
            ("RationalVector.into_elements()", &mut |v| {
                no_out!(v.into_elements());
            }),
            ("RationalVector.elements_ref()", &mut |v| {
                no_out!(v.elements_ref().len());
            }),
        ],
    );
}
