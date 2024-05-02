// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::prime_sieve::{
    limbs_prime_sieve_size, limbs_prime_sieve_u32, limbs_prime_sieve_u64,
};
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_26;
use malachite_base::test_util::num::factorization::prime_sieve::{
    limbs_prime_sieve_naive_1, limbs_prime_sieve_naive_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_prime_sieve_u32);
    register_demo!(runner, demo_limbs_prime_sieve_u64);
    register_bench!(runner, benchmark_limbs_prime_sieve_u32_algorithms);
    register_bench!(runner, benchmark_limbs_prime_sieve_u64_algorithms);
}

fn demo_limbs_prime_sieve_u32(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_26().get(gm, config).take(limit) {
        let len = limbs_prime_sieve_size::<u32>(n);
        let mut sieve = vec![0; len];
        limbs_prime_sieve_u32(&mut sieve, n);
        print!("limbs_prime_sieve_u32({n}): ");
        let mut first = true;
        for s in sieve {
            if first {
                first = false;
            } else {
                print!(", ");
            }
            print!("{s:b}");
        }
        println!();
    }
}

fn demo_limbs_prime_sieve_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_26().get(gm, config).take(limit) {
        let len = limbs_prime_sieve_size::<u64>(n);
        let mut sieve = vec![0; len];
        limbs_prime_sieve_u64(&mut sieve, n);
        print!("limbs_prime_sieve_u64({n}): ");
        let mut first = true;
        for s in sieve {
            if first {
                first = false;
            } else {
                print!(", ");
            }
            print!("{s:b}");
        }
        println!();
    }
}

fn benchmark_limbs_prime_sieve_u32_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_prime_sieve_u32(&mut [Limb], u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_26().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                let len = limbs_prime_sieve_size::<u32>(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_u32(&mut sieve, n);
            }),
            ("test each prime separately", &mut |n| {
                let len = limbs_prime_sieve_size::<u32>(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_naive_1::<u32>(&mut sieve, n);
            }),
            ("naive sieve", &mut |n| {
                let len = limbs_prime_sieve_size::<u32>(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_naive_2::<u32>(&mut sieve, n);
            }),
        ],
    );
}

fn benchmark_limbs_prime_sieve_u64_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_prime_sieve_u64(&mut [Limb], u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_26().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                let len = limbs_prime_sieve_size::<u64>(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_u64(&mut sieve, n);
            }),
            ("test each prime separately", &mut |n| {
                let len = limbs_prime_sieve_size::<u64>(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_naive_1::<u64>(&mut sieve, n);
            }),
            ("naive sieve", &mut |n| {
                let len = limbs_prime_sieve_size::<u64>(n);
                let mut sieve = vec![0; len];
                limbs_prime_sieve_naive_2::<u64>(&mut sieve, n);
            }),
        ],
    );
}
