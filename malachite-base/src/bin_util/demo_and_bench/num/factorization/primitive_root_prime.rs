// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::PrimitiveRootPrime;
use malachite_base::test_util::bench::bucketers::primitive_int_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_29;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u8_primitive_root_prime);
    register_demo!(runner, demo_u16_primitive_root_prime);
    register_demo!(runner, demo_u32_primitive_root_prime);
    register_demo!(runner, demo_u64_primitive_root_prime);
    register_demo!(runner, demo_usize_primitive_root_prime);

    register_bench!(runner, benchmark_u8_primitive_root_prime);
    register_bench!(runner, benchmark_u16_primitive_root_prime);
    register_bench!(runner, benchmark_u32_primitive_root_prime);
    register_bench!(runner, benchmark_u64_primitive_root_prime);
    register_bench!(runner, benchmark_usize_primitive_root_prime);
}

fn demo_u8_primitive_root_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u8>().get(gm, config).take(limit) {
        println!("primitive_root_prime({}) = {}", u, u.primitive_root_prime())
    }
}

fn demo_u16_primitive_root_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u16>().get(gm, config).take(limit) {
        println!("primitive_root_prime({}) = {}", u, u.primitive_root_prime())
    }
}

fn demo_u32_primitive_root_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u32>().get(gm, config).take(limit) {
        println!("primitive_root_prime({}) = {}", u, u.primitive_root_prime())
    }
}

fn demo_u64_primitive_root_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<u64>().get(gm, config).take(limit) {
        println!("primitive_root_prime({}) = {}", u, u.primitive_root_prime())
    }
}

fn demo_usize_primitive_root_prime(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_29::<usize>().get(gm, config).take(limit) {
        println!("primitive_root_prime({}) = {}", u, u.primitive_root_prime())
    }
}

fn benchmark_u8_primitive_root_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u8.primitive_root_prime()",
        BenchmarkType::Single,
        unsigned_gen_var_29::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.primitive_root_prime()))],
    );
}

fn benchmark_u16_primitive_root_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u16.primitive_root_prime()",
        BenchmarkType::Single,
        unsigned_gen_var_29::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.primitive_root_prime()))],
    );
}

fn benchmark_u32_primitive_root_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u32.primitive_root_prime()",
        BenchmarkType::Single,
        unsigned_gen_var_29::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.primitive_root_prime()))],
    );
}

fn benchmark_u64_primitive_root_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "u64.primitive_root_prime()",
        BenchmarkType::Single,
        unsigned_gen_var_29::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.primitive_root_prime()))],
    );
}

fn benchmark_usize_primitive_root_prime(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "usize.primitive_root_prime()",
        BenchmarkType::Single,
        unsigned_gen_var_29::<usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(u.primitive_root_prime()))],
    );
}
