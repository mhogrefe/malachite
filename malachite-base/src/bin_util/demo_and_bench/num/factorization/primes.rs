// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::primes::{
    prime_indicator_sequence, prime_indicator_sequence_less_than,
    prime_indicator_sequence_less_than_or_equal_to,
};
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_pair_gen_var_28, unsigned_triple_gen_var_24,
};
use malachite_base::test_util::num::factorization::primes::primes_naive;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_primes_less_than);
    register_unsigned_demos!(runner, demo_primes_less_than_or_equal_to);
    register_unsigned_demos!(runner, demo_primes);
    register_demo!(runner, demo_prime_indicator_sequence_less_than);
    register_demo!(runner, demo_prime_indicator_sequence_less_than_or_equal_to);
    register_demo!(runner, demo_prime_indicator_sequence);
    runner.register_demo(
        "demo_primes_less_than_jump_after_u8",
        &demo_primes_less_than_jump_after::<u8>,
    );
    runner.register_demo(
        "demo_primes_less_than_jump_after_u16",
        &demo_primes_less_than_jump_after::<u16>,
    );
    runner.register_demo(
        "demo_primes_less_than_jump_after_u32",
        &demo_primes_less_than_jump_after::<u32>,
    );
    runner.register_demo(
        "demo_primes_less_than_jump_after_u64",
        &demo_primes_less_than_jump_after::<u64>,
    );
    runner.register_demo(
        "demo_primes_less_than_jump_after_u128",
        &demo_primes_less_than_jump_after::<u128>,
    );
    runner.register_demo(
        "demo_primes_less_than_jump_after_usize",
        &demo_primes_less_than_jump_after::<usize>,
    );
    runner.register_demo("demo_primes_jump_after_u8", &demo_primes_jump_after::<u8>);
    runner.register_demo("demo_primes_jump_after_u16", &demo_primes_jump_after::<u16>);
    runner.register_demo("demo_primes_jump_after_u32", &demo_primes_jump_after::<u32>);
    runner.register_demo("demo_primes_jump_after_u64", &demo_primes_jump_after::<u64>);
    runner.register_demo(
        "demo_primes_jump_after_u128",
        &demo_primes_jump_after::<u128>,
    );
    runner.register_demo(
        "demo_primes_jump_after_usize",
        &demo_primes_jump_after::<usize>,
    );

    register_unsigned_benches!(runner, benchmark_primes_less_than_algorithms);
    register_unsigned_benches!(runner, benchmark_primes_less_than_algorithms_2);
    register_unsigned_benches!(runner, benchmark_primes_less_than_or_equal_to_algorithms);
    register_unsigned_benches!(runner, benchmark_primes_less_than_or_equal_to_algorithms_2);
    register_bench!(runner, benchmark_prime_indicator_sequence_less_than);
    register_bench!(
        runner,
        benchmark_prime_indicator_sequence_less_than_or_equal_to
    );
}

fn demo_primes_less_than<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5::<T>().get(gm, config).take(limit) {
        println!(
            "primes_less_than({}) = {:?}",
            n,
            T::primes_less_than(&n).collect_vec()
        );
    }
}

fn demo_primes_less_than_or_equal_to<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen_var_5::<T>().get(gm, config).take(limit) {
        println!(
            "primes_less_than_or_equal_to({}) = {:?}",
            n,
            T::primes_less_than_or_equal_to(&n).collect_vec()
        );
    }
}

fn demo_primes<T: PrimitiveUnsigned>(_gm: GenMode, _config: &GenConfig, limit: usize) {
    for p in T::primes().take(limit) {
        println!("{p}");
    }
}

fn demo_prime_indicator_sequence_less_than(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!(
            "prime_indicator_sequence_less_than({}) = {:?}",
            n,
            prime_indicator_sequence_less_than(n).collect_vec()
        );
    }
}

fn demo_prime_indicator_sequence_less_than_or_equal_to(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!(
            "prime_indicator_sequence_less_than_or_equal_to({}) = {:?}",
            n,
            prime_indicator_sequence_less_than_or_equal_to(n).collect_vec()
        );
    }
}

fn demo_prime_indicator_sequence(_gm: GenMode, _config: &GenConfig, limit: usize) {
    for b in prime_indicator_sequence().take(limit) {
        println!("{b}");
    }
}

fn demo_primes_less_than_jump_after<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    let mut config = config.clone();
    config.insert("mean_small_n", 10000);
    for (size, jump, skip) in unsigned_triple_gen_var_24::<T, usize>()
        .get(gm, &config)
        .take(limit)
    {
        let mut ps = T::primes_less_than(&size);
        for _ in 0..skip {
            ps.next();
        }
        if ps.jump_after(jump) {
            let next = ps.next().unwrap();
            println!(
                "Created iterator less than {size}, skipped {skip}, jumped after {jump}, \
                then returned {next}"
            );
        } else {
            println!(
                "Created iterator less than {size}, skipped {skip}, and jumped after {jump}, \
                which was too far"
            );
        }
    }
}

fn demo_primes_jump_after<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    let mut config = config.clone();
    config.insert("mean_small_n", 10000);
    for (jump, skip) in unsigned_pair_gen_var_28::<T, usize>()
        .get(gm, &config)
        .take(limit)
    {
        let mut ps = T::primes();
        for _ in 0..skip {
            ps.next();
        }
        if ps.jump_after(jump) {
            let next = ps.next().unwrap();
            println!("Created iterator, skipped {skip}, jumped after {jump}, then returned {next}");
        } else {
            println!(
                "Created iterator, skipped {skip}, and jumped after {jump}, which was too far"
            );
        }
    }
}

fn benchmark_primes_less_than_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::primes_less_than(&n).count())),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p < n).count());
            }),
            ("naive", &mut |n| {
                no_out!(primes_naive::<T>().take_while(|&p| p < n).count());
            }),
        ],
    );
}

fn benchmark_primes_less_than_algorithms_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than(&{})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::primes_less_than(&n).count())),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p < n).count());
            }),
        ],
    );
}

fn benchmark_primes_less_than_or_equal_to_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than_or_equal_to(&{})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(T::primes_less_than_or_equal_to(&n).count());
            }),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p <= n).count());
            }),
            ("naive", &mut |n| {
                no_out!(primes_naive::<T>().take_while(|&p| p <= n).count());
            }),
        ],
    );
}

fn benchmark_primes_less_than_or_equal_to_algorithms_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<T>,
{
    run_benchmark(
        &format!("{}::primes_less_than_or_equal_to({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| {
                no_out!(T::primes_less_than_or_equal_to(&n).count());
            }),
            ("using primes", &mut |n| {
                no_out!(T::primes().take_while(|&p| p <= n).count());
            }),
        ],
    );
}

fn benchmark_prime_indicator_sequence_less_than(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "prime_indicator_sequence_less_than(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(prime_indicator_sequence_less_than(n).count());
        })],
    );
}

fn benchmark_prime_indicator_sequence_less_than_or_equal_to(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "prime_indicator_sequence_less_than_or_equal_to(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(prime_indicator_sequence_less_than_or_equal_to(n).count());
        })],
    );
}
