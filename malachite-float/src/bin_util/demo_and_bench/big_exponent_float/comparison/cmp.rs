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
use malachite_float::big_exponent_float::{
    ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use malachite_float::test_util::bench::bucketers::{
    pair_2_pair_big_exponent_float_max_complexity_bucketer,
    pair_big_exponent_float_max_complexity_bucketer,
};
use malachite_float::test_util::generators::{
    big_exponent_float_pair_gen, big_exponent_float_pair_gen_var_1_float,
};
use std::cmp::Ordering::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_partial_cmp);
    register_demo!(runner, demo_comparable_big_exponent_float_cmp);
    register_demo!(runner, demo_comparable_big_exponent_float_ref_cmp);

    register_bench!(
        runner,
        benchmark_big_exponent_float_partial_cmp_float_comparison
    );
    register_bench!(
        runner,
        benchmark_comparable_big_exponent_float_cmp_evaluation_strategy
    );
}

fn demo_big_exponent_float_partial_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        match x.partial_cmp(&y) {
            None => println!("{x:#x} and {y:#x} are incomparable"),
            Some(Less) => println!("{x:#x} < {y:#x}"),
            Some(Equal) => println!("{x:#x} = {y:#x}"),
            Some(Greater) => println!("{x:#x} > {y:#x}"),
        }
    }
}

fn demo_comparable_big_exponent_float_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableBigExponentFloat(x.clone());
        let cy = ComparableBigExponentFloat(y.clone());
        match cx.cmp(&cy) {
            Less => println!("{cx:#x} < {cy:#x}"),
            Equal => println!("{cx:#x} = {cy:#x}"),
            Greater => println!("{cx:#x} > {cy:#x}"),
        }
    }
}

fn demo_comparable_big_exponent_float_ref_cmp(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in big_exponent_float_pair_gen().get(gm, config).take(limit) {
        let cx = ComparableBigExponentFloatRef(&x);
        let cy = ComparableBigExponentFloatRef(&y);
        match cx.cmp(&cy) {
            Less => println!("{cx:#x} < {cy:#x}"),
            Equal => println!("{cx:#x} = {cy:#x}"),
            Greater => println!("{cx:#x} > {cy:#x}"),
        }
    }
}

fn benchmark_big_exponent_float_partial_cmp_float_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat.partial_cmp(&BigExponentFloat)",
        BenchmarkType::LibraryComparison,
        big_exponent_float_pair_gen_var_1_float().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            ("BigExponentFloat", &mut |(_, (x, y))| {
                no_out!(x.partial_cmp(&y));
            }),
            ("Float", &mut |((x, y), _)| {
                no_out!(x.partial_cmp(&y));
            }),
        ],
    );
}

fn benchmark_comparable_big_exponent_float_cmp_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "ComparableBigExponentFloat.cmp(&ComparableBigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_big_exponent_float_max_complexity_bucketer("x", "y"),
        &mut [
            (
                "BigExponentFloat.partial_cmp(&BigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(x.partial_cmp(&y));
                },
            ),
            (
                "ComparableBigExponentFloat.cmp(&ComparableBigExponentFloat)",
                &mut |(x, y)| {
                    no_out!(ComparableBigExponentFloat(x).cmp(&ComparableBigExponentFloat(y)));
                },
            ),
            (
                "ComparableBigExponentFloatRef.cmp(&ComparableBigExponentFloatRef)",
                &mut |(x, y)| {
                    no_out!(
                        ComparableBigExponentFloatRef(&x).cmp(&ComparableBigExponentFloatRef(&y))
                    );
                },
            ),
        ],
    );
}
