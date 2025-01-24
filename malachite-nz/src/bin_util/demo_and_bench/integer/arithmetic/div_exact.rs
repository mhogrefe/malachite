// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen_var_2, integer_pair_gen_var_2_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_div_exact);
    register_demo!(runner, demo_integer_div_exact_val_ref);
    register_demo!(runner, demo_integer_div_exact_ref_val);
    register_demo!(runner, demo_integer_div_exact_ref_ref);
    register_demo!(runner, demo_integer_div_exact_assign);
    register_demo!(runner, demo_integer_div_exact_assign_ref);

    register_bench!(runner, benchmark_integer_div_exact_library_comparison);
    register_bench!(runner, benchmark_integer_div_exact_algorithms);
    register_bench!(runner, benchmark_integer_div_exact_evaluation_strategy);
    register_bench!(runner, benchmark_integer_div_exact_assign_algorithms);
    register_bench!(
        runner,
        benchmark_integer_div_exact_assign_evaluation_strategy
    );
}

fn demo_integer_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.div_exact({}) = {}", x_old, y_old, x.div_exact(y));
    }
}

fn demo_integer_div_exact_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{}.div_exact(&{}) = {}", x_old, y, x.div_exact(&y));
    }
}

fn demo_integer_div_exact_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_2().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).div_exact({}) = {}", x, y_old, (&x).div_exact(y));
    }
}

fn demo_integer_div_exact_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_2().get(gm, config).take(limit) {
        println!("(&{}).div_exact(&{}) = {}", x, y, (&x).div_exact(&y));
    }
}

fn demo_integer_div_exact_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.div_exact_assign(y.clone());
        println!("x := {x_old}; x.div_exact_assign({y}); x = {x}");
    }
}

fn demo_integer_div_exact_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_2().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.div_exact_assign(&y);
        println!("x := {x_old}; x.div_exact_assign(&{y}); x = {x}");
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_div_exact_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_exact(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_2_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("num", &mut |((x, y), _, _)| no_out!(x / y)),
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.div_exact(y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.div_exact(&y))),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_exact(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("ordinary division", &mut |(x, y)| no_out!(x / y)),
            ("exact division", &mut |(x, y)| no_out!(x.div_exact(y))),
        ],
    );
}

fn benchmark_integer_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.div_exact(Integer)", &mut |(x, y)| {
                no_out!(x.div_exact(y))
            }),
            ("Integer.div_exact(&Integer)", &mut |(x, y)| {
                no_out!(x.div_exact(&y))
            }),
            ("(&Integer).div_exact(Integer)", &mut |(x, y)| {
                no_out!((&x).div_exact(y))
            }),
            ("(&Integer).div_exact(&Integer)", &mut |(x, y)| {
                no_out!((&x).div_exact(&y))
            }),
        ],
    );
}

fn benchmark_integer_div_exact_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_exact_assign(Integer)",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("ordinary division", &mut |(mut x, y)| x /= y),
            ("exact division", &mut |(mut x, y)| x.div_exact_assign(y)),
        ],
    );
}

fn benchmark_integer_div_exact_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.div_exact_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.div_exact_assign(Integer)", &mut |(mut x, y)| {
                x.div_exact_assign(y)
            }),
            ("Integer.div_exact_assign(&Integer)", &mut |(mut x, y)| {
                x.div_exact_assign(&y)
            }),
        ],
    );
}
