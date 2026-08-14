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
use malachite_float::ComparableFloat;
use malachite_float::test_util::bench::bucketers::{
    pair_2_pair_float_max_complexity_bucketer,
    pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    pair_2_triple_1_2_float_max_complexity_bucketer,
    pair_2_triple_float_float_primitive_int_max_complexity_bucketer,
    pair_float_max_complexity_bucketer,
    quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer,
    triple_1_2_float_max_complexity_bucketer,
    triple_float_float_primitive_int_max_complexity_bucketer,
};
use malachite_float::test_util::float::comparison::min_max::{
    rug_max, rug_max_prec, rug_max_prec_round, rug_max_round, rug_min, rug_min_prec,
    rug_min_prec_round, rug_min_round,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_39, float_float_rounding_mode_triple_gen_var_39_rm,
    float_float_unsigned_rounding_mode_quadruple_gen_var_16,
    float_float_unsigned_rounding_mode_quadruple_gen_var_16_rm,
    float_float_unsigned_rounding_mode_quadruple_gen_var_17,
    float_float_unsigned_rounding_mode_quadruple_gen_var_17_rm,
    float_float_unsigned_triple_gen_var_1, float_float_unsigned_triple_gen_var_1_rm,
    float_pair_gen, float_pair_gen_rm, float_pair_gen_var_10,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_min);
    register_demo!(runner, demo_float_min_debug);
    register_demo!(runner, demo_float_min_val_ref);
    register_demo!(runner, demo_float_min_val_ref_debug);
    register_demo!(runner, demo_float_min_ref_val);
    register_demo!(runner, demo_float_min_ref_val_debug);
    register_demo!(runner, demo_float_min_ref_ref);
    register_demo!(runner, demo_float_min_ref_ref_debug);
    register_demo!(runner, demo_float_min_extreme);
    register_demo!(runner, demo_float_min_extreme_debug);
    register_demo!(runner, demo_float_min_prec);
    register_demo!(runner, demo_float_min_prec_debug);
    register_demo!(runner, demo_float_min_prec_val_ref);
    register_demo!(runner, demo_float_min_prec_val_ref_debug);
    register_demo!(runner, demo_float_min_prec_ref_val);
    register_demo!(runner, demo_float_min_prec_ref_val_debug);
    register_demo!(runner, demo_float_min_prec_ref_ref);
    register_demo!(runner, demo_float_min_prec_ref_ref_debug);
    register_demo!(runner, demo_float_min_round);
    register_demo!(runner, demo_float_min_round_debug);
    register_demo!(runner, demo_float_min_round_val_ref);
    register_demo!(runner, demo_float_min_round_val_ref_debug);
    register_demo!(runner, demo_float_min_round_ref_val);
    register_demo!(runner, demo_float_min_round_ref_val_debug);
    register_demo!(runner, demo_float_min_round_ref_ref);
    register_demo!(runner, demo_float_min_round_ref_ref_debug);
    register_demo!(runner, demo_float_min_prec_round);
    register_demo!(runner, demo_float_min_prec_round_debug);
    register_demo!(runner, demo_float_min_prec_round_val_ref);
    register_demo!(runner, demo_float_min_prec_round_val_ref_debug);
    register_demo!(runner, demo_float_min_prec_round_ref_val);
    register_demo!(runner, demo_float_min_prec_round_ref_val_debug);
    register_demo!(runner, demo_float_min_prec_round_ref_ref);
    register_demo!(runner, demo_float_min_prec_round_ref_ref_debug);
    register_demo!(runner, demo_float_max);
    register_demo!(runner, demo_float_max_debug);
    register_demo!(runner, demo_float_max_val_ref);
    register_demo!(runner, demo_float_max_val_ref_debug);
    register_demo!(runner, demo_float_max_ref_val);
    register_demo!(runner, demo_float_max_ref_val_debug);
    register_demo!(runner, demo_float_max_ref_ref);
    register_demo!(runner, demo_float_max_ref_ref_debug);
    register_demo!(runner, demo_float_max_extreme);
    register_demo!(runner, demo_float_max_extreme_debug);
    register_demo!(runner, demo_float_max_prec);
    register_demo!(runner, demo_float_max_prec_debug);
    register_demo!(runner, demo_float_max_prec_val_ref);
    register_demo!(runner, demo_float_max_prec_val_ref_debug);
    register_demo!(runner, demo_float_max_prec_ref_val);
    register_demo!(runner, demo_float_max_prec_ref_val_debug);
    register_demo!(runner, demo_float_max_prec_ref_ref);
    register_demo!(runner, demo_float_max_prec_ref_ref_debug);
    register_demo!(runner, demo_float_max_round);
    register_demo!(runner, demo_float_max_round_debug);
    register_demo!(runner, demo_float_max_round_val_ref);
    register_demo!(runner, demo_float_max_round_val_ref_debug);
    register_demo!(runner, demo_float_max_round_ref_val);
    register_demo!(runner, demo_float_max_round_ref_val_debug);
    register_demo!(runner, demo_float_max_round_ref_ref);
    register_demo!(runner, demo_float_max_round_ref_ref_debug);
    register_demo!(runner, demo_float_max_prec_round);
    register_demo!(runner, demo_float_max_prec_round_debug);
    register_demo!(runner, demo_float_max_prec_round_val_ref);
    register_demo!(runner, demo_float_max_prec_round_val_ref_debug);
    register_demo!(runner, demo_float_max_prec_round_ref_val);
    register_demo!(runner, demo_float_max_prec_round_ref_val_debug);
    register_demo!(runner, demo_float_max_prec_round_ref_ref);
    register_demo!(runner, demo_float_max_prec_round_ref_ref_debug);

    register_bench!(runner, benchmark_float_min_evaluation_strategy);
    register_bench!(runner, benchmark_float_min_library_comparison);
    register_bench!(runner, benchmark_float_min_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_min_prec_library_comparison);
    register_bench!(runner, benchmark_float_min_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_min_round_library_comparison);
    register_bench!(runner, benchmark_float_min_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_min_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_max_evaluation_strategy);
    register_bench!(runner, benchmark_float_max_library_comparison);
    register_bench!(runner, benchmark_float_max_prec_evaluation_strategy);
    register_bench!(runner, benchmark_float_max_prec_library_comparison);
    register_bench!(runner, benchmark_float_max_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_max_round_library_comparison);
    register_bench!(runner, benchmark_float_max_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_max_prec_round_library_comparison);
}

