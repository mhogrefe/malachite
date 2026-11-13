// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::big_exponent_float::arithmetic::add_mul::add_mul_prec_round_generic;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_big_exponent_float_unsigned_quadruple_gen_var_1 as quadruple_gen_var_1,
    big_exponent_float_big_exponent_float_big_exponent_float_unsigned_rounding_mode_quintuple_gen_var_1 as quintuple_gen_var_1,
    big_exponent_float_big_exponent_float_big_exponent_float_unsigned_rounding_mode_quintuple_gen_var_1_float as quintuple_gen_var_1_float,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_add_mul);
    register_demo!(runner, demo_big_exponent_float_add_mul_prec_round);
    register_demo!(runner, demo_big_exponent_float_add_mul_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_add_mul_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_add_mul_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_add_mul_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_add_mul_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_add_mul_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in big_exponent_float_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{x:#x}.add_mul({y:#x}, {z:#x}) = {:#x}",
            (&x).add_mul(&y, &z)
        );
    }
}

fn demo_big_exponent_float_add_mul_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec, rm) in quintuple_gen_var_1().get(gm, config).take(limit) {
        let (r, o) = x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm);
        println!("{x:#x}.add_mul_prec_round({y:#x}, {z:#x}, {prec}, {rm}) = ({r:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_add_mul_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, prec) in quadruple_gen_var_1().get(gm, config).take(limit) {
        let (r, o) = x.add_mul_prec_ref_ref_ref(&y, &z, prec);
        println!("{x:#x}.add_mul_prec({y:#x}, {z:#x}, {prec}) = ({r:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_add_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.add_mul(BigExponentFloat, BigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_big_exponent_float_max_complexity_bucketer(),
        &mut [
            (
                "BigExponentFloat.add_mul(BigExponentFloat, BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!(x.add_mul(y, z));
                },
            ),
            (
                "BigExponentFloat.add_mul(BigExponentFloat, &BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!(x.add_mul(y, &z));
                },
            ),
            (
                "BigExponentFloat.add_mul(&BigExponentFloat, BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!(x.add_mul(&y, z));
                },
            ),
            (
                "BigExponentFloat.add_mul(&BigExponentFloat, &BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!(x.add_mul(&y, &z));
                },
            ),
            (
                "(&BigExponentFloat).add_mul(BigExponentFloat, BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!((&x).add_mul(y, z));
                },
            ),
            (
                "(&BigExponentFloat).add_mul(BigExponentFloat, &BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!((&x).add_mul(y, &z));
                },
            ),
            (
                "(&BigExponentFloat).add_mul(&BigExponentFloat, BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!((&x).add_mul(&y, z));
                },
            ),
            (
                "(&BigExponentFloat).add_mul(&BigExponentFloat, &BigExponentFloat)",
                &mut |(x, y, z)| {
                    no_out!((&x).add_mul(&y, &z));
                },
            ),
            (
                "BigExponentFloat.add_mul_assign(BigExponentFloat, BigExponentFloat)",
                &mut |(mut x, y, z)| {
                    x.add_mul_assign(y, z);
                },
            ),
            (
                "BigExponentFloat.add_mul_assign(BigExponentFloat, &BigExponentFloat)",
                &mut |(mut x, y, z)| {
                    x.add_mul_assign(y, &z);
                },
            ),
            (
                "BigExponentFloat.add_mul_assign(&BigExponentFloat, BigExponentFloat)",
                &mut |(mut x, y, z)| {
                    x.add_mul_assign(&y, z);
                },
            ),
            (
                "BigExponentFloat.add_mul_assign(&BigExponentFloat, &BigExponentFloat)",
                &mut |(mut x, y, z)| {
                    x.add_mul_assign(&y, &z);
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_add_mul_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.add_mul_prec_round(BigExponentFloat, BigExponentFloat, u64, \
        RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        quintuple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_4_big_exponent_float_max_complexity_bucketer("x", "y", "z", "prec"),
        &mut [
            (
                "BigExponentFloat.add_mul_prec_round(BigExponentFloat, BigExponentFloat, u64, \
                RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round(y, z, prec, rm));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_round_val_val_ref(BigExponentFloat, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_val_val_ref(y, &z, prec, rm));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_round_val_ref_val(&BigExponentFloat, \
                BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_val_ref_val(&y, z, prec, rm));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_round_val_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_val_ref_ref(&y, &z, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_round_ref_val_val(BigExponentFloat, \
                BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_ref_val_val(y, z, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_round_ref_val_ref(BigExponentFloat, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_ref_val_ref(y, &z, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_round_ref_ref_val(&BigExponentFloat, \
                BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_ref_ref_val(&y, z, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_round_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_round_assign(BigExponentFloat, BigExponentFloat, \
                u64, RoundingMode)",
                &mut |(mut x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_assign(y, z, prec, rm));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_round_assign_val_ref(BigExponentFloat, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_assign_val_ref(y, &z, prec, rm));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_round_assign_ref_val(&BigExponentFloat, \
                BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_assign_ref_val(&y, z, prec, rm));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_round_assign_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, z, prec, rm)| {
                    no_out!(x.add_mul_prec_round_assign_ref_ref(&y, &z, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_add_mul_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.add_mul_prec(BigExponentFloat, BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_4_big_exponent_float_max_complexity_bucketer("x", "y", "z", "prec"),
        &mut [
            (
                "BigExponentFloat.add_mul_prec(BigExponentFloat, BigExponentFloat, u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec(y, z, prec));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_val_val_ref(BigExponentFloat, &BigExponentFloat, \
                u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec_val_val_ref(y, &z, prec));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_val_ref_val(&BigExponentFloat, BigExponentFloat, \
                u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec_val_ref_val(&y, z, prec));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_val_ref_ref(&BigExponentFloat, &BigExponentFloat, \
                u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec_val_ref_ref(&y, &z, prec));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_ref_val_val(BigExponentFloat, BigExponentFloat, \
                u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec_ref_val_val(y, z, prec));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_ref_val_ref(BigExponentFloat, \
                &BigExponentFloat, u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec_ref_val_ref(y, &z, prec));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_ref_ref_val(&BigExponentFloat, \
                BigExponentFloat, u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec_ref_ref_val(&y, z, prec));
                },
            ),
            (
                "(&BigExponentFloat).add_mul_prec_ref_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, u64)",
                &mut |(x, y, z, prec)| {
                    no_out!(x.add_mul_prec_ref_ref_ref(&y, &z, prec));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_assign(BigExponentFloat, BigExponentFloat, u64)",
                &mut |(mut x, y, z, prec)| {
                    no_out!(x.add_mul_prec_assign(y, z, prec));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_assign_val_ref(BigExponentFloat, \
                &BigExponentFloat, u64)",
                &mut |(mut x, y, z, prec)| {
                    no_out!(x.add_mul_prec_assign_val_ref(y, &z, prec));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_assign_ref_val(&BigExponentFloat, \
                BigExponentFloat, u64)",
                &mut |(mut x, y, z, prec)| {
                    no_out!(x.add_mul_prec_assign_ref_val(&y, z, prec));
                },
            ),
            (
                "BigExponentFloat.add_mul_prec_assign_ref_ref(&BigExponentFloat, \
                &BigExponentFloat, u64)",
                &mut |(mut x, y, z, prec)| {
                    no_out!(x.add_mul_prec_assign_ref_ref(&y, &z, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_add_mul_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.add_mul_prec_round(BigExponentFloat, BigExponentFloat, u64, \
        RoundingMode)",
        BenchmarkType::Algorithms,
        quintuple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_2_3_4_big_exponent_float_max_complexity_bucketer("x", "y", "z", "prec"),
        &mut [
            ("Float kernels", &mut |(x, y, z, prec, rm)| {
                no_out!(x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm));
            }),
            ("generic", &mut |(x, y, z, prec, rm)| {
                no_out!(add_mul_prec_round_generic(&x, &y, &z, false, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_add_mul_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.add_mul_prec_round(BigExponentFloat, BigExponentFloat, u64, \
        RoundingMode)",
        BenchmarkType::LibraryComparison,
        quintuple_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quintuple_1_2_3_4_big_exponent_float_max_complexity_bucketer("x", "y", "z", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, z, prec, rm))| {
                no_out!(x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm));
            }),
            ("Float", &mut |((x, y, z, prec, rm), _)| {
                no_out!(x.add_mul_prec_round_ref_ref_ref(&y, &z, prec, rm));
            }),
        ],
    );
}
