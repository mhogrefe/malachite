// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Product;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::*;
use malachite_float::test_util::big_exponent_float::big_exponent_float_exponent_is_small;
use malachite_float::test_util::generators::*;
use malachite_float::{BigExponentFloat, Float};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_product);
    register_demo!(runner, demo_big_exponent_float_product_prec_round);
    register_demo!(runner, demo_big_exponent_float_product_prec);

    register_bench!(
        runner,
        benchmark_big_exponent_float_product_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_big_exponent_float_product_prec_round_float_comparison
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

fn demo_big_exponent_float_product(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in big_exponent_float_vec_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let r: BigExponentFloat = xs.iter().product();
        println!("product({}) = {r:#x}", hex_slice(&xs));
    }
}

fn demo_big_exponent_float_product_prec_round(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec, rm) in big_exponent_float_vec_unsigned_rounding_mode_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::product_prec_round(&xs, prec, rm);
        println!(
            "product_prec_round({}, {prec}, {rm}) = ({r:#x}, {o:?})",
            hex_slice(&xs)
        );
    }
}

fn demo_big_exponent_float_product_prec(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, prec) in big_exponent_float_vec_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (r, o) = BigExponentFloat::product_prec(&xs, prec);
        println!("product_prec({}, {prec}) = ({r:#x}, {o:?})", hex_slice(&xs));
    }
}

fn benchmark_big_exponent_float_product_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::product(Iterator<Item = BigExponentFloat>)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_vec_unsigned_pair_gen_var_1()
            .get(gm, config)
            .filter(|(xs, _)| xs.iter().all(big_exponent_float_exponent_is_small)),
        gm.name(),
        limit,
        file_name,
        &pair_big_exponent_float_vec_sum_complexity_bucketer("xs", "_"),
        &mut [
            (
                "BigExponentFloat::product(Iterator<Item = BigExponentFloat>)",
                &mut |(xs, _)| {
                    no_out!(BigExponentFloat::product(xs.into_iter()));
                },
            ),
            (
                "BigExponentFloat::product(Iterator<Item = &BigExponentFloat>)",
                &mut |(xs, _)| {
                    no_out!(BigExponentFloat::product(xs.iter()));
                },
            ),
        ],
    );
}

fn benchmark_big_exponent_float_product_prec_round_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::product_prec_round(&[BigExponentFloat], u64, RoundingMode)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_vec_unsigned_rounding_mode_triple_gen_var_2_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_triple_1_2_big_exponent_float_vec_sum_complexity_bucketer("xs", "prec"),
        &mut [
            ("BigExponentFloat", &mut |(_, (xs, prec, rm))| {
                no_out!(BigExponentFloat::product_prec_round(&xs, prec, rm));
            }),
            ("Float", &mut |((xs, prec, rm), _)| {
                no_out!(Float::product_prec_round(&xs, prec, rm));
            }),
        ],
    );
}
