// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    natural_bit_bucketer, triple_3_natural_bit_bucketer,
};
use malachite_nz::test_util::generators::{natural_gen, natural_gen_nrm};
use malachite_nz::test_util::natural::arithmetic::neg::neg_num;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_neg);
    register_demo!(runner, demo_natural_neg_ref);
    register_bench!(runner, benchmark_natural_neg_library_comparison);
    register_bench!(runner, benchmark_natural_neg_evaluation_strategy);
}

fn demo_natural_neg(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("-{} = {}", n.clone(), -n);
    }
}

fn demo_natural_neg_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("-(&{}) = {}", n.clone(), -n);
    }
}

#[allow(unused_must_use, clippy::no_effect, clippy::unnecessary_operation)]
fn benchmark_natural_neg_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Natural",
        BenchmarkType::LibraryComparison,
        natural_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(-x)),
            ("num", &mut |(x, _, _)| no_out!(neg_num(x))),
            ("rug", &mut |(_, x, _)| no_out!(-x)),
        ],
    );
}

#[allow(unused_must_use, clippy::no_effect)]
fn benchmark_natural_neg_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "-Natural",
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("x"),
        &mut [("-Natural", &mut |x| no_out!(-x)), ("-&Natural", &mut |x| no_out!(-&x))],
    );
}
