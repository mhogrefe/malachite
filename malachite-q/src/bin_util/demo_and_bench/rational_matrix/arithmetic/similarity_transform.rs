// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SimilarityTransform, SimilarityTransformAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::rational_matrix_unsigned_rational_triple_gen_var_1;
use malachite_q::test_util::rational_matrix::arithmetic::similarity_transform::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_similarity_transform);
    register_demo!(runner, demo_rational_matrix_similarity_transform_ref);
    register_demo!(runner, demo_rational_matrix_similarity_transform_assign);
    register_bench!(
        runner,
        benchmark_rational_matrix_similarity_transform_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_rational_matrix_similarity_transform_algorithms
    );
}

fn demo_rational_matrix_similarity_transform(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, r, d) in rational_matrix_unsigned_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let d_old = d.clone();
        println!(
            "{a_old}.similarity_transform({r}, {d_old}) = {}",
            a.similarity_transform(r, d)
        );
    }
}

fn demo_rational_matrix_similarity_transform_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, r, d) in rational_matrix_unsigned_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).similarity_transform({r}, &{d}) = {}",
            (&a).similarity_transform(r, &d)
        );
    }
}

fn demo_rational_matrix_similarity_transform_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, r, d) in rational_matrix_unsigned_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.similarity_transform_assign(r, &d);
        println!("a := {a_old}; a.similarity_transform_assign({r}, &{d}); a = {a}");
    }
}

fn benchmark_rational_matrix_similarity_transform_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.similarity_transform(u64, Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_unsigned_rational_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_matrix_bit_bucketer("a"),
        &mut [
            (
                "RationalMatrix.similarity_transform(u64, Rational)",
                &mut |(a, r, d)| {
                    no_out!(a.similarity_transform(r, d));
                },
            ),
            (
                "RationalMatrix.similarity_transform(u64, &Rational)",
                &mut |(a, r, d)| {
                    no_out!(a.similarity_transform(r, &d));
                },
            ),
            (
                "(&RationalMatrix).similarity_transform(u64, Rational)",
                &mut |(a, r, d)| {
                    no_out!((&a).similarity_transform(r, d));
                },
            ),
            (
                "(&RationalMatrix).similarity_transform(u64, &Rational)",
                &mut |(a, r, d)| {
                    no_out!((&a).similarity_transform(r, &d));
                },
            ),
            (
                "RationalMatrix.similarity_transform_assign(u64, Rational)",
                &mut |(mut a, r, d)| {
                    a.similarity_transform_assign(r, d);
                },
            ),
            (
                "RationalMatrix.similarity_transform_assign(u64, &Rational)",
                &mut |(mut a, r, d)| {
                    a.similarity_transform_assign(r, &d);
                },
            ),
        ],
    );
}

fn benchmark_rational_matrix_similarity_transform_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalMatrix).similarity_transform(u64, &Rational)",
        BenchmarkType::Algorithms,
        rational_matrix_unsigned_rational_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, r, d)| {
                no_out!((&a).similarity_transform(r, &d));
            }),
            ("by definition", &mut |(a, r, d)| {
                no_out!(rational_matrix_similarity_transform_by_definition(
                    &a, r, &d
                ));
            }),
        ],
    );
}
