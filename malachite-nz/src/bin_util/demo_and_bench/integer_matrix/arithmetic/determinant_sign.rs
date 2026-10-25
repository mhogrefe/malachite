// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Determinant, DeterminantSign, Sign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::determinant::determinant_cofactor;
use malachite_nz::integer_matrix::arithmetic::determinant_sign::determinant_sign_preconditioned;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_1, integer_matrix_gen_var_2, integer_matrix_pair_gen_var_7,
};
use malachite_nz::test_util::integer_matrix::arithmetic::solve::integer_matrix_low_rank;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_determinant_sign);
    register_demo!(runner, demo_integer_matrix_determinant_sign_ref);
    register_demo!(runner, demo_integer_matrix_determinant_sign_low_rank);
    register_demo!(runner, demo_integer_matrix_determinant_sign_preconditioned);
    register_bench!(runner, benchmark_integer_matrix_determinant_sign_algorithms);
    register_bench!(
        runner,
        benchmark_integer_matrix_determinant_sign_small_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_determinant_sign_evaluation_strategy
    );
}

fn demo_integer_matrix_determinant_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("determinant_sign({a_old}) = {:?}", a.determinant_sign());
    }
}

fn demo_integer_matrix_determinant_sign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("(&{a}).determinant_sign() = {:?}", (&a).determinant_sign());
    }
}

// Square matrices of rank at most the width of a second matrix, when that is smaller than their
// dimension, so that many are singular.
fn demo_integer_matrix_determinant_sign_low_rank(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen_var_7().get(gm, config).take(limit) {
        let a = integer_matrix_low_rank(&a, &b);
        println!("determinant_sign({a}) = {:?}", (&a).determinant_sign());
    }
}

fn demo_integer_matrix_determinant_sign_preconditioned(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen_var_1()
        .get(gm, config)
        .filter(|a| a.height() != 0)
        .take(limit)
    {
        println!(
            "determinant_sign_preconditioned({a}) = {:?}",
            determinant_sign_preconditioned(&a)
        );
    }
}

fn benchmark_integer_matrix_determinant_sign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.determinant_sign()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.determinant_sign());
            }),
            ("preconditioning, then the determinant", &mut |a| {
                no_out!(
                    if a.height() == 0 {
                        None
                    } else {
                        determinant_sign_preconditioned(&a)
                    }
                    .unwrap_or_else(|| a.determinant().sign())
                );
            }),
            ("sign of the determinant", &mut |a| {
                no_out!(a.determinant().sign());
            }),
        ],
    );
}

fn benchmark_integer_matrix_determinant_sign_small_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.determinant_sign()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.determinant_sign());
            }),
            ("cofactor expansion", &mut |a| {
                no_out!(determinant_cofactor(&a).sign());
            }),
            ("preconditioning, then the determinant", &mut |a| {
                no_out!(
                    if a.height() == 0 {
                        None
                    } else {
                        determinant_sign_preconditioned(&a)
                    }
                    .unwrap_or_else(|| determinant_cofactor(&a).sign())
                );
            }),
        ],
    );
}

fn benchmark_integer_matrix_determinant_sign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.determinant_sign()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.determinant_sign()", &mut |a| {
                no_out!(a.determinant_sign());
            }),
            ("(&IntegerMatrix).determinant_sign()", &mut |a| {
                no_out!((&a).determinant_sign());
            }),
        ],
    );
}
