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
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_rem_prec_round);
    register_demo!(runner, demo_big_exponent_float_rem_prec);
    register_demo!(runner, demo_big_exponent_float_ieee_remainder_prec_round);
    register_demo!(runner, demo_big_exponent_float_ieee_remainder_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_rem_and_quotient_bits_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_rem_and_quotient_bits_prec);
    register_demo!(runner, demo_big_exponent_float_rem);
    register_demo!(runner, demo_big_exponent_float_ieee_remainder);
    register_demo!(runner, demo_big_exponent_float_rem_and_quotient_bits);

    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_ieee_remainder_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_ieee_remainder_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_and_quotient_bits_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_and_quotient_bits_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_rem_evaluation_strategy);
    register_demo!(runner, demo_big_exponent_float_rem_rational_prec_round);
    register_demo!(runner, demo_big_exponent_float_rem_rational_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_ieee_remainder_rational_prec_round
    );
    register_demo!(runner, demo_big_exponent_float_ieee_remainder_rational_prec);
    register_demo!(
        runner,
        demo_big_exponent_float_rem_rational_and_quotient_bits_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rem_rational_and_quotient_bits_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_rem_big_exponent_float_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_rem_big_exponent_float_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec
    );
    register_demo!(runner, demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round);
    register_demo!(
        runner,
        demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec
    );
    register_demo!(runner, demo_big_exponent_float_rem_unsigned_prec_round);
    register_demo!(runner, demo_big_exponent_float_rem_unsigned_prec);
    register_demo!(runner, demo_big_exponent_float_rem_unsigned);
    register_demo!(
        runner,
        demo_big_exponent_float_ieee_remainder_and_quotient_bits_prec_round
    );
    register_demo!(
        runner,
        demo_big_exponent_float_ieee_remainder_and_quotient_bits_prec
    );
    register_demo!(
        runner,
        demo_big_exponent_float_ieee_remainder_and_quotient_bits
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_rational_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_ieee_remainder_rational_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_ieee_remainder_rational_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_rational_and_quotient_bits_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_rational_and_quotient_bits_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec_round_float_comparison);
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_rem_big_exponent_float_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rational_rem_big_exponent_float_prec_round_float_comparison
    );
    register_bench!(runner, benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec_round_float_comparison);
    register_bench!(runner, benchmark_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec_round_float_comparison);
    register_bench!(runner, benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_evaluation_strategy);
    register_bench!(runner, benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_float_comparison);
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_unsigned_prec_round_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_rem_unsigned_prec_round_float_comparison
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_ieee_remainder_and_quotient_bits_prec_round_float_comparison
    );
}

fn demo_big_exponent_float_rem_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.rem_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.rem_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_rem_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.rem_prec_ref_ref(&y, prec);
        println!("{x:#x}.rem_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_rem_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.rem_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.rem_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.rem_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rem_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.rem_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.rem_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_ieee_remainder_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.ieee_remainder_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.ieee_remainder_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_ieee_remainder_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.ieee_remainder_prec_ref_ref(&y, prec);
        println!("{x:#x}.ieee_remainder_prec({y:#x}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_ieee_remainder_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ieee_remainder_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.ieee_remainder_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_prec_round_assign(BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_prec_round_assign_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_ieee_remainder_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ieee_remainder_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.ieee_remainder_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.ieee_remainder_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_rem_and_quotient_bits_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9()
            .get(gm, config)
            .take(limit)
    {
        let (z, o, q) = x.rem_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
        println!(
            "{x:#x}.rem_and_quotient_bits_prec_round({y:#x}, {prec}, {rm}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn demo_big_exponent_float_rem_and_quotient_bits_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, q) = x.rem_and_quotient_bits_prec_ref_ref(&y, prec);
        println!("{x:#x}.rem_and_quotient_bits_prec({y:#x}, {prec}) = ({z:#x}, {o:?}, {q})");
    }
}

fn benchmark_big_exponent_float_rem_and_quotient_bits_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_and_quotient_bits_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.rem_and_quotient_bits_prec_round(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_and_quotient_bits_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_and_quotient_bits_prec_round_val_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_and_quotient_bits_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_and_quotient_bits_prec_round_ref_val(BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_and_quotient_bits_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_and_quotient_bits_prec_round_ref_ref(&BigExponentFloat, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rem_and_quotient_bits_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_and_quotient_bits_prec_round(BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_9_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.rem_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.rem_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        println!("{x:#x} % {y:#x} = {:#x}", &x % &y);
    }
}

fn demo_big_exponent_float_ieee_remainder(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        println!(
            "{x:#x}.ieee_remainder({y:#x}) = {:#x}",
            x.ieee_remainder_ref_ref(&y)
        );
    }
}

fn demo_big_exponent_float_rem_and_quotient_bits(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        let (r, q) = x.rem_and_quotient_bits_ref_ref(&y);
        println!("{x:#x}.rem_and_quotient_bits({y:#x}) = ({r:#x}, {q})");
    }
}

fn benchmark_big_exponent_float_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat % BigExponentFloat",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("BigExponentFloat % BigExponentFloat", &mut |(x, y)| {
                no_out!(x % y);
            }),
            ("BigExponentFloat % &BigExponentFloat", &mut |(x, y)| {
                no_out!(x % &y);
            }),
            ("&BigExponentFloat % BigExponentFloat", &mut |(x, y)| {
                no_out!(&x % y);
            }),
            ("&BigExponentFloat % &BigExponentFloat", &mut |(x, y)| {
                no_out!(&x % &y);
            }),
        ],
    );
}

