// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    big_exponent_float_complexity_bucketer,
    max_pair_1_big_exponent_float_complexity_pair_2_bucketer,
    max_triple_1_big_exponent_float_complexity_triple_2_bucketer,
    pair_2_big_exponent_float_complexity_bucketer,
    pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer,
};
use malachite_float::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_to_significand);
    register_demo!(runner, demo_big_exponent_float_into_significand);
    register_demo!(runner, demo_big_exponent_float_significand_ref);
    register_demo!(runner, demo_big_exponent_float_get_exponent);
    register_demo!(runner, demo_big_exponent_float_get_prec);
    register_demo!(runner, demo_big_exponent_float_round_to_prec_round);
    register_demo!(runner, demo_big_exponent_float_round_to_prec_round_ref);
    register_demo!(runner, demo_big_exponent_float_round_to_prec_round_assign);
    register_demo!(runner, demo_big_exponent_float_round_to_prec);
    register_demo!(runner, demo_big_exponent_float_round_to_prec_ref);
    register_demo!(runner, demo_big_exponent_float_round_to_prec_assign);

    register_bench!(
        runner,
        benchmark_big_exponent_float_to_significand_evaluation_strategy
    );
    register_bench!(runner, benchmark_big_exponent_float_significand_ref);
    register_bench!(runner, benchmark_big_exponent_float_get_exponent);
    register_bench!(
        runner,
        benchmark_big_exponent_float_get_prec_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_round_to_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_round_to_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_round_to_prec_evaluation_strategy
    );
}

fn demo_big_exponent_float_to_significand(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.to_significand() = {:?}", x.to_significand());
    }
}

fn demo_big_exponent_float_into_significand(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!(
            "{x:#x}.into_significand() = {:?}",
            x.clone().into_significand()
        );
    }
}

fn demo_big_exponent_float_significand_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.significand_ref() = {:?}", x.significand_ref());
    }
}

fn demo_big_exponent_float_get_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.get_exponent() = {:?}", x.get_exponent());
    }
}

fn demo_big_exponent_float_get_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        println!("{x:#x}.get_prec() = {:?}", x.get_prec());
    }
}

fn demo_big_exponent_float_round_to_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({x:#x}).round_to_prec_round({prec}, {rm}) = {:?}",
            x.clone().round_to_prec_round(prec, rm)
        );
    }
}

fn demo_big_exponent_float_round_to_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x:#x}).round_to_prec_round_ref({prec}, {rm}) = {:?}",
            x.round_to_prec_round_ref(prec, rm)
        );
    }
}

fn demo_big_exponent_float_round_to_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, prec, rm) in big_exponent_float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.round_to_prec_round_assign(prec, rm);
        println!("x := {old_x:#x}; x.round_to_prec_round_assign({prec}, {rm}) = {o:?}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_round_to_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in big_exponent_float_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({x:#x}).round_to_prec({prec}) = {:?}",
            x.clone().round_to_prec(prec)
        );
    }
}

fn demo_big_exponent_float_round_to_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in big_exponent_float_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x:#x}).round_to_prec_ref({prec}) = {:?}",
            x.round_to_prec_ref(prec)
        );
    }
}

fn demo_big_exponent_float_round_to_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in big_exponent_float_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.round_to_prec_assign(prec);
        println!("x := {old_x:#x}; x.round_to_prec_assign({prec}) = {o:?}; x = {x:#x}");
    }
}

fn benchmark_big_exponent_float_to_significand_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.to_significand()",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("to_significand", &mut |x| {
                no_out!(x.to_significand());
            }),
            ("into_significand", &mut |x| {
                no_out!(x.into_significand());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_significand_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.significand_ref()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.significand_ref()))],
    );
}

fn benchmark_big_exponent_float_get_exponent(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.get_exponent()",
        BenchmarkType::Single,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.get_exponent()))],
    );
}

fn benchmark_big_exponent_float_get_prec_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.get_prec()",
        BenchmarkType::LibraryComparison,
        big_exponent_float_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("BigExponentFloat", &mut |(_, x)| {
                no_out!(x.get_prec());
            }),
            ("Float.get_min_prec", &mut |(f, _)| {
                no_out!(f.get_min_prec());
            }),
        ],
    );
}

fn benchmark_big_exponent_float_round_to_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.round_to_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("round_to_prec_round", &mut |(x, prec, rm)| {
                no_out!(x.round_to_prec_round(prec, rm));
            }),
            ("round_to_prec_round_ref", &mut |(x, prec, rm)| {
                no_out!(x.round_to_prec_round_ref(prec, rm));
            }),
            ("round_to_prec_round_assign", &mut |(mut x, prec, rm)| {
                no_out!(x.round_to_prec_round_assign(prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_round_to_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.round_to_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_rounding_mode_triple_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_big_exponent_float_complexity_triple_2_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (mut x, prec, rm))| {
                no_out!(x.round_to_prec_round_assign(prec, rm));
            }),
            ("Float.set_prec_round", &mut |((mut f, prec, rm), _)| {
                no_out!(f.set_prec_round(prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_round_to_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.round_to_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_pair_1_big_exponent_float_complexity_pair_2_bucketer("x", "prec"),
        &mut [
            ("round_to_prec", &mut |(x, prec)| {
                no_out!(x.round_to_prec(prec));
            }),
            ("round_to_prec_ref", &mut |(x, prec)| {
                no_out!(x.round_to_prec_ref(prec));
            }),
            ("round_to_prec_assign", &mut |(mut x, prec)| {
                no_out!(x.round_to_prec_assign(prec));
            }),
        ],
    );
}
