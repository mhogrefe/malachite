// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Root, RootAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::Exact;
use malachite_base::test_util::bench::bucketers::{pair_2_bucketer, quadruple_3_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_signed_pair_gen, primitive_float_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::Float;
use malachite_float::arithmetic::root::{
    primitive_float_root_s, primitive_float_root_s_rational, primitive_float_root_u,
    primitive_float_root_u_rational,
};
use malachite_float::test_util::bench::bucketers::{
    pair_float_signed_max_complexity_bucketer, pair_float_unsigned_max_complexity_bucketer,
    quadruple_1_float_complexity_bucketer, triple_1_2_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    float_signed_pair_gen, float_signed_unsigned_rounding_mode_quadruple_gen_var_13,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_14,
    float_signed_unsigned_triple_gen_var_1, float_unsigned_pair_gen,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_14,
    float_unsigned_unsigned_triple_gen_var_1,
    rational_signed_unsigned_rounding_mode_quadruple_gen_var_2,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3,
};
use malachite_q::test_util::generators::{rational_signed_pair_gen, rational_unsigned_pair_gen};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_root_u_prec_round_extreme);
    register_demo!(runner, demo_float_root_u_prec_extreme);
    register_demo!(runner, demo_float_root_u_prec_round);
    register_demo!(runner, demo_float_root_u_prec_round_debug);
    register_demo!(runner, demo_float_root_u_prec);
    register_demo!(runner, demo_float_root_u_prec_debug);
    register_demo!(runner, demo_float_root_u_round);
    register_demo!(runner, demo_float_root_u_round_debug);
    register_demo!(runner, demo_float_root_u);
    register_demo!(runner, demo_float_root_u_debug);
    register_demo!(runner, demo_float_root_u_assign);
    register_bench!(runner, benchmark_float_root_u_prec_round);
    register_bench!(runner, benchmark_float_root_u_prec);
    register_bench!(runner, benchmark_float_root_u_evaluation_strategy);
    register_demo!(runner, demo_float_root_s_prec_round_extreme);
    register_demo!(runner, demo_float_root_s_prec_extreme);
    register_demo!(runner, demo_float_root_s_prec_round);
    register_demo!(runner, demo_float_root_s_prec_round_debug);
    register_demo!(runner, demo_float_root_s_prec);
    register_demo!(runner, demo_float_root_s_prec_debug);
    register_demo!(runner, demo_float_root_s_round);
    register_demo!(runner, demo_float_root_s_round_debug);
    register_demo!(runner, demo_float_root_s);
    register_demo!(runner, demo_float_root_s_debug);
    register_demo!(runner, demo_float_root_s_assign);
    register_bench!(runner, benchmark_float_root_s_prec_round);
    register_bench!(runner, benchmark_float_root_s_prec);
    register_bench!(runner, benchmark_float_root_s_evaluation_strategy);
    register_demo!(runner, demo_float_root_u_rational_prec_round);
    register_demo!(runner, demo_float_root_u_rational_prec_round_debug);
    register_demo!(runner, demo_float_root_u_rational_prec);
    register_demo!(runner, demo_float_root_u_rational_prec_debug);
    register_demo!(runner, demo_float_root_s_rational_prec_round);
    register_demo!(runner, demo_float_root_s_rational_prec_round_debug);
    register_demo!(runner, demo_float_root_s_rational_prec);
    register_demo!(runner, demo_float_root_s_rational_prec_debug);
    register_bench!(runner, benchmark_float_root_u_rational_prec_round);
    register_bench!(runner, benchmark_float_root_s_rational_prec_round);
    register_primitive_float_demos!(runner, demo_primitive_float_root_u);
    register_primitive_float_benches!(runner, benchmark_primitive_float_root_u);
    register_primitive_float_demos!(runner, demo_primitive_float_root_s);
    register_primitive_float_demos!(runner, demo_primitive_float_root_u_rational);
    register_primitive_float_demos!(runner, demo_primitive_float_root_s_rational);
}

fn demo_float_root_u_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_u_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.root_u_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_root_u_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (root, o) = x.root_u_prec_round(n, prec, rm);
        println!(
            "({:#x}).root_u_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            rm,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_u_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_u_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.root_u_prec(n, prec)
        );
    }
}

fn demo_float_root_u_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_unsigned_unsigned_triple_gen_var_1::<u64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (root, o) = x.root_u_prec(n, prec);
        println!(
            "({:#x}).root_u_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_u_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_u_round({}, {}) = {:?}",
            x_old,
            n,
            rm,
            x.root_u_round(n, rm)
        );
    }
}

