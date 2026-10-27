// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModCharacteristicPolynomial;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_matrix_size_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_11, unsigned_matrix_unsigned_pair_gen_var_12,
};
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_matrix::arithmetic::mod_characteristic_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_matrix_mod_characteristic_polynomial);
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_characteristic_polynomial_ref
    );
    register_demo!(
        runner,
        demo_unsigned_matrix_mod_characteristic_polynomial_large_dimensions
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_characteristic_polynomial_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_characteristic_polynomial_algorithms
    );
    register_bench!(
        runner,
        benchmark_unsigned_matrix_mod_characteristic_polynomial_prime_algorithms
    );
}

fn demo_unsigned_matrix_mod_characteristic_polynomial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!(
            "{a_old}.mod_characteristic_polynomial({m}) = {}",
            a.mod_characteristic_polynomial(m)
        );
    }
}

fn demo_unsigned_matrix_mod_characteristic_polynomial_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_characteristic_polynomial({m}) = {}",
            (&a).mod_characteristic_polynomial(m)
        );
    }
}

fn demo_unsigned_matrix_mod_characteristic_polynomial_large_dimensions(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    let mut config = config.clone();
    config.insert("mean_dimension_n", 20);
    for (a, m) in unsigned_matrix_unsigned_pair_gen_var_11()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "(&{a}).mod_characteristic_polynomial({m}) = {}",
            (&a).mod_characteristic_polynomial(m)
        );
    }
}

fn benchmark_unsigned_matrix_mod_characteristic_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedMatrix.mod_characteristic_polynomial(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_matrix_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            (
                "UnsignedMatrix.mod_characteristic_polynomial(u64)",
                &mut |(a, m)| {
                    no_out!(a.mod_characteristic_polynomial(m));
                },
            ),
            (
                "(&UnsignedMatrix).mod_characteristic_polynomial(u64)",
                &mut |(a, m)| {
                    no_out!((&a).mod_characteristic_polynomial(m));
                },
            ),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_characteristic_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_characteristic_polynomial(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_characteristic_polynomial(m));
            }),
            ("Berkowitz", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_characteristic_polynomial_berkowitz(
                    &a, m
                ));
            }),
            ("Hessenberg", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_characteristic_polynomial_hessenberg(
                    &a, m
                ));
            }),
        ],
    );
}

fn benchmark_unsigned_matrix_mod_characteristic_polynomial_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&UnsignedMatrix).mod_characteristic_polynomial(u64)",
        BenchmarkType::Algorithms,
        unsigned_matrix_unsigned_pair_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_matrix_size_bucketer("a"),
        &mut [
            ("default", &mut |(a, m)| {
                no_out!((&a).mod_characteristic_polynomial(m));
            }),
            ("Berkowitz", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_characteristic_polynomial_berkowitz(
                    &a, m
                ));
            }),
            ("Hessenberg", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_characteristic_polynomial_hessenberg(
                    &a, m
                ));
            }),
            ("Danilevsky", &mut |(a, m)| {
                no_out!(unsigned_matrix_mod_characteristic_polynomial_danilevsky(
                    &a, m
                ));
            }),
        ],
    );
}
