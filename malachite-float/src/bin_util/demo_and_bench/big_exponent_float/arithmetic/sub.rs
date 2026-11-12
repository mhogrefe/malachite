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
use malachite_float::big_exponent_float::arithmetic::add::add_prec_round_generic;
use malachite_float::test_util::bench::bucketers::{
    pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer,
    pair_big_exponent_float_max_complexity_bucketer,
    quadruple_1_2_3_big_exponent_float_max_complexity_bucketer,
    triple_1_2_3_big_exponent_float_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3,
    big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3_float,
    big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1,
    big_exponent_float_pair_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_sub);
    register_demo!(runner, demo_big_exponent_float_sub_prec_round);
    register_demo!(runner, demo_big_exponent_float_sub_prec);

    register_bench!(runner, benchmark_big_exponent_float_sub_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_sub_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("{x:#x} - {y:#x} = {:#x}", &x - &y);
    }
}

fn demo_big_exponent_float_sub_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.sub_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.sub_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_sub_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.sub_prec_ref_ref(&y, prec);
        println!("{x:#x}.sub_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat - BigExponentFloat",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("BigExponentFloat - BigExponentFloat", &mut |(x, y)| {
                no_out!(x - y);
            }),
            ("BigExponentFloat - &BigExponentFloat", &mut |(x, y)| {
                no_out!(x - &y);
            }),
            ("&BigExponentFloat - BigExponentFloat", &mut |(x, y)| {
                no_out!(&x - y);
            }),
            ("&BigExponentFloat - &BigExponentFloat", &mut |(x, y)| {
                no_out!(&x - &y);
            }),
            ("BigExponentFloat -= BigExponentFloat", &mut |(mut x, y)| {
                x -= y;
            }),
            (
                "BigExponentFloat -= &BigExponentFloat",
                &mut |(mut x, y)| {
                    x -= &y;
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sub_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.sub_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.sub_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.sub_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.sub_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.sub_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sub_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.sub_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.sub_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_prec_assign(BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.sub_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.sub_prec_assign_ref(&BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.sub_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_sub_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float kernels", &mut |(x, y, prec, rm)| {
                no_out!(x.sub_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(add_prec_round_generic(&x, &y, true, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_sub_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.sub_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_3_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.sub_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.sub_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}
