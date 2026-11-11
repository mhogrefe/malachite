// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{
    IntegerMantissaAndExponent, RawMantissaAndExponent, SciMantissaAndExponent,
};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::bench::bucketers::{
    big_exponent_float_complexity_bucketer, pair_1_big_exponent_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_rounding_mode_pair_gen_var_1,
};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_signed_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_raw_mantissa_and_exponent);
    register_demo!(
        runner,
        demo_big_exponent_float_integer_mantissa_and_exponent
    );
    register_demo!(
        runner,
        demo_big_exponent_float_from_integer_mantissa_and_exponent
    );
    register_demo!(runner, demo_big_exponent_float_sci_mantissa_and_exponent);
    register_demo!(
        runner,
        demo_big_exponent_float_sci_mantissa_and_exponent_round
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_raw_mantissa_and_exponent_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_integer_mantissa_and_exponent_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_from_integer_mantissa_and_exponent
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sci_mantissa_and_exponent_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sci_mantissa_and_exponent_round
    );
}

fn demo_big_exponent_float_raw_mantissa_and_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in big_exponent_float_gen()
        .get(gm, config)
        .filter(BigExponentFloat::is_normal)
        .take(limit)
    {
        println!(
            "({x:#x}).raw_mantissa_and_exponent() = {:?}",
            (&x).raw_mantissa_and_exponent()
        );
    }
}

fn demo_big_exponent_float_integer_mantissa_and_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in big_exponent_float_gen()
        .get(gm, config)
        .filter(BigExponentFloat::is_normal)
        .take(limit)
    {
        println!(
            "({x:#x}).integer_mantissa_and_exponent() = {:?}",
            (&x).integer_mantissa_and_exponent()
        );
    }
}

fn demo_big_exponent_float_from_integer_mantissa_and_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (m, e) in natural_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::from_integer_mantissa_and_exponent({m}, {e}) = {:?}",
            BigExponentFloat::from_integer_mantissa_and_exponent(m.clone(), Integer::from(e))
        );
    }
}

fn demo_big_exponent_float_sci_mantissa_and_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in big_exponent_float_gen()
        .get(gm, config)
        .filter(BigExponentFloat::is_normal)
        .take(limit)
    {
        println!(
            "({x:#x}).sci_mantissa_and_exponent() = {:?}",
            (&x).sci_mantissa_and_exponent()
        );
    }
}

fn demo_big_exponent_float_sci_mantissa_and_exponent_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, rm) in big_exponent_float_rounding_mode_pair_gen_var_1()
        .get(gm, config)
        .filter(|(x, _)| x.is_normal())
        .take(limit)
    {
        println!(
            "({x:#x}).sci_mantissa_and_exponent_round::<f64>({rm}) = {:?}",
            x.sci_mantissa_and_exponent_round::<f64>(rm)
                .map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    }
}

fn benchmark_big_exponent_float_raw_mantissa_and_exponent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.raw_mantissa_and_exponent()",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen()
            .get(gm, config)
            .filter(BigExponentFloat::is_normal),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("by value", &mut |x| {
                no_out!(x.raw_mantissa_and_exponent());
            }),
            ("by reference", &mut |x| {
                no_out!((&x).raw_mantissa_and_exponent());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_integer_mantissa_and_exponent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.integer_mantissa_and_exponent()",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen()
            .get(gm, config)
            .filter(BigExponentFloat::is_normal),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("by value", &mut |x| {
                no_out!(x.integer_mantissa_and_exponent());
            }),
            ("by reference", &mut |x| {
                no_out!((&x).integer_mantissa_and_exponent());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_from_integer_mantissa_and_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::from_integer_mantissa_and_exponent(Natural, Integer)",
        BenchmarkType::Single,
        natural_signed_pair_gen::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("m"),
        &mut [("Malachite", &mut |(m, e)| {
            no_out!(BigExponentFloat::from_integer_mantissa_and_exponent(
                m,
                Integer::from(e)
            ));
        })],
    );
}

fn benchmark_big_exponent_float_sci_mantissa_and_exponent_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sci_mantissa_and_exponent()",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen()
            .get(gm, config)
            .filter(BigExponentFloat::is_normal),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("by value", &mut |x| {
                no_out!(x.sci_mantissa_and_exponent());
            }),
            ("by reference", &mut |x| {
                no_out!((&x).sci_mantissa_and_exponent());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_sci_mantissa_and_exponent_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sci_mantissa_and_exponent_round::<f64>(RoundingMode)",
        BenchmarkType::Single,
        big_exponent_float_rounding_mode_pair_gen_var_1()
            .get(gm, config)
            .filter(|(x, _)| x.is_normal()),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, rm)| {
            no_out!(x.sci_mantissa_and_exponent_round::<f64>(rm));
        })],
    );
}
