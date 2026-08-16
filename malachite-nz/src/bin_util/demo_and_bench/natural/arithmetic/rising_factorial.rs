// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BinomialCoefficient, Factorial, RisingFactorial};
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_rising_factorial);
    register_demo!(runner, demo_natural_rising_factorial_ref);

    register_bench!(
        runner,
        benchmark_natural_rising_factorial_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_rising_factorial_algorithms);
}

fn demo_natural_rising_factorial(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in natural_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("{x_old}.rising_factorial({n}) = {}", x.rising_factorial(n));
    }
}

fn demo_natural_rising_factorial_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in natural_unsigned_pair_gen_var_4::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).rising_factorial({n}) = {}",
            (&x).rising_factorial(n)
        );
    }
}

fn benchmark_natural_rising_factorial_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.rising_factorial(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_4::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("n"),
        &mut [
            ("Natural.rising_factorial(u64)", &mut |(x, n)| {
                no_out!(x.rising_factorial(n));
            }),
            ("(&Natural).rising_factorial(u64)", &mut |(x, n)| {
                no_out!((&x).rising_factorial(n));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_rising_factorial_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.rising_factorial(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("n"),
        &mut [
            ("default", &mut |(x, n)| {
                no_out!((&x).rising_factorial(n));
            }),
            ("binomial identity", &mut |(x, n)| {
                if n != 0 && x != 0u32 {
                    no_out!(
                        Natural::binomial_coefficient(&x + Natural::from(n - 1), Natural::from(n))
                            * Natural::factorial(n)
                    );
                }
            }),
        ],
    );
}
