// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ToSci;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_bit_bucketer, rational_bit_bucketer,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_to_sci_options_pair_gen, rational_to_sci_options_pair_gen_var_1,
    rational_unsigned_pair_gen_var_5,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_length_after_point_in_small_base);
    register_demo!(runner, demo_rational_to_sci);
    register_demo!(runner, demo_rational_fmt_sci_valid);
    register_demo!(runner, demo_rational_to_sci_with_options);

    register_bench!(runner, benchmark_length_after_point_in_small_base);
    register_bench!(runner, benchmark_rational_to_sci);
    register_bench!(runner, benchmark_rational_fmt_sci_valid);
    register_bench!(runner, benchmark_rational_to_sci_with_options);
}

fn demo_length_after_point_in_small_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for (q, base) in rational_unsigned_pair_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "length_after_point_in_small_base({}, {}) = {:?}",
            q,
            base,
            q.length_after_point_in_small_base(base)
        );
    }
}

fn demo_rational_to_sci(gm: GenMode, config: &GenConfig, limit: usize) {
    for q in rational_gen().get(gm, config).take(limit) {
        println!("{}.to_sci() = {}", q, q.to_sci());
    }
}

fn demo_rational_fmt_sci_valid(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in rational_to_sci_options_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        if x.fmt_sci_valid(options) {
            println!("{x} can be converted to sci using {options:?}");
        } else {
            println!("{x} cannot be converted to sci using {options:?}");
        }
    }
}

fn demo_rational_to_sci_with_options(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, options) in rational_to_sci_options_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "to_sci_with_options({}, {:?}) = {}",
            x,
            options,
            x.to_sci_with_options(options)
        );
    }
}

fn benchmark_length_after_point_in_small_base(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.length_after_point_in_small_base(u8)",
        BenchmarkType::Single,
        rational_unsigned_pair_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |(q, base)| {
            no_out!(q.length_after_point_in_small_base(base))
        })],
    );
}

fn benchmark_rational_to_sci(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.to_sci()",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("q"),
        &mut [("Malachite", &mut |q| no_out!(q.to_sci().to_string()))],
    );
}

fn benchmark_rational_fmt_sci_valid(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.fmt_sci_valid(ToSciOptions)",
        BenchmarkType::Single,
        rational_to_sci_options_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.fmt_sci_valid(options))
        })],
    );
}

fn benchmark_rational_to_sci_with_options(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.to_sci_with_options(ToSciOptions)",
        BenchmarkType::Single,
        rational_to_sci_options_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_bit_bucketer("n"),
        &mut [("Malachite", &mut |(x, options)| {
            no_out!(x.to_sci_with_options(options).to_string())
        })],
    );
}
