// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::big_exponent_float::arithmetic::mul_add_mul::mul_add_mul_prec_round_generic;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_big_exponent_float_big_exponent_float_unsigned_quintuple_gen_var_1 as quintuple_gen_var_1,
    big_exponent_float_big_exponent_float_big_exponent_float_big_exponent_float_unsigned_rounding_mode_sextuple_gen_var_2 as sextuple_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_mul_sub_mul);
    register_demo!(runner, demo_big_exponent_float_mul_sub_mul_prec_round);
    register_demo!(runner, demo_big_exponent_float_mul_sub_mul_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_sub_mul_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_sub_mul_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_sub_mul_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_sub_mul_prec_round_algorithms
    );
}

fn demo_big_exponent_float_mul_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in big_exponent_float_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{x:#x}.mul_sub_mul({y:#x}, {z:#x}, {w:#x}) = {:#x}",
            (&x).mul_sub_mul(&y, &z, &w)
        );
    }
}

fn demo_big_exponent_float_mul_sub_mul_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w, prec, rm) in sextuple_gen_var_2().get(gm, config).take(limit) {
        let (r, o) = x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm);
        println!(
            "{x:#x}.mul_sub_mul_prec_round({y:#x}, {z:#x}, {w:#x}, {prec}, {rm}) = ({r:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_mul_sub_mul_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w, prec) in quintuple_gen_var_1().get(gm, config).take(limit) {
        let (r, o) = x.mul_sub_mul_prec_ref_ref_ref_ref(&y, &z, &w, prec);
        println!("{x:#x}.mul_sub_mul_prec({y:#x}, {z:#x}, {w:#x}, {prec}) = ({r:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_mul_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_sub_mul(BigExponentFloat, BigExponentFloat, BigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_big_exponent_float_max_complexity_bucketer(),
        &mut [
            (
                "BigExponentFloat.mul_sub_mul(BigExponentFloat, BigExponentFloat, \
                BigExponentFloat)",
                &mut |(x, y, z, w)| {
                    no_out!(x.mul_sub_mul(y, z, w));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul(&BigExponentFloat, &BigExponentFloat, \
                &BigExponentFloat)",
                &mut |(x, y, z, w)| {
                    no_out!(x.mul_sub_mul(&y, &z, &w));
                },
            ),
            (
                "(&BigExponentFloat).mul_sub_mul(&BigExponentFloat, &BigExponentFloat, \
                &BigExponentFloat)",
                &mut |(x, y, z, w)| {
                    no_out!((&x).mul_sub_mul(&y, &z, &w));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_assign(BigExponentFloat, BigExponentFloat, \
                BigExponentFloat)",
                &mut |(mut x, y, z, w)| {
                    x.mul_sub_mul_assign(y, z, w);
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_assign(&BigExponentFloat, &BigExponentFloat, \
                &BigExponentFloat)",
                &mut |(mut x, y, z, w)| {
                    x.mul_sub_mul_assign(&y, &z, &w);
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_mul_sub_mul_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_sub_mul_prec_round(BigExponentFloat, BigExponentFloat, \
        BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        sextuple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sextuple_1_2_3_4_5_big_exponent_float_max_complexity_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat.mul_sub_mul_prec_round(BigExponentFloat, BigExponentFloat, \
                BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, w, prec, rm)| {
                    no_out!(x.mul_sub_mul_prec_round(y, z, w, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_prec_round_val_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, &BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, w, prec, rm)| {
                    no_out!(x.mul_sub_mul_prec_round_val_ref_ref_ref(&y, &z, &w, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).mul_sub_mul_prec_round_ref_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, &BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, w, prec, rm)| {
                    no_out!(x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_prec_round_assign(BigExponentFloat, \
                BigExponentFloat, BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, z, w, prec, rm)| {
                    no_out!(x.mul_sub_mul_prec_round_assign(y, z, w, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_prec_round_assign_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, &BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, z, w, prec, rm)| {
                    no_out!(x.mul_sub_mul_prec_round_assign_ref_ref_ref(&y, &z, &w, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_mul_sub_mul_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_sub_mul_prec(BigExponentFloat, BigExponentFloat, BigExponentFloat, \
        u64)",
        BenchmarkType::EvaluationStrategy,
        quintuple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_4_5_big_exponent_float_max_complexity_bucketer("prec"),
        &mut [
            (
                "BigExponentFloat.mul_sub_mul_prec(BigExponentFloat, BigExponentFloat, \
                BigExponentFloat, u64)",
                &mut |(x, y, z, w, prec)| {
                    no_out!(x.mul_sub_mul_prec(y, z, w, prec));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_prec_val_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, &BigExponentFloat, u64)",
                &mut |(x, y, z, w, prec)| {
                    no_out!(x.mul_sub_mul_prec_val_ref_ref_ref(&y, &z, &w, prec));
                },
            ),
            (
                "(&BigExponentFloat).mul_sub_mul_prec_ref_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, &BigExponentFloat, u64)",
                &mut |(x, y, z, w, prec)| {
                    no_out!(x.mul_sub_mul_prec_ref_ref_ref_ref(&y, &z, &w, prec));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_prec_assign(BigExponentFloat, BigExponentFloat, \
                BigExponentFloat, u64)",
                &mut |(mut x, y, z, w, prec)| {
                    no_out!(x.mul_sub_mul_prec_assign(y, z, w, prec));
                },
            ),
            (
                "BigExponentFloat.mul_sub_mul_prec_assign_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, &BigExponentFloat, u64)",
                &mut |(mut x, y, z, w, prec)| {
                    no_out!(x.mul_sub_mul_prec_assign_ref_ref_ref(&y, &z, &w, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_mul_sub_mul_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_sub_mul_prec_round(BigExponentFloat, BigExponentFloat, \
        BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        sextuple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &sextuple_1_2_3_4_5_big_exponent_float_max_complexity_bucketer("prec"),
        &mut [
            ("Float kernels", &mut |(x, y, z, w, prec, rm)| {
                no_out!(x.mul_sub_mul_prec_round_ref_ref_ref_ref(&y, &z, &w, prec, rm));
            }),
            ("generic", &mut |(x, y, z, w, prec, rm)| {
                no_out!(mul_add_mul_prec_round_generic(
                    &x, &y, &z, &w, true, prec, rm
                ));
            }),
        ],
    );
}
