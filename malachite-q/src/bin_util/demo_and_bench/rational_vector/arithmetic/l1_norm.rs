// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::L1Norm;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_to_l1_norm);
    register_demo!(runner, demo_rational_vector_into_l1_norm);
    register_demo!(runner, demo_rational_vector_l1_norm_significant_bits);
    register_bench!(
        runner,
        benchmark_rational_vector_l1_norm_evaluation_strategy
    );
}

fn demo_rational_vector_to_l1_norm(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("{v}.to_l1_norm() = {}", v.to_l1_norm());
    }
}

fn demo_rational_vector_into_l1_norm(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.into_l1_norm() = {}", v.into_l1_norm());
    }
}

fn demo_rational_vector_l1_norm_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!(
            "{v}.l1_norm_significant_bits() = {}",
            v.l1_norm_significant_bits()
        );
    }
}

fn benchmark_rational_vector_l1_norm_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.to_l1_norm()",
        BenchmarkType::EvaluationStrategy,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector.to_l1_norm()", &mut |v| {
                no_out!(v.to_l1_norm());
            }),
            ("RationalVector.into_l1_norm()", &mut |v| {
                no_out!(v.into_l1_norm());
            }),
        ],
    );
}
