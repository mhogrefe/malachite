// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_32, unsigned_gen_var_33, unsigned_gen_var_34,
};
use malachite_base::test_util::num::arithmetic::fibonacci::{
    checked_fibonacci_naive, checked_fibonacci_pair_naive, checked_lucas_number_naive,
    checked_lucas_number_pair_naive,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_fibonacci);
    register_unsigned_demos!(runner, demo_checked_fibonacci);
    register_unsigned_demos!(runner, demo_fibonacci_pair);
    register_unsigned_demos!(runner, demo_checked_fibonacci_pair);
    register_unsigned_demos!(runner, demo_lucas_number);
    register_unsigned_demos!(runner, demo_checked_lucas_number);
    register_unsigned_demos!(runner, demo_lucas_number_pair);
    register_unsigned_demos!(runner, demo_checked_lucas_number_pair);

    register_unsigned_benches!(runner, benchmark_fibonacci_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_fibonacci);
    register_unsigned_benches!(runner, benchmark_fibonacci_pair_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_fibonacci_pair);
    register_unsigned_benches!(runner, benchmark_lucas_number_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_lucas_number);
    register_unsigned_benches!(runner, benchmark_lucas_number_pair_algorithms);
    register_unsigned_benches!(runner, benchmark_checked_lucas_number_pair);
}

fn demo_fibonacci<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_32::<T>().get(gm, config).take(limit) {
        println!("F({}) = {}", n, T::fibonacci(n));
    }
}

fn demo_checked_fibonacci<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("F({}) = {:?}", n, T::checked_fibonacci(n));
    }
}

fn demo_fibonacci_pair<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_32::<T>().get(gm, config).take(limit) {
        println!("fibonacci_pair({}) = {:?}", n, T::fibonacci_pair(n));
    }
}

fn demo_checked_fibonacci_pair<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!(
            "checked_fibonacci_pair({}) = {:?}",
            n,
            T::checked_fibonacci_pair(n)
        );
    }
}

fn demo_lucas_number<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_33::<T>().get(gm, config).take(limit) {
        println!("L({}) = {}", n, T::lucas_number(n));
    }
}

fn demo_checked_lucas_number<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!("L({}) = {:?}", n, T::checked_lucas_number(n));
    }
}

fn demo_lucas_number_pair<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_34::<T>().get(gm, config).take(limit) {
        println!("lucas_number_pair({}) = {:?}", n, T::lucas_number_pair(n));
    }
}

fn demo_checked_lucas_number_pair<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for n in unsigned_gen().get(gm, config).take(limit) {
        println!(
            "checked_lucas_number_pair({}) = {:?}",
            n,
            T::checked_lucas_number_pair(n)
        );
    }
}

fn benchmark_fibonacci_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::fibonacci(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_32::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::fibonacci(n))),
            ("naive", &mut |n| {
                no_out!(checked_fibonacci_naive::<T>(n).unwrap());
            }),
        ],
    );
}

fn benchmark_checked_fibonacci<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_fibonacci(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_fibonacci(n)))],
    );
}

fn benchmark_fibonacci_pair_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::fibonacci_pair(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_32::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::fibonacci_pair(n))),
            ("naive", &mut |n| {
                no_out!(checked_fibonacci_pair_naive::<T>(n).unwrap());
            }),
        ],
    );
}

fn benchmark_checked_fibonacci_pair<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_fibonacci_pair(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_fibonacci_pair(n)))],
    );
}

fn benchmark_lucas_number_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::lucas_number(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_33::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::lucas_number(n))),
            ("naive", &mut |n| {
                no_out!(checked_lucas_number_naive::<T>(n).unwrap());
            }),
        ],
    );
}

fn benchmark_checked_lucas_number<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_lucas_number(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| no_out!(T::checked_lucas_number(n)))],
    );
}

fn benchmark_lucas_number_pair_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::lucas_number_pair(u64)", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_gen_var_34::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(T::lucas_number_pair(n))),
            ("naive", &mut |n| {
                no_out!(checked_lucas_number_pair_naive::<T>(n).unwrap());
            }),
        ],
    );
}

fn benchmark_checked_lucas_number_pair<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_lucas_number_pair(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |n| {
            no_out!(T::checked_lucas_number_pair(n));
        })],
    );
}