fn demo_big_exponent_float_rem_rational_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.rem_rational_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.rem_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_rem_rational_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.rem_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.rem_rational_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_rem_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.rem_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_prec_round_assign(Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.rem_rational_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_prec_round_assign_ref(&Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.rem_rational_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rem_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.rem_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.rem_rational_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_ieee_remainder_rational_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) = x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, rm);
        println!("{x:#x}.ieee_remainder_rational_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_ieee_remainder_rational_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.ieee_remainder_rational_prec_ref_ref(&y, prec);
        println!("{x:#x}.ieee_remainder_rational_prec({y}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn benchmark_big_exponent_float_ieee_remainder_rational_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ieee_remainder_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.ieee_remainder_rational_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_rational_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_rational_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_rational_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_prec_round_assign(Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_rational_prec_round_assign(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_prec_round_assign_ref(&Rational, u64, RoundingMode)",
                &mut |(mut x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_rational_prec_round_assign_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_ieee_remainder_rational_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ieee_remainder_rational_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.ieee_remainder_rational_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_rem_rational_and_quotient_bits_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, q) = x.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
        println!(
            "{x:#x}.rem_rational_and_quotient_bits_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn demo_big_exponent_float_rem_rational_and_quotient_bits_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, q) = x.rem_rational_and_quotient_bits_prec_ref_ref(&y, prec);
        println!("{x:#x}.rem_rational_and_quotient_bits_prec({y}, {prec}) = ({z:#x}, {o:?}, {q})");
    }
}

fn benchmark_big_exponent_float_rem_rational_and_quotient_bits_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_rational_and_quotient_bits_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.rem_rational_and_quotient_bits_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_and_quotient_bits_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_and_quotient_bits_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_and_quotient_bits_prec_round_val_ref(&y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_and_quotient_bits_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_and_quotient_bits_prec_round_ref_val(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.rem_rational_and_quotient_bits_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rem_rational_and_quotient_bits_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_rational_and_quotient_bits_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_9_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10()
            .get(gm, config)
            .take(limit)
    {
        let (z, o, q) =
            x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
        println!(
            "{x:#x}.ieee_remainder_rational_and_quotient_bits_prec_round({y}, {prec}, {rm}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn demo_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, q) = x.ieee_remainder_rational_and_quotient_bits_prec_ref_ref(&y, prec);
        println!(
            "{x:#x}.ieee_remainder_rational_and_quotient_bits_prec({y}, {prec}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn benchmark_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ieee_remainder_rational_and_quotient_bits_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat.ieee_remainder_rational_and_quotient_bits_prec_round(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(x.ieee_remainder_rational_and_quotient_bits_prec_round(y, prec, rm));
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(
                        x.ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
                            &y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(
                        x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(y, prec, rm)
                    );
                },
            ),
            (
                "BigExponentFloat.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&Rational, u64, RoundingMode)",
                &mut |(x, y, prec, rm)| {
                    no_out!(
                        x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
                            &y, prec, rm
                        )
                    );
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_ieee_remainder_rational_and_quotient_bits_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ieee_remainder_rational_and_quotient_bits_prec_round(Rational, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_10_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(
                    x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm)
                );
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(
                    x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&y, prec, rm)
                );
            }),
        ],
    );
}

