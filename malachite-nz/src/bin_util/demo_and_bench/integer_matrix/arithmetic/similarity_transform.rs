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
use malachite_nz::test_util::bench::bucketers::triple_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_unsigned_integer_triple_gen_var_1;
use malachite_nz::test_util::integer_matrix::arithmetic::similarity_transform::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_similarity_transform);
    register_demo!(runner, demo_integer_matrix_similarity_transform_ref);
    register_demo!(runner, demo_integer_matrix_similarity_transform_assign);
    register_bench!(
        runner,
        benchmark_integer_matrix_similarity_transform_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_similarity_transform_algorithms
    );
}

fn demo_integer_matrix_similarity_transform(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, r, d) in integer_matrix_unsigned_integer_triple_gen_var_1()
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

fn demo_integer_matrix_similarity_transform_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, r, d) in integer_matrix_unsigned_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).similarity_transform({r}, &{d}) = {}",
            (&a).similarity_transform(r, &d)
        );
    }
}

fn demo_integer_matrix_similarity_transform_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, r, d) in integer_matrix_unsigned_integer_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.similarity_transform_assign(r, &d);
        println!("a := {a_old}; a.similarity_transform_assign({r}, &{d}); a = {a}");
    }
}

fn benchmark_integer_matrix_similarity_transform_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.similarity_transform(u64, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_unsigned_integer_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_matrix_bit_bucketer("a"),
        &mut [
            (
                "IntegerMatrix.similarity_transform(u64, Integer)",
                &mut |(a, r, d)| {
                    no_out!(a.similarity_transform(r, d));
                },
            ),
            (
                "IntegerMatrix.similarity_transform(u64, &Integer)",
                &mut |(a, r, d)| {
                    no_out!(a.similarity_transform(r, &d));
                },
            ),
            (
                "(&IntegerMatrix).similarity_transform(u64, Integer)",
                &mut |(a, r, d)| {
                    no_out!((&a).similarity_transform(r, d));
                },
            ),
            (
                "(&IntegerMatrix).similarity_transform(u64, &Integer)",
                &mut |(a, r, d)| {
                    no_out!((&a).similarity_transform(r, &d));
                },
            ),
            (
                "IntegerMatrix.similarity_transform_assign(u64, Integer)",
                &mut |(mut a, r, d)| {
                    a.similarity_transform_assign(r, d);
                },
            ),
            (
                "IntegerMatrix.similarity_transform_assign(u64, &Integer)",
                &mut |(mut a, r, d)| {
                    a.similarity_transform_assign(r, &d);
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_similarity_transform_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerMatrix).similarity_transform(u64, &Integer)",
        BenchmarkType::Algorithms,
        integer_matrix_unsigned_integer_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, r, d)| {
                no_out!((&a).similarity_transform(r, &d));
            }),
            ("FLINT's loops", &mut |(a, r, d)| {
                no_out!(integer_matrix_similarity_transform_flint_loops(&a, r, &d));
            }),
            ("by definition", &mut |(a, r, d)| {
                no_out!(integer_matrix_similarity_transform_by_definition(&a, r, &d));
            }),
        ],
    );
}
