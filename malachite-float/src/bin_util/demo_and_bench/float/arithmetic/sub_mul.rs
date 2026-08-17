// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::{
    pair_2_quadruple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer,
    pair_2_quadruple_1_2_3_float_max_complexity_bucketer,
    pair_2_quintuple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_3_float_max_complexity_bucketer,
    quadruple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer,
    quadruple_1_2_3_float_max_complexity_bucketer,
    quintuple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer,
    quintuple_1_2_3_float_float_rational_max_complexity_bucketer,
    triple_1_2_3_float_max_complexity_bucketer,
};
use malachite_float::test_util::float::arithmetic::sub_mul::{
    rug_sub_mul, rug_sub_mul_prec, rug_sub_mul_prec_round, rug_sub_mul_round,
    sub_mul_prec_round_naive, sub_mul_rational_prec_round_naive,
};
use malachite_float::test_util::generators::{
    float_float_float_rounding_mode_quadruple_gen_var_2,
    float_float_float_rounding_mode_quadruple_gen_var_2_rm,
    float_float_float_unsigned_quadruple_gen_var_1,
    float_float_float_unsigned_quadruple_gen_var_1_rm,
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_3,
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_3_rm,
    float_float_float_unsigned_rounding_mode_quintuple_gen_var_4,
    float_float_rational_rounding_mode_quadruple_gen_var_2, float_float_rational_triple_gen,
    float_float_rational_unsigned_quadruple_gen_var_1,
    float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3, float_triple_gen,
    float_triple_gen_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_sub_mul_prec_round);
    register_demo!(runner, demo_float_sub_mul_prec_round_debug);
    register_demo!(runner, demo_float_sub_mul_prec_round_extreme);
    register_demo!(runner, demo_float_sub_mul_prec_round_extreme_debug);
    register_demo!(runner, demo_float_sub_mul_prec_round_ref_ref_ref);
    register_demo!(runner, demo_float_sub_mul_prec_round_ref_ref_ref_debug);

    register_demo!(runner, demo_float_sub_mul);
    register_demo!(runner, demo_float_sub_mul_debug);
    register_demo!(runner, demo_float_sub_mul_val_val_ref);
    register_demo!(runner, demo_float_sub_mul_val_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_val_ref_val);
    register_demo!(runner, demo_float_sub_mul_val_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_val_ref_ref);
    register_demo!(runner, demo_float_sub_mul_val_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_ref_val_val);
    register_demo!(runner, demo_float_sub_mul_ref_val_val_debug);
    register_demo!(runner, demo_float_sub_mul_ref_val_ref);
    register_demo!(runner, demo_float_sub_mul_ref_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_ref_ref_val);
    register_demo!(runner, demo_float_sub_mul_ref_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_ref_ref_ref);
    register_demo!(runner, demo_float_sub_mul_ref_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_assign);
    register_demo!(runner, demo_float_sub_mul_assign_debug);
    register_demo!(runner, demo_float_sub_mul_assign_val_ref);
    register_demo!(runner, demo_float_sub_mul_assign_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_assign_ref_val);
    register_demo!(runner, demo_float_sub_mul_assign_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_assign_ref_ref);
    register_demo!(runner, demo_float_sub_mul_assign_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_prec);
    register_demo!(runner, demo_float_sub_mul_prec_debug);
    register_demo!(runner, demo_float_sub_mul_prec_val_val_ref);
    register_demo!(runner, demo_float_sub_mul_prec_val_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_prec_val_ref_val);
    register_demo!(runner, demo_float_sub_mul_prec_val_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_prec_val_ref_ref);
    register_demo!(runner, demo_float_sub_mul_prec_val_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_prec_ref_val_val);
    register_demo!(runner, demo_float_sub_mul_prec_ref_val_val_debug);
    register_demo!(runner, demo_float_sub_mul_prec_ref_val_ref);
    register_demo!(runner, demo_float_sub_mul_prec_ref_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_prec_ref_ref_val);
    register_demo!(runner, demo_float_sub_mul_prec_ref_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_prec_ref_ref_ref);
    register_demo!(runner, demo_float_sub_mul_prec_ref_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_prec_assign);
    register_demo!(runner, demo_float_sub_mul_prec_assign_debug);
    register_demo!(runner, demo_float_sub_mul_prec_assign_val_ref);
    register_demo!(runner, demo_float_sub_mul_prec_assign_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_prec_assign_ref_val);
    register_demo!(runner, demo_float_sub_mul_prec_assign_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_prec_assign_ref_ref);
    register_demo!(runner, demo_float_sub_mul_prec_assign_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_round);
    register_demo!(runner, demo_float_sub_mul_round_debug);
    register_demo!(runner, demo_float_sub_mul_round_val_val_ref);
    register_demo!(runner, demo_float_sub_mul_round_val_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_round_val_ref_val);
    register_demo!(runner, demo_float_sub_mul_round_val_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_round_val_ref_ref);
    register_demo!(runner, demo_float_sub_mul_round_val_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_round_ref_val_val);
    register_demo!(runner, demo_float_sub_mul_round_ref_val_val_debug);
    register_demo!(runner, demo_float_sub_mul_round_ref_val_ref);
    register_demo!(runner, demo_float_sub_mul_round_ref_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_round_ref_ref_val);
    register_demo!(runner, demo_float_sub_mul_round_ref_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_round_ref_ref_ref);
    register_demo!(runner, demo_float_sub_mul_round_ref_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_round_assign);
    register_demo!(runner, demo_float_sub_mul_round_assign_debug);
    register_demo!(runner, demo_float_sub_mul_round_assign_val_ref);
    register_demo!(runner, demo_float_sub_mul_round_assign_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_round_assign_ref_val);
    register_demo!(runner, demo_float_sub_mul_round_assign_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_round_assign_ref_ref);
    register_demo!(runner, demo_float_sub_mul_round_assign_ref_ref_debug);

    register_bench!(
        runner,
        benchmark_float_sub_mul_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sub_mul_prec_round_library_comparison
    );
    register_bench!(runner, benchmark_float_sub_mul_prec_round_algorithms);
    register_bench!(runner, benchmark_float_sub_mul_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_mul_library_comparison);
    register_bench!(runner, benchmark_float_sub_mul_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_mul_prec_library_comparison);
    register_bench!(runner, benchmark_float_sub_mul_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_sub_mul_round_library_comparison);
    register_demo!(runner, demo_float_sub_mul_rational_prec_round);
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_val_val_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_val_val_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_val_ref_val);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_val_ref_val_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_val_ref_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_val_ref_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_ref_val_val);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_ref_val_val_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_ref_val_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_ref_val_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_ref_ref_val);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_ref_ref_val_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_ref_ref_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_ref_ref_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_assign);
    register_demo!(runner, demo_float_sub_mul_rational_prec_round_assign_debug);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_assign_val_ref
    );
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_assign_val_ref_debug
    );
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_assign_ref_val
    );
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_assign_ref_val_debug
    );
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_assign_ref_ref
    );
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_round_assign_ref_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec);
    register_demo!(runner, demo_float_sub_mul_rational_prec_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_val_val_ref);
    register_demo!(runner, demo_float_sub_mul_rational_prec_val_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_val_ref_val);
    register_demo!(runner, demo_float_sub_mul_rational_prec_val_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_val_ref_ref);
    register_demo!(runner, demo_float_sub_mul_rational_prec_val_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_val_val);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_val_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_val_ref);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_ref_val);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_ref_ref);
    register_demo!(runner, demo_float_sub_mul_rational_prec_ref_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_assign);
    register_demo!(runner, demo_float_sub_mul_rational_prec_assign_debug);
    register_demo!(runner, demo_float_sub_mul_rational_prec_assign_val_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_assign_val_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_assign_ref_val);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_assign_ref_val_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_prec_assign_ref_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_prec_assign_ref_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_round);
    register_demo!(runner, demo_float_sub_mul_rational_round_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_val_val_ref);
    register_demo!(runner, demo_float_sub_mul_rational_round_val_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_val_ref_val);
    register_demo!(runner, demo_float_sub_mul_rational_round_val_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_val_ref_ref);
    register_demo!(runner, demo_float_sub_mul_rational_round_val_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_val_val);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_val_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_val_ref);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_ref_val);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_ref_ref);
    register_demo!(runner, demo_float_sub_mul_rational_round_ref_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_assign);
    register_demo!(runner, demo_float_sub_mul_rational_round_assign_debug);
    register_demo!(runner, demo_float_sub_mul_rational_round_assign_val_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_round_assign_val_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_round_assign_ref_val);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_round_assign_ref_val_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational_round_assign_ref_ref);
    register_demo!(
        runner,
        demo_float_sub_mul_rational_round_assign_ref_ref_debug
    );
    register_demo!(runner, demo_float_sub_mul_rational);
    register_demo!(runner, demo_float_sub_mul_rational_debug);
    register_demo!(runner, demo_float_sub_mul_rational_val_val_ref);
    register_demo!(runner, demo_float_sub_mul_rational_val_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_val_ref_val);
    register_demo!(runner, demo_float_sub_mul_rational_val_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_val_ref_ref);
    register_demo!(runner, demo_float_sub_mul_rational_val_ref_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_ref_val_val);
    register_demo!(runner, demo_float_sub_mul_rational_ref_val_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_ref_val_ref);
    register_demo!(runner, demo_float_sub_mul_rational_ref_val_ref_debug);
    register_demo!(runner, demo_float_sub_mul_rational_ref_ref_val);
    register_demo!(runner, demo_float_sub_mul_rational_ref_ref_val_debug);
    register_demo!(runner, demo_float_sub_mul_rational_ref_ref_ref);
    register_demo!(runner, demo_float_sub_mul_rational_ref_ref_ref_debug);
    register_bench!(
        runner,
        benchmark_float_sub_mul_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_float_sub_mul_rational_prec_round_algorithms
    );
}

