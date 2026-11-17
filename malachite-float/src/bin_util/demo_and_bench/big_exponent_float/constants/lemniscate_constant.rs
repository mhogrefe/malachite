// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::{pair_1_bucketer, unsigned_direct_bucketer};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_base::test_util::runner::Runner;
use malachite_float::{BigExponentFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_big_exponent_float_lemniscate_constant_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_lemniscate_constant_prec_round_debug
    );
    register_demo!(runner, demo_big_exponent_float_lemniscate_constant_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_lemniscate_constant_prec_debug
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_lemniscate_constant_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_lemniscate_constant_prec_float_comparison
    );
}

fn demo_big_exponent_float_lemniscate_constant_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::lemniscate_constant_prec_round(p, rm);
        println!("BigExponentFloat::lemniscate_constant_prec_round({p}, {rm}) = ({x}, {o:?})");
    }
}

fn demo_big_exponent_float_lemniscate_constant_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, rm) in unsigned_rounding_mode_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (x, o) = BigExponentFloat::lemniscate_constant_prec_round(p, rm);
        println!("BigExponentFloat::lemniscate_constant_prec_round({p}, {rm}) = ({x:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_lemniscate_constant_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::lemniscate_constant_prec_round(u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        unsigned_rounding_mode_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bucketer("prec"),
        &mut [
            ("BigExponentFloat", &mut |(p, rm)| {
                no_out!(BigExponentFloat::lemniscate_constant_prec_round(p, rm));
            }),
            ("Float", &mut |(p, rm)| {
                no_out!(Float::lemniscate_constant_prec_round(p, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_lemniscate_constant_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        let (x, o) = BigExponentFloat::lemniscate_constant_prec(p);
        println!("BigExponentFloat::lemniscate_constant_prec({p}) = ({x}, {o:?})");
    }
}

fn demo_big_exponent_float_lemniscate_constant_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in unsigned_gen_var_11().get(gm, config).take(limit) {
        let (x, o) = BigExponentFloat::lemniscate_constant_prec(p);
        println!("BigExponentFloat::lemniscate_constant_prec({p}) = ({x:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_lemniscate_constant_prec_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::lemniscate_constant_prec(u64)",
        BenchmarkType::LibraryComparison,
        unsigned_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [
            ("BigExponentFloat", &mut |p| {
                no_out!(BigExponentFloat::lemniscate_constant_prec(p));
            }),
            ("Float", &mut |p| {
                no_out!(Float::lemniscate_constant_prec(p));
            }),
        ],
    );
}
