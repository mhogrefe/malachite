// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_round_to_integer_prec_round);
    register_demo!(runner, demo_big_exponent_float_round_to_integer_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_round_to_integer_ties_away_prec
    );
    register_demo!(runner, demo_big_exponent_float_round_to_integer_round);
    register_demo!(runner, demo_big_exponent_float_round_to_integer);
    register_demo!(runner, demo_big_exponent_float_round_to_integer_ties_away);
    register_demo!(
        runner,
        demo_big_exponent_float_round_to_integer_then_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_round_to_integer_ties_away_then_prec_round
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_round_to_integer_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_round_to_integer_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_round_to_integer_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, i) = x.round_to_integer_prec_round_ref(prec, rm);
        println!("{x:#x}.round_to_integer_prec_round({prec}, {rm}) = ({z:#x}, {o:?}, {i})");
    }
}

fn demo_big_exponent_float_round_to_integer_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in big_exponent_float_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, i) = x.round_to_integer_prec_ref(prec);
        println!("{x:#x}.round_to_integer_prec({prec}) = ({z:#x}, {o:?}, {i})");
    }
}

fn demo_big_exponent_float_round_to_integer_ties_away_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec) in big_exponent_float_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, i) = x.round_to_integer_ties_away_prec_ref(prec);
        println!("{x:#x}.round_to_integer_ties_away_prec({prec}) = ({z:#x}, {o:?}, {i})");
    }
}

fn demo_big_exponent_float_round_to_integer_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, _, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, i) = x.round_to_integer_round_ref(rm);
        println!("{x:#x}.round_to_integer_round({rm}) = ({z:#x}, {o:?}, {i})");
    }
}

fn demo_big_exponent_float_round_to_integer(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        let (z, o, i) = x.round_to_integer_ref();
        println!("{x:#x}.round_to_integer() = ({z:#x}, {o:?}, {i})");
    }
}

fn demo_big_exponent_float_round_to_integer_ties_away(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        let (z, o, i) = x.round_to_integer_ties_away_ref();
        println!("{x:#x}.round_to_integer_ties_away() = ({z:#x}, {o:?}, {i})");
    }
}

fn demo_big_exponent_float_round_to_integer_then_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    let irms = [Floor, Ceiling, Down, Up, Nearest];
    for (i, (x, prec, rm)) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
        .enumerate()
    {
        let irm = irms[i % irms.len()];
        let (z, o) = x.round_to_integer_then_prec_round_ref(irm, prec, rm);
        println!("{x:#x}.round_to_integer_then_prec_round({irm}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_round_to_integer_ties_away_then_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_7()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.round_to_integer_ties_away_then_prec_round_ref(prec, rm);
        println!(
            "{x:#x}.round_to_integer_ties_away_then_prec_round({prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_round_to_integer_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.round_to_integer_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.round_to_integer_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.round_to_integer_prec_round(prec, rm));
                },
            ),
            (
                "BigExponentFloat.round_to_integer_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| {
                    no_out!(x.round_to_integer_prec_round_ref(prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_round_to_integer_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.round_to_integer_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_7_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, prec, rm))| {
                no_out!(x.round_to_integer_prec_round_ref(prec, rm));
            }),
            ("Float", &mut |((x, prec, rm), _)| {
                no_out!(x.round_to_integer_prec_round_ref(prec, rm));
            }),
        ],
    );
}