fn demo_float_sub_mul_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_float_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "({}).sub_mul_prec_round({}, {}, {}, {}) = {:?}",
            x_old,
            y_old,
            z_old,
            prec,
            rm,
            x.sub_mul_prec_round(y, z, prec, rm)
        );
    }
}

fn demo_float_sub_mul_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_float_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        let (diff, o) = x.sub_mul_prec_round(y, z, prec, rm);
        println!(
            "({:#x}).sub_mul_prec_round({:#x}, {:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(z_old),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_mul_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_float_unsigned_rounding_mode_quintuple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "({}).sub_mul_prec_round({}, {}, {}, {}) = {:?}",
            x_old,
            y_old,
            z_old,
            prec,
            rm,
            x.sub_mul_prec_round(y, z, prec, rm)
        );
    }
}

fn demo_float_sub_mul_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_float_unsigned_rounding_mode_quintuple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        let (diff, o) = x.sub_mul_prec_round(y, z, prec, rm);
        println!(
            "({:#x}).sub_mul_prec_round({:#x}, {:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(z_old),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn demo_float_sub_mul_prec_round_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_float_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_prec_round_ref_ref_ref(&{}, &{}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm)
        );
    }
}

fn demo_float_sub_mul_prec_round_ref_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_float_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (diff, o) = x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
        println!(
            "(&{:#x}).sub_mul_prec_round_ref_ref_ref(&{:#x}, &{:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            rm,
            ComparableFloat(diff),
            o
        );
    }
}

