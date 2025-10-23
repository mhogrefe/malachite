// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, max_pair_1_complexity_pair_2_bucketer,
    max_triple_1_float_complexity_triple_2_bucketer, pair_2_float_complexity_bucketer,
    pair_2_max_pair_1_complexity_pair_2_bucketer,
    pair_2_max_triple_1_float_complexity_triple_2_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_gen_var_12, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_1_rm, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_1_rm,
    float_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_to_significand);
    register_demo!(runner, demo_float_to_significand_debug);
    register_demo!(runner, demo_float_to_significand_extreme);
    register_demo!(runner, demo_float_to_significand_extreme_debug);
    register_demo!(runner, demo_float_into_significand);
    register_demo!(runner, demo_float_into_significand_debug);
    register_demo!(runner, demo_float_significand_ref);
    register_demo!(runner, demo_float_significand_ref_debug);
    register_demo!(runner, demo_float_get_exponent);
    register_demo!(runner, demo_float_get_exponent_debug);
    register_demo!(runner, demo_float_get_exponent_extreme);
    register_demo!(runner, demo_float_get_exponent_extreme_debug);
    register_demo!(runner, demo_float_get_prec);
    register_demo!(runner, demo_float_get_prec_debug);
    register_demo!(runner, demo_float_get_prec_extreme);
    register_demo!(runner, demo_float_get_prec_extreme_debug);
    register_demo!(runner, demo_float_get_min_prec);
    register_demo!(runner, demo_float_get_min_prec_debug);
    register_demo!(runner, demo_float_get_min_prec_extreme);
    register_demo!(runner, demo_float_get_min_prec_extreme_debug);
    register_demo!(runner, demo_float_set_prec_round);
    register_demo!(runner, demo_float_set_prec_round_debug);
    register_demo!(runner, demo_float_set_prec_round_extreme);
    register_demo!(runner, demo_float_set_prec_round_extreme_debug);
    register_demo!(runner, demo_float_set_prec);
    register_demo!(runner, demo_float_set_prec_debug);
    register_demo!(runner, demo_float_set_prec_extreme);
    register_demo!(runner, demo_float_set_prec_extreme_debug);
    register_demo!(runner, demo_float_from_float_prec_round);
    register_demo!(runner, demo_float_from_float_prec_round_debug);
    register_demo!(runner, demo_float_from_float_prec_round_ref);
    register_demo!(runner, demo_float_from_float_prec_round_ref_debug);
    register_demo!(runner, demo_float_from_float_prec);
    register_demo!(runner, demo_float_from_float_prec_debug);
    register_demo!(runner, demo_float_from_float_prec_ref);
    register_demo!(runner, demo_float_from_float_prec_ref_debug);

    register_bench!(runner, benchmark_float_to_significand_evaluation_strategy);
    register_bench!(runner, benchmark_float_significand_ref_library_comparison);
    register_bench!(runner, benchmark_float_get_exponent_library_comparison);
    register_bench!(runner, benchmark_float_get_prec_library_comparison);
    register_bench!(runner, benchmark_float_get_min_prec);
    register_bench!(runner, benchmark_float_set_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_set_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_float_set_prec_library_comparison);
    register_bench!(runner, benchmark_float_set_prec_evaluation_strategy);
}

fn demo_float_to_significand(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_significand({}) = {:?}", x, x.to_significand());
    }
}

fn demo_float_to_significand_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "to_significand({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.to_significand()
        );
    }
}

fn demo_float_to_significand_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!("to_significand({}) = {:?}", x, x.to_significand());
    }
}

fn demo_float_to_significand_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "to_significand({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.to_significand()
        );
    }
}

fn demo_float_into_significand(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "into_significand({}) = {:?}",
            x.clone(),
            x.into_significand()
        );
    }
}

fn demo_float_into_significand_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "into_significand({:#x}) = {:?}",
            ComparableFloat(x.clone()),
            x.into_significand()
        );
    }
}

fn demo_float_significand_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("significand_ref({}) = {:?}", x, x.significand_ref());
    }
}

