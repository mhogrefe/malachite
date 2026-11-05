// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::ModEvaluate;
use malachite_base::test_util::bench::bucketers::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_at_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate_at_matrix);
    register_demo!(runner, demo_unsigned_polynomial_mod_evaluate_at_matrix_ref);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_at_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_mod_evaluate_at_matrix_algorithms
    );
}

fn demo_unsigned_polynomial_mod_evaluate_at_matrix(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, a, m) in unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (p_old, a_old) = (p.clone(), a.clone());
        println!(
            "({p_old}).mod_evaluate({a_old}, {m}) = {}",
            p.mod_evaluate(a, m)
        );
    }
}

fn demo_unsigned_polynomial_mod_evaluate_at_matrix_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, a, m) in unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate(&{a}, {m}) = {}",
            (&p).mod_evaluate(&a, m)
        );
    }
}

fn benchmark_unsigned_polynomial_mod_evaluate_at_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_evaluate(UnsignedMatrix<u64>, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_len_times_matrix_height_bucketer("p", "a"),
        &mut [
            ("p.mod_evaluate(a, m)", &mut |(p, a, m)| {
                no_out!(p.mod_evaluate(a, m));
            }),
            ("p.mod_evaluate(&a, m)", &mut |(p, a, m)| {
                no_out!(p.mod_evaluate(&a, m));
            }),
            ("(&p).mod_evaluate(a, m)", &mut |(p, a, m)| {
                no_out!((&p).mod_evaluate(a, m));
            }),
            ("(&p).mod_evaluate(&a, m)", &mut |(p, a, m)| {
                no_out!((&p).mod_evaluate(&a, m));
            }),
        ],
    );
}

fn benchmark_unsigned_polynomial_mod_evaluate_at_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.mod_evaluate(UnsignedMatrix<u64>, u64)",
        BenchmarkType::Algorithms,
        unsigned_polynomial_unsigned_matrix_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_len_times_matrix_height_bucketer("p", "a"),
        &mut [
            ("default", &mut |(p, a, m)| {
                no_out!((&p).mod_evaluate(&a, m));
            }),
            ("Horner", &mut |(p, a, m)| {
                no_out!(mod_evaluate_at_matrix_horner(&p, &a, m));
            }),
            ("Paterson-Stockmeyer", &mut |(p, a, m)| {
                no_out!(mod_evaluate_at_matrix_paterson_stockmeyer(&p, &a, m));
            }),
            ("naive", &mut |(p, a, m)| {
                no_out!(mod_evaluate_at_matrix_naive(&p, &a, m));
            }),
        ],
    );
}
