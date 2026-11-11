// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ToStringBase;
use malachite_base::strings::{ToBinaryString, ToLowerHexString, ToUpperHexString};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    big_exponent_float_complexity_bucketer, pair_2_big_exponent_float_complexity_bucketer,
};
use malachite_float::test_util::big_exponent_float::big_exponent_float_exact_string;
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_to_lower_hex_string);
    register_demo!(runner, demo_big_exponent_float_to_upper_hex_string);
    register_demo!(runner, demo_big_exponent_float_debug);
    register_demo!(runner, demo_big_exponent_float_to_binary_string);
    register_demo!(runner, demo_big_exponent_float_to_octal_string);
    register_demo!(runner, demo_big_exponent_float_to_string_base);
    register_demo!(runner, demo_big_exponent_float_to_string_base_upper);

    register_bench!(
        runner,
        benchmark_big_exponent_float_to_lower_hex_string_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_to_upper_hex_string);
    register_bench!(runner, benchmark_big_exponent_float_debug);
    register_bench!(runner, benchmark_big_exponent_float_to_binary_string);
    register_bench!(
        runner,
        benchmark_big_exponent_float_to_string_base_float_comparison
    );
}

fn demo_big_exponent_float_to_lower_hex_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "to_lower_hex_string({}) = {}",
            big_exponent_float_exact_string(&x),
            x.to_lower_hex_string()
        );
    }
}

fn demo_big_exponent_float_to_upper_hex_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "to_upper_hex_string({}) = {}",
            big_exponent_float_exact_string(&x),
            x.to_upper_hex_string()
        );
    }
}

fn demo_big_exponent_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "format!(\"{{:?}}\", {}) = {x:?}",
            big_exponent_float_exact_string(&x)
        );
    }
}

fn benchmark_big_exponent_float_to_lower_hex_string_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_lower_hex_string()",
        BenchmarkType::LibraryComparison,
        big_exponent_float_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat", &mut |(_, x)| {
                no_out!(x.to_lower_hex_string());
            }),
            ("Float", &mut |(f, _)| {
                no_out!(f.to_lower_hex_string());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_to_upper_hex_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_upper_hex_string()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.to_upper_hex_string()))],
    );
}

fn benchmark_big_exponent_float_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{:?}\", BigExponentFloat)",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(format!("{x:?}")))],
    );
}

fn demo_big_exponent_float_to_binary_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "format!(\"{{:#b}}\", {}) = {x:#b}",
            big_exponent_float_exact_string(&x)
        );
    }
}

fn demo_big_exponent_float_to_octal_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "format!(\"{{:#o}}\", {}) = {x:#o}",
            big_exponent_float_exact_string(&x)
        );
    }
}

fn demo_big_exponent_float_to_string_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        for base in [2, 4, 8, 16, 32] {
            println!(
                "({}).to_string_base({base}) = {}",
                big_exponent_float_exact_string(&x),
                x.to_string_base(base)
            );
        }
    }
}

fn demo_big_exponent_float_to_string_base_upper(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        for base in [2, 4, 8, 16, 32] {
            println!(
                "({}).to_string_base_upper({base}) = {}",
                big_exponent_float_exact_string(&x),
                x.to_string_base_upper(base)
            );
        }
    }
}

fn benchmark_big_exponent_float_to_binary_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_binary_string()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.to_binary_string());
        })],
    );
}

fn benchmark_big_exponent_float_to_string_base_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_string_base(32)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat", &mut |(_, x)| {
                no_out!(x.to_string_base(32));
            }),
            ("Float", &mut |(f, _)| {
                no_out!(f.to_string_base(32));
            }),
        ],
    );
}
