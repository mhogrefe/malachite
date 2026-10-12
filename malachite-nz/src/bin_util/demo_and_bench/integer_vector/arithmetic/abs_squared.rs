// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Dot};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_nz::test_util::integer_vector::arithmetic::abs_squared::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_abs_squared);
    register_demo!(runner, demo_integer_vector_abs_squared_ref);
    register_bench!(runner, benchmark_integer_vector_abs_squared_algorithms);
    register_bench!(
        runner,
        benchmark_integer_vector_abs_squared_evaluation_strategy
    );
}

fn demo_integer_vector_abs_squared(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.abs_squared() = {}", v.abs_squared());
    }
}

fn demo_integer_vector_abs_squared_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        println!("(&{v}).abs_squared() = {}", (&v).abs_squared());
    }
}

fn benchmark_integer_vector_abs_squared_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.abs_squared()",
        BenchmarkType::Algorithms,
        integer_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |v| {
                no_out!((&v).abs_squared());
            }),
            ("dot product with itself", &mut |v| {
                no_out!((&v).dot(&v));
            }),
            ("naive", &mut |v| {
                no_out!(integer_vector_abs_squared_naive(&v));
            }),
        ],
    );
}

fn benchmark_integer_vector_abs_squared_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.abs_squared()",
        BenchmarkType::EvaluationStrategy,
        integer_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_vector_bit_bucketer("v"),
        &mut [
            ("IntegerVector.abs_squared()", &mut |v| {
                no_out!(v.abs_squared());
            }),
            ("(&IntegerVector).abs_squared()", &mut |v| {
                no_out!((&v).abs_squared());
            }),
        ],
    );
}
