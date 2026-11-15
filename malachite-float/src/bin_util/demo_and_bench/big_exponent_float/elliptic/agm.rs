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
use malachite_float::big_exponent_float::elliptic::agm::agm_prec_round_generic;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::test_util::bench::bucketers::{
    quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer,
    triple_rational_rational_primitive_int_max_bit_bucketer,
};
use malachite_q::test_util::generators::rational_rational_unsigned_triple_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_agm_prec_round);
    register_demo!(runner, demo_big_exponent_float_agm_prec_round_in_domain);
    register_demo!(runner, demo_big_exponent_float_agm_prec);
    register_demo!(runner, demo_big_exponent_float_agm_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_agm_rational_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_agm_big_exponent_float_rational_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_agm_big_exponent_float_rational_prec_round_in_domain
    );
    register_demo!(
        runner,
        demo_big_exponent_float_agm_big_exponent_float_rational_prec
    );

    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_prec_round_algorithms
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_rational_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_big_exponent_float_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_big_exponent_float_rational_prec_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_agm_big_exponent_float_rational_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_agm_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_15()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.agm_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.agm_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_agm_prec_round_in_domain(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_16()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.agm_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.agm_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_agm_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.agm_prec_ref_ref(&y, prec);
        println!("{x:#x}.agm_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_agm_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in rational_rational_unsigned_rounding_mode_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::agm_rational_prec_round_ref_ref(&x, &y, prec, rm);
        println!(
            "BigExponentFloat::agm_rational_prec_round({x}, {y}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_agm_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in rational_rational_unsigned_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::agm_rational_prec_ref_ref(&x, &y, prec);
        println!("BigExponentFloat::agm_rational_prec({x}, {y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_agm_big_exponent_float_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_19()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.agm_big_exponent_float_rational_prec_round_ref_ref(&y, prec, rm);
        println!(
            "{x:#x}.agm_big_exponent_float_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, \
            {o:?})"
        );
    }
}

fn demo_big_exponent_float_agm_big_exponent_float_rational_prec_round_in_domain(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_20()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.agm_big_exponent_float_rational_prec_round_ref_ref(&y, prec, rm);
        println!(
            "{x:#x}.agm_big_exponent_float_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, \
            {o:?})"
        );
    }
}

fn demo_big_exponent_float_agm_big_exponent_float_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.agm_big_exponent_float_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.agm_big_exponent_float_rational_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_agm_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.agm_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_15()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.agm_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.agm_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.agm_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_agm_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.agm_prec(BigExponentFloat, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.agm_prec(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_prec_val_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_prec_ref_val(BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_prec_ref_ref(&BigExponentFloat, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_prec_assign(BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.agm_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_prec_assign_ref(&BigExponentFloat, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.agm_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_agm_prec_round_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.agm_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::Algorithms,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_15()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("Float kernel", &mut |(x, y, prec, rm)| {
                no_out!(x.agm_prec_round_ref_ref(&y, prec, rm));
            }),
            ("generic", &mut |(x, y, prec, rm)| {
                no_out!(agm_prec_round_generic(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_agm_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.agm_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_15_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.agm_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.agm_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_agm_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::agm_rational_prec_round(Rational, Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::agm_rational_prec_round(Rational, Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(BigExponentFloat::agm_rational_prec_round(x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::agm_rational_prec_round_val_ref(Rational, &Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(BigExponentFloat::agm_rational_prec_round_val_ref(
                        x, &y, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::agm_rational_prec_round_ref_val(&Rational, Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(BigExponentFloat::agm_rational_prec_round_ref_val(
                        &x, y, prec, rm
                    ));
                },
            ),
            (
                "BigExponentFloat::agm_rational_prec_round_ref_ref(&Rational, &Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(BigExponentFloat::agm_rational_prec_round_ref_ref(
                        &x, &y, prec, rm
                    ));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_agm_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::agm_rational_prec(Rational, Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_unsigned_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_rational_rational_primitive_int_max_bit_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::agm_rational_prec(Rational, Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(BigExponentFloat::agm_rational_prec(x, y, prec));
                },
            ),
            (
                "BigExponentFloat::agm_rational_prec_val_ref(Rational, &Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(BigExponentFloat::agm_rational_prec_val_ref(x, &y, prec));
                },
            ),
            (
                "BigExponentFloat::agm_rational_prec_ref_val(&Rational, Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(BigExponentFloat::agm_rational_prec_ref_val(&x, y, prec));
                },
            ),
            (
                "BigExponentFloat::agm_rational_prec_ref_ref(&Rational, &Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(BigExponentFloat::agm_rational_prec_ref_ref(&x, &y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_agm_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::agm_rational_prec_round(&Rational, &Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        rational_rational_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_rational_primitive_int_max_bit_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(x, y, prec, rm)| {
                no_out!(BigExponentFloat::agm_rational_prec_round_ref_ref(
                    &x, &y, prec, rm
                ));
            }),
            ("Float", &mut |(x, y, prec, rm)| {
                no_out!(Float::agm_rational_prec_round_ref_ref(&x, &y, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_agm_big_exponent_float_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.agm_big_exponent_float_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_19().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_round_assign(Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_round_assign_ref(&Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_agm_big_exponent_float_rational_prec_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.agm_big_exponent_float_rational_prec(Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_big_exponent_float_rational_prec(y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_val_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_val_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_ref_val(Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_ref_val(y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_ref_ref(&Rational, u64)",
                &mut |(x, y, prec)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_ref_ref(&y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_assign(Rational, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_assign(y, prec));
                },
            ),
            (
                "BigExponentFloat.agm_big_exponent_float_rational_prec_assign_ref(&Rational, u64)",
                &mut |(mut x, y, prec)| {
                    no_out!(x.agm_big_exponent_float_rational_prec_assign_ref(&y, prec));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_agm_big_exponent_float_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.agm_big_exponent_float_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_19_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.agm_big_exponent_float_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.agm_float_rational_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}
