// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{LogBase, LogBaseAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::{
    quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_float_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_35, float_float_rounding_mode_triple_gen_var_36,
    float_float_unsigned_rounding_mode_quadruple_gen_var_11,
    float_float_unsigned_rounding_mode_quadruple_gen_var_12,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_log_base_float_base);
    register_demo!(runner, demo_float_log_base_float_base_debug);
    register_demo!(runner, demo_float_log_base_float_base_ref);
    register_demo!(runner, demo_float_log_base_float_base_assign);
    register_demo!(runner, demo_float_log_base_float_base_prec);
    register_demo!(runner, demo_float_log_base_float_base_prec_debug);
    register_demo!(runner, demo_float_log_base_float_base_prec_extreme);
    register_demo!(runner, demo_float_log_base_float_base_round);
    register_demo!(runner, demo_float_log_base_float_base_round_debug);
    register_demo!(runner, demo_float_log_base_float_base_round_extreme);
    register_demo!(runner, demo_float_log_base_float_base_prec_round);
    register_demo!(runner, demo_float_log_base_float_base_prec_round_debug);
    register_demo!(runner, demo_float_log_base_float_base_prec_round_extreme);
    register_demo!(runner, demo_float_log_base_float_base_prec_round_assign);

    register_bench!(
        runner,
        benchmark_float_log_base_float_base_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_log_base_float_base_assign);
    register_bench!(
        runner,
        benchmark_float_log_base_float_base_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_float_base_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_float_base_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_log_base_float_base_prec_round_assign
    );
}

fn demo_float_log_base_float_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base({}) = {}",
            x.clone(),
            base.clone(),
            x.log_base(base)
        );
    }
}

fn demo_float_log_base_float_base_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({:#x}).log_base({:#x}) = {:#x}",
            ComparableFloat(x.clone()),
            ComparableFloat(base.clone()),
            ComparableFloat(x.log_base(base))
        );
    }
}

fn demo_float_log_base_float_base_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).log_base(&{}) = {}", x, base, (&x).log_base(&base));
    }
}

fn demo_float_log_base_float_base_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, _, _) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_assign(&base);
        println!("x := {x_old}; x.log_base_assign(&{base}); x = {x}");
    }
}

fn demo_float_log_base_float_base_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_float_base_prec(&{}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            x.log_base_float_base_prec(&base, prec)
        );
    }
}

fn demo_float_log_base_float_base_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.clone().log_base_float_base_prec(&base, prec);
        println!(
            "({:#x}).log_base_float_base_prec(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(base),
            prec,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_float_base_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_float_unsigned_rounding_mode_quadruple_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_float_base_prec(&{}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            x.log_base_float_base_prec(&base, prec)
        );
    }
}

fn demo_float_log_base_float_base_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_float_rounding_mode_triple_gen_var_35()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_float_base_round(&{}, {}) = {:?}",
            x.clone(),
            base,
            rm,
            x.log_base_float_base_round(&base, rm)
        );
    }
}

fn demo_float_log_base_float_base_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_float_rounding_mode_triple_gen_var_35()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.clone().log_base_float_base_round(&base, rm);
        println!(
            "({:#x}).log_base_float_base_round(&{:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(base),
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_float_base_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_float_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_float_base_round(&{}, {}) = {:?}",
            x.clone(),
            base,
            rm,
            x.log_base_float_base_round(&base, rm)
        );
    }
}

fn demo_float_log_base_float_base_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_float_base_prec_round(&{}, {}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            rm,
            x.log_base_float_base_prec_round(&base, prec, rm)
        );
    }
}

fn demo_float_log_base_float_base_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.clone().log_base_float_base_prec_round(&base, prec, rm);
        println!(
            "({:#x}).log_base_float_base_prec_round(&{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(base),
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_float_base_prec_round_extreme(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, base, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_12()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).log_base_float_base_prec_round(&{}, {}, {}) = {:?}",
            x.clone(),
            base,
            prec,
            rm,
            x.log_base_float_base_prec_round(&base, prec, rm)
        );
    }
}

fn demo_float_log_base_float_base_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_float_base_prec_round_assign(&base, prec, rm);
        println!(
            "x := {x_old}; x.log_base_float_base_prec_round_assign(&{base}, {prec}, {rm}) = {o:?}; \
             x = {x}"
        );
    }
}

fn benchmark_float_log_base_float_base_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base(Float)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_35().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_float_max_complexity_bucketer("x", "base"),
        &mut [
            ("Float.log_base(Float)", &mut |(x, base, _)| {
                no_out!(x.log_base(base));
            }),
            ("(&Float).log_base(&Float)", &mut |(x, base, _)| {
                no_out!((&x).log_base(&base));
            }),
        ],
    );
}

fn benchmark_float_log_base_float_base_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_assign(&Float)",
        BenchmarkType::Single,
        float_float_rounding_mode_triple_gen_var_35().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_float_max_complexity_bucketer("x", "base"),
        &mut [("Float.log_base_assign(&Float)", &mut |(mut x, base, _)| {
            x.log_base_assign(&base);
        })],
    );
}

fn benchmark_float_log_base_float_base_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_float_base_prec(&Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "base", "prec"),
        &mut [
            (
                "Float.log_base_float_base_prec(&Float, u64)",
                &mut |(x, base, prec, _)| no_out!(x.log_base_float_base_prec(&base, prec)),
            ),
            (
                "(&Float).log_base_float_base_prec_ref(&Float, u64)",
                &mut |(x, base, prec, _)| no_out!(x.log_base_float_base_prec_ref(&base, prec)),
            ),
        ],
    );
}

fn benchmark_float_log_base_float_base_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_float_base_round(&Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_35().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_float_max_complexity_bucketer("x", "base"),
        &mut [
            (
                "Float.log_base_float_base_round(&Float, RoundingMode)",
                &mut |(x, base, rm)| no_out!(x.log_base_float_base_round(&base, rm)),
            ),
            (
                "(&Float).log_base_float_base_round_ref(&Float, RoundingMode)",
                &mut |(x, base, rm)| no_out!(x.log_base_float_base_round_ref(&base, rm)),
            ),
        ],
    );
}

fn benchmark_float_log_base_float_base_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_float_base_prec_round(&Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "base", "prec"),
        &mut [
            (
                "Float.log_base_float_base_prec_round(&Float, u64, RoundingMode)",
                &mut |(x, base, prec, rm)| {
                    no_out!(x.log_base_float_base_prec_round(&base, prec, rm));
                },
            ),
            (
                "(&Float).log_base_float_base_prec_round_ref(&Float, u64, RoundingMode)",
                &mut |(x, base, prec, rm)| {
                    no_out!(x.log_base_float_base_prec_round_ref(&base, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_log_base_float_base_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_float_base_prec_round_assign(&Float, u64, RoundingMode)",
        BenchmarkType::Single,
        float_float_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "base", "prec"),
        &mut [(
            "Float.log_base_float_base_prec_round_assign(&Float, u64, RoundingMode)",
            &mut |(mut x, base, prec, rm)| {
                no_out!(x.log_base_float_base_prec_round_assign(&base, prec, rm));
            },
        )],
    );
}
