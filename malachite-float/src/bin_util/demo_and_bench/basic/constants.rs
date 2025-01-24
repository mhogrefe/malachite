// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_11;
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::Float;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_min_positive_value_prec);
    register_demo!(runner, demo_float_min_positive_value_prec_debug);
    register_demo!(runner, demo_float_max_finite_value_with_prec);
    register_demo!(runner, demo_float_max_finite_value_with_prec_debug);
    register_demo!(runner, demo_float_one_prec);
    register_demo!(runner, demo_float_one_prec_debug);
    register_demo!(runner, demo_float_two_prec);
    register_demo!(runner, demo_float_two_prec_debug);
    register_demo!(runner, demo_float_negative_one_prec);
    register_demo!(runner, demo_float_negative_one_prec_debug);
    register_demo!(runner, demo_float_one_half_prec);
    register_demo!(runner, demo_float_one_half_prec_debug);

    register_bench!(
        runner,
        benchmark_float_min_positive_value_prec_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_max_finite_value_with_prec_library_comparison
    );
    register_bench!(runner, benchmark_float_one_prec_library_comparison);
    register_bench!(runner, benchmark_float_two_prec_library_comparison);
    register_bench!(runner, benchmark_float_negative_one_prec_library_comparison);
    register_bench!(runner, benchmark_float_one_half_prec_library_comparison);
}

fn demo_float_min_positive_value_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "min_positive_value_prec({}) = {}",
            p,
            Float::min_positive_value_prec(p)
        );
    }
}

fn demo_float_min_positive_value_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "min_positive_value_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::min_positive_value_prec(p))
        );
    }
}

fn demo_float_max_finite_value_with_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "max_finite_value_with_prec({}) = {}",
            p,
            Float::max_finite_value_with_prec(p)
        );
    }
}

fn demo_float_max_finite_value_with_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "max_finite_value_with_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::max_finite_value_with_prec(p))
        );
    }
}

fn demo_float_one_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("one_prec({}) = {}", p, Float::one_prec(p));
    }
}

fn demo_float_one_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "one_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::one_prec(p))
        );
    }
}

fn demo_float_two_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("two_prec({}) = {}", p, Float::two_prec(p));
    }
}

fn demo_float_two_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "two_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::two_prec(p))
        );
    }
}

fn demo_float_negative_one_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("negative_one_prec({}) = {}", p, Float::negative_one_prec(p));
    }
}

fn demo_float_negative_one_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "negative_one_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::negative_one_prec(p))
        );
    }
}

fn demo_float_one_half_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("one_half_prec({}) = {}", p, Float::one_half_prec(p));
    }
}

fn demo_float_one_half_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "one_half_prec({}) = {:#x}",
            p,
            ComparableFloat(Float::one_half_prec(p))
        );
    }
}

fn benchmark_float_min_positive_value_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min_positive_value_prec(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |p| {
            no_out!(Float::min_positive_value_prec(p))
        })],
    );
}

fn benchmark_float_max_finite_value_with_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max_finite_value_with_prec(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |p| {
            no_out!(Float::max_finite_value_with_prec(p))
        })],
    );
}

fn benchmark_float_one_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.one_prec(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::one_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), 1.0))
            }),
        ],
    );
}

fn benchmark_float_two_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.two_prec(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::two_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), 2.0))
            }),
        ],
    );
}

fn benchmark_float_negative_one_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.negative_one_prec(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::negative_one_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), -1.0))
            }),
        ],
    );
}

fn benchmark_float_one_half_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.one_half_prec(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::one_half_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug::Float::with_val(u32::exact_from(p), 0.5))
            }),
        ],
    );
}
