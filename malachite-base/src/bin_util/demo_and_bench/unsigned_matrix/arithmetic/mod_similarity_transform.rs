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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::quadruple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::similarity_transform::*;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::ModRing;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_matrix_mod_similarity_transform);
    register_unsigned_demos!(runner, demo_unsigned_matrix_mod_similarity_transform_ref);
    register_unsigned_demos!(runner, demo_unsigned_matrix_mod_similarity_transform_assign);
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_matrix_mod_similarity_transform_evaluation_strategy
    );
    register_unsigned_benches!(
        runner,
        benchmark_unsigned_matrix_mod_similarity_transform_algorithms
    );
}

fn demo_unsigned_matrix_mod_similarity_transform<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, r, d, m) in unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_similarity_transform({r}, {d}, {m}) = {}",
            a.mod_similarity_transform(r, d, m)
        );
    }
}

fn demo_unsigned_matrix_mod_similarity_transform_ref<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, r, d, m) in unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_similarity_transform({r}, {d}, {m}) = {}",
            (&a).mod_similarity_transform(r, d, m)
        );
    }
}

fn demo_unsigned_matrix_mod_similarity_transform_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, r, d, m) in unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_similarity_transform_assign(r, d, m);
        println!("a := {a_old}; a.mod_similarity_transform_assign({r}, {d}, {m}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_similarity_transform_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "UnsignedMatrix<{}>.mod_similarity_transform(u64, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_similarity_transform(...)",
                &mut |(a, r, d, m)| {
                    no_out!(a.mod_similarity_transform(r, d, m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_similarity_transform(...)",
                &mut |(a, r, d, m)| {
                    no_out!((&a).mod_similarity_transform(r, d, m));
                },
            ),
            (
                "UnsignedMatrix.mod_similarity_transform_assign(...)",
                &mut |(mut a, r, d, m)| {
                    a.mod_similarity_transform_assign(r, d, m);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_similarity_transform_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "(&UnsignedMatrix<{}>).mod_similarity_transform(u64, {}, {})",
            T::NAME,
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, r, d, m)| {
                no_out!((&a).mod_similarity_transform(r, d, m));
            }),
            ("FLINT's loops", &mut |(a, r, d, m)| {
                let n = usize::exact_from(a.height());
                no_out!(matrix_similarity_transform_flint_loops(
                    &ModRing::new(m),
                    a.elements_ref(),
                    n,
                    usize::exact_from(r),
                    &d
                ));
            }),
        ],
    );
}