fn demo_big_exponent_float_rational_rem_big_exponent_float_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) =
            BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(&x, &y, prec, rm);
        println!(
            "BigExponentFloat::rational_rem_big_exponent_float_prec_round({x}, {y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_rational_rem_big_exponent_float_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = BigExponentFloat::rational_rem_big_exponent_float_prec_ref_ref(&x, &y, prec);
        println!(
            "BigExponentFloat::rational_rem_big_exponent_float_prec({x}, {y:#x}, {prec}) = ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_rational_rem_big_exponent_float_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_rem_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_rem_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_rem_big_exponent_float_prec_round(
                            x, y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_rem_big_exponent_float_prec_round_val_ref(
                            x, &y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(&Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_val(
                            &x, y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(&Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
                            &x, &y, prec, rm
                        )
                    );
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_rem_big_exponent_float_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_rem_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (y, x, prec, rm))| {
                no_out!(
                    BigExponentFloat::rational_rem_big_exponent_float_prec_round_ref_ref(
                        &x, &y, prec, rm
                    )
                );
            }),
            ("Float", &mut |((y, x, prec, rm), _)| {
                no_out!(Float::rational_rem_float_prec_round_ref_ref(
                    &x, &y, prec, rm
                ));
            }),
        ],
    );
}

fn demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12()
            .get(gm, config)
            .take(limit)
    {
        let (z, o) =
            BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
                &x, &y, prec, rm,
            );
        println!(
            "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round({x}, {y:#x}, {prec}, {rm}) = ({z:#x}, {o:?})"
        );
    }
}

fn demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) =
            BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_ref_ref(&x, &y, prec);
        println!(
            "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec({x}, {y:#x}, {prec}) = ({z:#x}, {o:?})"
        );
    }
}

fn benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(
                        BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(
                            x, y, prec, rm
                        )
                    );
                },
            ),
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_val_ref(x, &y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(&Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_val(&x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(&Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(&x, &y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (y, x, prec, rm))| {
                no_out!(
                    BigExponentFloat::rational_ieee_remainder_big_exponent_float_prec_round_ref_ref(
                        &x, &y, prec, rm
                    )
                );
            }),
            ("Float", &mut |((y, x, prec, rm), _)| {
                no_out!(Float::rational_ieee_remainder_float_prec_round_ref_ref(
                    &x, &y, prec, rm
                ));
            }),
        ],
    );
}

fn demo_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11()
            .get(gm, config)
            .take(limit)
    {
        let (z, o, q) =
            BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(
                &x, &y, prec, rm,
            );
        println!(
            "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round({x}, {y:#x}, {prec}, {rm}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn demo_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, q) =
            BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_ref_ref(
                &x, &y, prec,
            );
        println!(
            "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec({x}, {y:#x}, {prec}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn benchmark_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_val_ref(x, &y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_val(&x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&x, &y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_rem_big_exponent_float_and_quotient_bits_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_11_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (y, x, prec, rm))| {
                no_out!(BigExponentFloat::rational_rem_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&x, &y, prec, rm));
            }),
            ("Float", &mut |((y, x, prec, rm), _)| {
                no_out!(
                    Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref(
                        &x, &y, prec, rm
                    )
                );
            }),
        ],
    );
}

