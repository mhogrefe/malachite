// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Permanent;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::arithmetic::permanent::*;
use malachite_nz::test_util::bench::bucketers::integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::{integer_matrix_gen_var_2, integer_matrix_gen_var_4};
use malachite_nz::test_util::integer_matrix::arithmetic::permanent::integer_matrix_permanent_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_permanent);
    register_demo!(runner, demo_integer_matrix_permanent_ref);
    register_demo!(runner, demo_integer_matrix_permanent_cofactor);
    register_demo!(runner, demo_integer_matrix_permanent_glynn);
    register_demo!(runner, demo_integer_matrix_permanent_dense);
    register_bench!(runner, benchmark_integer_matrix_permanent_algorithms);
    register_bench!(runner, benchmark_integer_matrix_permanent_small_algorithms);
    register_bench!(
        runner,
        benchmark_integer_matrix_permanent_evaluation_strategy
    );
}

fn demo_integer_matrix_permanent(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_4().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!("permanent({a_old}) = {}", a.permanent());
    }
}

fn demo_integer_matrix_permanent_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_4().get(gm, config).take(limit) {
        println!("(&{a}).permanent() = {}", (&a).permanent());
    }
}

fn demo_integer_matrix_permanent_cofactor(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_2().get(gm, config).take(limit) {
        println!("permanent_cofactor({a}) = {}", permanent_cofactor(&a));
    }
}

fn demo_integer_matrix_permanent_glynn(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_4()
        .get(gm, config)
        .filter(|a| a.height() != 0)
        .take(limit)
    {
        println!("permanent_glynn({a}) = {}", permanent_glynn(&a));
    }
}

fn demo_integer_matrix_permanent_dense(gm: GenMode, config: &GenConfig, limit: usize) {
    for a in integer_matrix_gen_var_4().get(gm, config).take(limit) {
        println!("permanent_dense({a}) = {}", permanent_dense(&a));
    }
}

fn benchmark_integer_matrix_permanent_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.permanent()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.permanent());
            }),
            ("without looking for structure", &mut |a| {
                no_out!(permanent_dense(&a));
            }),
            ("cofactor expansion", &mut |a| {
                no_out!(permanent_cofactor(&a));
            }),
            ("naive", &mut |a| {
                no_out!(integer_matrix_permanent_naive(&a));
            }),
        ],
    );
}

fn benchmark_integer_matrix_permanent_small_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.permanent()",
        BenchmarkType::Algorithms,
        integer_matrix_gen_var_2()
            .get(gm, config)
            .filter(|a| a.height() != 0),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |a| {
                no_out!(a.permanent());
            }),
            ("cofactor expansion", &mut |a| {
                no_out!(permanent_cofactor(&a));
            }),
            ("Glynn", &mut |a| {
                no_out!(permanent_glynn(&a));
            }),
        ],
    );
}

fn benchmark_integer_matrix_permanent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.permanent()",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.permanent()", &mut |a| {
                no_out!(a.permanent());
            }),
            ("(&IntegerMatrix).permanent()", &mut |a| {
                no_out!((&a).permanent());
            }),
        ],
    );
}
