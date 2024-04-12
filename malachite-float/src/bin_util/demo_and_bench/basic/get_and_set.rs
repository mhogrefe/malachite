// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
    pair_2_max_pair_1_complexity_pair_2_bucketer,
    pair_2_max_triple_1_float_complexity_triple_2_bucketer,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_rm, float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_1_rm,
    float_unsigned_rounding_mode_triple_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_1_rm,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_to_significand);
    register_demo!(runner, demo_float_to_significand_debug);
    register_demo!(runner, demo_float_into_significand);
    register_demo!(runner, demo_float_into_significand_debug);
    register_demo!(runner, demo_float_significand_ref);
    register_demo!(runner, demo_float_significand_ref_debug);
    register_demo!(runner, demo_float_get_exponent);
    register_demo!(runner, demo_float_get_exponent_debug);
    register_demo!(runner, demo_float_get_prec);
    register_demo!(runner, demo_float_get_prec_debug);
    register_demo!(runner, demo_float_get_min_prec);
    register_demo!(runner, demo_float_get_min_prec_debug);
    register_demo!(runner, demo_float_set_prec_round);
    register_demo!(runner, demo_float_set_prec_round_debug);
    register_demo!(runner, demo_float_set_prec);
    register_demo!(runner, demo_float_set_prec_debug);

    register_bench!(runner, benchmark_float_to_significand_evaluation_strategy);
    register_bench!(runner, benchmark_float_significand_ref_library_comparison);
    register_bench!(runner, benchmark_float_get_exponent_library_comparison);
    register_bench!(runner, benchmark_float_get_prec_library_comparison);
    register_bench!(runner, benchmark_float_get_min_prec);
    register_bench!(runner, benchmark_float_set_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_set_prec_library_comparison);
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

fn demo_float_set_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, p, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.set_prec_round(p, rm);
        println!("x := {old_x}; x.set_prec_round({p}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_float_set_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, p, rm) in float_unsigned_rounding_mode_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x.clone();
        let o = x.set_prec_round(p, rm);
        println!(
            "x := {:#x}; x.set_prec_round({}, {}) = {:?}; x = {:#x}",
            ComparableFloat(old_x),
            p,
            rm,
            o,
            ComparableFloat(x)
        );
    }
}

fn demo_float_set_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, p) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let old_x = x.clone();
        let o = x.set_prec(p);
        println!("x := {old_x}; x.set_prec({p}) = {o:?}; x = {x}");
    }
}

fn demo_float_set_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, p) in float_unsigned_pair_gen_var_1().get(gm, config).take(limit) {
        let old_x = x.clone();
        let o = x.set_prec(p);
        println!(
            "x := {:#x}; x.set_prec({}) = {:?}; x = {:#x}",
            ComparableFloat(old_x),
            p,
            o,
            ComparableFloat(x)
        );
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
                no_out!(x.to_significand())
            }),
            ("Float.into_significand()", &mut |x| {
                no_out!(x.into_significand())
            }),
            ("Float.significand_ref()", &mut |x| {
                no_out!(x.significand_ref())
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
            ("Malachite", &mut |(_, (mut x, p, rm))| {
                no_out!(x.set_prec_round(p, rm))
            }),
            ("rug", &mut |((mut x, p, rm), _)| {
                no_out!(x.set_prec_round(u32::exact_from(p), rm))
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
            ("Malachite", &mut |(_, (mut x, p))| no_out!(x.set_prec(p))),
            ("rug", &mut |((mut x, p), _)| {
                no_out!(x.set_prec(u32::exact_from(p)))
            }),
        ],
    );
}
