// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::linear_algebra::traits::{GramSchmidt, GramSchmidtAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen_var_3;
use malachite_q::test_util::rational_matrix::linear_algebra::gram_schmidt::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_gram_schmidt);
    register_demo!(runner, demo_rational_matrix_gram_schmidt_ref);
    register_demo!(runner, demo_rational_matrix_gram_schmidt_assign);
    register_bench!(
        runner,
        benchmark_rational_matrix_gram_schmidt_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_matrix_gram_schmidt_algorithms);
}

fn demo_rational_matrix_gram_schmidt(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.gram_schmidt() = {}", a.gram_schmidt());
    }
}

fn demo_rational_matrix_gram_schmidt_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        println!("(&{a}).gram_schmidt() = {}", (&a).gram_schmidt());
    }
}

fn demo_rational_matrix_gram_schmidt_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut a in rational_matrix_gen_var_3().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.gram_schmidt_assign();
        println!("a := {a_old}; a.gram_schmidt_assign(); a = {a}");
    }
}

fn benchmark_rational_matrix_gram_schmidt_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.gram_schmidt()",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.gram_schmidt()", &mut |a| {
                no_out!(a.gram_schmidt());
            }),
            ("(&RationalMatrix).gram_schmidt()", &mut |a| {
                no_out!((&a).gram_schmidt());
            }),
            ("RationalMatrix.gram_schmidt_assign()", &mut |mut a| {
                no_out!(a.gram_schmidt_assign());
            }),
        ],
    );
}

fn benchmark_rational_matrix_gram_schmidt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalMatrix).gram_schmidt()",
        BenchmarkType::Algorithms,
        rational_matrix_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("classical", &mut |a| {
                no_out!((&a).gram_schmidt());
            }),
            ("modified", &mut |a| {
                no_out!(gram_schmidt_modified_rational(&a));
            }),
        ],
    );
}
