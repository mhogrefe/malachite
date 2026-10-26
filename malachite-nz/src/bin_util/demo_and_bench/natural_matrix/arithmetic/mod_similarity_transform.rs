// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModSimilarityTransform, ModSimilarityTransformAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::similarity_transform::*;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::ModRing;
use malachite_nz::test_util::bench::bucketers::quadruple_1_natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_mod_similarity_transform);
    register_demo!(runner, demo_natural_matrix_mod_similarity_transform_ref);
    register_demo!(runner, demo_natural_matrix_mod_similarity_transform_assign);
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_similarity_transform_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_matrix_mod_similarity_transform_algorithms
    );
}

fn demo_natural_matrix_mod_similarity_transform(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, r, d, m) in natural_matrix_unsigned_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        let d_old = d.clone();
        println!(
            "{a_old}.mod_similarity_transform({r}, {d_old}, {m}) = {}",
            a.mod_similarity_transform(r, d, m.clone())
        );
    }
}

fn demo_natural_matrix_mod_similarity_transform_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, r, d, m) in natural_matrix_unsigned_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_similarity_transform({r}, &{d}, &{m}) = {}",
            (&a).mod_similarity_transform(r, &d, &m)
        );
    }
}

fn demo_natural_matrix_mod_similarity_transform_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, r, d, m) in natural_matrix_unsigned_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_similarity_transform_assign(r, &d, &m);
        println!("a := {a_old}; a.mod_similarity_transform_assign({r}, &{d}, &{m}); a = {a}");
    }
}

fn benchmark_natural_matrix_mod_similarity_transform_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.mod_similarity_transform(u64, Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_unsigned_natural_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            (
                "NaturalMatrix.mod_similarity_transform(u64, Natural, Natural)",
                &mut |(a, r, d, m)| {
                    no_out!(a.mod_similarity_transform(r, d, m));
                },
            ),
            (
                "NaturalMatrix.mod_similarity_transform(u64, &Natural, &Natural)",
                &mut |(a, r, d, m)| {
                    no_out!(a.mod_similarity_transform(r, &d, &m));
                },
            ),
            (
                "(&NaturalMatrix).mod_similarity_transform(u64, &Natural, &Natural)",
                &mut |(a, r, d, m)| {
                    no_out!((&a).mod_similarity_transform(r, &d, &m));
                },
            ),
            (
                "NaturalMatrix.mod_similarity_transform_assign(u64, Natural, Natural)",
                &mut |(mut a, r, d, m)| {
                    a.mod_similarity_transform_assign(r, d, m);
                },
            ),
            (
                "NaturalMatrix.mod_similarity_transform_assign(u64, &Natural, &Natural)",
                &mut |(mut a, r, d, m)| {
                    a.mod_similarity_transform_assign(r, &d, &m);
                },
            ),
        ],
    );
}

fn benchmark_natural_matrix_mod_similarity_transform_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalMatrix).mod_similarity_transform(u64, &Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_matrix_unsigned_natural_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, r, d, m)| {
                no_out!((&a).mod_similarity_transform(r, &d, &m));
            }),
            ("FLINT's loops", &mut |(a, r, d, m)| {
                let n = usize::exact_from(a.height());
                no_out!(matrix_similarity_transform_flint_loops(
                    &ModRing::new(m.clone()),
                    a.elements_ref(),
                    n,
                    usize::exact_from(r),
                    &d
                ));
            }),
        ],
    );
}
