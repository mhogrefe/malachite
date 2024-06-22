// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::RawMantissaAndExponent;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::generators::float_gen_var_3;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_signed_pair_gen_var_4;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_raw_mantissa_and_exponent);
    register_demo!(runner, demo_float_raw_mantissa_and_exponent_debug);
    register_demo!(runner, demo_float_raw_mantissa_and_exponent_ref);
    register_demo!(runner, demo_float_raw_mantissa_and_exponent_ref_debug);
    register_demo!(runner, demo_float_raw_mantissa);
    register_demo!(runner, demo_float_raw_mantissa_debug);
    register_demo!(runner, demo_float_raw_mantissa_ref);
    register_demo!(runner, demo_float_raw_mantissa_ref_debug);
    register_demo!(runner, demo_float_raw_exponent);
    register_demo!(runner, demo_float_raw_exponent_debug);
    register_demo!(runner, demo_float_raw_exponent_ref);
    register_demo!(runner, demo_float_raw_exponent_ref_debug);
    register_demo!(runner, demo_float_from_raw_mantissa_and_exponent);
    register_demo!(runner, demo_float_from_raw_mantissa_and_exponent_debug);
    register_demo!(runner, demo_float_from_raw_mantissa_and_exponent_ref);
    register_demo!(runner, demo_float_from_raw_mantissa_and_exponent_ref_debug);

    register_bench!(
        runner,
        benchmark_float_raw_mantissa_and_exponent_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_raw_mantissa_evaluation_strategy);
    register_bench!(runner, benchmark_float_raw_exponent_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_float_from_raw_mantissa_and_exponent_evaluation_strategy
    );
}

fn demo_float_raw_mantissa_and_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{}.raw_mantissa_and_exponent() = {:?}",
            n.clone(),
            n.raw_mantissa_and_exponent()
        );
    }
}

fn demo_float_raw_mantissa_and_exponent_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.raw_mantissa_and_exponent() = {:?}",
            ComparableFloat(n.clone()),
            n.raw_mantissa_and_exponent()
        );
    }
}

fn demo_float_raw_mantissa_and_exponent_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{}).raw_mantissa_and_exponent() = {:?}",
            n,
            (&n).raw_mantissa_and_exponent()
        );
    }
}

fn demo_float_raw_mantissa_and_exponent_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).raw_mantissa_and_exponent() = {:?}",
            ComparableFloatRef(&n),
            (&n).raw_mantissa_and_exponent()
        );
    }
}

fn demo_float_raw_mantissa(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!("{}.raw_mantissa() = {}", n.clone(), n.raw_mantissa());
    }
}

fn demo_float_raw_mantissa_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.raw_mantissa() = {:#x}",
            ComparableFloat(n.clone()),
            n.raw_mantissa()
        );
    }
}

fn demo_float_raw_mantissa_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!("(&{}).raw_mantissa() = {}", n, (&n).raw_mantissa());
    }
}

fn demo_float_raw_mantissa_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).raw_mantissa() = {:#x}",
            ComparableFloatRef(&n),
            (&n).raw_mantissa()
        );
    }
}

fn demo_float_raw_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!("{}.raw_exponent() = {}", n.clone(), n.raw_exponent());
    }
}

fn demo_float_raw_exponent_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.raw_exponent() = {}",
            ComparableFloat(n.clone()),
            n.raw_exponent()
        );
    }
}

fn demo_float_raw_exponent_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!("{}.raw_exponent() = {}", n, (&n).raw_exponent());
    }
}

fn demo_float_raw_exponent_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in float_gen_var_3().get(gm, config).take(limit) {
        println!(
            "{:#x}.raw_exponent() = {}",
            ComparableFloatRef(&n),
            (&n).raw_exponent()
        );
    }
}

fn demo_float_from_raw_mantissa_and_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mantissa, exponent) in natural_signed_pair_gen_var_4::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <Float as RawMantissaAndExponent<Natural, i32, Float>>::from_raw_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!("Float::from_raw_mantissa_and_exponent({mantissa}, {exponent}) = {n}");
    }
}

fn demo_float_from_raw_mantissa_and_exponent_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mantissa, exponent) in natural_signed_pair_gen_var_4::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <Float as RawMantissaAndExponent<Natural, i32, Float>>::from_raw_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!(
            "Float::from_raw_mantissa_and_exponent({:#x}, {}) = {:#x}",
            mantissa,
            exponent,
            ComparableFloat(n)
        );
    }
}

fn demo_float_from_raw_mantissa_and_exponent_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mantissa, exponent) in natural_signed_pair_gen_var_4::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <&Float as RawMantissaAndExponent<Natural, i32, Float>>::from_raw_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!("Float::from_raw_mantissa_and_exponent({mantissa}, {exponent}) = {n}");
    }
}

fn demo_float_from_raw_mantissa_and_exponent_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in natural_signed_pair_gen_var_4::<i32>()
        .get(gm, config)
        .take(limit)
    {
        let n =
            <&Float as RawMantissaAndExponent<Natural, i32, Float>>::from_raw_mantissa_and_exponent(
                mantissa.clone(),
                exponent,
            );
        println!(
            "Float::from_raw_mantissa_and_exponent({:#x}, {}) = {:#x}",
            mantissa,
            exponent,
            ComparableFloat(n)
        );
    }
}

fn benchmark_float_raw_mantissa_and_exponent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.raw_mantissa_and_exponent()",
        BenchmarkType::EvaluationStrategy,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.raw_mantissa_and_exponent()", &mut |x| {
                no_out!(x.raw_mantissa_and_exponent())
            }),
            ("(&Float).raw_mantissa_and_exponent()", &mut |x| {
                no_out!((&x).raw_mantissa_and_exponent())
            }),
        ],
    );
}

fn benchmark_float_raw_mantissa_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.raw_mantissa()",
        BenchmarkType::EvaluationStrategy,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.raw_mantissa()", &mut |x| no_out!(x.raw_mantissa())),
            ("(&Float).raw_mantissa()", &mut |x| {
                no_out!((&x).raw_mantissa())
            }),
        ],
    );
}

fn benchmark_float_raw_exponent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.raw_exponent()",
        BenchmarkType::EvaluationStrategy,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.raw_exponent()", &mut |x| no_out!(x.raw_exponent())),
            ("(&Float).raw_exponent()", &mut |x| {
                no_out!((&x).raw_exponent())
            }),
        ],
    );
}

fn benchmark_float_from_raw_mantissa_and_exponent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_raw_mantissa_and_exponent(Float, i32)",
        BenchmarkType::EvaluationStrategy,
        natural_signed_pair_gen_var_4::<i32>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("x"),
        &mut [
            (
                "Float::from_raw_mantissa_and_exponent(Float, i32)",
                &mut |(mantissa, exponent)| {
                    no_out!(<Float as RawMantissaAndExponent::<
                        Natural,
                        i32,
                        Float,
                    >>::from_raw_mantissa_and_exponent(
                        mantissa, exponent
                    ))
                },
            ),
            (
                "(&Float)::from_raw_mantissa_and_exponent(Float, i32)",
                &mut |(mantissa, exponent)| {
                    no_out!(<&Float as RawMantissaAndExponent::<
                        Natural,
                        i32,
                        Float,
                    >>::from_raw_mantissa_and_exponent(
                        mantissa, exponent
                    ))
                },
            ),
        ],
    );
}
