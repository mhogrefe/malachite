// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::strings::{ToLowerHexString, ToUpperHexString};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::bench::bucketers::{
    big_exponent_float_complexity_bucketer, pair_2_big_exponent_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_to_lower_hex_string);
    register_demo!(runner, demo_big_exponent_float_to_upper_hex_string);
    register_demo!(runner, demo_big_exponent_float_debug);

    register_bench!(
        runner,
        benchmark_big_exponent_float_to_lower_hex_string_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_to_upper_hex_string);
    register_bench!(runner, benchmark_big_exponent_float_debug);
}

// The exact value of `x`, written independently of the hexadecimal formatter: `0`, `NaN`,
// `Infinity`, `-Infinity`, or `±m*2^k` with `m` odd.
fn exact(x: &BigExponentFloat) -> String {
    let Some(significand) = x.significand_ref() else {
        return format!("{x:#x}");
    };
    let zeros = significand.trailing_zeros().unwrap();
    let m = significand >> zeros;
    let k = x.get_exponent().unwrap() - Integer::from(m.significant_bits());
    let sign = if x.is_sign_negative() { "-" } else { "" };
    format!("{sign}{m}*2^{k}")
}

fn demo_big_exponent_float_to_lower_hex_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "to_lower_hex_string({}) = {}",
            exact(&x),
            x.to_lower_hex_string()
        );
    }
}

fn demo_big_exponent_float_to_upper_hex_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "to_upper_hex_string({}) = {}",
            exact(&x),
            x.to_upper_hex_string()
        );
    }
}

fn demo_big_exponent_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("format!(\"{{:?}}\", {}) = {x:?}", exact(&x));
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