fn demo_float_root_u_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        let (root, o) = x.root_u_round(n, rm);
        println!(
            "({:#x}).root_u_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            rm,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_u(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_unsigned_pair_gen::<u64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).root({}) = {}", x_old, n, x.root(n));
    }
}

fn demo_float_root_u_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_unsigned_pair_gen::<u64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).root({}) = {:#x}",
            ComparableFloat(x_old),
            n,
            ComparableFloat(x.root(n))
        );
    }
}

fn demo_float_root_u_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, n) in float_unsigned_pair_gen::<u64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.root_assign(n);
        println!("x := {x_old}; x.root_assign({n}); x = {x}");
    }
}

fn benchmark_float_root_u_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.root_u_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::Single,
        float_unsigned_unsigned_rounding_mode_quadruple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, n, prec, rm)| {
            no_out!(x.root_u_prec_round(n, prec, rm));
        })],
    );
}

fn benchmark_float_root_u_prec(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.root_u_prec(u64, u64)",
        BenchmarkType::Single,
        float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "n"),
        &mut [("Malachite", &mut |(x, n, prec)| {
            no_out!(x.root_u_prec(n, prec));
        })],
    );
}

fn benchmark_float_root_u_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.root(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_unsigned_max_complexity_bucketer("x", "n"),
        &mut [
            ("Float.root(u64)", &mut |(x, n)| no_out!(x.root(n))),
            ("(&Float).root(u64)", &mut |(x, n)| no_out!((&x).root(n))),
        ],
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_root_u<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, n) in primitive_float_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_root_u({}, {}) = {}",
            NiceFloat(x),
            n,
            NiceFloat(primitive_float_root_u::<T>(x, n))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn benchmark_primitive_float_root_u<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    run_benchmark(
        &format!("primitive_float_root_u({}, u64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("n"),
        &mut [("malachite", &mut |(x, n)| {
            no_out!(primitive_float_root_u::<T>(x, n));
        })],
    );
}

fn demo_float_root_s_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_s_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.root_s_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_root_s_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (root, o) = x.root_s_prec_round(n, prec, rm);
        println!(
            "({:#x}).root_s_prec_round({}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            rm,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_s_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_signed_unsigned_triple_gen_var_1::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_s_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.root_s_prec(n, prec)
        );
    }
}

fn demo_float_root_s_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in float_signed_unsigned_triple_gen_var_1::<i64, u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (root, o) = x.root_s_prec(n, prec);
        println!(
            "({:#x}).root_s_prec({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            prec,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_s_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_s_round({}, {}) = {:?}",
            x_old,
            n,
            rm,
            x.root_s_round(n, rm)
        );
    }
}

fn demo_float_root_s_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, _, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_13()
        .get(gm, config)
        .filter(|(_, _, _, rm)| *rm != Exact)
        .take(limit)
    {
        let x_old = x.clone();
        let (root, o) = x.root_s_round(n, rm);
        println!(
            "({:#x}).root_s_round({}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            n,
            rm,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_s(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).root({}) = {}", x_old, n, x.root(n));
    }
}

fn demo_float_root_s_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "({:#x}).root({}) = {:#x}",
            ComparableFloat(x_old),
            n,
            ComparableFloat(x.root(n))
        );
    }
}

fn demo_float_root_s_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, n) in float_signed_pair_gen::<i64>().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.root_assign(n);
        println!("x := {x_old}; x.root_assign({n}); x = {x}");
    }
}

fn benchmark_float_root_s_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.root_s_prec_round(i64, u64, RoundingMode)",
        BenchmarkType::Single,
        float_signed_unsigned_rounding_mode_quadruple_gen_var_13().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(x, n, prec, rm)| {
            no_out!(x.root_s_prec_round(n, prec, rm));
        })],
    );
}

fn benchmark_float_root_s_prec(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.root_s_prec(i64, u64)",
        BenchmarkType::Single,
        float_signed_unsigned_triple_gen_var_1::<i64, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_primitive_int_max_complexity_bucketer("x", "n"),
        &mut [("Malachite", &mut |(x, n, prec)| {
            no_out!(x.root_s_prec(n, prec));
        })],
    );
}

fn benchmark_float_root_s_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.root(i64)",
        BenchmarkType::EvaluationStrategy,
        float_signed_pair_gen::<i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_signed_max_complexity_bucketer("x", "n"),
        &mut [
            ("Float.root(i64)", &mut |(x, n)| no_out!(x.root(n))),
            ("(&Float).root(i64)", &mut |(x, n)| no_out!((&x).root(n))),
        ],
    );
}
fn demo_float_root_u_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_u_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.root_u_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_root_u_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, _) in float_unsigned_unsigned_rounding_mode_quadruple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_u_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.root_u_prec(n, prec)
        );
    }
}

