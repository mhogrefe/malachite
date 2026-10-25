// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Determinant, DeterminantSign, Sign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_matrix_bit_bucketer;
use malachite_nz::test_util::generators::natural_matrix_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_matrix_determinant_sign);
    register_demo!(runner, demo_natural_matrix_determinant_sign_ref);
    register_bench!(runner, benchmark_natural_matrix_determinant_sign_algorithms);
    register_bench!(
        runner,
        benchmark_natural_matrix_determinant_sign_evaluation_strategy
    );
}

fn demo_natural_matrix_determinant_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen_var_1().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("determinant_sign({a_old}) = {:?}", a.determinant_sign());
    }
}

fn demo_natural_matrix_determinant_sign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in natural_matrix_gen_var_1().get(gm, config).take(limit) {
        println!("(&{a}).determinant_sign() = {:?}", (&a).determinant_sign());
    }
}

fn benchmark_natural_matrix_determinant_sign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.determinant_sign()",
        BenchmarkType::Algorithms,
        natural_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.determinant_sign());
            }),
            ("sign of the determinant", &mut |a| {
                no_out!(a.determinant().sign());
            }),
        ],
    );
}

fn benchmark_natural_matrix_determinant_sign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalMatrix.determinant_sign()",
        BenchmarkType::EvaluationStrategy,
        natural_matrix_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_matrix_bit_bucketer("a"),
        &mut [
            ("NaturalMatrix.determinant_sign()", &mut |a| {
                no_out!(a.determinant_sign());
            }),
            ("(&NaturalMatrix).determinant_sign()", &mut |a| {
                no_out!((&a).determinant_sign());
            }),
        ],
    );
}
