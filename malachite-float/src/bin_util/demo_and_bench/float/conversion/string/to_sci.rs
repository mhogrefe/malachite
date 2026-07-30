// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ExactFrom, ToSci};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloatRef;
use malachite_float::float::conversion::string::to_sci::{to_sci_string, to_sci_valid};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_to_sci_options_pair_gen, float_to_sci_options_pair_gen_var_1,
};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_to_sci_string);
    register_demo!(runner, demo_to_sci_string_debug);
    register_demo!(runner, demo_to_sci_valid);
    register_demo!(runner, demo_float_to_sci);
    register_demo!(runner, demo_float_fmt_sci_valid);
    register_demo!(runner, demo_float_to_sci_with_options);
    register_demo!(runner, demo_float_to_sci_with_options_debug);
    register_bench!(runner, benchmark_to_sci_string);
    register_bench!(runner, benchmark_float_to_sci);
    register_bench!(runner, benchmark_float_fmt_sci_valid);
    register_bench!(runner, benchmark_float_to_sci_with_options_algorithms);
}

fn demo_to_sci_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_sci_string({x}, {options:?}) = {:?}",
            to_sci_string(&x, options)
        );
    }
}

fn demo_to_sci_string_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        println!(
            "to_sci_string({cx:#x}, {options:?}) = {:?}",
            to_sci_string(&x, options)
        );
    }
}

fn demo_to_sci_valid(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_sci_valid({x}, {options:?}) = {}",
            to_sci_valid(&x, options)
        );
    }
}

fn benchmark_to_sci_string(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "to_sci_string(&Float, ToSciOptions)",
        BenchmarkType::Single,
        float_to_sci_options_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(to_sci_string(&x, options));
        })],
    );
}

fn demo_float_to_sci(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("({x}).to_sci() = {}", x.to_sci());
    }
}

fn demo_float_fmt_sci_valid(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen().get(gm, config).take(limit) {
        if x.fmt_sci_valid(options) {
            println!("{x} can be converted to sci string using {options:?}");
        } else {
            println!("{x} cannot be converted to sci string using {options:?}");
        }
    }
}

fn demo_float_to_sci_with_options(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({x}).to_sci_with_options({options:?}) = {}",
            x.to_sci_with_options(options)
        );
    }
}

fn demo_float_to_sci_with_options_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in float_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let cx = ComparableFloatRef(&x);
        println!(
            "({cx:#x}).to_sci_with_options({options:?}) = {}",
            x.to_sci_with_options(options)
        );
    }
}

fn benchmark_float_to_sci(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.to_sci()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| {
            no_out!(x.to_sci().to_string());
        })],
    );
}

fn benchmark_float_fmt_sci_valid(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.fmt_sci_valid(ToSciOptions)",
        BenchmarkType::Single,
        float_to_sci_options_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.fmt_sci_valid(options));
        })],
    );
}

// Compares the direct `get_str`-based conversion with materializing the value as a `Rational` and
// using `Rational`'s `ToSci`. The generator is restricted to moderate exponents, since the
// `Rational` arm's size is proportional to the `Float`'s exponent magnitude.
fn benchmark_float_to_sci_with_options_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.to_sci_with_options(ToSciOptions)",
        BenchmarkType::Algorithms,
        float_to_sci_options_pair_gen_var_1()
            .get(gm, config)
            .filter(|(x, _)| {
                x.get_exponent()
                    .is_none_or(|exponent| exponent.unsigned_abs() <= 10_000)
            }),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("default", &mut |(x, options)| {
                no_out!(x.to_sci_with_options(options).to_string());
            }),
            ("via Rational", &mut |(x, options)| {
                no_out!(
                    Rational::exact_from(&x)
                        .to_sci_with_options(options)
                        .to_string()
                );
            }),
        ],
    );
}
