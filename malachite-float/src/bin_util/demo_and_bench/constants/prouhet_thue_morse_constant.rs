// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

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
use malachite_float::test_util::constants::prouhet_thue_morse_constant::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_prouhet_thue_morse_constant_prec_round);
    register_demo!(
        runner,
        demo_float_prouhet_thue_morse_constant_prec_round_debug
    );
    register_demo!(runner, demo_float_prouhet_thue_morse_constant_prec);
    register_demo!(runner, demo_float_prouhet_thue_morse_constant_prec_debug);

    register_bench!(
        runner,
        benchmark_float_prouhet_thue_morse_constant_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_float_prouhet_thue_morse_constant_prec_algorithms
    );
}

fn demo_float_prouhet_thue_morse_constant_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "prouhet_thue_morse_constant_prec_round({}, {}) = {:?}",
            p,
            rm,
            Float::prouhet_thue_morse_constant_prec_round(p, rm)
        );
    }
}

fn demo_float_prouhet_thue_morse_constant_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (tmc, o) = Float::prouhet_thue_morse_constant_prec_round(p, rm);
        println!(
            "prouhet_thue_morse_constant_prec_round({}, {}) = ({:#x}, {:?})",
            p,
            rm,
            ComparableFloat(tmc),
            o
        );
    }
}

fn demo_float_prouhet_thue_morse_constant_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        println!(
            "prouhet_thue_morse_constant_prec({}) = {:?}",
            p,
            Float::prouhet_thue_morse_constant_prec(p)
        );
    }
}

fn demo_float_prouhet_thue_morse_constant_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        let (tmc, o) = Float::prouhet_thue_morse_constant_prec(p);
        println!(
            "prouhet_thue_morse_constant_prec({}) = ({:#x}, {:?})",
            p,
            ComparableFloat(tmc),
            o
        );
    }
}

fn benchmark_float_prouhet_thue_morse_constant_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::prouhet_thue_morse_constant_prec_round(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        unsigned_rounding_mode_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bucketer("prec"),
        &mut [
            ("default", &mut |(p, rm)| {
                no_out!(Float::prouhet_thue_morse_constant_prec_round(p, rm));
            }),
            ("naive", &mut |(p, rm)| {
                no_out!(prouhet_thue_morse_constant_prec_round_naive(p, rm));
            }),
        ],
    );
}

fn benchmark_float_prouhet_thue_morse_constant_prec_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float::prouhet_thue_morse_constant_prec(u64)",
        BenchmarkType::Algorithms,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("default", &mut |p| {
                no_out!(Float::prouhet_thue_morse_constant_prec(p));
            }),
            ("naive", &mut |p| {
                no_out!(prouhet_thue_morse_constant_prec_round_naive(p, Nearest));
            }),
        ],
    );
}
