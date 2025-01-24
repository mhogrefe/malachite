// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::NotAssign;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, pair_2_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_gen, integer_gen_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_not_assign);
    register_demo!(runner, demo_integer_not);
    register_demo!(runner, demo_integer_not_ref);

    register_bench!(runner, benchmark_integer_not_assign);
    register_bench!(runner, benchmark_integer_not_library_comparison);
    register_bench!(runner, benchmark_integer_not_evaluation_strategy);
}

fn demo_integer_not_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut n in integer_gen().get(gm, config).take(limit) {
        let n_old = n.clone();
        n.not_assign();
        println!("n := {n_old}; n.not_assign(); n = {n}");
    }
}

fn demo_integer_not(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

fn demo_integer_not_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in integer_gen().get(gm, config).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

fn benchmark_integer_not_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.not_assign()",
        BenchmarkType::Single,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |mut n| n.not_assign())],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_not_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.not()",
        BenchmarkType::LibraryComparison,
        integer_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_integer_bit_bucketer("n"),
        &mut [("Malachite", &mut |(_, n)| no_out!(!n)), ("rug", &mut |(n, _)| no_out!(!n))],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_not_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.not()",
        BenchmarkType::EvaluationStrategy,
        integer_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("n"),
        &mut [("!Integer", &mut |n| no_out!(!n)), ("!&Integer", &mut |n| no_out!(!&n))],
    );
}