fn demo_float_min(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).min({}) = {:?}", x_old, y_old, x.min(y));
    }
}

fn demo_float_min_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min(y);
        println!(
            "({:#x}).min({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_val_ref({}) = {:?}",
            x_old,
            y_old,
            x.min_val_ref(&y)
        );
    }
}

fn demo_float_min_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_val_ref(&y);
        println!(
            "({:#x}).min_val_ref({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_ref_val({}) = {:?}",
            x_old,
            y_old,
            x.min_ref_val(y)
        );
    }
}

fn demo_float_min_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_ref_val(y);
        println!(
            "({:#x}).min_ref_val({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_ref_ref({}) = {:?}",
            x_old,
            y_old,
            x.min_ref_ref(&y)
        );
    }
}

fn demo_float_min_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_ref_ref(&y);
        println!(
            "({:#x}).min_ref_ref({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).min({}) = {:?}", x_old, y_old, x.min(y));
    }
}

fn demo_float_min_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min(y);
        println!(
            "({:#x}).min({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.min_prec(y, prec)
        );
    }
}

fn demo_float_min_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec(y, prec);
        println!(
            "({:#x}).min_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec_val_ref({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.min_prec_val_ref(&y, prec)
        );
    }
}

fn demo_float_min_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec_val_ref(&y, prec);
        println!(
            "({:#x}).min_prec_val_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec_ref_val({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.min_prec_ref_val(y, prec)
        );
    }
}

