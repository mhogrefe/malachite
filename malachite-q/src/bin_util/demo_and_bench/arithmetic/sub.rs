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
use malachite_q::test_util::arithmetic::sub::sub_naive;
use malachite_q::test_util::bench::bucketers::{
    pair_2_pair_rational_max_bit_bucketer, pair_rational_max_bit_bucketer,
    triple_3_pair_rational_max_bit_bucketer,
};
use malachite_q::test_util::generators::{
    rational_pair_gen, rational_pair_gen_nrm, rational_pair_gen_rm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sub);
    register_demo!(runner, demo_rational_sub_val_ref);
    register_demo!(runner, demo_rational_sub_ref_val);
    register_demo!(runner, demo_rational_sub_ref_ref);
    register_demo!(runner, demo_rational_sub_assign);
    register_demo!(runner, demo_rational_sub_assign_ref);

    register_bench!(runner, benchmark_rational_sub_library_comparison);
    register_bench!(runner, benchmark_rational_sub_evaluation_strategy);
    register_bench!(runner, benchmark_rational_sub_algorithms);
    register_bench!(runner, benchmark_rational_sub_assign_library_comparison);
    register_bench!(runner, benchmark_rational_sub_assign_evaluation_strategy);
}

fn demo_rational_sub(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_rational_sub_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_rational_sub_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_rational_sub_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in rational_pair_gen().get(gm, config).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn demo_rational_sub_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x -= y.clone();
        println!("x := {x_old}; x -= {y}; x = {x}");
    }
}

fn demo_rational_sub_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {x_old}; x -= &{y}; x = {x}");
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_rational_sub_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational - Rational",
        BenchmarkType::LibraryComparison,
        rational_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x - y)),
            ("num", &mut |((x, y), _, _)| no_out!(x - y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x - y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_sub_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational - Rational",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Rational - Rational", &mut |(x, y)| no_out!(x - y)),
            ("Rational - &Rational", &mut |(x, y)| no_out!(x - &y)),
            ("&Rational - Rational", &mut |(x, y)| no_out!(&x - y)),
            ("&Rational - &Rational", &mut |(x, y)| no_out!(&x - &y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_sub_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational - Rational",
        BenchmarkType::Algorithms,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x - y)),
            ("naive", &mut |(x, y)| no_out!(sub_naive(x, y))),
        ],
    );
}

fn benchmark_rational_sub_assign_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational -= Rational",
        BenchmarkType::LibraryComparison,
        rational_pair_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_rational_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x -= y), ("rug", &mut |((mut x, y), _)| x -= y)],
    );
}

fn benchmark_rational_sub_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational -= Rational",
        BenchmarkType::EvaluationStrategy,
        rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("Rational -= Rational", &mut |(mut x, y)| no_out!(x -= y)),
            ("Rational -= &Rational", &mut |(mut x, y)| no_out!(x -= &y)),
        ],
    );
}
