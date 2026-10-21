// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2KroneckerProduct, ModPowerOf2KroneckerProductAssign,
};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_power_of_2_kronecker_product::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_power_of_2_kronecker_product
    );
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_power_of_2_kronecker_product_assign
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_kronecker_product_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_power_of_2_kronecker_product_algorithms
    );
}

fn demo_unsigned_matrix_mod_power_of_2_kronecker_product(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, b, pow) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{a}.mod_power_of_2_kronecker_product({b}, {pow}) = {}",
            (&a).mod_power_of_2_kronecker_product(&b, pow)
        );
    }
}

fn demo_unsigned_matrix_mod_power_of_2_kronecker_product_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut a, b, pow) in unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.mod_power_of_2_kronecker_product_assign(&b, pow);
        println!("a := {a_old}; a.mod_power_of_2_kronecker_product_assign({b}, {pow}); a = {a}");
    }
}

fn benchmark_unsigned_matrix_mod_power_of_2_kronecker_product_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_power_of_2_kronecker_product(UnsignedMatrix, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_power_of_2_kronecker_product(UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!(a.mod_power_of_2_kronecker_product(b, pow));
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_kronecker_product(&UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!(a.mod_power_of_2_kronecker_product(&b, pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_kronecker_product(UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!((&a).mod_power_of_2_kronecker_product(b, pow));
                },
            ),
            (
                "(&UnsignedMatrix).mod_power_of_2_kronecker_product(&UnsignedMatrix, u64)",
                &mut |(a, b, pow)| {
                    no_out!((&a).mod_power_of_2_kronecker_product(&b, pow));
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_kronecker_product_assign(UnsignedMatrix, u64)",
                &mut |(mut a, b, pow)| {
                    a.mod_power_of_2_kronecker_product_assign(b, pow);
                },
            ),
            (
                "UnsignedMatrix.mod_power_of_2_kronecker_product_assign(&UnsignedMatrix, u64)",
                &mut |(mut a, b, pow)| {
                    a.mod_power_of_2_kronecker_product_assign(&b, pow);
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_power_of_2_kronecker_product_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_power_of_2_kronecker_product(&UnsignedMatrix, u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, b, pow)| {
                no_out!((&a).mod_power_of_2_kronecker_product(&b, pow));
            }),
            ("naive", &mut |(a, b, pow)| {
                no_out!(unsigned_matrix_mod_power_of_2_kronecker_product_naive(
                    &a, &b, pow
                ));
            }),
        ],
    );
}
