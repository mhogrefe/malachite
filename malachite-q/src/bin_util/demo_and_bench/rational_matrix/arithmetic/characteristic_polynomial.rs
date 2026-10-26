// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::{rational_matrix_gen_var_1, rational_matrix_gen_var_4};
use malachite_q::test_util::rational_matrix::arithmetic::characteristic_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_characteristic_polynomial);
    register_demo!(runner, demo_rational_matrix_characteristic_polynomial_ref);
    register_bench!(
        runner,
        benchmark_rational_matrix_characteristic_polynomial_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_matrix_characteristic_polynomial_algorithms
    );
}

fn demo_rational_matrix_characteristic_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in rational_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!(
            "{a_old}.characteristic_polynomial() = {}",
            a.characteristic_polynomial()
        );
    }
}

fn demo_rational_matrix_characteristic_polynomial_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in rational_matrix_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{a}).characteristic_polynomial() = {}",
            (&a).characteristic_polynomial()
        );
    }
}

fn benchmark_rational_matrix_characteristic_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.characteristic_polynomial()",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.characteristic_polynomial()", &mut |a| {
                no_out!(a.characteristic_polynomial());
            }),
            ("(&RationalMatrix).characteristic_polynomial()", &mut |a| {
                no_out!((&a).characteristic_polynomial());
            }),
        ],
    );
}

fn benchmark_rational_matrix_characteristic_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalMatrix).characteristic_polynomial()",
        BenchmarkType::Algorithms,
        rational_matrix_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!((&a).characteristic_polynomial());
            }),
            ("principal minors", &mut |a| {
                no_out!(rational_matrix_characteristic_polynomial_naive(&a));
            }),
        ],
    );
}
