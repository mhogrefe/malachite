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
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::trigonometric::atan::atan2_prec_round_generic;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_q::test_util::bench::bucketers::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_atan2_prec_round);
    register_demo!(runner, demo_big_exponent_float_atan2_prec_round_assign);
    register_demo!(runner, demo_big_exponent_float_atan2_prec_round_debug);
    register_demo!(runner, demo_big_exponent_float_atan2_rational_prec_round);
    register_demo!(
        runner,
        demo_big_exponent_float_atan2_rational_prec_round_debug
    );
    register_demo!(runner, demo_big_exponent_float_atan2_prec);
    register_demo!(runner, demo_big_exponent_float_atan2_prec_debug);
    register_demo!(runner, demo_big_exponent_float_atan2_rational_prec);
    register_demo!(runner, demo_big_exponent_float_atan2_rational_prec_debug);
    register_demo!(runner, demo_big_exponent_float_atan2_with_period_prec_round);
    register_demo!(
        runner,
        demo_big_exponent_float_atan2_with_period_prec_round_debug
    );
    register_demo!(
        runner,
        demo_big_exponent_float_atan2_with_period_rational_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_atan2_with_period_rational_prec_round_debug
    );
    register_demo!(runner, demo_big_exponent_float_atan2_with_period_prec);
    register_demo!(runner, demo_big_exponent_float_atan2_with_period_prec_debug);
    register_demo!(
        runner,
        demo_big_exponent_float_atan2_with_period_rational_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_atan2_with_period_rational_prec_debug
    );
    register_demo!(runner, demo_big_exponent_float_atan2_pi_prec_round);
    register_demo!(runner, demo_big_exponent_float_atan2_pi_prec_round_debug);
    register_demo!(runner, demo_big_exponent_float_atan2_pi_rational_prec_round);
    register_demo!(
        runner,
        demo_big_exponent_float_atan2_pi_rational_prec_round_debug
    );
    register_demo!(runner, demo_big_exponent_float_atan2_pi_prec);
    register_demo!(runner, demo_big_exponent_float_atan2_pi_prec_debug);
    register_demo!(runner, demo_big_exponent_float_atan2_pi_rational_prec);
    register_demo!(runner, demo_big_exponent_float_atan2_pi_rational_prec_debug);

    register_bench!(
        runner,
        benchmark_big_exponent_float_atan2_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_atan2_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_atan2_with_period_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_atan2_with_period_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_atan2_pi_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_atan2_pi_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_atan2_prec_round_algorithms
    );
}

