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
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_convertible_from_integer_vector);
    register_bench!(
        runner,
        benchmark_natural_vector_convertible_from_integer_vector_algorithms
    );
    register_demo!(runner, demo_natural_vector_try_from_integer_vector);
    register_demo!(runner, demo_natural_vector_try_from_integer_vector_ref);
    register_bench!(
        runner,
        benchmark_natural_vector_try_from_integer_vector_evaluation_strategy
    );
}

fn demo_natural_vector_try_from_integer_vector(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!(
            "NaturalVector::try_from({}) = {:?}",
            v_old,
            NaturalVector::try_from(v).map(|w| w.to_string())
        );
    }
}

fn demo_natural_vector_try_from_integer_vector_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        println!(
            "NaturalVector::try_from(&{}) = {:?}",
            v,
            NaturalVector::try_from(&v).map(|w| w.to_string())
        );
    }
}

fn benchmark_natural_vector_try_from_integer_vector_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::try_from(IntegerVector)",
        BenchmarkType::EvaluationStrategy,
        integer_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector::try_from(IntegerVector)", &mut |v| {
                let _ = NaturalVector::try_from(v);
            }),
            ("NaturalVector::try_from(&IntegerVector)", &mut |v| {
                let _ = NaturalVector::try_from(&v);
            }),
        ],
    );
}

fn demo_natural_vector_convertible_from_integer_vector(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for v in integer_vector_gen().get(gm, config).take(limit) {
        println!(
            "NaturalVector::convertible_from(&{}) = {}",
            v,
            NaturalVector::convertible_from(&v)
        );
    }
}

fn benchmark_natural_vector_convertible_from_integer_vector_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector::convertible_from(&IntegerVector)",
        BenchmarkType::Algorithms,
        integer_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_vector_bit_bucketer("v"),
        &mut [
            ("standard", &mut |v| {
                no_out!(NaturalVector::convertible_from(&v));
            }),
            ("using try_from", &mut |v| {
                no_out!(NaturalVector::try_from(&v).is_ok());
            }),
        ],
    );
}
