// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Ln, LnAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;
use malachite_float::Float;
use malachite_float::arithmetic::ln::primitive_float_ln;
use malachite_float::test_util::arithmetic::ln::{
    rug_ln, rug_ln_prec, rug_ln_prec_round, rug_ln_round,
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
    float_gen, float_gen_rm, float_gen_var_12, float_rounding_mode_pair_gen_var_34,
    float_rounding_mode_pair_gen_var_34_rm, float_rounding_mode_pair_gen_var_35,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_1_rm, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_19,
    float_unsigned_rounding_mode_triple_gen_var_19_rm,
    float_unsigned_rounding_mode_triple_gen_var_20,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_ln);
    register_demo!(runner, demo_float_ln_debug);
    register_demo!(runner, demo_float_ln_extreme);
    register_demo!(runner, demo_float_ln_extreme_debug);
    register_demo!(runner, demo_float_ln_ref);
    register_demo!(runner, demo_float_ln_ref_debug);
    register_demo!(runner, demo_float_ln_assign);
    register_demo!(runner, demo_float_ln_assign_debug);
    register_demo!(runner, demo_float_ln_prec);
    register_demo!(runner, demo_float_ln_prec_debug);
    register_demo!(runner, demo_float_ln_prec_extreme);
    register_demo!(runner, demo_float_ln_prec_extreme_debug);
    register_demo!(runner, demo_float_ln_prec_ref);
    register_demo!(runner, demo_float_ln_prec_ref_debug);
    register_demo!(runner, demo_float_ln_prec_assign);
    register_demo!(runner, demo_float_ln_prec_assign_debug);
    register_demo!(runner, demo_float_ln_round);
    register_demo!(runner, demo_float_ln_round_debug);
    register_demo!(runner, demo_float_ln_round_extreme);
    register_demo!(runner, demo_float_ln_round_extreme_debug);
    register_demo!(runner, demo_float_ln_round_ref);
    register_demo!(runner, demo_float_ln_round_ref_debug);
    register_demo!(runner, demo_float_ln_round_assign);
    register_demo!(runner, demo_float_ln_round_assign_debug);
    register_demo!(runner, demo_float_ln_prec_round);
    register_demo!(runner, demo_float_ln_prec_round_debug);
    register_demo!(runner, demo_float_ln_prec_round_extreme);
    register_demo!(runner, demo_float_ln_prec_round_extreme_debug);
    register_demo!(runner, demo_float_ln_prec_round_ref);
    register_demo!(runner, demo_float_ln_prec_round_ref_debug);
    register_demo!(runner, demo_float_ln_prec_round_assign);
    register_demo!(runner, demo_float_ln_prec_round_assign_debug);
    register_primitive_float_demos!(runner, demo_primitive_float_ln);

    register_bench!(runner, benchmark_float_ln_evaluation_strategy);
    register_bench!(runner, benchmark_float_ln_library_comparison);
    register_bench!(runner, benchmark_float_ln_assign);
    register_bench!(runner, benchmark_float_ln_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_ln_prec_library_comparison);
    register_bench!(runner, benchmark_float_ln_prec_assign);
    register_bench!(runner, benchmark_float_ln_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_ln_round_library_comparison);
    register_bench!(runner, benchmark_float_ln_round_assign);
    register_bench!(runner, benchmark_float_ln_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_ln_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_ln_prec_round_assign);
    register_primitive_float_benches!(runner, benchmark_primitive_float_ln);
}

fn demo_float_ln(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).ln() = {}", x_old, x.ln());
    }
}

fn demo_float_ln_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).ln() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.ln())
        );
    }
}

fn demo_float_ln_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).ln() = {}", x_old, x.ln());
    }
}

fn demo_float_ln_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).ln() = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x.ln())
        );
    }
}

fn demo_float_ln_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("(&{}).ln() = {}", x, (&x).ln());
    }
}

fn demo_float_ln_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "(&{:#x}).ln() = {:#x}",
            ComparableFloatRef(&x),
            ComparableFloat((&x).ln())
        );
    }
}

fn demo_float_ln_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ln_assign();
        println!("x := {x_old}; x.ln_assign(); x = {x}");
    }
}

fn demo_float_ln_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ln_assign();
        println!(
            "x := {:#x}; x.ln_assign(); x = {:#x}",
            ComparableFloat(x_old),
            ComparableFloat(x)
        );
    }
}

fn demo_float_ln_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).ln_prec({}) = {:?}", x_old, prec, x.ln_prec(prec));
    }
}

fn demo_float_ln_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (sum, o) = x.ln_prec(prec);
        println!(
            "({:#x}).ln_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).ln_prec({}) = {:?}", x_old, prec, x.ln_prec(prec));
    }
}

fn demo_float_ln_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let x_old = x.clone();
        let (sum, o) = x.ln_prec(prec);
        println!(
            "({:#x}).ln_prec({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{}).ln_prec_ref({}) = {:?}", x, prec, x.ln_prec_ref(prec));
    }
}

fn demo_float_ln_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (sum, o) = x.ln_prec_ref(prec);
        println!(
            "(&{:#x}).ln_prec_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.ln_prec_assign(prec);
        println!("x := {x_old}; x.ln_prec_assign({prec}); x = {x}");
    }
}

fn demo_float_ln_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let o = x.ln_prec_assign(prec);
        println!(
            "x := {:#x}; x.ln_prec_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_ln_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).ln_round({}) = {:?}", x_old, rm, x.ln_round(rm));
    }
}

fn demo_float_ln_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.ln_round(rm);
        println!(
            "({:#x}).ln_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_35()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({}).ln_round({}) = {:?}", x_old, rm, x.ln_round(rm));
    }
}

