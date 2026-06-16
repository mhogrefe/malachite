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
use malachite_float::test_util::bench::bucketers::{
    quadruple_1_3_float_primitive_int_max_complexity_bucketer, triple_1_float_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_unsigned_rounding_mode_triple_gen_var_27, float_unsigned_rounding_mode_triple_gen_var_28,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_log_base);
    register_demo!(runner, demo_float_log_base_debug);
    register_demo!(runner, demo_float_log_base_extreme);
    register_demo!(runner, demo_float_log_base_extreme_debug);
    register_demo!(runner, demo_float_log_base_ref);
    register_demo!(runner, demo_float_log_base_ref_debug);
    register_demo!(runner, demo_float_log_base_assign);
    register_demo!(runner, demo_float_log_base_assign_debug);
    register_demo!(runner, demo_float_log_base_prec);
    register_demo!(runner, demo_float_log_base_prec_debug);
    register_demo!(runner, demo_float_log_base_prec_ref);
    register_demo!(runner, demo_float_log_base_prec_ref_debug);
    register_demo!(runner, demo_float_log_base_prec_assign);
    register_demo!(runner, demo_float_log_base_prec_assign_debug);
    register_demo!(runner, demo_float_log_base_round);
    register_demo!(runner, demo_float_log_base_round_debug);
    register_demo!(runner, demo_float_log_base_round_extreme);
    register_demo!(runner, demo_float_log_base_round_extreme_debug);
    register_demo!(runner, demo_float_log_base_round_ref);
    register_demo!(runner, demo_float_log_base_round_ref_debug);
    register_demo!(runner, demo_float_log_base_round_assign);
    register_demo!(runner, demo_float_log_base_round_assign_debug);
    register_demo!(runner, demo_float_log_base_prec_round);
    register_demo!(runner, demo_float_log_base_prec_round_debug);
    register_demo!(runner, demo_float_log_base_prec_round_extreme);
    register_demo!(runner, demo_float_log_base_prec_round_extreme_debug);
    register_demo!(runner, demo_float_log_base_prec_round_ref);
    register_demo!(runner, demo_float_log_base_prec_round_ref_debug);
    register_demo!(runner, demo_float_log_base_prec_round_assign);
    register_demo!(runner, demo_float_log_base_prec_round_assign_debug);

    register_bench!(runner, benchmark_float_log_base_evaluation_strategy);
    register_bench!(runner, benchmark_float_log_base_assign);
    register_bench!(runner, benchmark_float_log_base_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_log_base_round_assign);
    register_bench!(
        runner,
        benchmark_float_log_base_prec_round_evaluation_strategy
    );
    register_bench!(runner, benchmark_float_log_base_prec_round_assign);
}

fn demo_float_log_base(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).log_base({}) = {}", x_old, base, x.log_base(base));
    }
}

fn demo_float_log_base_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({:#x}).log_base({}) = {:#x}",
            ComparableFloat(x_old),
            base,
            ComparableFloat(x.log_base(base))
        );
    }
}

fn demo_float_log_base_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).log_base({}) = {}", x_old, base, x.log_base(base));
    }
}

fn demo_float_log_base_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({:#x}).log_base({}) = {:#x}",
            ComparableFloat(x_old),
            base,
            ComparableFloat(x.log_base(base))
        );
    }
}

fn demo_float_log_base_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).log_base({}) = {}", x, base, (&x).log_base(base));
    }
}

fn demo_float_log_base_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{:#x}).log_base({}) = {:#x}",
            ComparableFloatRef(&x),
            base,
            ComparableFloat((&x).log_base(base))
        );
    }
}

fn demo_float_log_base_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_assign(base);
        println!("x := {x_old}; x.log_base_assign({base}); x = {x}");
    }
}

fn demo_float_log_base_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, _, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_assign(base);
        println!(
            "x := {:#x}; x.log_base_assign({}); x = {:#x}",
            ComparableFloat(x_old),
            base,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_prec({}, {}) = {:?}",
            x_old,
            base,
            prec,
            x.log_base_prec(base, prec)
        );
    }
}

fn demo_float_log_base_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_prec(base, prec);
        println!(
            "({:#x}).log_base_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            base,
            prec,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_prec_ref({}, {}) = {:?}",
            x,
            base,
            prec,
            x.log_base_prec_ref(base, prec)
        );
    }
}

fn demo_float_log_base_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.log_base_prec_ref(base, prec);
        println!(
            "(&{:#x}).log_base_prec_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            base,
            prec,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_prec_assign(base, prec);
        println!("x := {x_old}; x.log_base_prec_assign({base}, {prec}); x = {x}");
    }
}

