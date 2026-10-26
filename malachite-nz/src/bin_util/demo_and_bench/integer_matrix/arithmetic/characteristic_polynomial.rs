// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::characteristic_polynomial::*;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{integer_matrix_gen_var_1, integer_matrix_gen_var_4};
use malachite_nz::test_util::integer_matrix::arithmetic::characteristic_polynomial::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_characteristic_polynomial);
    register_demo!(runner, demo_integer_matrix_characteristic_polynomial_ref);
    register_demo!(runner, demo_integer_matrix_characteristic_polynomial_bound);
    register_bench!(
        runner,
        benchmark_integer_matrix_characteristic_polynomial_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_characteristic_polynomial_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_matrix_characteristic_polynomial_naive_algorithms
    );
    register_bench!(runner, benchmark_integer_matrix_characteristic_polynomial_bound);
}

fn demo_integer_matrix_characteristic_polynomial(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!(
            "{a_old}.characteristic_polynomial() = {}",
            a.characteristic_polynomial()
        );
    }
}

fn demo_integer_matrix_characteristic_polynomial_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{a}).characteristic_polynomial() = {}",
            (&a).characteristic_polynomial()
        );
    }
}

fn demo_integer_matrix_characteristic_polynomial_bound(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for a in integer_matrix_gen_var_1().get(gm, config).take(limit) {
        println!(
            "{a}.characteristic_polynomial_bound() = {}",
            a.characteristic_polynomial_bound()
        );
    }
}

fn benchmark_integer_matrix_characteristic_polynomial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.characteristic_polynomial()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.characteristic_polynomial()", &mut |a| {
                no_out!(a.characteristic_polynomial());
            }),
            ("(&IntegerMatrix).characteristic_polynomial()", &mut |a| {
                no_out!((&a).characteristic_polynomial());
            }),
        ],
    );
}

fn benchmark_integer_matrix_characteristic_polynomial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerMatrix).characteristic_polynomial()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!((&a).characteristic_polynomial());
            }),
            ("Berkowitz", &mut |a| {
                no_out!(charpoly_berkowitz(
                    a.elements_ref(),
                    usize::exact_from(a.height())
                ));
            }),
            ("multimodular", &mut |a| {
                no_out!(charpoly_modular(&a));
            }),
        ],
    );
}

fn benchmark_integer_matrix_characteristic_polynomial_naive_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerMatrix).characteristic_polynomial()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!((&a).characteristic_polynomial());
            }),
            ("principal minors", &mut |a| {
                no_out!(integer_matrix_characteristic_polynomial_naive(&a));
            }),
        ],
    );
}

fn benchmark_integer_matrix_characteristic_polynomial_bound(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.characteristic_polynomial_bound()",
        BenchmarkType::Single,
        integer_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [("Malachite", &mut |a| {
            no_out!(a.characteristic_polynomial_bound());
        })],
    );
}