fn demo_float_ln_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_35()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.ln_round(rm);
        println!(
            "({:#x}).ln_round({}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).ln_round_ref({}) = {:?}", x, rm, x.ln_round_ref(rm));
    }
}

fn demo_float_ln_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, rm) in float_rounding_mode_pair_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.ln_round_ref(rm);
        println!(
            "(&{:#x}).ln_round_ref({}) = ({:#x}, {:?})",
            ComparableFloat(x),
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.ln_round_assign(rm);
        println!("x := {x_old}; x.ln_round_assign({rm}); x = {x}");
    }
}

fn demo_float_ln_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, rm) in float_rounding_mode_pair_gen_var_34()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_round_assign(rm);
        println!(
            "x := {:#x}; x.ln_round_assign({}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_ln_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).ln_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.ln_prec_round(prec, rm)
        );
    }
}

fn demo_float_ln_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.ln_prec_round(prec, rm);
        println!(
            "({:#x}).ln_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).ln_prec_round({}, {}) = {:?}",
            x_old,
            prec,
            rm,
            x.ln_prec_round(prec, rm)
        );
    }
}

fn demo_float_ln_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_20()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.ln_prec_round(prec, rm);
        println!(
            "({:#x}).ln_prec_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).ln_prec_round_ref({}, {}) = {:?}",
            x,
            prec,
            rm,
            x.ln_prec_round_ref(prec, rm)
        );
    }
}

fn demo_float_ln_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.ln_prec_round_ref(prec, rm);
        println!(
            "({:#x}).ln_prec_round_ref({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            prec,
            rm,
            ComparableFloat(sum),
            o
        );
    }
}

fn demo_float_ln_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_prec_round_assign(prec, rm);
        println!("x := {x_old}; x.ln_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_ln_prec_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_19()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.ln_prec_round_assign(prec, rm);
        println!(
            "x := {:#x}; x.ln_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(x_old),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_ln<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for x in primitive_float_gen::<T>().get(gm, config).take(limit) {
        println!(
            "primitive_float_ln({}) = {}",
            NiceFloat(x),
            NiceFloat(primitive_float_ln(x))
        );
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_float_ln_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.ln()", &mut |x| no_out!(x.ln())),
            ("(&Float).ln()", &mut |x| no_out!((&x).ln())),
        ],
    );
}

fn benchmark_float_ln_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!((&x).ln())),
            ("rug", &mut |(x, _)| no_out!(rug_ln(&x))),
        ],
    );
}

fn benchmark_float_ln_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.ln_assign()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Float.ln_assign()", &mut |mut x| x.ln_assign())],
    );
}

fn benchmark_float_ln_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Float.ln_prec(u64)", &mut |(x, prec)| {
                no_out!(x.ln_prec(prec));
            }),
            ("(&Float).ln_prec_ref(u64)", &mut |(x, prec)| {
                no_out!(x.ln_prec_ref(prec));
            }),
        ],
    );
}

fn benchmark_float_ln_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec))| {
                no_out!(x.ln_prec_ref(prec));
            }),
            ("rug", &mut |((x, prec), _)| {
                no_out!(rug_ln_prec(&x, prec));
            }),
        ],
    );
}

fn benchmark_float_ln_prec_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.ln_prec_assign(u64)",
        BenchmarkType::Single,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [("Float.ln_prec_assign(u64)", &mut |(mut x, prec)| {
            no_out!(x.ln_prec_assign(prec));
        })],
    );
}

fn benchmark_float_ln_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_round(RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_rounding_mode_pair_gen_var_34().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Float.ln_round(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.ln_round(rm));
            }),
            ("(&Float).ln_round_ref(RoundingMode)", &mut |(x, rm)| {
                no_out!(x.ln_round_ref(rm));
            }),
        ],
    );
}

fn benchmark_float_ln_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_round(RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_rounding_mode_pair_gen_var_34_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, rm))| {
                no_out!(x.ln_round_ref(rm));
            }),
            ("rug", &mut |((x, rm), _)| no_out!(rug_ln_round(&x, rm))),
        ],
    );
}

fn benchmark_float_ln_round_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.ln_round_assign(RoundingMode)",
        BenchmarkType::Single,
        float_rounding_mode_pair_gen_var_34().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_float_complexity_bucketer("x"),
        &mut [("Float.ln_round_assign(RoundingMode)", &mut |(mut x, rm)| {
            no_out!(x.ln_round_assign(rm));
        })],
    );
}

fn benchmark_float_ln_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            (
                "Float.ln_prec_round(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.ln_prec_round(prec, rm)),
            ),
            (
                "(&Float).ln_prec_round_ref(u64, RoundingMode)",
                &mut |(x, prec, rm)| no_out!(x.ln_prec_round_ref(prec, rm)),
            ),
        ],
    );
}

fn benchmark_float_ln_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_19_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, prec, rm))| {
                no_out!(x.ln_prec_round_ref(prec, rm));
            }),
            ("rug", &mut |((x, prec, rm), _)| {
                no_out!(rug_ln_prec_round(&x, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_ln_prec_round_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.ln_prec_round_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_rounding_mode_triple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "prec"),
        &mut [(
            "Float.ln_prec_round_assign(u64, RoundingMode)",
            &mut |(mut x, prec, rm)| no_out!(x.ln_prec_round_assign(prec, rm)),
        )],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_ln<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_ln({})", T::NAME),
        BenchmarkType::EvaluationStrategy,
        primitive_float_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("x"),
        &mut [("malachite", &mut |x| {
            no_out!(primitive_float_ln(x));
        })],
    );
}
