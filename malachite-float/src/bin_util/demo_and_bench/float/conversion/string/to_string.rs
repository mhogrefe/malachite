// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::{
    ToBinaryString, ToDebugString, ToLowerHexString, ToOctalString, ToUpperHexString,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::test_util::bench::bucketers::pair_2_float_complexity_bucketer;
use malachite_float::test_util::generators::{float_gen, float_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_to_string);
    register_demo!(runner, demo_float_to_debug_string);
    register_demo!(runner, demo_float_to_binary_string);
    register_demo!(runner, demo_float_to_binary_string_with_0b);
    register_demo!(runner, demo_float_to_octal_string);
    register_demo!(runner, demo_float_to_octal_string_with_0o);
    register_demo!(runner, demo_float_to_lower_hex_string);
    register_demo!(runner, demo_float_to_lower_hex_string_with_0x);
    register_demo!(runner, demo_float_to_upper_hex_string);
    register_demo!(runner, demo_float_to_upper_hex_string_with_0x);

    register_bench!(runner, benchmark_float_to_string_library_comparison);
    register_bench!(runner, benchmark_float_to_binary_string_library_comparison);
    register_bench!(runner, benchmark_float_to_octal_string_library_comparison);
    register_bench!(
        runner,
        benchmark_float_to_lower_hex_string_library_comparison
    );
    register_bench!(
        runner,
        benchmark_float_to_upper_hex_string_library_comparison
    );
}

fn demo_float_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_string({:#x}) = {}", ComparableFloatRef(&x), x);
    }
}

fn demo_float_to_debug_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "to_debug_string({:#x}) = {}",
            ComparableFloatRef(&x),
            x.to_debug_string()
        );
    }
}

fn demo_float_to_binary_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_binary_string({x}) = {x:b}");
    }
}

fn demo_float_to_binary_string_with_0b(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("{x} in binary is {x:#b}");
    }
}

fn demo_float_to_octal_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_octal_string({x}) = {x:o}");
    }
}

fn demo_float_to_octal_string_with_0o(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("{x} in octal is {x:#o}");
    }
}

fn demo_float_to_lower_hex_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_lower_hex_string({x}) = {x:x}");
    }
}

fn demo_float_to_lower_hex_string_with_0x(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("{x} in hex is {x:#x}");
    }
}

fn demo_float_to_upper_hex_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_upper_hex_string({x}) = {x:X}");
    }
}

fn demo_float_to_upper_hex_string_with_0x(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("{x} in hex is {x:#X}");
    }
}

fn benchmark_float_to_string_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.to_string()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.to_string())),
            ("rug", &mut |(x, _)| no_out!(x.to_string())),
        ],
    );
}

fn benchmark_float_to_binary_string_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{:b}\", Float)",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.to_binary_string())),
            ("rug", &mut |(x, _)| no_out!(x.to_binary_string())),
        ],
    );
}

fn benchmark_float_to_octal_string_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{:o}\", Float)",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.to_octal_string())),
            ("rug", &mut |(x, _)| no_out!(x.to_octal_string())),
        ],
    );
}

fn benchmark_float_to_lower_hex_string_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{:x}\", Float)",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.to_lower_hex_string())),
            ("rug", &mut |(x, _)| no_out!(x.to_lower_hex_string())),
        ],
    );
}

fn benchmark_float_to_upper_hex_string_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{:X}\", Float)",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.to_upper_hex_string())),
            ("rug", &mut |(x, _)| no_out!(x.to_upper_hex_string())),
        ],
    );
}
