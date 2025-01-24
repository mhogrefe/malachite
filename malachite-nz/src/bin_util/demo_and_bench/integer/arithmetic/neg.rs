// Copyright © 2025 Mikhail Hogrefe
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
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, triple_3_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_gen, integer_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_neg);
    register_demo!(runner, demo_integer_neg_ref);
    register_demo!(runner, demo_integer_neg_assign);

    register_bench!(runner, benchmark_integer_neg_library_comparison);
    register_bench!(runner, benchmark_integer_neg_evaluation_strategy);
    register_bench!(runner, benchmark_integer_neg_assign);
}

fn demo_integer_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("-{} = {}", n.clone(), -n);
    }
}

fn demo_integer_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("-(&{}) = {}", n.clone(), -n);
    }
}

fn demo_integer_neg_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in integer_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {n_old}; n.neg_assign(); n = {n}");
    }
}

#[allow(unused_must_use, clippy::no_effect, clippy::unnecessary_operation)]
fn benchmark_integer_neg_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Integer",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(-x)),
            ("num", &mut |(x, _, _)| no_out!(-x)),
            ("rug", &mut |(_, x, _)| no_out!(-x)),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_integer_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Integer",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("-Integer", &mut |x| no_out!(-x)), ("-&Integer", &mut |x| no_out!(-&x))],
    );
}

fn benchmark_integer_neg_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "-Integer",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.neg_assign())],
    );
}
