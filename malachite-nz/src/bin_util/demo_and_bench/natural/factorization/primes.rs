// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::factorization::traits::Primes;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::natural_bucketer;
use malachite_nz::test_util::generators::natural_gen_var_9;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_primes_less_than);
    register_demo!(runner, demo_natural_primes_less_than_or_equal_to);
    register_demo!(runner, demo_natural_primes);

    register_bench!(runner, benchmark_natural_primes_less_than_algorithms);
    register_bench!(
        runner,
        benchmark_natural_primes_less_than_or_equal_to_algorithms
    );
}

fn demo_natural_primes_less_than(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_9().get(gm, config).take(limit) {
        println!(
            "primes_less_than({}) = {:?}",
            n,
            Natural::primes_less_than(&n).collect_vec()
        );
    }
}

fn demo_natural_primes_less_than_or_equal_to(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_9().get(gm, config).take(limit) {
        println!(
            "primes_less_than_or_equal_to({}) = {:?}",
            n,
            Natural::primes_less_than_or_equal_to(&n).collect_vec()
        );
    }
}
fn demo_natural_primes(_gm: GenMode, _config: &GenConfig, limit: usize) {
    for p in Natural::primes().take(limit) {
        println!("{p}");
    }
}

fn benchmark_natural_primes_less_than_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::primes_less_than(&Natural)",
        BenchmarkType::Algorithms,
        natural_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bucketer("n"),
        &mut [
            ("default", &mut |n| {
                no_out!(Natural::primes_less_than(&n).count())
            }),
            ("using primes", &mut |n| {
                no_out!(Natural::primes().take_while(|p| *p < n).count())
            }),
        ],
    );
}

fn benchmark_natural_primes_less_than_or_equal_to_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::primes_less_than_or_equal_to(&Natural)",
        BenchmarkType::Algorithms,
        natural_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bucketer("n"),
        &mut [
            ("default", &mut |n| {
                no_out!(Natural::primes_less_than_or_equal_to(&n).count())
            }),
            ("using primes", &mut |n| {
                no_out!(Natural::primes().take_while(|p| *p <= n).count())
            }),
        ],
    );
}
