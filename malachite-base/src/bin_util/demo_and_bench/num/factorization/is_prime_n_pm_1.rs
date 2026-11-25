// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::is_prime_n_pm_1::u128_prove_prime;
use malachite_base::num::factorization::traits::IsProbablePrime;
use malachite_base::test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_37;
use malachite_base::test_util::num::factorization::is_prime_pseudosquare::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u128_prove_prime);
    register_demo!(runner, demo_u128_is_prime_pseudosquare);
    register_bench!(runner, benchmark_u128_prove_prime_algorithms);
}

fn demo_u128_prove_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_37().get(gm, config).take(limit) {
        match u128_prove_prime(u) {
            Some(true) => println!("{u} is proven prime"),
            Some(false) => println!("{u} is proven composite"),
            None => println!("{u} is not settled"),
        }
    }
}

fn demo_u128_is_prime_pseudosquare(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_37().get(gm, config).take(limit) {
        match u128_is_prime_pseudosquare(u) {
            Some(true) => println!("{u} is proven prime"),
            Some(false) => println!("{u} is proven composite"),
            None => println!("{u} is too large for the pseudosquare test"),
        }
    }
}

fn benchmark_u128_prove_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u128_prove_prime(u128)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_37().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("default", &mut |u| {
                std::hint::black_box(u128_prove_prime(u));
            }),
            ("pseudosquare", &mut |u| {
                std::hint::black_box(u128_is_prime_pseudosquare(u));
            }),
            ("Baillie-PSW", &mut |u| {
                std::hint::black_box(u.is_probable_prime());
            }),
        ],
    );
}
