// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.

use malachite_base::num::arithmetic::traits::FallingFactorial;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;
use malachite_nz::test_util::natural::arithmetic::falling_factorial::falling_factorial_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_falling_factorial);
    register_demo!(runner, demo_natural_falling_factorial_ref);

    register_bench!(
        runner,
        benchmark_natural_falling_factorial_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_falling_factorial_algorithms);
}

fn demo_natural_falling_factorial(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in natural_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "{x_old}.falling_factorial({n}) = {}",
            x.falling_factorial(n)
        );
    }
}

fn demo_natural_falling_factorial_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in natural_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).falling_factorial({n}) = {}",
            (&x).falling_factorial(n)
        );
    }
}

fn benchmark_natural_falling_factorial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.falling_factorial(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_4::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("n"),
        &mut [
            ("Natural.falling_factorial(u64)", &mut |(x, n)| {
                no_out!(x.falling_factorial(n));
            }),
            ("(&Natural).falling_factorial(u64)", &mut |(x, n)| {
                no_out!((&x).falling_factorial(n));
            }),
        ],
    );
}

fn benchmark_natural_falling_factorial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.falling_factorial(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("n"),
        &mut [
            ("default", &mut |(x, n)| {
                no_out!((&x).falling_factorial(n));
            }),
            ("naive", &mut |(x, n)| {
                no_out!(falling_factorial_naive(&x, n));
            }),
        ],
    );
}
