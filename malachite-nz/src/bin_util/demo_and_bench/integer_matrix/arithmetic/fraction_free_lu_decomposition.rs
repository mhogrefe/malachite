// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::FractionFreeLuDecomposition;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};
use malachite_nz::test_util::integer_matrix::arithmetic::fraction_free_lu_decomposition::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_fraction_free_lu_decomposition);
    register_demo!(
        runner,
        demo_integer_matrix_fraction_free_lu_decomposition_low_rank
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_fraction_free_lu_decomposition_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_fraction_free_lu_decomposition_algorithms
    );
}

fn print_decomposition(a: &IntegerMatrix) {
    let (p, l, u, den) = a.fraction_free_lu_decomposition();
    println!(
        "{a}.fraction_free_lu_decomposition() = ([{}], {l}, {u}, {den})",
        p.iter().join(", ")
    );
}

fn demo_integer_matrix_fraction_free_lu_decomposition(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen().get(gm, config).take(limit) {
        print_decomposition(&a);
    }
}

// Products of matrices whose inner dimension is often smaller than the outer ones, so that many
// have less than full rank.
fn demo_integer_matrix_fraction_free_lu_decomposition_low_rank(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b) in integer_matrix_pair_gen_var_4().get(gm, config).take(limit) {
        print_decomposition(&(a * b));
    }
}

fn benchmark_integer_matrix_fraction_free_lu_decomposition_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.fraction_free_lu_decomposition()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.fraction_free_lu_decomposition()", &mut |a| {
                no_out!(a.fraction_free_lu_decomposition());
            }),
            (
                "(&IntegerMatrix).fraction_free_lu_decomposition()",
                &mut |a| {
                    no_out!((&a).fraction_free_lu_decomposition());
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_fraction_free_lu_decomposition_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.fraction_free_lu_decomposition()",
        BenchmarkType::Algorithms,
        integer_matrix_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.fraction_free_lu_decomposition());
            }),
            ("Integer arithmetic only", &mut |a| {
                no_out!(integer_matrix_fraction_free_lu_decomposition_general(&a));
            }),
            ("naive", &mut |a| {
                no_out!(integer_matrix_fraction_free_lu_decomposition_naive(&a));
            }),
        ],
    );
}
