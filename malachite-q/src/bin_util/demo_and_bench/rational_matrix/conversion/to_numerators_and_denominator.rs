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
use malachite_q::test_util::rational_matrix::conversion::to_numerators_and_denominator::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_to_numerators_and_denominator);
    register_demo!(runner, demo_rational_matrix_into_numerators_and_denominator);
    register_bench!(
        runner,
        benchmark_rational_matrix_to_numerators_and_denominator_algorithms
    );
    register_bench!(
        runner,
        benchmark_rational_matrix_to_numerators_and_denominator_evaluation_strategy
    );
}

fn demo_rational_matrix_to_numerators_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in rational_matrix_gen().get(gm, config).take(limit) {
        let (ns, d) = a.to_numerators_and_denominator();
        println!("{a}.to_numerators_and_denominator() = ({ns}, {d})");
    }
}

fn demo_rational_matrix_into_numerators_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in rational_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let (ns, d) = a.into_numerators_and_denominator();
        println!("{a_old}.into_numerators_and_denominator() = ({ns}, {d})");
    }
}

fn benchmark_rational_matrix_to_numerators_and_denominator_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.to_numerators_and_denominator()",
        BenchmarkType::Algorithms,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("least common multiple", &mut |a| {
                no_out!(a.to_numerators_and_denominator());
            }),
            ("product, then gcd", &mut |a| {
                no_out!(rational_matrix_to_numerators_and_denominator_naive(&a));
            }),
        ],
    );
}

fn benchmark_rational_matrix_to_numerators_and_denominator_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.to_numerators_and_denominator()",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.to_numerators_and_denominator()", &mut |a| {
                no_out!(a.to_numerators_and_denominator());
            }),
            (
                "RationalMatrix.into_numerators_and_denominator()",
                &mut |a| {
                    no_out!(a.into_numerators_and_denominator());
                },
            ),
        ],
    );
}