fn benchmark_float_sub_mul_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_prec_round(Float, Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_float_unsigned_rounding_mode_quintuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer(
            "x", "y", "z", "prec",
        ),
        &mut [
            (
                "Float.sub_mul_prec_round(Float, Float, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| no_out!(x.sub_mul_prec_round(y, z, prec, rm)),
            ),
            (
                "(&Float).sub_mul_prec_round_ref_ref_ref(&Float, &Float, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_sub_mul_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_prec_round(Float, Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_float_unsigned_rounding_mode_quintuple_gen_var_3_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quintuple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer(
            "x", "y", "z", "prec",
        ),
        &mut [
            ("Malachite", &mut |(_, (x, y, z, prec, rm))| {
                no_out!(x.sub_mul_prec_round_ref_ref_ref(&y, &z, prec, rm));
            }),
            ("rug", &mut |((x, y, z, prec, rm), _)| {
                no_out!(rug_sub_mul_prec_round(&x, &y, &z, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_sub_mul_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_prec_round(Float, Float, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_float_float_unsigned_rounding_mode_quintuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer(
            "x", "y", "z", "prec",
        ),
        &mut [
            ("default", &mut |(x, y, z, prec, rm)| {
                no_out!(x.sub_mul_prec_round(y, z, prec, rm));
            }),
            ("naive", &mut |(x, y, z, prec, rm)| {
                no_out!(sub_mul_prec_round_naive(&x, &y, &z, prec, rm));
            }),
        ],
    );
}

fn demo_float_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(y.clone(), z.clone())
        );
    }
}

fn demo_float_sub_mul_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = x.clone().sub_mul(y.clone(), z.clone());
        println!(
            "({:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(y.clone(), &z)
        );
    }
}

fn demo_float_sub_mul_val_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = x.clone().sub_mul(y.clone(), &z);
        println!(
            "({:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(&y, z.clone())
        );
    }
}

fn demo_float_sub_mul_val_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = x.clone().sub_mul(&y, z.clone());
        println!(
            "({:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(&y, &z)
        );
    }
}

fn demo_float_sub_mul_val_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = x.clone().sub_mul(&y, &z);
        println!(
            "({:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(y.clone(), z.clone())
        );
    }
}

fn demo_float_sub_mul_ref_val_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = (&x).sub_mul(y.clone(), z.clone());
        println!(
            "(&{:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(y.clone(), &z)
        );
    }
}

fn demo_float_sub_mul_ref_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = (&x).sub_mul(y.clone(), &z);
        println!(
            "(&{:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(&y, z.clone())
        );
    }
}

fn demo_float_sub_mul_ref_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = (&x).sub_mul(&y, z.clone());
        println!(
            "(&{:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(&y, &z)
        );
    }
}

fn demo_float_sub_mul_ref_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let res = (&x).sub_mul(&y, &z);
        println!(
            "(&{:#x}).sub_mul({:#x}, {:#x}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            res
        );
    }
}