fn demo_float_root_s_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in float_signed_unsigned_rounding_mode_quadruple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_s_prec_round({}, {}, {}) = {:?}",
            x_old,
            n,
            prec,
            rm,
            x.root_s_prec_round(n, prec, rm)
        );
    }
}

fn demo_float_root_s_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, _) in float_signed_unsigned_rounding_mode_quadruple_gen_var_14()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({}).root_s_prec({}, {}) = {:?}",
            x_old,
            n,
            prec,
            x.root_s_prec(n, prec)
        );
    }
}

fn demo_float_root_u_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::root_u_rational_prec_round({}, {}, {}, {:?}) = {:?}",
            x.clone(),
            k,
            prec,
            rm,
            Float::root_u_rational_prec_round(x, k, prec, rm)
        );
    }
}

fn demo_float_root_u_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, rm) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (root, o) = Float::root_u_rational_prec_round(x.clone(), k, prec, rm);
        println!(
            "Float::root_u_rational_prec_round({}, {}, {}, {:?}) = ({:#x}, {:?})",
            x,
            k,
            prec,
            rm,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_u_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::root_u_rational_prec({}, {}, {}) = {:?}",
            x.clone(),
            k,
            prec,
            Float::root_u_rational_prec(x, k, prec)
        );
    }
}

fn demo_float_root_u_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, _) in rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (root, o) = Float::root_u_rational_prec(x.clone(), k, prec);
        println!(
            "Float::root_u_rational_prec({}, {}, {}) = ({:#x}, {:?})",
            x,
            k,
            prec,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_s_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, rm) in rational_signed_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::root_s_rational_prec_round({}, {}, {}, {:?}) = {:?}",
            x.clone(),
            k,
            prec,
            rm,
            Float::root_s_rational_prec_round(x, k, prec, rm)
        );
    }
}

fn demo_float_root_s_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, rm) in rational_signed_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (root, o) = Float::root_s_rational_prec_round(x.clone(), k, prec, rm);
        println!(
            "Float::root_s_rational_prec_round({}, {}, {}, {:?}) = ({:#x}, {:?})",
            x,
            k,
            prec,
            rm,
            ComparableFloat(root),
            o
        );
    }
}

fn demo_float_root_s_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, _) in rational_signed_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Float::root_s_rational_prec({}, {}, {}) = {:?}",
            x.clone(),
            k,
            prec,
            Float::root_s_rational_prec(x, k, prec)
        );
    }
}

fn demo_float_root_s_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, k, prec, _) in rational_signed_unsigned_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (root, o) = Float::root_s_rational_prec(x.clone(), k, prec);
        println!(
            "Float::root_s_rational_prec({}, {}, {}) = ({:#x}, {:?})",
            x,
            k,
            prec,
            ComparableFloat(root),
            o
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_root_s<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, k) in primitive_float_signed_pair_gen::<T, i64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_root_s({}, {}) = {}",
            NiceFloat(x),
            k,
            NiceFloat(primitive_float_root_s::<T>(x, k))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_root_u_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, k) in rational_unsigned_pair_gen::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_root_u_rational({}, {}) = {}",
            x.clone(),
            k,
            NiceFloat(primitive_float_root_u_rational::<T>(&x, k))
        );
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn demo_primitive_float_root_s_rational<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    for (x, k) in rational_signed_pair_gen::<i64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "primitive_float_root_s_rational({}, {}) = {}",
            x.clone(),
            k,
            NiceFloat(primitive_float_root_s_rational::<T>(&x, k))
        );
    }
}

fn benchmark_float_root_u_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::root_u_rational_prec_round(Rational, u64, u64, RoundingMode)",
        BenchmarkType::Single,
        rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [("Malachite", &mut |(x, k, prec, rm)| {
            no_out!(Float::root_u_rational_prec_round(x, k, prec, rm));
        })],
    );
}

fn benchmark_float_root_s_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::root_s_rational_prec_round(Rational, i64, u64, RoundingMode)",
        BenchmarkType::Single,
        rational_signed_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_3_bucketer("prec"),
        &mut [("Malachite", &mut |(x, k, prec, rm)| {
            no_out!(Float::root_s_rational_prec_round(x, k, prec, rm));
        })],
    );
}
