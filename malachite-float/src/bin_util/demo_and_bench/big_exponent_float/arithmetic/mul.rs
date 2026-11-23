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
use malachite_float::big_exponent_float::arithmetic::mul::mul_prec_round_generic;
use malachite_float::test_util::bench::bucketers::{
    pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer,
    pair_big_exponent_float_max_complexity_bucketer,
    quadruple_1_2_3_big_exponent_float_max_complexity_bucketer,
    triple_1_2_3_big_exponent_float_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4,
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4_float,
    big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1,
    big_exponent_float_pair_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_mul);
    register_demo!(runner, demo_big_exponent_float_mul_val_ref);
    register_demo!(runner, demo_big_exponent_float_mul_ref_val);
    register_demo!(runner, demo_big_exponent_float_mul_ref_ref);
    register_demo!(runner, demo_big_exponent_float_mul_assign);
    register_demo!(runner, demo_big_exponent_float_mul_assign_ref);
    register_demo!(runner, demo_big_exponent_float_mul_debug);
    register_demo!(runner, demo_big_exponent_float_mul_val_ref_debug);
    register_demo!(runner, demo_big_exponent_float_mul_ref_val_debug);
    register_demo!(runner, demo_big_exponent_float_mul_ref_ref_debug);
    register_demo!(runner, demo_big_exponent_float_mul_assign_debug);
    register_demo!(runner, demo_big_exponent_float_mul_assign_ref_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_val_ref);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_ref_val);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_ref_ref);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_assign);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_assign_ref);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_val_ref_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_ref_val_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_ref_ref_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_round_assign_debug);
    register_demo!(
        runner,
        demo_big_exponent_float_mul_prec_round_assign_ref_debug
    );
    register_demo!(runner, demo_big_exponent_float_mul_prec);
    register_demo!(runner, demo_big_exponent_float_mul_prec_val_ref);
    register_demo!(runner, demo_big_exponent_float_mul_prec_ref_val);
    register_demo!(runner, demo_big_exponent_float_mul_prec_ref_ref);
    register_demo!(runner, demo_big_exponent_float_mul_prec_assign);
    register_demo!(runner, demo_big_exponent_float_mul_prec_assign_ref);
    register_demo!(runner, demo_big_exponent_float_mul_prec_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_val_ref_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_ref_val_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_ref_ref_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_assign_debug);
    register_demo!(runner, demo_big_exponent_float_mul_prec_assign_ref_debug);

    register_bench!(runner, benchmark_big_exponent_float_mul_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_mul_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{x} * {y} = {}", &x * &y);
    }
}

fn demo_big_exponent_float_mul_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{x:#x} * {y:#x} = {:#x}", &x * &y);
    }
}

fn demo_big_exponent_float_mul_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.mul_prec_round_ref_ref(&y, prec, rm);
        println!("({x}).mul_prec_round({y}, {prec}, {rm}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_round_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.mul_prec_round_ref_ref(&y, prec, rm);
        println!("({x:#x}).mul_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.mul_prec_ref_ref(&y, prec);
        println!("({x}).mul_prec({y}, {prec}) = ({z}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.mul_prec_ref_ref(&y, prec);
        println!("({x:#x}).mul_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("{x_old} * &{y} = {}", x * &y);
    }
}

fn demo_big_exponent_float_mul_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("{x_old:#x} * &{:#x} = {:#x}", &y, x * &y);
    }
}

fn demo_big_exponent_float_mul_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!("&{x} * {y_old} = {}", &x * y);
    }
}

fn demo_big_exponent_float_mul_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!("&{:#x} * {y_old:#x} = {:#x}", &x, &x * y);
    }
}

fn demo_big_exponent_float_mul_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("&{x} * &{y} = {}", &x * &y);
    }
}

fn demo_big_exponent_float_mul_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("&{:#x} * &{:#x} = {:#x}", &x, &y, &x * &y);
    }
}

fn demo_big_exponent_float_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {x_old}; x *= {y}; x = {x}");
    }
}

fn demo_big_exponent_float_mul_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {x_old:#x}; x *= {y:#x}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_mul_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x *= &y;
        println!("x := {x_old}; x *= &{y}; x = {x}");
    }
}

fn demo_big_exponent_float_mul_assign_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x *= &y;
        println!("x := {x_old:#x}; x *= &{y:#x}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_mul_prec_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({x_old}).mul_prec_val_ref(&{y}, {prec}) = {:?}",
            x.mul_prec_val_ref(&y, prec)
        );
    }
}

fn demo_big_exponent_float_mul_prec_val_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.mul_prec_val_ref(&y, prec);
        println!("({x_old:#x}).mul_prec_val_ref(&{y:#x}, {prec}) = ({sum:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{x}).mul_prec_ref_val({y_old}, {prec}) = {:?}",
            x.mul_prec_ref_val(y, prec)
        );
    }
}

fn demo_big_exponent_float_mul_prec_ref_val_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (sum, o) = x.mul_prec_ref_val(y, prec);
        println!("(&{x:#x}).mul_prec_ref_val({y_old:#x}, {prec}) = ({sum:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mul_prec_ref_ref(&{y}, {prec}) = {:?}",
            x.mul_prec_ref_ref(&y, prec)
        );
    }
}