fn demo_float_min_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec_ref_val(y, prec);
        println!(
            "({:#x}).min_prec_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec_ref_ref({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.min_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_float_min_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec_ref_ref(&y, prec);
        println!(
            "({:#x}).min_prec_ref_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.min_round(y, rm)
        );
    }
}

fn demo_float_min_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_round(y, rm);
        println!(
            "({:#x}).min_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_round_val_ref({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.min_round_val_ref(&y, rm)
        );
    }
}

fn demo_float_min_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_round_val_ref(&y, rm);
        println!(
            "({:#x}).min_round_val_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_round_ref_val({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.min_round_ref_val(y, rm)
        );
    }
}

fn demo_float_min_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_round_ref_val(y, rm);
        println!(
            "({:#x}).min_round_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_round_ref_ref({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.min_round_ref_ref(&y, rm)
        );
    }
}

fn demo_float_min_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_round_ref_ref(&y, rm);
        println!(
            "({:#x}).min_round_ref_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.min_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_min_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec_round(y, prec, rm);
        println!(
            "({:#x}).min_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec_round_val_ref({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.min_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_float_min_prec_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec_round_val_ref(&y, prec, rm);
        println!(
            "({:#x}).min_prec_round_val_ref({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec_round_ref_val({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.min_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_float_min_prec_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec_round_ref_val(y, prec, rm);
        println!(
            "({:#x}).min_prec_round_ref_val({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_min_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).min_prec_round_ref_ref({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.min_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_float_min_prec_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_16()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.min_prec_round_ref_ref(&y, prec, rm);
        println!(
            "({:#x}).min_prec_round_ref_ref({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).max({}) = {:?}", x_old, y_old, x.max(y));
    }
}

fn demo_float_max_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max(y);
        println!(
            "({:#x}).max({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_val_ref({}) = {:?}",
            x_old,
            y_old,
            x.max_val_ref(&y)
        );
    }
}

fn demo_float_max_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_val_ref(&y);
        println!(
            "({:#x}).max_val_ref({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_ref_val({}) = {:?}",
            x_old,
            y_old,
            x.max_ref_val(y)
        );
    }
}

fn demo_float_max_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_ref_val(y);
        println!(
            "({:#x}).max_ref_val({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_ref_ref({}) = {:?}",
            x_old,
            y_old,
            x.max_ref_ref(&y)
        );
    }
}

fn demo_float_max_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_ref_ref(&y);
        println!(
            "({:#x}).max_ref_ref({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({}).max({}) = {:?}", x_old, y_old, x.max(y));
    }
}

fn demo_float_max_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in float_pair_gen_var_10().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max(y);
        println!(
            "({:#x}).max({:#x}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.max_prec(y, prec)
        );
    }
}

fn demo_float_max_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec(y, prec);
        println!(
            "({:#x}).max_prec({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec_val_ref({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.max_prec_val_ref(&y, prec)
        );
    }
}

fn demo_float_max_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec_val_ref(&y, prec);
        println!(
            "({:#x}).max_prec_val_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec_ref_val({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.max_prec_ref_val(y, prec)
        );
    }
}

fn demo_float_max_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec_ref_val(y, prec);
        println!(
            "({:#x}).max_prec_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec_ref_ref({}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            x.max_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_float_max_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in float_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec_ref_ref(&y, prec);
        println!(
            "({:#x}).max_prec_ref_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_round({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.max_round(y, rm)
        );
    }
}

fn demo_float_max_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_round(y, rm);
        println!(
            "({:#x}).max_round({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_round_val_ref({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.max_round_val_ref(&y, rm)
        );
    }
}

fn demo_float_max_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_round_val_ref(&y, rm);
        println!(
            "({:#x}).max_round_val_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_round_ref_val({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.max_round_ref_val(y, rm)
        );
    }
}

