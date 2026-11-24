// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::IsStrongProbablePrime;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen;
use malachite_base::test_util::num::factorization::is_strong_probable_prime::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u8_is_strong_probable_prime);
    register_demo!(runner, demo_u16_is_strong_probable_prime);
    register_demo!(runner, demo_u32_is_strong_probable_prime);
    register_demo!(runner, demo_u64_is_strong_probable_prime);
    register_demo!(runner, demo_u128_is_strong_probable_prime);
    register_demo!(runner, demo_usize_is_strong_probable_prime);
    register_bench!(runner, benchmark_u8_is_strong_probable_prime);
    register_bench!(runner, benchmark_u16_is_strong_probable_prime);
    register_bench!(runner, benchmark_u32_is_strong_probable_prime);
    register_bench!(runner, benchmark_u64_is_strong_probable_prime);
    register_bench!(runner, benchmark_u128_is_strong_probable_prime);
    register_bench!(runner, benchmark_usize_is_strong_probable_prime);
    register_bench!(runner, benchmark_u128_is_strong_probable_prime_algorithms);
}

fn demo_u8_is_strong_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in unsigned_pair_gen::<u8, u8>().get(gm, config).take(limit) {
        if n.is_strong_probable_prime(a) {
            println!("{n} is a strong probable prime to base {a}");
        } else {
            println!("{n} is not a strong probable prime to base {a}");
        }
    }
}

fn demo_u16_is_strong_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in unsigned_pair_gen::<u16, u16>().get(gm, config).take(limit) {
        if n.is_strong_probable_prime(a) {
            println!("{n} is a strong probable prime to base {a}");
        } else {
            println!("{n} is not a strong probable prime to base {a}");
        }
    }
}

fn demo_u32_is_strong_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in unsigned_pair_gen::<u32, u32>().get(gm, config).take(limit) {
        if n.is_strong_probable_prime(a) {
            println!("{n} is a strong probable prime to base {a}");
        } else {
            println!("{n} is not a strong probable prime to base {a}");
        }
    }
}

fn demo_u64_is_strong_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in unsigned_pair_gen::<u64, u64>().get(gm, config).take(limit) {
        if n.is_strong_probable_prime(a) {
            println!("{n} is a strong probable prime to base {a}");
        } else {
            println!("{n} is not a strong probable prime to base {a}");
        }
    }
}

fn demo_u128_is_strong_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in unsigned_pair_gen::<u128, u128>()
        .get(gm, config)
        .take(limit)
    {
        if n.is_strong_probable_prime(a) {
            println!("{n} is a strong probable prime to base {a}");
        } else {
            println!("{n} is not a strong probable prime to base {a}");
        }
    }
}

fn demo_usize_is_strong_probable_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, a) in unsigned_pair_gen::<usize, usize>()
        .get(gm, config)
        .take(limit)
    {
        if n.is_strong_probable_prime(a) {
            println!("{n} is a strong probable prime to base {a}");
        } else {
            println!("{n} is not a strong probable prime to base {a}");
        }
    }
}

fn benchmark_u8_is_strong_probable_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u8.is_strong_probable_prime(u8)",
        BenchmarkType::Single,
        unsigned_pair_gen::<u8, u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "a"),
        &mut [("Malachite", &mut |(n, a)| {
            std::hint::black_box(n.is_strong_probable_prime(a));
        })],
    );
}

fn benchmark_u16_is_strong_probable_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u16.is_strong_probable_prime(u16)",
        BenchmarkType::Single,
        unsigned_pair_gen::<u16, u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "a"),
        &mut [("Malachite", &mut |(n, a)| {
            std::hint::black_box(n.is_strong_probable_prime(a));
        })],
    );
}

fn benchmark_u32_is_strong_probable_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.is_strong_probable_prime(u32)",
        BenchmarkType::Single,
        unsigned_pair_gen::<u32, u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "a"),
        &mut [("Malachite", &mut |(n, a)| {
            std::hint::black_box(n.is_strong_probable_prime(a));
        })],
    );
}

fn benchmark_u64_is_strong_probable_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.is_strong_probable_prime(u64)",
        BenchmarkType::Single,
        unsigned_pair_gen::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "a"),
        &mut [("Malachite", &mut |(n, a)| {
            std::hint::black_box(n.is_strong_probable_prime(a));
        })],
    );
}

fn benchmark_u128_is_strong_probable_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u128.is_strong_probable_prime(u128)",
        BenchmarkType::Single,
        unsigned_pair_gen::<u128, u128>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "a"),
        &mut [("Malachite", &mut |(n, a)| {
            std::hint::black_box(n.is_strong_probable_prime(a));
        })],
    );
}

fn benchmark_usize_is_strong_probable_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "usize.is_strong_probable_prime(usize)",
        BenchmarkType::Single,
        unsigned_pair_gen::<usize, usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "a"),
        &mut [("Malachite", &mut |(n, a)| {
            std::hint::black_box(n.is_strong_probable_prime(a));
        })],
    );
}

fn benchmark_u128_is_strong_probable_prime_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u128.is_strong_probable_prime(u128)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen::<u128, u128>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("n", "a"),
        &mut [
            ("default", &mut |(n, a)| {
                std::hint::black_box(n.is_strong_probable_prime(a));
            }),
            ("naive", &mut |(n, a)| {
                std::hint::black_box(is_strong_probable_prime_naive(n, a));
            }),
        ],
    );
}
