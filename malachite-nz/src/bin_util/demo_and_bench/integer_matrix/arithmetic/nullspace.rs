// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Nullspace;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_nullspace);
    register_demo!(runner, demo_integer_matrix_nullspace_val);
    register_demo!(runner, demo_integer_matrix_nullspace_low_rank);
    register_bench!(
        runner,
        benchmark_integer_matrix_nullspace_evaluation_strategy
    );
}

fn demo_integer_matrix_nullspace(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        println!("nullspace({a}) = {}", (&a).nullspace());
    }
}

fn demo_integer_matrix_nullspace_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("{a_old}.nullspace() = {}", a.nullspace());
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_nullspace_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        let a = a * b;
        println!("nullspace({a}) = {}", (&a).nullspace());
    }
}

fn benchmark_integer_matrix_nullspace_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.nullspace()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.nullspace()", &mut |a| {
                no_out!(a.nullspace());
            }),
            ("(&IntegerMatrix).nullspace()", &mut |a| {
                no_out!((&a).nullspace());
            }),
        ],
    );
}
