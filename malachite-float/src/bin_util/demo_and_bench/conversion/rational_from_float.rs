// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{float_gen, float_gen_rm};
use malachite_float::{ComparableFloat, ComparableFloatRef};
use malachite_q::Rational;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_try_from_float);
    register_demo!(runner, demo_rational_try_from_float_debug);
    register_demo!(runner, demo_rational_try_from_float_ref);
    register_demo!(runner, demo_rational_try_from_float_ref_debug);
    register_demo!(runner, demo_rational_convertible_from_float);
    register_demo!(runner, demo_rational_convertible_from_float_debug);

    register_bench!(
        runner,
        benchmark_rational_try_from_float_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_try_from_float_library_comparison);
    register_bench!(runner, benchmark_rational_convertible_from_float);
}

fn demo_rational_try_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Rational::try_from({}) = {:?}",
            f.clone(),
            Rational::try_from(f)
        );
    }
}

fn demo_rational_try_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Rational::try_from({:#x}) = {:#x?}",
            ComparableFloat(f.clone()),
            Rational::try_from(f)
        );
    }
}

fn demo_rational_try_from_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!("Rational::try_from(&{}) = {:?}", f, Rational::try_from(&f));
    }
}

fn demo_rational_try_from_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Rational::try_from(&{:#x}) = {:#x?}",
            ComparableFloatRef(&f),
            Rational::try_from(&f)
        );
    }
}

fn demo_rational_convertible_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Rational",
            f,
            if Rational::convertible_from(&f) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_rational_convertible_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{:#x} is {}convertible to a Rational",
            ComparableFloatRef(&f),
            if Rational::convertible_from(&f) {
                ""
            } else {
                "not "
            },
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_rational_try_from_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::try_from(Float)",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Rational::try_from(Float)", &mut |f| {
                no_out!(Rational::try_from(f))
            }),
            ("Rational::try_from(&Float)", &mut |f| {
                no_out!(Rational::try_from(&f))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_rational_try_from_float_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::try_from(Float)",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, f)| no_out!(Rational::try_from(f))),
            ("rug", &mut |(f, _)| no_out!(rug::Rational::try_from(&f))),
        ],
    );
}

fn benchmark_rational_convertible_from_float(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational::convertible_from(&Float)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |f| {
            no_out!(Rational::convertible_from(&f))
        })],
    );
}
