// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{KroneckerProduct, KroneckerProductAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_integer_matrix_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_pair_gen;
use malachite_nz::test_util::integer_matrix::arithmetic::kronecker_product::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_kronecker_product);
    register_demo!(runner, demo_integer_matrix_kronecker_product_assign);
    register_bench!(
        runner,
        benchmark_integer_matrix_kronecker_product_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_kronecker_product_algorithms
    );
}

fn demo_integer_matrix_kronecker_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b) in integer_matrix_pair_gen().get(gm, config).take(limit) {
        println!(
            "{a}.kronecker_product({b}) = {}",
            (&a).kronecker_product(&b)
        );
    }
}

fn demo_integer_matrix_kronecker_product_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, b) in integer_matrix_pair_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        a.kronecker_product_assign(&b);
        println!("a := {a_old}; a.kronecker_product_assign({b}); a = {a}");
    }
}

fn benchmark_integer_matrix_kronecker_product_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.kronecker_product(IntegerMatrix)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            (
                "IntegerMatrix.kronecker_product(IntegerMatrix)",
                &mut |(a, b)| {
                    no_out!(a.kronecker_product(b));
                },
            ),
            (
                "IntegerMatrix.kronecker_product(&IntegerMatrix)",
                &mut |(a, b)| {
                    no_out!(a.kronecker_product(&b));
                },
            ),
            (
                "(&IntegerMatrix).kronecker_product(IntegerMatrix)",
                &mut |(a, b)| {
                    no_out!((&a).kronecker_product(b));
                },
            ),
            (
                "(&IntegerMatrix).kronecker_product(&IntegerMatrix)",
                &mut |(a, b)| {
                    no_out!((&a).kronecker_product(&b));
                },
            ),
            (
                "IntegerMatrix.kronecker_product_assign(IntegerMatrix)",
                &mut |(mut a, b)| {
                    a.kronecker_product_assign(b);
                },
            ),
            (
                "IntegerMatrix.kronecker_product_assign(&IntegerMatrix)",
                &mut |(mut a, b)| {
                    a.kronecker_product_assign(&b);
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_kronecker_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerMatrix).kronecker_product(&IntegerMatrix)",
        BenchmarkType::Algorithms,
        integer_matrix_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_matrix_max_bit_bucketer("a", "b"),
        &mut [
            ("default", &mut |(a, b)| {
                no_out!((&a).kronecker_product(&b));
            }),
            ("naive", &mut |(a, b)| {
                no_out!(integer_matrix_kronecker_product_naive(&a, &b));
            }),
        ],
    );
}
