// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{EntrywiseAbs, EntrywiseAbsAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_entrywise_abs);
    register_demo!(runner, demo_rational_vector_entrywise_abs_ref);
    register_demo!(runner, demo_rational_vector_entrywise_abs_assign);

    register_bench!(
        runner,
        benchmark_rational_vector_entrywise_abs_evaluation_strategy
    );
}

fn demo_rational_vector_entrywise_abs(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.entrywise_abs() = {}", v.entrywise_abs());
    }
}

fn demo_rational_vector_entrywise_abs_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!("(&{v}).entrywise_abs() = {}", (&v).entrywise_abs());
    }
}

fn demo_rational_vector_entrywise_abs_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut v in rational_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        v.entrywise_abs_assign();
        println!("v := {v_old}; v.entrywise_abs_assign(); v = {v}");
    }
}

fn benchmark_rational_vector_entrywise_abs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.entrywise_abs()",
        BenchmarkType::EvaluationStrategy,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector.entrywise_abs()", &mut |v| {
                no_out!(v.entrywise_abs());
            }),
            ("(&RationalVector).entrywise_abs()", &mut |v| {
                no_out!((&v).entrywise_abs());
            }),
            ("RationalVector.entrywise_abs_assign()", &mut |mut v| {
                v.entrywise_abs_assign();
            }),
        ],
    );
}
