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
use malachite_nz::integer_matrix::arithmetic::determinant::{
    determinant_bareiss, determinant_cofactor,
};
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{integer_matrix_gen_var_1, integer_matrix_gen_var_2};
use malachite_nz::test_util::integer_matrix::arithmetic::determinant::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_determinant_bareiss);
    register_demo!(runner, demo_integer_matrix_determinant_cofactor);
    register_bench!(runner, benchmark_integer_matrix_determinant_algorithms);
    register_bench!(
        runner,
        benchmark_integer_matrix_determinant_small_algorithms
    );
}

fn demo_integer_matrix_determinant_bareiss(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("determinant_bareiss({a}) = {}", determinant_bareiss(&a));
    }
}

fn demo_integer_matrix_determinant_cofactor(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_2().get(gm, config).take(limit) {
        println!("determinant_cofactor({a}) = {}", determinant_cofactor(&a));
    }
}

fn benchmark_integer_matrix_determinant_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "determinant(&IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("Bareiss", &mut |a| {
                no_out!(determinant_bareiss(&a));
            }),
            ("Bareiss, Integer arithmetic only", &mut |a| {
                no_out!(integer_matrix_determinant_bareiss_general(&a));
            }),
            ("naive", &mut |a| {
                no_out!(integer_matrix_determinant_naive(&a));
            }),
        ],
    );
}

fn benchmark_integer_matrix_determinant_small_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "determinant(&IntegerMatrix) for dimensions up to 4",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("cofactor", &mut |a| {
                no_out!(determinant_cofactor(&a));
            }),
            ("Bareiss", &mut |a| {
                no_out!(determinant_bareiss(&a));
            }),
            ("naive", &mut |a| {
                no_out!(integer_matrix_determinant_naive(&a));
            }),
        ],
    );
}
