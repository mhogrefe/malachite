// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ShrRoundWithOrdering, ShrRoundWithOrderingAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1;
use malachite_nz::test_util::natural::arithmetic::shr_round_with_ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_shr_round_with_ordering);
    register_demo!(runner, demo_natural_shr_round_with_ordering_ref);
    register_demo!(runner, demo_natural_shr_round_with_ordering_assign);

    register_bench!(
        runner,
        benchmark_natural_shr_round_with_ordering_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_shr_round_with_ordering_algorithms);
}

fn demo_natural_shr_round_with_ordering(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, bits, rm, o) in natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({x_old}).shr_round_with_ordering({bits}, {rm}, {o:?}) = {:?}",
            x.shr_round_with_ordering(bits, rm, o)
        );
    }
}

fn demo_natural_shr_round_with_ordering_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, bits, rm, o) in natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).shr_round_with_ordering({bits}, {rm}, {o:?}) = {:?}",
            (&x).shr_round_with_ordering(bits, rm, o)
        );
    }
}

fn demo_natural_shr_round_with_ordering_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, bits, rm, o) in natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o_out = x.shr_round_with_ordering_assign(bits, rm, o);
        println!(
            "x := {x_old}; x.shr_round_with_ordering_assign({bits}, {rm}, {o:?}) = {o_out:?}; x = {x}"
        );
    }
}

fn benchmark_natural_shr_round_with_ordering_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.shr_round_with_ordering(u64, RoundingMode, Ordering)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_bit_bucketer("x"),
        &mut [
            (
                "Natural.shr_round_with_ordering(u64, RoundingMode, Ordering)",
                &mut |(x, bits, rm, o)| {
                    no_out!(x.shr_round_with_ordering(bits, rm, o));
                },
            ),
            (
                "(&Natural).shr_round_with_ordering(u64, RoundingMode, Ordering)",
                &mut |(x, bits, rm, o)| {
                    no_out!((&x).shr_round_with_ordering(bits, rm, o));
                },
            ),
            (
                "Natural.shr_round_with_ordering_assign(u64, RoundingMode, Ordering)",
                &mut |(mut x, bits, rm, o)| {
                    no_out!(x.shr_round_with_ordering_assign(bits, rm, o));
                },
            ),
        ],
    );
}

fn benchmark_natural_shr_round_with_ordering_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.shr_round_with_ordering(u64, RoundingMode, Ordering)",
        BenchmarkType::Algorithms,
        natural_unsigned_rounding_mode_ordering_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_bit_bucketer("x"),
        &mut [
            ("default", &mut |(x, bits, rm, o)| {
                no_out!((&x).shr_round_with_ordering(bits, rm, o));
            }),
            ("naive", &mut |(x, bits, rm, o)| {
                no_out!(natural_shr_round_with_ordering_naive(&x, bits, rm, o));
            }),
        ],
    );
}
