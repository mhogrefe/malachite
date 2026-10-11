// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ElementLcm;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_to_element_lcm);
    register_demo!(runner, demo_integer_vector_into_element_lcm);
    register_bench!(
        runner,
        benchmark_integer_vector_element_lcm_evaluation_strategy
    );
}

fn demo_integer_vector_to_element_lcm(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        println!("{v}.to_element_lcm() = {}", v.to_element_lcm());
    }
}

fn demo_integer_vector_into_element_lcm(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.into_element_lcm() = {}", v.into_element_lcm());
    }
}

fn benchmark_integer_vector_element_lcm_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.to_element_lcm()",
        BenchmarkType::EvaluationStrategy,
        integer_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_vector_bit_bucketer("v"),
        &mut [
            ("IntegerVector.to_element_lcm()", &mut |v| {
                no_out!(v.to_element_lcm());
            }),
            ("IntegerVector.into_element_lcm()", &mut |v| {
                no_out!(v.into_element_lcm());
            }),
        ],
    );
}
