// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Exp, ExpAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::arithmetic::exp::{
    rug_exp, rug_exp_prec, rug_exp_prec_round, rug_exp_round,
};
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_1_float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_pair_1_float_complexity_bucketer,
    pair_2_pair_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer,
    pair_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_rounding_mode_pair_gen_var_47_rm, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_1_rm, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_36_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_exp);
    register_demo!(runner, demo_float_exp_debug);
    register_demo!(runner, demo_float_exp_extreme);
    register_demo!(runner, demo_float_exp_extreme_debug);
    register_demo!(runner, demo_float_exp_ref);
    register_demo!(runner, demo_float_exp_ref_debug);
    register_demo!(runner, demo_float_exp_assign);
    register_demo!(runner, demo_float_exp_assign_debug);
    register_demo!(runner, demo_float_exp_prec);
    register_demo!(runner, demo_float_exp_prec_debug);
    register_demo!(runner, demo_float_exp_prec_extreme);
    register_demo!(runner, demo_float_exp_prec_ref);
    register_demo!(runner, demo_float_exp_prec_assign);
    register_demo!(runner, demo_float_exp_round);
    register_demo!(runner, demo_float_exp_round_debug);
    register_demo!(runner, demo_float_exp_round_ref);
    register_demo!(runner, demo_float_exp_round_assign);
    register_demo!(runner, demo_float_exp_prec_round);
    register_demo!(runner, demo_float_exp_prec_round_debug);
    register_demo!(runner, demo_float_exp_prec_round_ref);
    register_demo!(runner, demo_float_exp_prec_round_assign);

    register_bench!(runner, benchmark_float_exp_evaluation_strategy);
    register_bench!(runner, benchmark_float_exp_library_comparison);
    register_bench!(runner, benchmark_float_exp_assign);
    register_bench!(runner, benchmark_float_exp_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_exp_prec_library_comparison);
    register_bench!(runner, benchmark_float_exp_prec_assign);
    register_bench!(runner, benchmark_float_exp_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_exp_round_library_comparison);
    register_bench!(runner, benchmark_float_exp_round_assign);
    register_bench!(runner, benchmark_float_exp_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_exp_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_exp_prec_round_assign);
}

fn demo_float_exp(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).exp() = {}", x_old, x.exp());
    }
}

fn demo_float_exp_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).exp() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.exp())
        );
    }
}

fn demo_float_exp_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).exp() = {}", x_old, x.exp());
    }
}

fn demo_float_exp_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).exp() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.exp())
        );
    }
}

fn demo_float_exp_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).exp() = {}", x, (&x).exp());
    }
}

fn demo_float_exp_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).exp() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).exp())
        );
    }
}

fn demo_float_exp_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.exp_assign();
        println!("x := {x_old}; x.exp_assign(); x = {x}");
    }
}

fn demo_float_exp_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.exp_assign();
        println!(
            "x := {:#x}; x.exp_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_exp_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).exp_prec({}) = {:?}", x_old, prec, x.exp_prec(prec));
    }
}

fn demo_float_exp_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (e, o) = x.exp_prec(prec);
        println!(
            "({:#x}).exp_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(e),
            o
        );
    }
}

fn demo_float_exp_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).exp_prec({}) = {:?}", x_old, prec, x.exp_prec(prec));
    }
}

fn demo_float_exp_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "(&{}).exp_prec_ref({}) = {:?}",
            x,
            prec,
            x.exp_prec_ref(prec)
        );
    }
}

fn demo_float_exp_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.exp_prec_assign(prec);
        println!("x := {x_old}; x.exp_prec_assign({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_exp_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).exp_round({}) = {:?}", x_old, rm, x.exp_round(rm));
    }
}

fn demo_float_exp_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (e, o) = x.exp_round(rm);
        println!(
            "({:#x}).exp_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(e),
            o
        );
    }
}

fn demo_float_exp_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).exp_round_ref({}) = {:?}", x, rm, x.exp_round_ref(rm));
    }
}

fn demo_float_exp_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_47()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.exp_round_assign(rm);
        println!("x := {x_old}; x.exp_round_assign({rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_exp_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).exp_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.exp_prec_round(prec, rm)
        );
    }
}

fn demo_float_exp_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (e, o) = x.exp_prec_round(prec, rm);
        println!(
            "({:#x}).exp_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(e),
            o
        );
    }
}

fn demo_float_exp_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).exp_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.exp_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_exp_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_36()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.exp_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.exp_prec_round_assign({prec}, {rm}) = {o:?}; x = {x}");
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_exp_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.exp()", &mut |x| no_out!(x.exp())),
            ("(&Float).exp()", &mut |x| no_out!((&x).exp())),
        ],
    );
}

fn benchmark_float_exp_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!((&x).exp())),
            ("rug", &mut |(x, _)| no_out!(rug_exp(&x))),
        ],
    );
}

fn benchmark_float_exp_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.exp_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.exp_assign()", &mut |mut x| x.exp_assign())],
    );
}

fn benchmark_float_exp_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.exp_prec(u64)", &mut |(x, prec)| {
                no_out!(x.exp_prec(prec));
            }),
            ("(&Float).exp_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.exp_prec_ref(prec));
            }),
        ],
    );
}

fn benchmark_float_exp_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec))| {
                no_out!(x.exp_prec_ref(prec));
            }),
            ("rug", &mut |((x, prec), _)| {
                no_out!(rug_exp_prec(&x, prec));
            }),
        ],
    );
}

fn benchmark_float_exp_prec_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.exp_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Float.exp_prec_assign(u64)", &mut |(mut x, prec)| {
            no_out!(x.exp_prec_assign(prec));
        })],
    );
}

fn benchmark_float_exp_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_47().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.exp_round(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.exp_round(rm));
            }),
            ("(&Float).exp_round_ref(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.exp_round_ref(rm));
            }),
        ],
    );
}

fn benchmark_float_exp_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_round(RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rounding_mode_pair_gen_var_47_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, rm))| {
                no_out!(x.exp_round_ref(rm));
            }),
            ("rug", &mut |((x, rm), _)| no_out!(rug_exp_round(&x, rm))),
        ],
    );
}

fn benchmark_float_exp_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_47().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Float.exp_round_assign(RoundingMode)", &mut |(
            mut x,
            rm,
        )| {
            no_out!(x.exp_round_assign(rm));
        })],
    );
}

fn benchmark_float_exp_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.exp_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.exp_prec_round(prec, rm)),
            ),
            (
                "(&Float).exp_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.exp_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_exp_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_36_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.exp_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_exp_prec_round(&x, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_exp_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.exp_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_36().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.exp_prec_round_assign(u64, RoundingMode)",
            &mut |(mut x, prec, rm)| no_out!(x.exp_prec_round_assign(prec, rm)),
        )],
    );
}
