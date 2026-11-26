// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_8, natural_gen_var_10};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_is_prime);
    register_demo!(runner, demo_natural_is_prime_odd);
    register_demo!(runner, demo_natural_is_prime_primes);
    register_bench!(runner, benchmark_natural_is_prime_algorithms);
}

fn demo_natural_is_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        if n.is_prime() {
            println!("{n} is prime");
        } else {
            println!("{n} is not prime");
        }
    }
}

fn demo_natural_is_prime_odd(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_8().get(gm, config).take(limit) {
        if n.is_prime() {
            println!("{n} is prime");
        } else {
            println!("{n} is not prime");
        }
    }
}

fn demo_natural_is_prime_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_10().get(gm, config).take(limit) {
        if n.is_prime() {
            println!("{n} is prime");
        } else {
            println!("{n} is not prime");
        }
    }
}

fn benchmark_natural_is_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.is_prime()",
        BenchmarkType::Algorithms,
        natural_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            ("default", &mut |n| {
                std::hint::black_box(n.is_prime());
            }),
            ("Baillie-PSW", &mut |n| {
                std::hint::black_box(n.is_probable_prime());
            }),
        ],
    );
}
