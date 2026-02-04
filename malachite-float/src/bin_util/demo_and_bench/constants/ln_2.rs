// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Two;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::bucketers::{pair_1_bucketer, unsigned_direct_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::ComparableFloat;
use malachite_float::Float;
use malachite_float::test_util::common::rug_round_exact_from_rounding_mode;
use malachite_float::test_util::constants::ln_2::rug_ln_2_prec_round;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_ln_2_prec_round);
    register_demo!(runner, demo_float_ln_2_prec_round_debug);
    register_demo!(runner, demo_float_ln_2_prec);
    register_demo!(runner, demo_float_ln_2_prec_debug);

    register_bench!(runner, benchmark_float_ln_2_prec_round_library_comparison);
    register_bench!(runner, benchmark_float_ln_2_prec_round_algorithms);
    register_bench!(runner, benchmark_float_ln_2_prec_library_comparison);
    register_bench!(runner, benchmark_float_ln_2_prec_algorithms);
}

fn demo_float_ln_2_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "ln_2_prec_round({}, {}) = {:?}",
            p,
            rm,
            Float::ln_2_prec_round(p, rm)
        );
    }
}

fn demo_float_ln_2_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (pc, o) = Float::ln_2_prec_round(p, rm);
        println!(
            "ln_2_prec_round({}, {}) = ({:#x}, {:?})",
            p,
            rm,
            ComparableFloat(pc),
            o
        );
    }
}

fn demo_float_ln_2_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!("ln_2_prec({}) = {:?}", p, Float::ln_2_prec(p));
    }
}

fn demo_float_ln_2_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        let (pc, o) = Float::ln_2_prec(p);
        println!("ln_2_prec({}) = ({:#x}, {:?})", p, ComparableFloat(pc), o);
    }
}

fn benchmark_float_ln_2_prec_round_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::ln_2_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        unsigned_rounding_mode_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bucketer("prec"),
        &mut [
            ("Malachite", &mut |(p, rm)| {
                no_out!(Float::ln_2_prec_round(p, rm));
            }),
            ("rug", &mut |(p, rm)| {
                no_out!(rug_ln_2_prec_round(
                    p,
                    rug_round_exact_from_rounding_mode(rm)
                ));
            }),
        ],
    );
}

fn benchmark_float_ln_2_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::ln_2_prec_round(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        unsigned_rounding_mode_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bucketer("prec"),
        &mut [
            ("default", &mut |(p, rm)| {
                no_out!(Float::ln_2_prec_round(p, rm));
            }),
            ("using ln", &mut |(p, rm)| {
                no_out!(Float::ln_prec_round(Float::TWO, p, rm));
            }),
        ],
    );
}

fn benchmark_float_ln_2_prec_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::ln_2_prec(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("Malachite", &mut |p| no_out!(Float::ln_2_prec(p))),
            ("rug", &mut |p| {
                no_out!(rug_ln_2_prec_round(
                    p,
                    rug_round_exact_from_rounding_mode(Nearest)
                ));
            }),
        ],
    );
}

fn benchmark_float_ln_2_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::ln_2_prec(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |p| no_out!(Float::ln_2_prec(p))),
            ("using ln", &mut |p| {
                no_out!(Float::ln_prec(Float::TWO, p));
            }),
        ],
    );
}