fn demo_float_significand_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "significand_ref({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.significand_ref()
        );
    }
}

fn demo_float_get_exponent(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("get_exponent({}) = {:?}", x, x.get_exponent());
    }
}

fn demo_float_get_exponent_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "get_exponent({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.get_exponent()
        );
    }
}

fn demo_float_get_exponent_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!("get_exponent({}) = {:?}", x, x.get_exponent());
    }
}

fn demo_float_get_exponent_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "get_exponent({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.get_exponent()
        );
    }
}

fn demo_float_get_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("get_prec({}) = {:?}", x, x.get_prec());
    }
}

fn demo_float_get_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "get_prec({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.get_prec()
        );
    }
}

fn demo_float_get_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!("get_prec({}) = {:?}", x, x.get_prec());
    }
}

fn demo_float_get_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "get_prec({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.get_prec()
        );
    }
}

fn demo_float_get_min_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("get_min_prec({}) = {:?}", x, x.get_min_prec());
    }
}

fn demo_float_get_min_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "get_min_prec({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.get_min_prec()
        );
    }
}

fn demo_float_get_min_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!("get_min_prec({}) = {:?}", x, x.get_min_prec());
    }
}

fn demo_float_get_min_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "get_min_prec({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.get_min_prec()
        );
    }
}

fn demo_float_set_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.set_prec_round(prec, rm);
        println!("x := {old_x}; x.set_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_set_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.set_prec_round(prec, rm);
        println!(
            "x := {:#x}; x.set_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(old_x),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_set_prec_round_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.set_prec_round(prec, rm);
        println!("x := {old_x}; x.set_prec_round({prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_set_prec_round_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.set_prec_round(prec, rm);
        println!(
            "x := {:#x}; x.set_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(old_x),
            prec,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_set_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let old_x = x.clone();
        let o = x.set_prec(prec);
        println!("x := {old_x}; x.set_prec({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_set_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let old_x = x.clone();
        let o = x.set_prec(prec);
        println!(
            "x := {:#x}; x.set_prec({}) = {:?}; x = {:#x}",
            ComparableFloat(old_x),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_set_prec_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let old_x = x.clone();
        let o = x.set_prec(prec);
        println!("x := {old_x}; x.set_prec({prec}) = {o:?}; x = {x}");
    }
}

fn demo_float_set_prec_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, prec) in float_unsigned_pair_gen_var_4().get(gm, config).take(limit) {
        let old_x = x.clone();
        let o = x.set_prec(prec);
        println!(
            "x := {:#x}; x.set_prec({}) = {:?}; x = {:#x}",
            ComparableFloat(old_x),
            prec,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_from_float_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (y, o) = Float::from_float_prec_round(x.clone(), prec, rm);
        println!("Float::from_float_prec_round({x}, {prec}, {rm}) = ({y}, {o:?})");
    }
}

fn demo_float_from_float_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (y, o) = Float::from_float_prec_round(x.clone(), prec, rm);
        println!("Float::from_float_prec_round({x:#x}, {prec}, {rm}) = ({y:#x}, {o:?})");
    }
}

fn demo_float_from_float_prec_round_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (y, o) = Float::from_float_prec_round_ref(&x, prec, rm);
        println!("Float::from_float_prec_round_ref(&{x}, {prec}, {rm}) = ({y}, {o:?})");
    }
}

fn demo_float_from_float_prec_round_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (y, o) = Float::from_float_prec_round_ref(&x, prec, rm);
        println!("Float::from_float_prec_round_ref(&{x:#x}, {prec}, {rm}) = ({y:#x}, {o:?})");
    }
}

fn demo_float_from_float_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (y, o) = Float::from_float_prec(x.clone(), prec);
        println!("Float::from_float_prec({x}, {prec}) = ({y}, {o:?})");
    }
}

fn demo_float_from_float_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (y, o) = Float::from_float_prec(x.clone(), prec);
        println!("Float::from_float_prec({x:#x}, {prec}) = ({y:#x}, {o:?})");
    }
}

