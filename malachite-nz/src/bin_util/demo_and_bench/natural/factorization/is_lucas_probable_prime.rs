// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::IsLucasProbablePrime;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::natural_bit_bucketer;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_8};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_is_lucas_probable_prime);
    register_demo!(runner, demo_natural_is_lucas_probable_prime_odd);
    register_bench!(runner, benchmark_natural_is_lucas_probable_prime_odd);
}

fn demo_natural_is_lucas_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        if n.is_lucas_probable_prime() {
            println!("{n} is a Lucas probable prime");
        } else {
            println!("{n} is not a Lucas probable prime");
        }
    }
}

fn demo_natural_is_lucas_probable_prime_odd(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen_var_8().get(gm, config).take(limit) {
        if n.is_lucas_probable_prime() {
            println!("{n} is a Lucas probable prime");
        } else {
            println!("{n} is not a Lucas probable prime");
        }
    }
}

fn benchmark_natural_is_lucas_probable_prime_odd(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.is_lucas_probable_prime()",
        BenchmarkType::Single,
        natural_gen_var_8().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(n.is_lucas_probable_prime()))],
    );
}
