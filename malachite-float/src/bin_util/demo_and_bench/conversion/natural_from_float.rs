// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{float_gen, float_rounding_mode_pair_gen_var_1};
use malachite_float::{ComparableFloat, ComparableFloatRef};
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_rounding_from_float);
    register_demo!(runner, demo_natural_rounding_from_float_debug);
    register_demo!(runner, demo_natural_rounding_from_float_ref);
    register_demo!(runner, demo_natural_rounding_from_float_ref_debug);
    register_demo!(runner, demo_natural_try_from_float);
    register_demo!(runner, demo_natural_try_from_float_debug);
    register_demo!(runner, demo_natural_try_from_float_ref);
    register_demo!(runner, demo_natural_try_from_float_ref_debug);
    register_demo!(runner, demo_natural_convertible_from_float);
    register_demo!(runner, demo_natural_convertible_from_float_debug);

    register_bench!(
        runner,
        benchmark_natural_rounding_from_float_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_try_from_float_evaluation_strategy);
    register_bench!(runner, benchmark_natural_convertible_from_float);
}

fn demo_natural_rounding_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::rounding_from({}, {}) = {:?}",
            f.clone(),
            rm,
            Natural::rounding_from(f, rm)
        );
    }
}

fn demo_natural_rounding_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::rounding_from({:#x}, {}) = {:?}",
            ComparableFloat(f.clone()),
            rm,
            Natural::rounding_from(f, rm)
        );
    }
}

fn demo_natural_rounding_from_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::rounding_from(&{}, {}) = {:?}",
            f,
            rm,
            Natural::rounding_from(&f, rm)
        );
    }
}

fn demo_natural_rounding_from_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::rounding_from(&{:#x}, {}) = {:?}",
            ComparableFloatRef(&f),
            rm,
            Natural::rounding_from(&f, rm)
        );
    }
}

fn demo_natural_try_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Natural::try_from({}) = {:?}",
            f.clone(),
            Natural::try_from(f)
        );
    }
}

fn demo_natural_try_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Natural::try_from({:#x}) = {:#x?}",
            ComparableFloat(f.clone()),
            Natural::try_from(f)
        );
    }
}

fn demo_natural_try_from_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!("Natural::try_from(&{}) = {:?}", f, Natural::try_from(&f));
    }
}

fn demo_natural_try_from_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Natural::try_from(&{:#x}) = {:#x?}",
            ComparableFloatRef(&f),
            Natural::try_from(&f)
        );
    }
}

fn demo_natural_convertible_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to a Natural",
            f,
            if Natural::convertible_from(&f) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_natural_convertible_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{:#x} is {}convertible to a Natural",
            ComparableFloatRef(&f),
            if Natural::convertible_from(&f) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_natural_rounding_from_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::rounding_from(Float)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Natural::rounding_from(Float)", &mut |(f, rm)| {
                no_out!(Natural::rounding_from(f, rm))
            }),
            ("Natural::rounding_from(&Float)", &mut |(f, rm)| {
                no_out!(Natural::rounding_from(&f, rm))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_try_from_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::try_from(Float)",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Natural::try_from(Float)", &mut |f| {
                no_out!(Natural::try_from(f))
            }),
            ("Natural::try_from(&Float)", &mut |f| {
                no_out!(Natural::try_from(&f))
            }),
        ],
    );
}

fn benchmark_natural_convertible_from_float(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::convertible_from(&Float)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |f| no_out!(Natural::convertible_from(&f)))],
    );
}
