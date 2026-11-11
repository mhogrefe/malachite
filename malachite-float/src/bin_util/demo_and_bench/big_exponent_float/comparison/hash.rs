// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::hash::hash;
use malachite_base::test_util::runner::Runner;
use malachite_float::big_exponent_float::{
    ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use malachite_float::test_util::bench::bucketers::big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::big_exponent_float_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_comparable_big_exponent_float_hash);
    register_demo!(runner, demo_comparable_big_exponent_float_ref_hash);

    register_bench!(
        runner,
        benchmark_comparable_big_exponent_float_hash_evaluation_strategy
    );
}

fn demo_comparable_big_exponent_float_hash(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        let x = ComparableBigExponentFloat(x);
        println!("hash({x:#x}) = {}", hash(&x));
    }
}

fn demo_comparable_big_exponent_float_ref_hash(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in big_exponent_float_gen().get(gm, config).take(limit) {
        let x = ComparableBigExponentFloatRef(&x);
        println!("hash({x:#x}) = {}", hash(&x));
    }
}

fn benchmark_comparable_big_exponent_float_hash_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "hash(&ComparableBigExponentFloat)",
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &big_exponent_float_complexity_bucketer("x"),
        &mut [
            ("ComparableBigExponentFloat", &mut |x| {
                no_out!(hash(&ComparableBigExponentFloat(x)));
            }),
            ("ComparableBigExponentFloatRef", &mut |x| {
                no_out!(hash(&ComparableBigExponentFloatRef(&x)));
            }),
        ],
    );
}