fn demo_float_sub_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_assign(y.clone(), z.clone());
        println!("x := {x_old}; x.sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_float_sub_mul_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_assign(y.clone(), z.clone());
        println!(
            "x := {:#x}; x.sub_mul_assign({:#x}, {:#x}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_assign(y.clone(), &z);
        println!("x := {x_old}; x.sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_float_sub_mul_assign_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_assign(y.clone(), &z);
        println!(
            "x := {:#x}; x.sub_mul_assign({:#x}, {:#x}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_assign(&y, z.clone());
        println!("x := {x_old}; x.sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_float_sub_mul_assign_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_assign(&y, z.clone());
        println!(
            "x := {:#x}; x.sub_mul_assign({:#x}, {:#x}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_assign(&y, &z);
        println!("x := {x_old}; x.sub_mul_assign({y}, {z}); x = {x}");
    }
}

fn demo_float_sub_mul_assign_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_triple_gen().get(gm, config).take(limit) {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_assign(&y, &z);
        println!(
            "x := {:#x}; x.sub_mul_assign({:#x}, {:#x}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_prec({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone().sub_mul_prec(y.clone(), z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_prec(y.clone(), z.clone(), prec);
        println!(
            "({:#x}).sub_mul_prec({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_prec_val_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone().sub_mul_prec_val_val_ref(y.clone(), &z, prec)
        );
    }
}

fn demo_float_sub_mul_prec_val_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_prec_val_val_ref(y.clone(), &z, prec);
        println!(
            "({:#x}).sub_mul_prec_val_val_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_prec_val_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone().sub_mul_prec_val_ref_val(&y, z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_prec_val_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_prec_val_ref_val(&y, z.clone(), prec);
        println!(
            "({:#x}).sub_mul_prec_val_ref_val({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_prec_val_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone().sub_mul_prec_val_ref_ref(&y, &z, prec)
        );
    }
}

fn demo_float_sub_mul_prec_val_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_prec_val_ref_ref(&y, &z, prec);
        println!(
            "({:#x}).sub_mul_prec_val_ref_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_prec_ref_val_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_prec_ref_val_val(y.clone(), z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_prec_ref_val_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_prec_ref_val_val(y.clone(), z.clone(), prec);
        println!(
            "(&{:#x}).sub_mul_prec_ref_val_val({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_prec_ref_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_prec_ref_val_ref(y.clone(), &z, prec)
        );
    }
}

fn demo_float_sub_mul_prec_ref_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_prec_ref_val_ref(y.clone(), &z, prec);
        println!(
            "(&{:#x}).sub_mul_prec_ref_val_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_prec_ref_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_prec_ref_ref_val(&y, z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_prec_ref_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_prec_ref_ref_val(&y, z.clone(), prec);
        println!(
            "(&{:#x}).sub_mul_prec_ref_ref_val({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_prec_ref_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_prec_ref_ref_ref(&y, &z, prec)
        );
    }
}

fn demo_float_sub_mul_prec_ref_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_prec_ref_ref_ref(&y, &z, prec);
        println!(
            "(&{:#x}).sub_mul_prec_ref_ref_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_prec_assign(y.clone(), z.clone(), prec);
        println!("x := {x_old}; x.sub_mul_prec_assign({y}, {z}, {prec}); x = {x}");
    }
}

fn demo_float_sub_mul_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_prec_assign(y.clone(), z.clone(), prec);
        println!(
            "x := {:#x}; x.sub_mul_prec_assign({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_prec_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_prec_assign_val_ref(y.clone(), &z, prec);
        println!("x := {x_old}; x.sub_mul_prec_assign_val_ref({y}, {z}, {prec}); x = {x}");
    }
}

fn demo_float_sub_mul_prec_assign_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_prec_assign_val_ref(y.clone(), &z, prec);
        println!(
            "x := {:#x}; x.sub_mul_prec_assign_val_ref({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_prec_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_prec_assign_ref_val(&y, z.clone(), prec);
        println!("x := {x_old}; x.sub_mul_prec_assign_ref_val({y}, {z}, {prec}); x = {x}");
    }
}

fn demo_float_sub_mul_prec_assign_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_prec_assign_ref_val(&y, z.clone(), prec);
        println!(
            "x := {:#x}; x.sub_mul_prec_assign_ref_val({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_prec_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_prec_assign_ref_ref(&y, &z, prec);
        println!("x := {x_old}; x.sub_mul_prec_assign_ref_ref({y}, {z}, {prec}); x = {x}");
    }
}

fn demo_float_sub_mul_prec_assign_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_float_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_prec_assign_ref_ref(&y, &z, prec);
        println!(
            "x := {:#x}; x.sub_mul_prec_assign_ref_ref({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_round({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone().sub_mul_round(y.clone(), z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_round(y.clone(), z.clone(), rm);
        println!(
            "({:#x}).sub_mul_round({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_round_val_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone().sub_mul_round_val_val_ref(y.clone(), &z, rm)
        );
    }
}