fn demo_float_max_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_round_ref_val(y, rm);
        println!(
            "({:#x}).max_round_ref_val({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_round_ref_ref({}, {}) = {:?}",
            x_old,
            y_old,
            rm,
            x.max_round_ref_ref(&y, rm)
        );
    }
}

fn demo_float_max_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in float_float_rounding_mode_triple_gen_var_39()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_round_ref_ref(&y, rm);
        println!(
            "({:#x}).max_round_ref_ref({:#x}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec_round({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.max_prec_round(y, prec, rm)
        );
    }
}

fn demo_float_max_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec_round(y, prec, rm);
        println!(
            "({:#x}).max_prec_round({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec_round_val_ref({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.max_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_float_max_prec_round_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec_round_val_ref(&y, prec, rm);
        println!(
            "({:#x}).max_prec_round_val_ref({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec_round_ref_val({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.max_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_float_max_prec_round_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec_round_ref_val(y, prec, rm);
        println!(
            "({:#x}).max_prec_round_ref_val({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn demo_float_max_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).max_prec_round_ref_ref({}, {}, {}) = {:?}",
            x_old,
            y_old,
            prec,
            rm,
            x.max_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_float_max_prec_round_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in float_float_unsigned_rounding_mode_quadruple_gen_var_17()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (result, o) = x.max_prec_round_ref_ref(&y, prec, rm);
        println!(
            "({:#x}).max_prec_round_ref_ref({:#x}, {}, {}) = ({:#x}, {:?})",
            ComparableFloat(x_old),
            ComparableFloat(y_old),
            prec,
            rm,
            ComparableFloat(result),
            o
        );
    }
}

fn benchmark_float_min_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.min(Float)", &mut |(x, y)| {
                no_out!(x.min(y));
            }),
            ("Float.min_val_ref(&Float)", &mut |(x, y)| {
                no_out!(x.min_val_ref(&y));
            }),
            ("(&Float).min_ref_val(Float)", &mut |(x, y)| {
                no_out!(x.min_ref_val(y));
            }),
            ("(&Float).min_ref_ref(&Float)", &mut |(x, y)| {
                no_out!(x.min_ref_ref(&y));
            }),
        ],
    );
}

fn benchmark_float_min_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min(Float)",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.min_ref_ref(&y));
            }),
            ("rug", &mut |((x, y), _)| {
                no_out!(rug_min(&x, &y));
            }),
        ],
    );
}

fn benchmark_float_min_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min_prec(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.min_prec(Float, u64)", &mut |(x, y, prec)| {
                no_out!(x.min_prec(y, prec));
            }),
            ("Float.min_prec_val_ref(&Float, u64)", &mut |(
                x,
                y,
                prec,
            )| {
                no_out!(x.min_prec_val_ref(&y, prec));
            }),
            (
                "(&Float).min_prec_ref_val(Float, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_prec_ref_val(y, prec));
                },
            ),
            (
                "(&Float).min_prec_ref_ref(&Float, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.min_prec_ref_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_float_min_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min_prec(Float, u64)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec))| {
                no_out!(x.min_prec_ref_ref(&y, prec));
            }),
            ("rug", &mut |((x, y, prec), _)| {
                no_out!(rug_min_prec(&x, &y, prec));
            }),
        ],
    );
}

fn benchmark_float_min_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min_round(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_39().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.min_round(Float, RoundingMode)", &mut |(x, y, rm)| {
                no_out!(x.min_round(y, rm));
            }),
            (
                "Float.min_round_val_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.min_round_val_ref(&y, rm));
                },
            ),
            (
                "(&Float).min_round_ref_val(Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.min_round_ref_val(y, rm));
                },
            ),
            (
                "(&Float).min_round_ref_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.min_round_ref_ref(&y, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_min_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min_round(Float, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_rounding_mode_triple_gen_var_39_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, rm))| {
                no_out!(x.min_round_ref_ref(&y, rm));
            }),
            ("rug", &mut |((x, y, rm), _)| {
                no_out!(rug_min_round(&x, &y, rm));
            }),
        ],
    );
}

