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
    register_demo!(runner, demo_big_exponent_float_dot);
    register_demo!(runner, demo_big_exponent_float_dot_prec_round);
    register_demo!(runner, demo_big_exponent_float_dot_prec);

    register_bench!(runner, benchmark_big_exponent_float_dot_prec_round);
    register_bench!(
        runner,
        benchmark_big_exponent_float_dot_prec_round_float_comparison
    );
}

fn hex_slice(xs: &[BigExponentFloat]) -> String {
    let mut s = String::from("[");
    for (i, x) in xs.iter().enumerate() {
        if i != 0 {
            s.push_str(", ");
        }
        s.push_str(&format!("{x:#x}"));
    }
    s.push(']');
    s
}

fn demo_big_exponent_float_dot(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in big_exponent_float_vec_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let r = BigExponentFloat::dot(&xs, &ys);
        println!("dot({}, {}) = {r:#x}", hex_slice(&xs), hex_slice(&ys));
    }
}

fn demo_big_exponent_float_dot_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec, rm) in
        big_exponent_float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let (r, o) = BigExponentFloat::dot_prec_round(&xs, &ys, prec, rm);
        println!(
            "dot_prec_round({}, {}, {prec}, {rm}) = ({r:#x}, {o:?})",
            hex_slice(&xs),
            hex_slice(&ys)
        );
    }
}

fn demo_big_exponent_float_dot_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, prec) in big_exponent_float_vec_pair_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::dot_prec(&xs, &ys, prec);
        println!(
            "dot_prec({}, {}, {prec}) = ({r:#x}, {o:?})",
            hex_slice(&xs),
            hex_slice(&ys)
        );
    }
}

fn benchmark_big_exponent_float_dot_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::dot_prec_round(&[BigExponentFloat], &[BigExponentFloat], u64, \
        RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1_float()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_quadruple_1_2_3_big_exponent_float_vec_pair_sum_complexity_bucketer("prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (xs, ys, prec, rm))| {
                no_out!(BigExponentFloat::dot_prec_round(&xs, &ys, prec, rm));
            }),
            ("Float", &mut |((xs, ys, prec, rm), _)| {
                no_out!(Float::dot_prec_round(&xs, &ys, prec, rm));
            }),
        ],
    );
}

fn benchmark_big_exponent_float_dot_prec_round(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::dot_prec_round(&[BigExponentFloat], &[BigExponentFloat], u64, \
        RoundingMode)",
        BenchmarkType::Single,
        big_exponent_float_vec_pair_unsigned_rounding_mode_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_big_exponent_float_vec_pair_sum_complexity_bucketer("prec"),
        &mut [("Malachite", &mut |(xs, ys, prec, rm)| {
            no_out!(BigExponentFloat::dot_prec_round(&xs, &ys, prec, rm));
        })],
    );
}