fn demo_float_sub_mul_round_val_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_round_val_val_ref(y.clone(), &z, rm);
        println!(
            "({:#x}).sub_mul_round_val_val_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_round_val_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone().sub_mul_round_val_ref_val(&y, z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_round_val_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_round_val_ref_val(&y, z.clone(), rm);
        println!(
            "({:#x}).sub_mul_round_val_ref_val({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_round_val_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone().sub_mul_round_val_ref_ref(&y, &z, rm)
        );
    }
}

fn demo_float_sub_mul_round_val_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_round_val_ref_ref(&y, &z, rm);
        println!(
            "({:#x}).sub_mul_round_val_ref_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_round_ref_val_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_round_ref_val_val(y.clone(), z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_round_ref_val_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_round_ref_val_val(y.clone(), z.clone(), rm);
        println!(
            "(&{:#x}).sub_mul_round_ref_val_val({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_round_ref_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_round_ref_val_ref(y.clone(), &z, rm)
        );
    }
}

fn demo_float_sub_mul_round_ref_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_round_ref_val_ref(y.clone(), &z, rm);
        println!(
            "(&{:#x}).sub_mul_round_ref_val_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_round_ref_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_round_ref_ref_val(&y, z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_round_ref_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_round_ref_ref_val(&y, z.clone(), rm);
        println!(
            "(&{:#x}).sub_mul_round_ref_ref_val({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_round_ref_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_round_ref_ref_ref(&y, &z, rm)
        );
    }
}

fn demo_float_sub_mul_round_ref_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_round_ref_ref_ref(&y, &z, rm);
        println!(
            "(&{:#x}).sub_mul_round_ref_ref_ref({:#x}, {:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_round_assign(y.clone(), z.clone(), rm);
        println!("x := {x_old}; x.sub_mul_round_assign({y}, {z}, {rm}); x = {x}");
    }
}

fn demo_float_sub_mul_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_round_assign(y.clone(), z.clone(), rm);
        println!(
            "x := {:#x}; x.sub_mul_round_assign({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_round_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_round_assign_val_ref(y.clone(), &z, rm);
        println!("x := {x_old}; x.sub_mul_round_assign_val_ref({y}, {z}, {rm}); x = {x}");
    }
}

fn demo_float_sub_mul_round_assign_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_round_assign_val_ref(y.clone(), &z, rm);
        println!(
            "x := {:#x}; x.sub_mul_round_assign_val_ref({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_round_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_round_assign_ref_val(&y, z.clone(), rm);
        println!("x := {x_old}; x.sub_mul_round_assign_ref_val({y}, {z}, {rm}); x = {x}");
    }
}

fn demo_float_sub_mul_round_assign_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_round_assign_ref_val(&y, z.clone(), rm);
        println!(
            "x := {:#x}; x.sub_mul_round_assign_ref_val({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_round_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_round_assign_ref_ref(&y, &z, rm);
        println!("x := {x_old}; x.sub_mul_round_assign_ref_ref({y}, {z}, {rm}); x = {x}");
    }
}

fn demo_float_sub_mul_round_assign_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_float_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_round_assign_ref_ref(&y, &z, rm);
        println!(
            "x := {:#x}; x.sub_mul_round_assign_ref_ref({:#x}, {:#x}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            ComparableFloat(z),
            rm,
            ComparableFloat(x)
        );
    }
}

fn benchmark_float_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul(Float, Float)",
        BenchmarkType::EvaluationStrategy,
        float_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_float_max_complexity_bucketer("x", "y", "z"),
        &mut [
            ("all by value", &mut |(x, y, z)| {
                no_out!(x.sub_mul(y, z));
            }),
            ("all by reference", &mut |(x, y, z)| {
                no_out!((&x).sub_mul(&y, &z));
            }),
        ],
    );
}

fn benchmark_float_sub_mul_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul(Float, Float)",
        BenchmarkType::LibraryComparison,
        float_triple_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_3_float_max_complexity_bucketer("x", "y", "z"),
        &mut [
            ("Malachite", &mut |(_, (x, y, z))| {
                no_out!((&x).sub_mul(&y, &z));
            }),
            ("rug", &mut |((x, y, z), _)| {
                no_out!(rug_sub_mul(&x, &y, &z));
            }),
        ],
    );
}

fn benchmark_float_sub_mul_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_prec(Float, Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_float_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer(
            "x", "y", "z", "prec",
        ),
        &mut [
            ("all by value", &mut |(x, y, z, prec)| {
                no_out!(x.sub_mul_prec(y, z, prec));
            }),
            ("all by reference", &mut |(x, y, z, prec)| {
                no_out!(x.sub_mul_prec_ref_ref_ref(&y, &z, prec));
            }),
        ],
    );
}