fn benchmark_float_min_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_16().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.min_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round(y, prec, rm));
                },
            ),
            (
                "Float.min_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "(&Float).min_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "(&Float).min_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.min_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_min_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.min_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_rounding_mode_quadruple_gen_var_16_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.min_prec_round_ref_ref(&y, prec, rm));
            }),
            ("rug", &mut |((x, y, prec, rm), _)| {
                no_out!(rug_min_prec_round(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_float_max_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max(Float)",
        BenchmarkType::EvaluationStrategy,
        float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.max(Float)", &mut |(x, y)| {
                no_out!(x.max(y));
            }),
            ("Float.max_val_ref(&Float)", &mut |(x, y)| {
                no_out!(x.max_val_ref(&y));
            }),
            ("(&Float).max_ref_val(Float)", &mut |(x, y)| {
                no_out!(x.max_ref_val(y));
            }),
            ("(&Float).max_ref_ref(&Float)", &mut |(x, y)| {
                no_out!(x.max_ref_ref(&y));
            }),
        ],
    );
}

fn benchmark_float_max_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max(Float)",
        BenchmarkType::LibraryComparison,
        float_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| {
                no_out!(x.max_ref_ref(&y));
            }),
            ("rug", &mut |((x, y), _)| {
                no_out!(rug_max(&x, &y));
            }),
        ],
    );
}

fn benchmark_float_max_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max_prec(Float, u64)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float.max_prec(Float, u64)", &mut |(x, y, prec)| {
                no_out!(x.max_prec(y, prec));
            }),
            ("Float.max_prec_val_ref(&Float, u64)", &mut |(
                x,
                y,
                prec,
            )| {
                no_out!(x.max_prec_val_ref(&y, prec));
            }),
            (
                "(&Float).max_prec_ref_val(Float, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_prec_ref_val(y, prec));
                },
            ),
            (
                "(&Float).max_prec_ref_ref(&Float, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.max_prec_ref_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_float_max_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max_prec(Float, u64)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec))| {
                no_out!(x.max_prec_ref_ref(&y, prec));
            }),
            ("rug", &mut |((x, y, prec), _)| {
                no_out!(rug_max_prec(&x, &y, prec));
            }),
        ],
    );
}

fn benchmark_float_max_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max_round(Float, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_rounding_mode_triple_gen_var_39().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Float.max_round(Float, RoundingMode)", &mut |(x, y, rm)| {
                no_out!(x.max_round(y, rm));
            }),
            (
                "Float.max_round_val_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.max_round_val_ref(&y, rm));
                },
            ),
            (
                "(&Float).max_round_ref_val(Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.max_round_ref_val(y, rm));
                },
            ),
            (
                "(&Float).max_round_ref_ref(&Float, RoundingMode)",
                &mut |(x, y, rm)| {
                    no_out!(x.max_round_ref_ref(&y, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_max_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max_round(Float, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_rounding_mode_triple_gen_var_39_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y, rm))| {
                no_out!(x.max_round_ref_ref(&y, rm));
            }),
            ("rug", &mut |((x, y, rm), _)| {
                no_out!(rug_max_round(&x, &y, rm));
            }),
        ],
    );
}

fn benchmark_float_max_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_float_unsigned_rounding_mode_quadruple_gen_var_17().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "Float.max_prec_round(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round(y, prec, rm));
                },
            ),
            (
                "Float.max_prec_round_val_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "(&Float).max_prec_round_ref_val(Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "(&Float).max_prec_round_ref_ref(&Float, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.max_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_float_max_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.max_prec_round(Float, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_float_unsigned_rounding_mode_quadruple_gen_var_17_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Malachite", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.max_prec_round_ref_ref(&y, prec, rm));
            }),
            ("rug", &mut |((x, y, prec, rm), _)| {
                no_out!(rug_max_prec_round(&x, &y, prec, rm));
            }),
        ],
    );
}