fn demo_float_log_base_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_prec_assign(base, prec);
        println!(
            "x := {:#x}; x.log_base_prec_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            base,
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_round({}, {}) = {:?}",
            x_old,
            base,
            rm,
            x.log_base_round(base, rm)
        );
    }
}

fn demo_float_log_base_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_round(base, rm);
        println!(
            "({:#x}).log_base_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            base,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_28()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_round({}, {}) = {:?}",
            x_old,
            base,
            rm,
            x.log_base_round(base, rm)
        );
    }
}

fn demo_float_log_base_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_28()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_round(base, rm);
        println!(
            "({:#x}).log_base_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            base,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_round_ref({}, {}) = {:?}",
            x,
            base,
            rm,
            x.log_base_round_ref(base, rm)
        );
    }
}

fn demo_float_log_base_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.log_base_round_ref(base, rm);
        println!(
            "(&{:#x}).log_base_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            base,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.log_base_round_assign(base, rm);
        println!("x := {x_old}; x.log_base_round_assign({base}, {rm}); x = {x}");
    }
}

fn demo_float_log_base_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, rm) in float_unsigned_rounding_mode_triple_gen_var_27()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_round_assign(base, rm);
        println!(
            "x := {:#x}; x.log_base_round_assign({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            base,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_log_base_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_prec_round({}, {}, {}) = {:?}",
            x_old,
            base,
            prec,
            rm,
            x.log_base_prec_round(base, prec, rm)
        );
    }
}

fn demo_float_log_base_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_prec_round(base, prec, rm);
        println!(
            "({:#x}).log_base_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            base,
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).log_base_prec_round({}, {}, {}) = {:?}",
            x_old,
            base,
            prec,
            rm,
            x.log_base_prec_round(base, prec, rm)
        );
    }
}

fn demo_float_log_base_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (log, o) = x.log_base_prec_round(base, prec, rm);
        println!(
            "({:#x}).log_base_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            base,
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).log_base_prec_round_ref({}, {}, {}) = {:?}",
            x,
            base,
            prec,
            rm,
            x.log_base_prec_round_ref(base, prec, rm)
        );
    }
}

fn demo_float_log_base_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let (log, o) = x.log_base_prec_round_ref(base, prec, rm);
        println!(
            "(&{:#x}).log_base_prec_round_ref({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            base,
            prec,
            rm,
            ComparableFloat(log),
            o
        );
    }
}

fn demo_float_log_base_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_prec_round_assign(base, prec, rm);
        println!(
            "x := {x_old}; x.log_base_prec_round_assign({base}, {prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_float_log_base_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, base, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.log_base_prec_round_assign(base, prec, rm);
        println!(
            "x := {:#x}; x.log_base_prec_round_assign({}, {}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            base,
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_log_base_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.log_base(u64)", &mut |(x, base, _)| {
                no_out!(x.log_base(base));
            }),
            ("(&Float).log_base(u64)", &mut |(x, base, _)| {
                no_out!((&x).log_base(base));
            }),
        ],
    );
}

fn benchmark_float_log_base_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [("Float.log_base_assign(u64)", &mut |(mut x, base, _)| {
            x.log_base_assign(base);
        })],
    );
}

fn benchmark_float_log_base_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [
            (
                "Float.log_base_round(u64, RoundingMode)",
                &mut |(x, base, rm)| no_out!(x.log_base_round(base, rm)),
            ),
            (
                "(&Float).log_base_round_ref(u64, RoundingMode)",
                &mut |(x, base, rm)| no_out!(x.log_base_round_ref(base, rm)),
            ),
        ],
    );
}

fn benchmark_float_log_base_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_27().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_float_complexity_bucketer("x"),
        &mut [(
            "Float.log_base_round_assign(u64, RoundingMode)",
            &mut |(mut x, base, rm)| no_out!(x.log_base_round_assign(base, rm)),
        )],
    );
}

fn benchmark_float_log_base_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.log_base_prec_round(u64, u64, RoundingMode)",
                &mut |(x, base, prec, rm)| no_out!(x.log_base_prec_round(base, prec, rm)),
            ),
            (
                "(&Float).log_base_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, base, prec, rm)| no_out!(x.log_base_prec_round_ref(base, prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_log_base_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.log_base_prec_round_assign(u64, u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_3_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.log_base_prec_round_assign(u64, u64, RoundingMode)",
            &mut |(mut x, base, prec, rm)| no_out!(x.log_base_prec_round_assign(base, prec, rm)),
        )],
    );
}