fn demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec, rm) in
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12()
            .get(gm, config)
            .take(limit)
    {
        let (z, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&x, &y, prec, rm);
        println!(
            "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round({x}, {y:#x}, {prec}, {rm}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn demo_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (y, x, prec) in big_exponent_float_rational_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, q) = BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_ref_ref(&x, &y, prec);
        println!(
            "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec({x}, {y:#x}, {prec}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_val_ref(x, &y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&Rational, BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_val(&x, y, prec, rm));
                },
            ),
            (
                "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&Rational, &BigExponentFloat, u64, RoundingMode)",
                &mut |(y, x, prec, rm)| {
                    no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&x, &y, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round(Rational, BigExponentFloat, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_rational_unsigned_rounding_mode_quadruple_gen_var_12_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_rational_max_complexity_bucketer(
            "x", "y", "prec",
        ),
        &mut [
            ("BigExponentFloat", &mut |(_, (y, x, prec, rm))| {
                no_out!(BigExponentFloat::rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round_ref_ref(&x, &y, prec, rm));
            }),
            ("Float", &mut |((y, x, prec, rm), _)| {
                no_out!(
                    Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref(
                        &x, &y, prec, rm
                    )
                );
            }),
        ],
    );
}

fn demo_big_exponent_float_rem_unsigned_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec, rm) in big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.rem_unsigned_prec_round_ref(n, prec, rm);
        println!("{x:#x}.rem_unsigned_prec_round({n}, {prec}, {rm}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_rem_unsigned_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n, prec) in big_exponent_float_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o) = x.rem_unsigned_prec_ref(n, prec);
        println!("{x:#x}.rem_unsigned_prec({n}, {prec}) = ({z:#x}, {o:?})");
    }
}

fn demo_big_exponent_float_rem_unsigned(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, n) in big_exponent_float_unsigned_pair_gen::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("{x:#x}.rem_unsigned({n}) = {:#x}", x.rem_unsigned_ref(n));
    }
}

fn benchmark_big_exponent_float_rem_unsigned_prec_round_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_unsigned_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            (
                "BigExponentFloat.rem_unsigned_prec_round(u64, u64, RoundingMode)",
                &mut |(x, n, prec, rm)| {
                    no_out!(x.rem_unsigned_prec_round(n, prec, rm));
                },
            ),
            (
                "(&BigExponentFloat).rem_unsigned_prec_round_ref(u64, u64, RoundingMode)",
                &mut |(x, n, prec, rm)| {
                    no_out!(x.rem_unsigned_prec_round_ref(n, prec, rm));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_rem_unsigned_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.rem_unsigned_prec_round(u64, u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_unsigned_unsigned_rounding_mode_quadruple_gen_var_3_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_max_quadruple_1_big_exponent_float_complexity_quadruple_3_bucketer("x", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, n, prec, rm))| {
                no_out!(x.rem_unsigned_prec_round_ref(n, prec, rm));
            }),
            ("Float", &mut |((x, n, prec, rm), _)| {
                no_out!(x.rem_unsigned_prec_round_ref(n, prec, rm));
            }),
        ],
    );
}

fn demo_big_exponent_float_ieee_remainder_and_quotient_bits_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec, rm) in
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10()
            .get(gm, config)
            .take(limit)
    {
        let (z, o, q) = x.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&y, prec, rm);
        println!(
            "{x:#x}.ieee_remainder_and_quotient_bits_prec_round({y:#x}, {prec}, {rm}) = \
            ({z:#x}, {o:?}, {q})"
        );
    }
}

fn demo_big_exponent_float_ieee_remainder_and_quotient_bits_prec(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, prec) in big_exponent_float_big_exponent_float_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (z, o, q) = x.ieee_remainder_and_quotient_bits_prec_ref_ref(&y, prec);
        println!(
            "{x:#x}.ieee_remainder_and_quotient_bits_prec({y:#x}, {prec}) = ({z:#x}, {o:?}, {q})"
        );
    }
}

fn demo_big_exponent_float_ieee_remainder_and_quotient_bits(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        let (r, q) = x.ieee_remainder_and_quotient_bits_ref_ref(&y);
        println!("{x:#x}.ieee_remainder_and_quotient_bits({y:#x}) = ({r:#x}, {q})");
    }
}

fn benchmark_big_exponent_float_ieee_remainder_and_quotient_bits_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.ieee_remainder_and_quotient_bits_prec_round(BigExponentFloat, u64, \
        RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_big_exponent_float_unsigned_rounding_mode_quadruple_gen_var_10_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_max_complexity_bucketer("x", "y", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y, prec, rm))| {
                no_out!(x.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
            }),
            ("Float", &mut |((x, y, prec, rm), _)| {
                no_out!(x.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&y, prec, rm));
            }),
        ],
    );
}
