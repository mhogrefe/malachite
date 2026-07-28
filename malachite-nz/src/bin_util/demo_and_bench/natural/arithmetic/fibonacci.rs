// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Fibonacci, LucasNumber};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_gen_var_11};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::arithmetic::fibonacci::{
    fibonacci_naive, lucas_number_naive,
};
use rug::Complete;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_fibonacci);
    register_demo!(runner, demo_fibonacci_pair);
    register_demo!(runner, demo_lucas_number);
    register_demo!(runner, demo_lucas_number_pair);

    register_bench!(runner, benchmark_fibonacci_algorithms);
    register_bench!(runner, benchmark_fibonacci_library_comparison);
    register_bench!(runner, benchmark_fibonacci_pair_algorithms);
    register_bench!(runner, benchmark_fibonacci_pair_library_comparison);
    register_bench!(runner, benchmark_lucas_number_algorithms);
    register_bench!(runner, benchmark_lucas_number_library_comparison);
    register_bench!(runner, benchmark_lucas_number_pair_algorithms);
    register_bench!(runner, benchmark_lucas_number_pair_library_comparison);
}

fn demo_fibonacci(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("F({}) = {}", n, Natural::fibonacci(n));
    }
}

fn demo_fibonacci_pair(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("fibonacci_pair({}) = {:?}", n, Natural::fibonacci_pair(n));
    }
}

fn demo_lucas_number(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("L({}) = {}", n, Natural::lucas_number(n));
    }
}

fn demo_lucas_number_pair(gm: GenMode, config: &GenConfig, limit: usize) {
    // The pair is not defined for n == 0, since L(-1) = -1.
    for n in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "lucas_number_pair({}) = {:?}",
            n,
            Natural::lucas_number_pair(n)
        );
    }
}

fn benchmark_fibonacci_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.fibonacci(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(Natural::fibonacci(n))),
            ("naive", &mut |n| no_out!(fibonacci_naive(n))),
        ],
    );
}

fn benchmark_fibonacci_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.fibonacci(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |n| no_out!(Natural::fibonacci(n))),
            ("rug", &mut |n| {
                no_out!(rug::Integer::fibonacci(u32::exact_from(n)).complete());
            }),
        ],
    );
}

fn benchmark_fibonacci_pair_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.fibonacci_pair(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(Natural::fibonacci_pair(n))),
            ("using fibonacci twice", &mut |n| {
                no_out!((Natural::fibonacci(n), Natural::fibonacci(n - 1)));
            }),
        ],
    );
}

fn benchmark_fibonacci_pair_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.fibonacci_pair(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |n| no_out!(Natural::fibonacci_pair(n))),
            ("rug", &mut |n| {
                no_out!(rug::Integer::fibonacci_2(u32::exact_from(n)).complete());
            }),
        ],
    );
}

fn benchmark_lucas_number_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lucas_number(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(Natural::lucas_number(n))),
            ("naive", &mut |n| no_out!(lucas_number_naive(n))),
        ],
    );
}

fn benchmark_lucas_number_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lucas_number(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |n| no_out!(Natural::lucas_number(n))),
            ("rug", &mut |n| {
                no_out!(rug::Integer::lucas(u32::exact_from(n)).complete());
            }),
        ],
    );
}

fn benchmark_lucas_number_pair_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lucas_number_pair(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |n| no_out!(Natural::lucas_number_pair(n))),
            ("using lucas_number twice", &mut |n| {
                no_out!((Natural::lucas_number(n), Natural::lucas_number(n - 1)));
            }),
        ],
    );
}

fn benchmark_lucas_number_pair_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    // The generator only produces positive n, since Natural::lucas_number_pair(0) panics; rug
    // returns L(-1) = -1 there, which is not a Natural.
    run_benchmark(
        "Natural.lucas_number_pair(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |n| no_out!(Natural::lucas_number_pair(n))),
            ("rug", &mut |n| {
                no_out!(rug::Integer::lucas_2(u32::exact_from(n)).complete());
            }),
        ],
    );
}
