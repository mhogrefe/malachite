// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::factorization::traits::Factor;
use malachite_base::test_util::bench::bucketers::primitive_int_direct_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_1;
use malachite_base::test_util::num::factorization::factor::factor_naive;
use malachite_base::test_util::runner::Runner;
use std::fmt::Write;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_u8_factor);
    register_demo!(runner, demo_u16_factor);
    register_demo!(runner, demo_u32_factor);
    register_demo!(runner, demo_u64_factor);
    register_demo!(runner, demo_usize_factor);

    register_bench!(runner, benchmark_u8_factor_algorithms);
    register_bench!(runner, benchmark_u16_factor_algorithms);
    register_bench!(runner, benchmark_u32_factor);
    register_bench!(runner, benchmark_u32_factor_algorithms);
    register_bench!(runner, benchmark_u64_factor);
    register_bench!(runner, benchmark_u64_factor_algorithms);
    register_bench!(runner, benchmark_usize_factor_algorithms);
}

fn demo_factor_helper<T: Factor + PrimitiveUnsigned>(u: T)
where
    <T as Factor>::FACTORS: IntoIterator<Item = (T, u8)>,
{
    let mut s = String::new();
    for (p, e) in u.factor() {
        if !s.is_empty() {
            write!(s, "×").ok();
        }
        if e == 1 {
            write!(s, "{p}").ok();
        } else {
            write!(s, "{p}^{e}").ok();
        }
    }
    if s.is_empty() {
        s = "1".to_string();
    }
    println!("factor({u}) = {s}");
}

fn demo_u8_factor(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_1::<u8>().get(gm, config).take(limit) {
        demo_factor_helper(u);
    }
}

fn demo_u16_factor(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_1::<u16>().get(gm, config).take(limit) {
        demo_factor_helper(u);
    }
}

fn demo_u32_factor(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_1::<u32>().get(gm, config).take(limit) {
        demo_factor_helper(u);
    }
}

fn demo_u64_factor(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_1::<u64>().get(gm, config).take(limit) {
        demo_factor_helper(u);
    }
}

fn demo_usize_factor(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen_var_1::<usize>().get(gm, config).take(limit) {
        demo_factor_helper(u);
    }
}

fn benchmark_u8_factor_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u8.factor()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                let fs = u.factor();
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
            ("naive", &mut |u| {
                let fs = factor_naive(u);
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
        ],
    );
}

fn benchmark_u16_factor_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u16.factor()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<u16>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                let fs = u.factor();
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
            ("naive", &mut |u| {
                let fs = factor_naive(u);
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
        ],
    );
}

fn benchmark_u32_factor(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u32.factor()",
        BenchmarkType::Single,
        unsigned_gen_var_1::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| {
            let fs = u.factor();
            unsafe {
                std::ptr::read_volatile(&fs);
            }
        })],
    );
}

fn benchmark_u32_factor_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u32.factor()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<u32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                let fs = u.factor();
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
            ("naive", &mut |u| {
                let fs = factor_naive(u);
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
        ],
    );
}

fn benchmark_u64_factor(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u64.factor()",
        BenchmarkType::Single,
        unsigned_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [("Malachite", &mut |u| {
            let fs = u.factor();
            unsafe {
                std::ptr::read_volatile(&fs);
            }
        })],
    );
}

fn benchmark_u64_factor_algorithms(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "u64.factor()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                let fs = u.factor();
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
            ("naive", &mut |u| {
                let fs = factor_naive(u);
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
        ],
    );
}

fn benchmark_usize_factor_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "usize.factor()",
        BenchmarkType::Algorithms,
        unsigned_gen_var_1::<usize>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_int_direct_bucketer(),
        &mut [
            ("default", &mut |u| {
                let fs = u.factor();
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
            ("naive", &mut |u| {
                let fs = factor_naive(u);
                unsafe {
                    std::ptr::read_volatile(&fs);
                }
            }),
        ],
    );
}
