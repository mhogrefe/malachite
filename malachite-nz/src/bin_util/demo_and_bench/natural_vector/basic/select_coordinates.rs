// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::bucketers::pair_2_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::{SelectCoordinates, SelectCoordinatesAssign};
use malachite_nz::test_util::generators::natural_vector_unsigned_vec_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_select_coordinates);
    register_demo!(runner, demo_natural_vector_select_coordinates_ref);
    register_demo!(runner, demo_natural_vector_select_coordinates_assign);
    register_bench!(
        runner,
        benchmark_natural_vector_select_coordinates_evaluation_strategy
    );
}

fn demo_natural_vector_select_coordinates(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, indices) in natural_vector_unsigned_vec_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.select_coordinates({}) = {}",
            indices.to_debug_string(),
            v.select_coordinates(&indices)
        );
    }
}

fn demo_natural_vector_select_coordinates_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, indices) in natural_vector_unsigned_vec_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).select_coordinates({}) = {}",
            indices.to_debug_string(),
            (&v).select_coordinates(&indices)
        );
    }
}

fn demo_natural_vector_select_coordinates_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, indices) in natural_vector_unsigned_vec_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.select_coordinates_assign(&indices);
        println!(
            "v := {v_old}; v.select_coordinates_assign({}); v = {v}",
            indices.to_debug_string()
        );
    }
}

fn benchmark_natural_vector_select_coordinates_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.select_coordinates(&[u64])",
        BenchmarkType::EvaluationStrategy,
        natural_vector_unsigned_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_vec_len_bucketer("indices"),
        &mut [
            (
                "NaturalVector.select_coordinates(&[u64])",
                &mut |(v, indices)| {
                    no_out!(v.select_coordinates(&indices));
                },
            ),
            (
                "(&NaturalVector).select_coordinates(&[u64])",
                &mut |(v, indices)| {
                    no_out!((&v).select_coordinates(&indices));
                },
            ),
            (
                "NaturalVector.select_coordinates_assign(&[u64])",
                &mut |(mut v, indices)| v.select_coordinates_assign(&indices),
            ),
        ],
    );
}