fn benchmark_float_sub_mul_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_prec(Float, Float, u64)",
        BenchmarkType::LibraryComparison,
        float_float_float_unsigned_quadruple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_4_float_float_float_primitive_int_max_complexity_bucketer(
            "x", "y", "z", "prec",
        ),
        &mut [
            ("Malachite", &mut |(_, (x, y, z, prec))| {
                no_out!(x.sub_mul_prec_ref_ref_ref(&y, &z, prec));
            }),
            ("rug", &mut |((x, y, z, prec), _)| {
                no_out!(rug_sub_mul_prec(&x, &y, &z, prec));
            }),
        ],
    );
}

fn benchmark_float_sub_mul_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_round(Float, Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_float_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_max_complexity_bucketer("x", "y", "z"),
        &mut [
            ("all by value", &mut |(x, y, z, rm)| {
                no_out!(x.sub_mul_round(y, z, rm));
            }),
            ("all by reference", &mut |(x, y, z, rm)| {
                no_out!(x.sub_mul_round_ref_ref_ref(&y, &z, rm));
            }),
        ],
    );
}

fn benchmark_float_sub_mul_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_round(Float, Float, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_float_rounding_mode_quadruple_gen_var_2_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_max_complexity_bucketer("x", "y", "z"),
        &mut [
            ("Malachite", &mut |(_, (x, y, z, rm))| {
                no_out!(x.sub_mul_round_ref_ref_ref(&y, &z, rm));
            }),
            ("rug", &mut |((x, y, z, rm), _)| {
                no_out!(rug_sub_mul_round(&x, &y, &z, rm));
            }),
        ],
    );
}

