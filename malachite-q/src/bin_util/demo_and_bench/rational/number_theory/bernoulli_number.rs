// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_q::Rational;
use malachite_q::rational::number_theory::bernoulli_number::*;
use malachite_q::test_util::rational::number_theory::bernoulli_number::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_bernoulli_number);
    register_demo!(runner, demo_bernoulli_numbers_prefix);
    register_demo!(runner, demo_even_bernoulli_numbers_prefix);
    register_bench!(runner, benchmark_rational_bernoulli_number_algorithms);
    register_bench!(runner, benchmark_bernoulli_numbers_prefix_algorithms);
    register_bench!(runner, benchmark_even_bernoulli_numbers_prefix_algorithms);
}

fn demo_rational_bernoulli_number(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "bernoulli_number({}) = {}",
            n,
            Rational::bernoulli_number(u64::from(n))
        );
    }
}

fn demo_bernoulli_numbers_prefix(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "bernoulli_numbers_prefix({}) = {:?}",
            n,
            bernoulli_numbers_prefix(u64::from(n))
        );
    }
}

fn demo_even_bernoulli_numbers_prefix(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<u8>().get(gm, config).take(limit) {
        println!(
            "even_bernoulli_numbers_prefix({}) = {:?}",
            n,
            even_bernoulli_numbers_prefix(u64::from(n))
        );
    }
}

fn benchmark_rational_bernoulli_number_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::bernoulli_number(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(Rational::bernoulli_number(u64::from(n)));
            }),
            ("vector", &mut |n| {
                no_out!(bernoulli_numbers_prefix(u64::from(n) + 1).pop());
            }),
        ],
    );
}

fn benchmark_bernoulli_numbers_prefix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "bernoulli_numbers_prefix(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(bernoulli_numbers_prefix(u64::from(n)));
            }),
            ("tangent", &mut |n| {
                no_out!(bernoulli_numbers_prefix_tangent(u64::from(n)));
            }),
        ],
    );
}

fn benchmark_even_bernoulli_numbers_prefix_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "even_bernoulli_numbers_prefix(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Ramanujan", &mut |n| {
                no_out!(even_bernoulli_numbers_recursive(usize::exact_from(n) + 1));
            }),
            ("multimodular", &mut |n| {
                no_out!(even_bernoulli_numbers_multi_mod(usize::exact_from(n) + 1));
            }),
            ("tangent", &mut |n| {
                no_out!(even_bernoulli_numbers_prefix_tangent(u64::from(n)));
            }),
        ],
    );
}
