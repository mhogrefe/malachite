// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::is_prime_aprcl::u128_is_prime_aprcl;
use malachite_base::num::factorization::traits::{IsPrime, IsProbablePrime};
use malachite_base::test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen_var_37, unsigned_gen_var_38};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u128_is_prime_aprcl);
    register_demo!(runner, demo_u128_is_prime_aprcl_primes);
    register_bench!(runner, benchmark_u128_is_prime_aprcl_algorithms);
}

fn demo_u128_is_prime_aprcl(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_38().get(gm, config).take(limit) {
        if u128_is_prime_aprcl(u) {
            println!("{u} is proven prime");
        } else {
            println!("{u} is proven composite");
        }
    }
}

fn demo_u128_is_prime_aprcl_primes(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_37().get(gm, config).take(limit) {
        if u128_is_prime_aprcl(u) {
            println!("{u} is proven prime");
        } else {
            println!("{u} is proven composite");
        }
    }
}

fn benchmark_u128_is_prime_aprcl_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u128_is_prime_aprcl(u128)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_37().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [
            ("APR-CL", &mut |u| {
                std::hint::black_box(u128_is_prime_aprcl(u));
            }),
            ("is_prime", &mut |u| {
                std::hint::black_box(u.is_prime());
            }),
            ("Baillie-PSW", &mut |u| {
                std::hint::black_box(u.is_probable_prime());
            }),
        ],
    );
}
