// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, SaturatingFrom};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::integer_bit_bucketer;
use malachite_nz::test_util::generators::{integer_gen, integer_gen_var_4};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_try_from_integer);
    register_demo!(runner, demo_natural_try_from_integer_ref);
    register_demo!(runner, demo_natural_exact_from_integer);
    register_demo!(runner, demo_natural_exact_from_integer_ref);
    register_demo!(runner, demo_natural_saturating_from_integer);
    register_demo!(runner, demo_natural_saturating_from_integer_ref);
    register_demo!(runner, demo_natural_convertible_from_integer);
    register_demo!(runner, demo_natural_convertible_from_integer_ref);

    register_bench!(
        runner,
        benchmark_natural_try_from_integer_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_exact_from_integer_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_from_integer_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_convertible_from_integer_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_convertible_from_integer_algorithms
    );
}

fn demo_natural_try_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        let n_clone = n.clone();
        println!(
            "Natural::try_from({}) = {:?}",
            n_clone,
            Natural::try_from(n)
        );
    }
}

fn demo_natural_try_from_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("Natural::try_from(&{}) = {:?}", n, Natural::try_from(&n));
    }
}

fn demo_natural_exact_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen_var_4().get(gm, config).take(limit) {
        let n_clone = n.clone();
        println!(
            "Natural::exact_from({}) = {}",
            n_clone,
            Natural::exact_from(n)
        );
    }
}

fn demo_natural_exact_from_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen_var_4().get(gm, config).take(limit) {
        println!("Natural::exact_from(&{}) = {}", n, Natural::exact_from(&n));
    }
}

fn demo_natural_saturating_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        let n_clone = n.clone();
        println!(
            "Natural::saturating_from({}) = {}",
            n_clone,
            Natural::saturating_from(n)
        );
    }
}

fn demo_natural_saturating_from_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "Natural::saturating_from(&{}) = {}",
            n,
            Natural::saturating_from(&n)
        );
    }
}

fn demo_natural_convertible_from_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        let n_clone = n.clone();
        println!(
            "{} is {}convertible to a Natural",
            n_clone,
            if Natural::convertible_from(n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_natural_convertible_from_integer_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Natural",
            n,
            if Natural::convertible_from(&n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_natural_try_from_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::try_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Natural::try_from(Integer)", &mut |n| {
                no_out!(Natural::try_from(n).ok())
            }),
            ("Natural::try_from(&Integer)", &mut |n| {
                no_out!(Natural::try_from(&n).ok())
            }),
        ],
    );
}

fn benchmark_natural_exact_from_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::exact_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Natural::exact_from(Integer)", &mut |n| {
                no_out!(Natural::exact_from(n))
            }),
            ("Natural::exact_from(&Integer)", &mut |n| {
                no_out!(Natural::exact_from(&n))
            }),
        ],
    );
}

fn benchmark_natural_saturating_from_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::saturating_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Natural::saturating_from(Integer)", &mut |n| {
                no_out!(Natural::saturating_from(n))
            }),
            ("Natural::saturating_from(&Integer)", &mut |n| {
                no_out!(Natural::saturating_from(&n))
            }),
        ],
    );
}

fn benchmark_natural_convertible_from_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::convertible_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("Natural::convertible_from(Integer)", &mut |n| {
                no_out!(Natural::convertible_from(n))
            }),
            ("Natural::convertible_from(&Integer)", &mut |n| {
                no_out!(Natural::convertible_from(&n))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_convertible_from_integer_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::convertible_from(Integer)",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [
            ("standard", &mut |n| no_out!(Natural::convertible_from(n))),
            ("using try_from", &mut |n| {
                no_out!(Natural::try_from(n).is_ok())
            }),
        ],
    );
}
