// Copyright © 2025 Mikhail Hogrefe
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
use malachite_float::test_util::generators::{float_gen, float_rounding_mode_pair_gen_var_2};
use malachite_float::{ComparableFloat, ComparableFloatRef};
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_rounding_from_float);
    register_demo!(runner, demo_integer_rounding_from_float_debug);
    register_demo!(runner, demo_integer_rounding_from_float_ref);
    register_demo!(runner, demo_integer_rounding_from_float_ref_debug);
    register_demo!(runner, demo_integer_try_from_float);
    register_demo!(runner, demo_integer_try_from_float_debug);
    register_demo!(runner, demo_integer_try_from_float_ref);
    register_demo!(runner, demo_integer_try_from_float_ref_debug);
    register_demo!(runner, demo_integer_convertible_from_float);
    register_demo!(runner, demo_integer_convertible_from_float_debug);

    register_bench!(
        runner,
        benchmark_integer_rounding_from_float_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_try_from_float_evaluation_strategy);
    register_bench!(runner, benchmark_integer_convertible_from_float);
}

fn demo_integer_rounding_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::rounding_from({}, {}) = {:?}",
            f.clone(),
            rm,
            Integer::rounding_from(f, rm)
        );
    }
}

fn demo_integer_rounding_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::rounding_from({:#x}, {}) = {:?}",
            ComparableFloat(f.clone()),
            rm,
            Integer::rounding_from(f, rm)
        );
    }
}

fn demo_integer_rounding_from_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::rounding_from(&{}, {}) = {:?}",
            f,
            rm,
            Integer::rounding_from(&f, rm)
        );
    }
}

fn demo_integer_rounding_from_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (f, rm) in float_rounding_mode_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Integer::rounding_from(&{:#x}, {}) = {:?}",
            ComparableFloatRef(&f),
            rm,
            Integer::rounding_from(&f, rm)
        );
    }
}

fn demo_integer_try_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Integer::try_from({}) = {:?}",
            f.clone(),
            Integer::try_from(f)
        );
    }
}

fn demo_integer_try_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Integer::try_from({:#x}) = {:?}",
            ComparableFloat(f.clone()),
            Integer::try_from(f)
        );
    }
}

fn demo_integer_try_from_float_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!("Integer::try_from(&{}) = {:?}", f, Integer::try_from(&f));
    }
}

fn demo_integer_try_from_float_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "Integer::try_from(&{:#x}) = {:?}",
            ComparableFloatRef(&f),
            Integer::try_from(&f)
        );
    }
}

fn demo_integer_convertible_from_float(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{} is {}convertible to an Integer",
            f,
            if Integer::convertible_from(&f) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_integer_convertible_from_float_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for f in float_gen().get(gm, config).take(limit) {
        println!(
            "{:#x} is {}convertible to an Integer",
            ComparableFloatRef(&f),
            if Integer::convertible_from(&f) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_integer_rounding_from_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::rounding_from(Float)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Integer::rounding_from(Float)", &mut |(f, rm)| {
                no_out!(Integer::rounding_from(f, rm))
            }),
            ("Integer::rounding_from(&Float)", &mut |(f, rm)| {
                no_out!(Integer::rounding_from(&f, rm))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_try_from_float_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::try_from(Float)",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Integer::try_from(Float)", &mut |f| {
                no_out!(Integer::try_from(f))
            }),
            ("Integer::try_from(&Float)", &mut |f| {
                no_out!(Integer::try_from(&f))
            }),
        ],
    );
}

fn benchmark_integer_convertible_from_float(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer::convertible_from(&Float)",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |f| no_out!(Integer::convertible_from(&f)))],
    );
}
