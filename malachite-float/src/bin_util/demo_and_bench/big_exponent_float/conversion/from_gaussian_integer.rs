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
use malachite_nz::test_util::bench::bucketers::gaussian_integer_bit_bucketer;
use malachite_nz::test_util::generators::gaussian_integer_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_big_exponent_float_try_from_gaussian_integer);

    register_bench!(
        runner,
        benchmark_big_exponent_float_try_from_gaussian_integer_evaluation_strategy
    );
}

fn demo_big_exponent_float_try_from_gaussian_integer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for z in gaussian_integer_gen().get(gm, config).take(limit) {
        println!(
            "BigExponentFloat::try_from({z}) = {:?}",
            BigExponentFloat::try_from(&z)
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_big_exponent_float_try_from_gaussian_integer_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "BigExponentFloat::try_from(GaussianInteger)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &gaussian_integer_bit_bucketer("z"),
        &mut [
            ("BigExponentFloat::try_from(GaussianInteger)", &mut |z| {
                no_out!(BigExponentFloat::try_from(z));
            }),
            ("BigExponentFloat::try_from(&GaussianInteger)", &mut |z| {
                no_out!(BigExponentFloat::try_from(&z));
            }),
        ],
    );
}
