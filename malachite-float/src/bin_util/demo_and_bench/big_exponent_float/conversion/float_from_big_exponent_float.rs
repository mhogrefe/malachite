// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    big_exponent_float_complexity_bucketer,
    max_triple_1_big_exponent_float_complexity_triple_2_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_float_try_from);
    register_demo!(runner, demo_big_exponent_float_float_convertible_from);
    register_demo!(
        runner,
        demo_big_exponent_float_float_from_big_exponent_float_prec_round
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_float_try_from_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_float_from_big_exponent_float_prec_round_evaluation_strategy
    );
}

fn demo_big_exponent_float_float_try_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "Float::try_from({x:#x}) = {:?}",
            Float::try_from(&x).map(|f| format!("{:#x}", ComparableFloat(f)))
        );
    }
}

fn demo_big_exponent_float_float_convertible_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "Float::convertible_from({x:#x}) = {}",
            Float::convertible_from(&x)
        );
    }
}

fn demo_big_exponent_float_float_from_big_exponent_float_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .filter(|(x, _, rm)| *rm != Exact || Float::convertible_from(x))
        .take(limit)
    {
        let (f, o) = Float::from_big_exponent_float_prec_round_ref(&x, prec, rm);
        println!(
            "Float::from_big_exponent_float_prec_round({x:#x}, {prec}, {rm}) = ({:#x}, {o:?})",
            ComparableFloat(f)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_big_exponent_float_float_try_from_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::try_from(BigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("Float::try_from(BigExponentFloat)", &mut |x| {
                no_out!(Float::try_from(x));
            }),
            ("Float::try_from(&BigExponentFloat)", &mut |x| {
                no_out!(Float::try_from(&x));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_float_from_big_exponent_float_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::from_big_exponent_float_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_1()
            .get(gm, config)
            .filter(|(x, _, rm)| *rm != Exact || Float::convertible_from(x)),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("by value", &mut |(x, prec, rm)| {
                no_out!(Float::from_big_exponent_float_prec_round(x, prec, rm));
            }),
            ("by reference", &mut |(x, prec, rm)| {
                no_out!(Float::from_big_exponent_float_prec_round_ref(&x, prec, rm));
            }),
        ],
    );
}
