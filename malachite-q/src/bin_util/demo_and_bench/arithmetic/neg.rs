// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    rational_bit_bucketer, triple_3_rational_bit_bucketer,
};
use malachite_q::test_util::generators::{rational_gen, rational_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_neg);
    register_demo!(runner, demo_rational_neg_ref);
    register_demo!(runner, demo_rational_neg_assign);

    register_bench!(runner, benchmark_rational_neg_library_comparison);
    register_bench!(runner, benchmark_rational_neg_evaluation_strategy);
    register_bench!(runner, benchmark_rational_neg_assign);
}

fn demo_rational_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen().get(gm, config).take(limit) {
        println!("-({}) = {}", n.clone(), -n);
    }
}

fn demo_rational_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in rational_gen().get(gm, config).take(limit) {
        println!("-(&{}) = {}", n, -&n);
    }
}

fn demo_rational_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in rational_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {n_old}; n.neg_assign(); n = {n}");
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_neg_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Rational",
        BenchmarkType::LibraryComparison,
        rational_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_rational_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(-n)),
            ("num", &mut |(n, _, _)| no_out!(-n)),
            ("rug", &mut |(_, n, _)| no_out!(-n)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Rational",
        BenchmarkType::EvaluationStrategy,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [
            ("Rational.neg()", &mut |n| no_out!(-n)),
            ("(&Rational).neg()", &mut |n| no_out!(-&n)),
        ],
    );
}

fn benchmark_rational_neg_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Rational.neg_assign()",
        BenchmarkType::Single,
        rational_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut n| n.neg_assign())],
    );
}