fn demo_float_from_float_prec_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (y, o) = Float::from_float_prec_ref(&x, prec);
        println!("Float::from_float_prec_ref(&{x}, {prec}) = ({y}, {o:?})");
    }
}

fn demo_float_from_float_prec_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, prec) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let (y, o) = Float::from_float_prec_ref(&x, prec);
        println!("Float::from_float_prec_ref(&{x:#x}, {prec}) = ({y:#x}, {o:?})");
    }
}

fn benchmark_float_to_significand_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.to_significand()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("Float.to_significand()", &mut |x| {
                no_out!(x.to_significand());
            }),
            ("Float.into_significand()", &mut |x| {
                no_out!(x.into_significand());
            }),
            ("Float.significand_ref()", &mut |x| {
                no_out!(x.significand_ref());
            }),
        ],
    );
}

fn benchmark_float_significand_ref_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.significand_ref()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.significand_ref())),
            ("rug", &mut |(x, _)| no_out!(x.get_significand())),
        ],
    );
}

fn benchmark_float_get_exponent_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.get_exponent()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.get_exponent())),
            ("rug", &mut |(x, _)| no_out!(x.get_exp())),
        ],
    );
}

fn benchmark_float_get_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.get_prec()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.get_prec())),
            ("rug", &mut |(x, _)| no_out!(x.prec())),
        ],
    );
}

fn benchmark_float_get_min_prec(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.get_min_prec()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.get_min_prec()))],
    );
}

fn benchmark_float_set_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.set_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        float_unsigned_rounding_mode_triple_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_triple_1_float_complexity_triple_2_bucketer("x", "precision"),
        &mut [
            ("Malachite", &mut |(_, (mut x, prec, rm))| {
                no_out!(x.set_prec_round(prec, rm));
            }),
            ("rug", &mut |((mut x, prec, rm), _)| {
                no_out!(x.set_prec_round(u32::exact_from(prec), rm));
            }),
        ],
    );
}

fn benchmark_float_set_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.set_prec_round(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_rounding_mode_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_triple_1_float_complexity_triple_2_bucketer("x", "precision"),
        &mut [
            ("Float::set_prec_round", &mut |(mut x, prec, rm)| {
                no_out!(x.set_prec_round(prec, rm));
            }),
            ("Float::from_float_prec_round", &mut |(x, prec, rm)| {
                no_out!(Float::from_float_prec_round(x, prec, rm));
            }),
            ("Float::from_float_prec_round_ref", &mut |(x, prec, rm)| {
                no_out!(Float::from_float_prec_round_ref(&x, prec, rm));
            }),
            ("clone and Float::set_prec_round", &mut |(x, prec, rm)| {
                let mut x = x.clone();
                no_out!(x.set_prec_round(prec, rm));
            }),
        ],
    );
}

fn benchmark_float_set_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.set_prec(u64)",
        BenchmarkType::LibraryComparison,
        float_unsigned_pair_gen_var_1_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_pair_1_complexity_pair_2_bucketer("x", "precision"),
        &mut [
            ("Malachite", &mut |(_, (mut x, prec))| {
                no_out!(x.set_prec(prec));
            }),
            ("rug", &mut |((mut x, prec), _)| {
                no_out!(x.set_prec(u32::exact_from(prec)));
            }),
        ],
    );
}

fn benchmark_float_set_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.set_prec(u64)",
        BenchmarkType::EvaluationStrategy,
        float_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_pair_1_complexity_pair_2_bucketer("x", "precision"),
        &mut [
            ("Float::set_prec", &mut |(mut x, prec)| {
                no_out!(x.set_prec(prec));
            }),
            ("Float::from_float_prec", &mut |(x, prec)| {
                no_out!(Float::from_float_prec(x, prec));
            }),
            ("Float::from_float_prec_ref", &mut |(x, prec)| {
                no_out!(Float::from_float_prec_ref(&x, prec));
            }),
            ("clone and Float::set_prec", &mut |(x, prec)| {
                let mut x = x.clone();
                no_out!(x.set_prec(prec));
            }),
        ],
    );
}
