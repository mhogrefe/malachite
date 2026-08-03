// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulShrRound, MulShrRoundAssign, ShrRound};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_2_natural_sum_bit_bucketer;
use malachite_nz::test_util::generators::{
    natural_natural_unsigned_rounding_mode_quadruple_gen_var_1,
    natural_natural_unsigned_rounding_mode_quadruple_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mul_shr_round);
    register_demo!(runner, demo_natural_mul_shr_round_ref_ref);
    register_demo!(runner, demo_natural_mul_shr_round_assign);

    register_bench!(runner, benchmark_natural_mul_shr_round_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mul_shr_round_algorithms);
}

fn demo_natural_mul_shr_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, bits, rm) in natural_natural_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({x_old}).mul_shr_round({y_old}, {bits}, {rm}) = {:?}",
            x.mul_shr_round(y, bits, rm)
        );
    }
}

fn demo_natural_mul_shr_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, bits, rm) in natural_natural_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mul_shr_round(&{y}, {bits}, {rm}) = {:?}",
            (&x).mul_shr_round(&y, bits, rm)
        );
    }
}

fn demo_natural_mul_shr_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, bits, rm) in natural_natural_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.mul_shr_round_assign(y, bits, rm);
        println!("x := {x_old}; x.mul_shr_round_assign({y_old}, {bits}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_natural_mul_shr_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mul_shr_round(Natural, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_natural_sum_bit_bucketer("x", "y"),
        &mut [
            (
                "Natural.mul_shr_round(Natural, u64, RoundingMode)",
                &mut |(x, y, bits, rm)| no_out!(x.mul_shr_round(y, bits, rm)),
            ),
            (
                "(&Natural).mul_shr_round(&Natural, u64, RoundingMode)",
                &mut |(x, y, bits, rm)| no_out!((&x).mul_shr_round(&y, bits, rm)),
            ),
        ],
    );
}

// The payoff benchmark: the fused operation against the full product followed by the shift, on
// inputs whose cut is near the top of the product.
#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_natural_mul_shr_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mul_shr_round(Natural, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        natural_natural_unsigned_rounding_mode_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_natural_sum_bit_bucketer("x", "y"),
        &mut [
            (
                "Natural.mul_shr_round(Natural, u64, RoundingMode)",
                &mut |(x, y, bits, rm)| no_out!((&x).mul_shr_round(&y, bits, rm)),
            ),
            (
                "(Natural * Natural).shr_round(u64, RoundingMode)",
                &mut |(x, y, bits, rm)| {
                    no_out!((&x * &y).shr_round(bits, rm));
                },
            ),
        ],
    );
}