fn demo_float_sub_mul_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec_round({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.clone()
                .sub_mul_rational_prec_round(y.clone(), z.clone(), prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_prec_round(y.clone(), z.clone(), prec, rm);
        println!(
            "({:#x}).sub_mul_rational_prec_round({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_val_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec_round_val_val_ref({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.clone()
                .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_prec_round_val_val_ref(y.clone(), &z, prec, rm);
        println!(
            "({:#x}).sub_mul_rational_prec_round_val_val_ref({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_val_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec_round_val_ref_val({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.clone()
                .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_prec_round_val_ref_val(&y, z.clone(), prec, rm);
        println!(
            "({:#x}).sub_mul_rational_prec_round_val_ref_val({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_val_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec_round_val_ref_ref({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.clone()
                .sub_mul_rational_prec_round_val_ref_ref(&y, &z, prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_prec_round_val_ref_ref(&y, &z, prec, rm);
        println!(
            "({:#x}).sub_mul_rational_prec_round_val_ref_ref({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_val_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_round_ref_val_val({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_round_ref_val_val(y.clone(), z.clone(), prec, rm);
        println!(
            "(&{:#x}).sub_mul_rational_prec_round_ref_val_val({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_round_ref_val_ref({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_round_ref_val_ref(y.clone(), &z, prec, rm);
        println!(
            "(&{:#x}).sub_mul_rational_prec_round_ref_val_ref({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_round_ref_ref_val({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_round_ref_ref_val(&y, z.clone(), prec, rm);
        println!(
            "(&{:#x}).sub_mul_rational_prec_round_ref_ref_val({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_round_ref_ref_ref({}, {}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            rm,
            x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm);
        println!(
            "(&{:#x}).sub_mul_rational_prec_round_ref_ref_ref({:#x}, {}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_round_assign(y.clone(), z.clone(), prec, rm);
        println!(
            "x := {x_old}; x.sub_mul_rational_prec_round_assign({y}, {z}, {prec}, {rm}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_round_assign(y.clone(), z.clone(), prec, rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_round_assign({:#x}, {}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign_val_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_round_assign_val_ref(y.clone(), &z, prec, rm);
        println!(
            "x := {x_old}; \
             x.sub_mul_rational_prec_round_assign_val_ref({y}, {z}, {prec}, {rm}); x = \
             {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_round_assign_val_ref(y.clone(), &z, prec, rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_round_assign_val_ref({:#x}, {}, {}, {}); x = \
             {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign_ref_val(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_round_assign_ref_val(&y, z.clone(), prec, rm);
        println!(
            "x := {x_old}; \
             x.sub_mul_rational_prec_round_assign_ref_val({y}, {z}, {prec}, {rm}); x = \
             {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_round_assign_ref_val(&y, z.clone(), prec, rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_round_assign_ref_val({:#x}, {}, {}, {}); x = \
             {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign_ref_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_round_assign_ref_ref(&y, &z, prec, rm);
        println!(
            "x := {x_old}; \
             x.sub_mul_rational_prec_round_assign_ref_ref({y}, {z}, {prec}, {rm}); x = \
             {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_round_assign_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec, rm) in float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_round_assign_ref_ref(&y, &z, prec, rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_round_assign_ref_ref({:#x}, {}, {}, {}); x = \
             {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone().sub_mul_rational_prec(y.clone(), z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_rational_prec(y.clone(), z.clone(), prec);
        println!(
            "({:#x}).sub_mul_rational_prec({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec_val_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone()
                .sub_mul_rational_prec_val_val_ref(y.clone(), &z, prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_prec_val_val_ref(y.clone(), &z, prec);
        println!(
            "({:#x}).sub_mul_rational_prec_val_val_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec_val_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone()
                .sub_mul_rational_prec_val_ref_val(&y, z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_prec_val_ref_val(&y, z.clone(), prec);
        println!(
            "({:#x}).sub_mul_rational_prec_val_ref_val({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_prec_val_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.clone().sub_mul_rational_prec_val_ref_ref(&y, &z, prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_rational_prec_val_ref_ref(&y, &z, prec);
        println!(
            "({:#x}).sub_mul_rational_prec_val_ref_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_ref_val_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_rational_prec_ref_val_val(y.clone(), z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_ref_val_val(y.clone(), z.clone(), prec);
        println!(
            "(&{:#x}).sub_mul_rational_prec_ref_val_val({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_ref_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_rational_prec_ref_val_ref(y.clone(), &z, prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_ref_val_ref(y.clone(), &z, prec);
        println!(
            "(&{:#x}).sub_mul_rational_prec_ref_val_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_ref_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_rational_prec_ref_ref_val(&y, z.clone(), prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_ref_ref_val(&y, z.clone(), prec);
        println!(
            "(&{:#x}).sub_mul_rational_prec_ref_ref_val({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_prec_ref_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            prec,
            x.sub_mul_rational_prec_ref_ref_ref(&y, &z, prec)
        );
    }
}

fn demo_float_sub_mul_rational_prec_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_prec_ref_ref_ref(&y, &z, prec);
        println!(
            "(&{:#x}).sub_mul_rational_prec_ref_ref_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            prec,
            res
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_assign(y.clone(), z.clone(), prec);
        println!(
            "x := {x_old}; x.sub_mul_rational_prec_assign({y}, {z}, {prec}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_assign(y.clone(), z.clone(), prec);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_assign({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_assign_val_ref(y.clone(), &z, prec);
        println!(
            "x := {x_old}; x.sub_mul_rational_prec_assign_val_ref({y}, {z}, {prec}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_assign_val_ref(y.clone(), &z, prec);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_assign_val_ref({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_assign_ref_val(&y, z.clone(), prec);
        println!(
            "x := {x_old}; x.sub_mul_rational_prec_assign_ref_val({y}, {z}, {prec}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_assign_ref_val(&y, z.clone(), prec);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_assign_ref_val({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_prec_assign_ref_ref(&y, &z, prec);
        println!(
            "x := {x_old}; x.sub_mul_rational_prec_assign_ref_ref({y}, {z}, {prec}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_prec_assign_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, prec) in float_float_rational_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_prec_assign_ref_ref(&y, &z, prec);
        println!(
            "x := {:#x}; x.sub_mul_rational_prec_assign_ref_ref({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            prec,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_round({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone().sub_mul_rational_round(y.clone(), z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_rational_round(y.clone(), z.clone(), rm);
        println!(
            "({:#x}).sub_mul_rational_round({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_round_val_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone()
                .sub_mul_rational_round_val_val_ref(y.clone(), &z, rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_val_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_round_val_val_ref(y.clone(), &z, rm);
        println!(
            "({:#x}).sub_mul_rational_round_val_val_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_round_val_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone()
                .sub_mul_rational_round_val_ref_val(&y, z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_val_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x
            .clone()
            .sub_mul_rational_round_val_ref_val(&y, z.clone(), rm);
        println!(
            "({:#x}).sub_mul_rational_round_val_ref_val({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul_rational_round_val_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.clone().sub_mul_rational_round_val_ref_ref(&y, &z, rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_val_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul_rational_round_val_ref_ref(&y, &z, rm);
        println!(
            "({:#x}).sub_mul_rational_round_val_ref_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_round_ref_val_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_rational_round_ref_val_val(y.clone(), z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_val_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_round_ref_val_val(y.clone(), z.clone(), rm);
        println!(
            "(&{:#x}).sub_mul_rational_round_ref_val_val({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_round_ref_val_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_rational_round_ref_val_ref(y.clone(), &z, rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_round_ref_val_ref(y.clone(), &z, rm);
        println!(
            "(&{:#x}).sub_mul_rational_round_ref_val_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_round_ref_ref_val({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_rational_round_ref_ref_val(&y, z.clone(), rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_round_ref_ref_val(&y, z.clone(), rm);
        println!(
            "(&{:#x}).sub_mul_rational_round_ref_ref_val({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul_rational_round_ref_ref_ref({}, {}, {}) = {:?}",
            x,
            y,
            z,
            rm,
            x.sub_mul_rational_round_ref_ref_ref(&y, &z, rm)
        );
    }
}

fn demo_float_sub_mul_rational_round_ref_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let res = x.sub_mul_rational_round_ref_ref_ref(&y, &z, rm);
        println!(
            "(&{:#x}).sub_mul_rational_round_ref_ref_ref({:#x}, {}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            rm,
            res
        );
    }
}

fn demo_float_sub_mul_rational_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_round_assign(y.clone(), z.clone(), rm);
        println!(
            "x := {x_old}; x.sub_mul_rational_round_assign({y}, {z}, {rm}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_round_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_round_assign(y.clone(), z.clone(), rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_round_assign({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_round_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_round_assign_val_ref(y.clone(), &z, rm);
        println!(
            "x := {x_old}; x.sub_mul_rational_round_assign_val_ref({y}, {z}, {rm}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_round_assign_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_round_assign_val_ref(y.clone(), &z, rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_round_assign_val_ref({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_round_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_round_assign_ref_val(&y, z.clone(), rm);
        println!(
            "x := {x_old}; x.sub_mul_rational_round_assign_ref_val({y}, {z}, {rm}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_round_assign_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_round_assign_ref_val(&y, z.clone(), rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_round_assign_ref_val({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational_round_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = x.clone();
        x.sub_mul_rational_round_assign_ref_ref(&y, &z, rm);
        println!(
            "x := {x_old}; x.sub_mul_rational_round_assign_ref_ref({y}, {z}, {rm}); x = {x}"
        );
    }
}

fn demo_float_sub_mul_rational_round_assign_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, z, rm) in float_float_rational_rounding_mode_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let mut x = x;
        let x_old = ComparableFloat(x.clone());
        x.sub_mul_rational_round_assign_ref_ref(&y, &z, rm);
        println!(
            "x := {:#x}; x.sub_mul_rational_round_assign_ref_ref({:#x}, {}, {}); x = {:#x}",
            x_old,
            ComparableFloat(y),
            z,
            rm,
            ComparableFloat(x)
        );
    }
}

fn demo_float_sub_mul_rational(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(y.clone(), z.clone())
        );
    }
}

fn demo_float_sub_mul_rational_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul(y.clone(), z.clone());
        println!(
            "({:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn demo_float_sub_mul_rational_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(y.clone(), &z)
        );
    }
}

fn demo_float_sub_mul_rational_val_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul(y.clone(), &z);
        println!(
            "({:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn demo_float_sub_mul_rational_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(&y, z.clone())
        );
    }
}

fn demo_float_sub_mul_rational_val_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul(&y, z.clone());
        println!(
            "({:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn demo_float_sub_mul_rational_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            x.clone().sub_mul(&y, &z)
        );
    }
}

fn demo_float_sub_mul_rational_val_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = x.clone().sub_mul(&y, &z);
        println!(
            "({:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn demo_float_sub_mul_rational_ref_val_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(y.clone(), z.clone())
        );
    }
}

fn demo_float_sub_mul_rational_ref_val_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = (&x).sub_mul(y.clone(), z.clone());
        println!(
            "(&{:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn demo_float_sub_mul_rational_ref_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(y.clone(), &z)
        );
    }
}

fn demo_float_sub_mul_rational_ref_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = (&x).sub_mul(y.clone(), &z);
        println!(
            "(&{:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn demo_float_sub_mul_rational_ref_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(&y, z.clone())
        );
    }
}

fn demo_float_sub_mul_rational_ref_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = (&x).sub_mul(&y, z.clone());
        println!(
            "(&{:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn demo_float_sub_mul_rational_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            x,
            y,
            z,
            (&x).sub_mul(&y, &z)
        );
    }
}

fn demo_float_sub_mul_rational_ref_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in float_float_rational_triple_gen()
        .get(gm, config)
        .take(limit)
    {
        let res = (&x).sub_mul(&y, &z);
        println!(
            "(&{:#x}).sub_mul({:#x}, {}) = {:?}",
            ComparableFloat(x),
            ComparableFloat(y),
            z,
            res
        );
    }
}

fn benchmark_float_sub_mul_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_rational_prec_round(Float, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_float_float_rational_max_complexity_bucketer("x", "y", "z"),
        &mut [
            (
                "Float.sub_mul_rational_prec_round(Float, Rational, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| no_out!(x.sub_mul_rational_prec_round(y, z, prec, rm)),
            ),
            (
                "(&Float).sub_mul_rational_prec_round_ref_ref_ref(&Float, &Rational, u64, \
                RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.sub_mul_rational_prec_round_ref_ref_ref(&y, &z, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_sub_mul_rational_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.sub_mul_rational_prec_round(Float, Rational, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        float_float_rational_unsigned_rounding_mode_quintuple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_float_float_rational_max_complexity_bucketer("x", "y", "z"),
        &mut [
            ("default", &mut |(x, y, z, prec, rm)| {
                no_out!(x.sub_mul_rational_prec_round(y, z, prec, rm));
            }),
            ("naive", &mut |(x, y, z, prec, rm)| {
                no_out!(sub_mul_rational_prec_round_naive(&x, &y, &z, prec, rm));
            }),
        ],
    );
}
