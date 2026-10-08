// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_vector::IntegerVector;
use malachite_q::test_util::bench::bucketers::rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_convertible_from_rational_vector);
    register_bench!(
        runner,
        benchmark_integer_vector_convertible_from_rational_vector_algorithms
    );
    register_demo!(runner, demo_integer_vector_try_from_rational_vector);
    register_demo!(runner, demo_integer_vector_try_from_rational_vector_ref);
    register_bench!(
        runner,
        benchmark_integer_vector_try_from_rational_vector_evaluation_strategy
    );
}

fn demo_integer_vector_try_from_rational_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!(
            "IntegerVector::try_from({}) = {:?}",
            v_old,
            IntegerVector::try_from(v).map(|w| w.to_string())
        );
    }
}

fn demo_integer_vector_try_from_rational_vector_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!(
            "IntegerVector::try_from(&{}) = {:?}",
            v,
            IntegerVector::try_from(&v).map(|w| w.to_string())
        );
    }
}

fn benchmark_integer_vector_try_from_rational_vector_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector::try_from(RationalVector)",
        BenchmarkType::EvaluationStrategy,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("IntegerVector::try_from(RationalVector)", &mut |v| {
                let _ = IntegerVector::try_from(v);
            }),
            ("IntegerVector::try_from(&RationalVector)", &mut |v| {
                let _ = IntegerVector::try_from(&v);
            }),
        ],
    );
}

fn demo_integer_vector_convertible_from_rational_vector(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for v in rational_vector_gen().get(gm, config).take(limit) {
        println!(
            "IntegerVector::convertible_from(&{}) = {}",
            v,
            IntegerVector::convertible_from(&v)
        );
    }
}

fn benchmark_integer_vector_convertible_from_rational_vector_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector::convertible_from(&RationalVector)",
        BenchmarkType::Algorithms,
        rational_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_vector_bit_bucketer("v"),
        &mut [
            ("standard", &mut |v| {
                no_out!(IntegerVector::convertible_from(&v));
            }),
            ("using try_from", &mut |v| {
                no_out!(IntegerVector::try_from(&v).is_ok());
            }),
        ],
    );
}
