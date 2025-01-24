// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Primorial;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::arithmetic::primorial::{
    primorial_naive, product_of_first_n_primes_naive,
};
use rug::Complete;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_primorial);
    register_demo!(runner, demo_product_of_first_n_primes);

    register_bench!(runner, benchmark_primorial_library_comparison);
    register_bench!(runner, benchmark_primorial_algorithms);
    register_bench!(runner, benchmark_product_of_first_n_primes_algorithms);
}

fn demo_primorial(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("{}# = {}", n, Natural::primorial(n));
    }
}

fn demo_product_of_first_n_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("p_{}# = {}", n, Natural::product_of_first_n_primes(n));
    }
}

fn benchmark_primorial_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.primorial(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |n| no_out!(Natural::primorial(n))),
            ("rug", &mut |n| {
                no_out!(rug::Integer::primorial(u32::exact_from(n)).complete())
            }),
        ],
    );
}

fn benchmark_primorial_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.primorial(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(Natural::primorial(n))),
            ("naive", &mut |n| no_out!(primorial_naive(n))),
        ],
    );
}

fn benchmark_product_of_first_n_primes_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.product_of_first_n_primes(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(Natural::product_of_first_n_primes(n))
            }),
            ("naive", &mut |n| {
                no_out!(product_of_first_n_primes_naive(n))
            }),
        ],
    );
}