fn demo_big_exponent_float_atan2_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = y.atan2_prec_round_ref_ref(&x, prec, rm);
        println!("({y}).atan2_prec_round({x}, {prec}, {rm}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = y.atan2_prec_round_ref_ref(&x, prec, rm);
        println!("({y:#x}).atan2_prec_round({x:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(&y, &x, prec, rm);
        println!(
            "BigExponentFloat::atan2_rational_prec_round({y}, {x}, {prec}, {rm}) = \
            ({z}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_rational_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(&y, &x, prec, rm);
        println!(
            "BigExponentFloat::atan2_rational_prec_round({y}, {x}, {prec}, {rm}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = y.atan2_prec_ref_ref(&x, prec);
        println!("({y}).atan2_prec({x}, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = y.atan2_prec_ref_ref(&x, prec);
        println!("({y:#x}).atan2_prec({x:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_rational_prec_ref_ref(&y, &x, prec);
        println!("BigExponentFloat::atan2_rational_prec({y}, {x}, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_rational_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_rational_prec_ref_ref(&y, &x, prec);
        println!("BigExponentFloat::atan2_rational_prec({y}, {x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_with_period_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = y.atan2_with_period_prec_round_ref_ref(&x, 360, prec, rm);
        println!(
            "({y}).atan2_with_period_prec_round({x}, 360, {prec}, {rm}) = \
            ({z}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_with_period_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = y.atan2_with_period_prec_round_ref_ref(&x, 360, prec, rm);
        println!(
            "({y:#x}).atan2_with_period_prec_round({x:#x}, 360, {prec}, {rm}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_with_period_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) =
            BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(&y, &x, 360, prec, rm);
        println!(
            "BigExponentFloat::atan2_with_period_rational_prec_round(\
            {y}, {x}, 360, {prec}, {rm}) = ({z}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_with_period_rational_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) =
            BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(&y, &x, 360, prec, rm);
        println!(
            "BigExponentFloat::atan2_with_period_rational_prec_round(\
            {y}, {x}, 360, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_with_period_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = y.atan2_with_period_prec_ref_ref(&x, 360, prec);
        println!("({y}).atan2_with_period_prec({x}, 360, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_with_period_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = y.atan2_with_period_prec_ref_ref(&x, 360, prec);
        println!("({y:#x}).atan2_with_period_prec({x:#x}, 360, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_with_period_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_with_period_rational_prec_ref_ref(&y, &x, 360, prec);
        println!(
            "BigExponentFloat::atan2_with_period_rational_prec({y}, {x}, 360, {prec}) = \
            ({z}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_with_period_rational_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_with_period_rational_prec_ref_ref(&y, &x, 360, prec);
        println!(
            "BigExponentFloat::atan2_with_period_rational_prec({y}, {x}, 360, {prec}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_pi_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = y.atan2_pi_prec_round_ref_ref(&x, prec, rm);
        println!("({y}).atan2_pi_prec_round({x}, {prec}, {rm}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_pi_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = y.atan2_pi_prec_round_ref_ref(&x, prec, rm);
        println!("({y:#x}).atan2_pi_prec_round({x:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_pi_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(&y, &x, prec, rm);
        println!(
            "BigExponentFloat::atan2_pi_rational_prec_round({y}, {x}, {prec}, {rm}) = \
            ({z}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_pi_rational_prec_round_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(&y, &x, prec, rm);
        println!(
            "BigExponentFloat::atan2_pi_rational_prec_round({y}, {x}, {prec}, {rm}) = \
            ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_atan2_pi_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = y.atan2_pi_prec_ref_ref(&x, prec);
        println!("({y}).atan2_pi_prec({x}, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_pi_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = y.atan2_pi_prec_ref_ref(&x, prec);
        println!("({y:#x}).atan2_pi_prec({x:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_pi_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_pi_rational_prec_ref_ref(&y, &x, prec);
        println!("BigExponentFloat::atan2_pi_rational_prec({y}, {x}, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_pi_rational_prec_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, _) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::atan2_pi_rational_prec_ref_ref(&y, &x, prec);
        println!("BigExponentFloat::atan2_pi_rational_prec({y}, {x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_atan2_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.atan2_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.atan2_prec_round_assign({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}


fn benchmark_big_exponent_float_atan2_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.atan2_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat.atan2_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_prec_round(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_prec_round_val_ref(&x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_prec_round_ref_val(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_prec_round_ref_ref(&x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut y, x, prec, rm)| {
                    no_out!(y.atan2_prec_round_assign(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut y, x, prec, rm)| {
                    no_out!(y.atan2_prec_round_assign_ref(&x, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_atan2_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::atan2_rational_prec_round(Rational, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat::atan2_rational_prec_round(Rational, Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_rational_prec_round(y, x, prec, rm));
                },
            ),
            (
                "BigExponentFloat::atan2_rational_prec_round_val_ref(Rational, &Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_rational_prec_round_val_ref(
                        y, &x, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::atan2_rational_prec_round_ref_val(&Rational, Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_rational_prec_round_ref_val(
                        &y, x, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::atan2_rational_prec_round_ref_ref(&Rational, &Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_rational_prec_round_ref_ref(
                        &y, &x, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_atan2_with_period_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.atan2_with_period_prec_round(BigExponentFloat, u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat.atan2_with_period_prec_round(BigExponentFloat, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_with_period_prec_round(x, 360, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_with_period_prec_round_val_ref(&BigExponentFloat, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_with_period_prec_round_val_ref(&x, 360, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_with_period_prec_round_ref_val(BigExponentFloat, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_with_period_prec_round_ref_val(x, 360, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_with_period_prec_round_ref_ref(&BigExponentFloat, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_with_period_prec_round_ref_ref(&x, 360, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_with_period_prec_round_assign(BigExponentFloat, u64, u64, RoundingMode)",
                &mut |(mut y, x, prec, rm)| {
                    no_out!(y.atan2_with_period_prec_round_assign(x, 360, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_with_period_prec_round_assign_ref(&BigExponentFloat, u64, u64, RoundingMode)",
                &mut |(mut y, x, prec, rm)| {
                    no_out!(y.atan2_with_period_prec_round_assign_ref(&x, 360, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_atan2_with_period_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::atan2_with_period_rational_prec_round(Rational, Rational, u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat::atan2_with_period_rational_prec_round(Rational, Rational, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_with_period_rational_prec_round(
                        y, x, 360, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(Rational, &Rational, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(
                            y, &x, 360, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(&Rational, Rational, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(
                            &y, x, 360, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(&Rational, &Rational, u64, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(
                            &y, &x, 360, prec, rm
                        )
                    );
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_atan2_pi_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.atan2_pi_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat.atan2_pi_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_pi_prec_round(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_pi_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_pi_prec_round_val_ref(&x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_pi_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_pi_prec_round_ref_val(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_pi_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(y.atan2_pi_prec_round_ref_ref(&x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_pi_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut y, x, prec, rm)| {
                    no_out!(y.atan2_pi_prec_round_assign(x, prec, rm));
                },
            ),
            (
                "BigExponentFloat.atan2_pi_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut y, x, prec, rm)| {
                    no_out!(y.atan2_pi_prec_round_assign_ref(&x, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_atan2_pi_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::atan2_pi_rational_prec_round(Rational, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer("y", "x", "prec"),
        &mut [
            (
                "BigExponentFloat::atan2_pi_rational_prec_round(Rational, Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_pi_rational_prec_round(
                        y, x, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::atan2_pi_rational_prec_round_val_ref(Rational, &Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_pi_rational_prec_round_val_ref(
                        y, &x, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::atan2_pi_rational_prec_round_ref_val(&Rational, Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_pi_rational_prec_round_ref_val(
                        &y, x, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(&Rational, &Rational, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(
                        &y, &x, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_atan2_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.atan2_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_17()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("y", "x", "prec"),
        &mut [
            ("Float functions", &mut |(y, x, prec, rm)| {
                no_out!(y.atan2_prec_round_ref_ref(&x, prec, rm));
            }),
            ("generic", &mut |(y, x, prec, rm)| {
                no_out!(atan2_prec_round_generic(&y, &x, None, prec, rm));
            }),
        ],
    );
}
