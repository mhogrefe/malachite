// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Two;
use malachite_base::num::factorization::traits::IsStrongProbablePrime;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen_var_8, natural_pair_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_is_strong_probable_prime);
    register_demo!(runner, demo_natural_is_strong_probable_prime_ref);
    register_demo!(runner, demo_natural_is_strong_probable_prime_odd_base_2);
    register_bench!(
        runner,
        benchmark_natural_is_strong_probable_prime_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_is_strong_probable_prime_odd_base_2
    );
}

fn demo_natural_is_strong_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in natural_pair_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        if n.is_strong_probable_prime(a) {
            println!("{n} is a strong probable prime to base {a_old}");
        } else {
            println!("{n} is not a strong probable prime to base {a_old}");
        }
    }
}

fn demo_natural_is_strong_probable_prime_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in natural_pair_gen().get(gm, config).take(limit) {
        if n.is_strong_probable_prime(&a) {
            println!("{n} is a strong probable prime to base {a}");
        } else {
            println!("{n} is not a strong probable prime to base {a}");
        }
    }
}

fn demo_natural_is_strong_probable_prime_odd_base_2(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_8().get(gm, config).take(limit) {
        if n.is_strong_probable_prime(Natural::TWO) {
            println!("{n} is a strong probable prime to base 2");
        } else {
            println!("{n} is not a strong probable prime to base 2");
        }
    }
}

fn benchmark_natural_is_strong_probable_prime_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.is_strong_probable_prime(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("n", "a"),
        &mut [
            (
                "Natural.is_strong_probable_prime(Natural)",
                &mut |(n, a)| {
                    no_out!(n.is_strong_probable_prime(a));
                },
            ),
            (
                "Natural.is_strong_probable_prime(&Natural)",
                &mut |(n, a)| {
                    no_out!(n.is_strong_probable_prime(&a));
                },
            ),
        ],
    );
}

fn benchmark_natural_is_strong_probable_prime_odd_base_2(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    let two = Natural::TWO;
    run_benchmark(
        "Natural.is_strong_probable_prime(2)",
        BenchmarkType::Single,
        natural_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| {
            no_out!(n.is_strong_probable_prime(&two));
        })],
    );
}
