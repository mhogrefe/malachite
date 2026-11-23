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
use malachite_float::big_exponent_float::arithmetic::hypot::hypot_prec_round_generic;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_hypot_prec_round);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_round_ref_ref);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_round_val_ref);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_round_ref_val);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_round_assign);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_round_assign_ref);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_round_debug);
    register_demo!(
        runner,
        demo_big_exponent_float_hypot_prec_round_ref_ref_debug
    );
    register_demo!(
        runner,
        demo_big_exponent_float_hypot_prec_round_val_ref_debug
    );
    register_demo!(
        runner,
        demo_big_exponent_float_hypot_prec_round_ref_val_debug
    );
    register_demo!(
        runner,
        demo_big_exponent_float_hypot_prec_round_assign_debug
    );
    register_demo!(
        runner,
        demo_big_exponent_float_hypot_prec_round_assign_ref_debug
    );
    register_demo!(runner, demo_big_exponent_float_hypot_prec);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_ref_ref);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_val_ref);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_ref_val);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_assign);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_assign_ref);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_debug);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_ref_ref_debug);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_val_ref_debug);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_ref_val_debug);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_assign_debug);
    register_demo!(runner, demo_big_exponent_float_hypot_prec_assign_ref_debug);

    register_bench!(
        runner,
        benchmark_big_exponent_float_hypot_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_hypot_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_hypot_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_hypot_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_hypot_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.hypot_prec_round_ref_ref(&y, prec, rm);
        println!("({x}).hypot_prec_round({y}, {prec}, {rm}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_hypot_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.hypot_prec_round_ref_ref(&y, prec, rm);
        println!("({x:#x}).hypot_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_hypot_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.hypot_prec_ref_ref(&y, prec);
        println!("({x}).hypot_prec({y}, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_hypot_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.hypot_prec_ref_ref(&y, prec);
        println!("({x:#x}).hypot_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_hypot_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).hypot_prec_ref_ref(&{y}, {prec}) = {:?}",
            x.hypot_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_big_exponent_float_hypot_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (hypot, o) = x.hypot_prec_ref_ref(&y, prec);
        println!(
            "(&{:#x}).hypot_prec_ref_ref(&{:#x}, {prec}) = ({hypot:#x}, {o:?})",
            &x, &y
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&{x}).hypot_prec_round_ref_ref(&{y}, {prec}, {rm}) = {:?}",
            x.hypot_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let (hypot, o) = x.hypot_prec_round_ref_ref(&y, prec, rm);
        println!(
            "(&{:#x}).hypot_prec_round_ref_ref(&{:#x}, {prec}, {rm}) = ({hypot:#x}, {o:?})",
            &x, &y
        );
    }
}

fn demo_big_exponent_float_hypot_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({x_old}).hypot_prec_val_ref(&{y}, {prec}) = {:?}",
            x.hypot_prec_val_ref(&y, prec)
        );
    }
}

fn demo_big_exponent_float_hypot_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (hypot, o) = x.hypot_prec_val_ref(&y, prec);
        println!(
            "({x_old:#x}).hypot_prec_val_ref(&{:#x}, {prec}) = ({hypot:#x}, {o:?})",
            &y
        );
    }
}

fn demo_big_exponent_float_hypot_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{x}).hypot_prec_ref_val({y_old}, {prec}) = {:?}",
            x.hypot_prec_ref_val(y, prec)
        );
    }
}

fn demo_big_exponent_float_hypot_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec_ref_val(y, prec);
        println!(
            "(&{:#x}).hypot_prec_ref_val({y_old:#x}, {prec}) = ({hypot:#x}, {o:?})",
            &x
        );
    }
}

fn demo_big_exponent_float_hypot_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_assign(y, prec);
        println!("x := {x_old}; x.hypot_prec_assign({y_old}, {prec}) = {o:?}; x = {x}");
    }
}

fn demo_big_exponent_float_hypot_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_assign(y, prec);
        println!("x := {x_old:#x}; x.hypot_prec_assign({y_old:#x}, {prec}) = {o:?}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_hypot_prec_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_assign_ref(&y, prec);
        println!("x := {x_old}; x.hypot_prec_assign_ref(&{y}, {prec}) = {o:?}; x = {x}");
    }
}

fn demo_big_exponent_float_hypot_prec_assign_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_assign_ref(&y, prec);
        println!(
            "x := {x_old:#x}; x.hypot_prec_assign_ref(&{:#x}, {prec}) = {o:?}; x = {x:#x}",
            &y
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({x_old}).hypot_prec_round_val_ref(&{y}, {prec}, {rm}) = {:?}",
            x.hypot_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let (hypot, o) = x.hypot_prec_round_val_ref(&y, prec, rm);
        println!(
            "({x_old:#x}).hypot_prec_round_val_ref(&{:#x}, {prec}, {rm}) = ({hypot:#x}, {o:?})",
            &y
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{x}).hypot_prec_round_ref_val({y_old}, {prec}, {rm}) = {:?}",
            x.hypot_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let y_old = y.clone();
        let (hypot, o) = x.hypot_prec_round_ref_val(y, prec, rm);
        println!(
            "(&{:#x}).hypot_prec_round_ref_val({y_old:#x}, {prec}, {rm}) = ({hypot:#x}, {o:?})",
            &x
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.hypot_prec_round_assign({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_big_exponent_float_hypot_prec_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.hypot_prec_round_assign(y, prec, rm);
        println!(
            "x := {x_old:#x}; x.hypot_prec_round_assign({y_old:#x}, {prec}, {rm}) = {o:?}; x = \
             {x:#x}"
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_assign_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {x_old}; x.hypot_prec_round_assign_ref(&{y}, {prec}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_big_exponent_float_hypot_prec_round_assign_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let o = x.hypot_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {x_old:#x}; x.hypot_prec_round_assign_ref(&{:#x}, {prec}, {rm}) = {o:?}; x = \
             {x:#x}",
            &y
        );
    }
}

fn benchmark_big_exponent_float_hypot_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.hypot_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.hypot_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.hypot_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.hypot_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.hypot_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.hypot_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.hypot_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.hypot_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_hypot_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.hypot_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.hypot_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.hypot_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.hypot_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.hypot_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.hypot_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_assign(BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.hypot_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.hypot_prec_assign_ref(&BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.hypot_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_hypot_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.hypot_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float kernels", &mut |(x, y, prec, rm)| {
                no_out!(x.hypot_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(hypot_prec_round_generic(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_hypot_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.hypot_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_6_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.hypot_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.hypot_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}
