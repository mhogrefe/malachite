// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_gen;
use malachite_q::test_util::rational_matrix::conversion::to_numerators_and_column_denominators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_rational_matrix_to_numerators_and_column_denominators
    );
    register_demo!(
        runner,
        demo_rational_matrix_into_numerators_and_column_denominators
    );
    register_bench!(
        runner,
        benchmark_rational_matrix_to_numerators_and_column_denominators_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_matrix_to_numerators_and_column_denominators_evaluation_strategy
    );
}

fn demo_rational_matrix_to_numerators_and_column_denominators(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in rational_matrix_gen().get(gm, config).take(limit) {
        let (ns, ds) = a.to_numerators_and_column_denominators();
        println!("{a}.to_numerators_and_column_denominators() = ({ns}, {ds})");
    }
}

fn demo_rational_matrix_into_numerators_and_column_denominators(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in rational_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let (ns, ds) = a.into_numerators_and_column_denominators();
        println!("{a_old}.into_numerators_and_column_denominators() = ({ns}, {ds})");
    }
}

fn benchmark_rational_matrix_to_numerators_and_column_denominators_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.to_numerators_and_column_denominators()",
        BenchmarkType::Algorithms,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.to_numerators_and_column_denominators());
            }),
            ("one column at a time, as vectors", &mut |a| {
                no_out!(rational_matrix_to_numerators_and_column_denominators_naive(
                    &a
                ));
            }),
        ],
    );
}

fn benchmark_rational_matrix_to_numerators_and_column_denominators_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.to_numerators_and_column_denominators()",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            (
                "RationalMatrix.to_numerators_and_column_denominators()",
                &mut |a| {
                    no_out!(a.to_numerators_and_column_denominators());
                },
            ),
            (
                "RationalMatrix.into_numerators_and_column_denominators()",
                &mut |a| {
                    no_out!(a.into_numerators_and_column_denominators());
                },
            ),
        ],
    );
}
