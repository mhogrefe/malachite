// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::ModPowerOf2Evaluate;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::*;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::modular::mod_evaluate_at_matrix::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_evaluate_at_matrix
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_evaluate_at_matrix_ref
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_evaluate_at_matrix_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_evaluate_at_matrix_algorithms
    );
}

fn demo_natural_polynomial_mod_power_of_2_evaluate_at_matrix(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, a, pow) in natural_polynomial_natural_matrix_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (p_old, a_old) = (p.clone(), a.clone());
        println!(
            "({p_old}).mod_power_of_2_evaluate({a_old}, {pow}) = {}",
            p.mod_power_of_2_evaluate(a, pow)
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_evaluate_at_matrix_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, a, pow) in natural_polynomial_natural_matrix_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_evaluate(&{a}, {pow}) = {}",
            (&p).mod_power_of_2_evaluate(&a, pow)
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_evaluate_at_matrix_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_evaluate(NaturalMatrix, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_natural_matrix_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_len_times_matrix_height_bucketer("p", "a"),
        &mut [
            ("p.mod_power_of_2_evaluate(a, pow)", &mut |(p, a, pow)| {
                no_out!(p.mod_power_of_2_evaluate(a, pow));
            }),
            ("p.mod_power_of_2_evaluate(&a, pow)", &mut |(p, a, pow)| {
                no_out!(p.mod_power_of_2_evaluate(&a, pow));
            }),
            ("(&p).mod_power_of_2_evaluate(a, pow)", &mut |(
                p,
                a,
                pow,
            )| {
                no_out!((&p).mod_power_of_2_evaluate(a, pow));
            }),
            (
                "(&p).mod_power_of_2_evaluate(&a, pow)",
                &mut |(p, a, pow)| {
                    no_out!((&p).mod_power_of_2_evaluate(&a, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_evaluate_at_matrix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.mod_power_of_2_evaluate(NaturalMatrix, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_natural_matrix_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_natural_polynomial_len_times_matrix_height_bucketer("p", "a"),
        &mut [
            ("default", &mut |(p, a, pow)| {
                no_out!((&p).mod_power_of_2_evaluate(&a, pow));
            }),
            ("Horner", &mut |(p, a, pow)| {
                no_out!(mod_power_of_2_evaluate_at_matrix_horner(&p, &a, pow));
            }),
            ("Paterson-Stockmeyer", &mut |(p, a, pow)| {
                no_out!(mod_power_of_2_evaluate_at_matrix_paterson_stockmeyer(
                    &p, &a, pow
                ));
            }),
            ("naive", &mut |(p, a, pow)| {
                no_out!(mod_power_of_2_evaluate_at_matrix_naive(&p, &a, pow));
            }),
        ],
    );
}