fn demo_big_exponent_float_mul_prec_ref_ref_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (sum, o) = x.mul_prec_ref_ref(&y, prec);
        println!("(&{x:#x}).mul_prec_ref_ref(&{y:#x}, {prec}) = ({sum:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mul_prec_assign(y, prec);
        println!("x := {x_old}; x.mul_prec_assign({y_old}, {prec}); x = {x}");
    }
}

fn demo_big_exponent_float_mul_prec_assign_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.mul_prec_assign(y, prec);
        println!("x := {x_old:#x}; x.mul_prec_assign({y_old:#x}, {prec}) = {o:?}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_mul_prec_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mul_prec_assign_ref(&y, prec);
        println!("x := {x_old}; x.mul_prec_assign({y}, {prec}); x = {x}");
    }
}

fn demo_big_exponent_float_mul_prec_assign_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let o = x.mul_prec_assign_ref(&y, prec);
        println!("x := {x_old:#x}; x.mul_prec_assign({y:#x}, {prec}) = {o:?}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_mul_prec_round_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        println!(
            "({x_old}).mul_prec_round(&{y}, {prec}, {rm}) = {:?}",
            x.mul_prec_round_val_ref(&y, prec, rm)
        );
    }
}

fn demo_big_exponent_float_mul_prec_round_val_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let (sum, o) = x.mul_prec_round_val_ref(&y, prec, rm);
        println!("({x_old:#x}).mul_prec_round_val_ref(&{y:#x}, {prec}, {rm}) = ({sum:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_round_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let y_old = y.clone();
        println!(
            "(&{x}).mul_prec_round_ref_val({y_old}, {prec}, {rm}) = {:?}",
            x.mul_prec_round_ref_val(y, prec, rm)
        );
    }
}

fn demo_big_exponent_float_mul_prec_round_ref_val_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let y_old = y.clone();
        let (sum, o) = x.mul_prec_round_ref_val(y, prec, rm);
        println!("(&{x:#x}).mul_prec_round_ref_val({y_old:#x}, {prec}, {rm}) = ({sum:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_round_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "({x}).mul_prec_round({y}, {prec}, {rm}) = {:?}",
            x.mul_prec_round_ref_ref(&y, prec, rm)
        );
    }
}

fn demo_big_exponent_float_mul_prec_round_ref_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let (sum, o) = x.mul_prec_round_ref_ref(&y, prec, rm);
        println!("({x:#x}).mul_prec_round_ref_ref(&{y:#x}, {prec}, {rm}) = ({sum:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_mul_prec_round_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.mul_prec_round_assign(y, prec, rm);
        println!("x := {x_old}; x.mul_prec_round({y_old}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_big_exponent_float_mul_prec_round_assign_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let o = x.mul_prec_round_assign(y, prec, rm);
        println!("x := {x_old:#x}; x.mul_prec_round({y_old:#x}, {prec}, {rm}) = {o:?}; x = {x:#x}");
    }
}

fn demo_big_exponent_float_mul_prec_round_assign_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let o = x.mul_prec_round_assign_ref(&y, prec, rm);
        println!("x := {x_old}; x.mul_prec_round_ref(&{y}, {prec}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_big_exponent_float_mul_prec_round_assign_ref_debug(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config)
            .take(limit)
    {
        let x_old = x.clone();
        let o = x.mul_prec_round_assign_ref(&y, prec, rm);
        println!(
            "x := {x_old:#x}; x.mul_prec_round_ref(&{y:#x}, {prec}, {rm}) = {o:?}; x = {x:#x}"
        );
    }
}

fn benchmark_big_exponent_float_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat * BigExponentFloat",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("BigExponentFloat * BigExponentFloat", &mut |(x, y)| {
                no_out!(x * y);
            }),
            ("BigExponentFloat * &BigExponentFloat", &mut |(x, y)| {
                no_out!(x * &y);
            }),
            ("&BigExponentFloat * BigExponentFloat", &mut |(x, y)| {
                no_out!(&x * y);
            }),
            ("&BigExponentFloat * &BigExponentFloat", &mut |(x, y)| {
                no_out!(&x * &y);
            }),
            ("BigExponentFloat *= BigExponentFloat", &mut |(mut x, y)| {
                x *= y;
            }),
            (
                "BigExponentFloat *= &BigExponentFloat",
                &mut |(mut x, y)| {
                    x *= &y;
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_mul_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.mul_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.mul_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.mul_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.mul_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.mul_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.mul_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.mul_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.mul_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_mul_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.mul_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.mul_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.mul_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.mul_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.mul_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.mul_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.mul_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.mul_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.mul_prec_assign(BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.mul_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.mul_prec_assign_ref(&BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.mul_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_mul_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float kernels", &mut |(x, y, prec, rm)| {
                no_out!(x.mul_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(mul_prec_round_generic(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_mul_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.mul_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_4_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.mul_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.mul_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}
