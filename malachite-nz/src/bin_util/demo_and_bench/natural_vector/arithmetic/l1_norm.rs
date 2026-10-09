// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::L1Norm;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_to_l1_norm);
    register_demo!(runner, demo_natural_vector_into_l1_norm);
    register_demo!(runner, demo_natural_vector_l1_norm_significant_bits);
    register_bench!(runner, benchmark_natural_vector_l1_norm_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_natural_vector_l1_norm_significant_bits_algorithms
    );
}

fn demo_natural_vector_to_l1_norm(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!("{v}.to_l1_norm() = {}", v.to_l1_norm());
    }
}

fn demo_natural_vector_into_l1_norm(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        let v_old = v.clone();
        println!("{v_old}.into_l1_norm() = {}", v.into_l1_norm());
    }
}

fn demo_natural_vector_l1_norm_significant_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for v in natural_vector_gen().get(gm, config).take(limit) {
        println!(
            "{v}.l1_norm_significant_bits() = {}",
            v.l1_norm_significant_bits()
        );
    }
}

fn benchmark_natural_vector_l1_norm_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.to_l1_norm()",
        BenchmarkType::EvaluationStrategy,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.to_l1_norm()", &mut |v| {
                no_out!(v.to_l1_norm());
            }),
            ("NaturalVector.into_l1_norm()", &mut |v| {
                no_out!(v.into_l1_norm());
            }),
        ],
    );
}

fn benchmark_natural_vector_l1_norm_significant_bits_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.l1_norm_significant_bits()",
        BenchmarkType::Algorithms,
        natural_vector_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |v| {
                no_out!(v.l1_norm_significant_bits());
            }),
            ("using to_l1_norm", &mut |v| {
                no_out!(v.to_l1_norm().significant_bits());
            }),
        ],
    );
}
