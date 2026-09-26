// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::{MakeMonic, MakeMonicAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_make_monic);
    register_demo!(runner, demo_rational_polynomial_make_monic_assign);
    register_bench!(
        runner,
        benchmark_rational_polynomial_make_monic_evaluation_strategy
    );
}

fn demo_rational_polynomial_make_monic(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).make_monic() = {}", (&p).make_monic());
    }
}

fn demo_rational_polynomial_make_monic_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        p.make_monic_assign();
        println!("p := {p_old}; p.make_monic_assign(); p = {p}");
    }
}

fn benchmark_rational_polynomial_make_monic_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.make_monic()",
        BenchmarkType::EvaluationStrategy,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [
            ("RationalPolynomial.make_monic()", &mut |p| {
                no_out!(p.make_monic());
            }),
            ("(&RationalPolynomial).make_monic()", &mut |p| {
                no_out!((&p).make_monic());
            }),
            ("RationalPolynomial.make_monic_assign()", &mut |mut p| {
                p.make_monic_assign();
            }),
        ],
    );
}
