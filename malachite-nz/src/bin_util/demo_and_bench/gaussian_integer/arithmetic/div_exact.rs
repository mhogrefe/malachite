// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_gaussian_integer_max_bit_bucketer;
use malachite_nz::test_util::gaussian_integer::arithmetic::div_exact::*;
use malachite_nz::test_util::generators::{
    gaussian_integer_pair_gen_var_1, gaussian_integer_pair_gen_var_2,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_div_exact_assign);
    register_demo!(runner, demo_gaussian_integer_div_exact_assign_ref);
    register_demo!(runner, demo_gaussian_integer_div_exact);
    register_demo!(runner, demo_gaussian_integer_div_exact_val_ref);
    register_demo!(runner, demo_gaussian_integer_div_exact_ref_val);
    register_demo!(runner, demo_gaussian_integer_div_exact_ref_ref);

    register_bench!(runner, benchmark_gaussian_integer_div_exact_algorithms);
    register_bench!(
        runner,
        benchmark_gaussian_integer_div_exact_small_quotient_algorithms
    );
    register_bench!(
        runner,
        benchmark_gaussian_integer_div_exact_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_gaussian_integer_div_exact_assign_evaluation_strategy
    );
}

fn demo_gaussian_integer_div_exact_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.div_exact_assign(y);
        println!("x := {x_old}; x.div_exact_assign({y_old}); x = {x}");
    }
}

fn demo_gaussian_integer_div_exact_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.div_exact_assign(&y);
        println!("x := {x_old}; x.div_exact_assign(&{y}); x = {x}");
    }
}

fn demo_gaussian_integer_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({x_old}).div_exact({y_old}) = {}", x.div_exact(y));
    }
}

fn demo_gaussian_integer_div_exact_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({x_old}).div_exact(&{y}) = {}", x.div_exact(&y));
    }
}

fn demo_gaussian_integer_div_exact_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!("(&{x}).div_exact({y_old}) = {}", (&x).div_exact(y));
    }
}

fn demo_gaussian_integer_div_exact_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{x}).div_exact(&{y}) = {}", (&x).div_exact(&y));
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.div_exact(GaussianInteger)",
        BenchmarkType::Algorithms,
        gaussian_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!((&x).div_exact(&y));
            }),
            ("naive", &mut |(x, y)| {
                no_out!(gaussian_integer_div_exact_naive(&x, &y));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_div_exact_small_quotient_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.div_exact(GaussianInteger) (small quotient)",
        BenchmarkType::Algorithms,
        gaussian_integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!((&x).div_exact(&y));
            }),
            ("naive", &mut |(x, y)| {
                no_out!(gaussian_integer_div_exact_naive(&x, &y));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.div_exact(GaussianInteger)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            (
                "GaussianInteger.div_exact(GaussianInteger)",
                &mut |(x, y)| {
                    no_out!(x.div_exact(y));
                },
            ),
            (
                "GaussianInteger.div_exact(&GaussianInteger)",
                &mut |(x, y)| {
                    no_out!(x.div_exact(&y));
                },
            ),
            (
                "(&GaussianInteger).div_exact(GaussianInteger)",
                &mut |(x, y)| {
                    no_out!((&x).div_exact(y));
                },
            ),
            (
                "(&GaussianInteger).div_exact(&GaussianInteger)",
                &mut |(x, y)| {
                    no_out!((&x).div_exact(&y));
                },
            ),
        ],
    );
}

fn benchmark_gaussian_integer_div_exact_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.div_exact_assign(GaussianInteger)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            (
                "GaussianInteger.div_exact_assign(GaussianInteger)",
                &mut |(mut x, y)| {
                    x.div_exact_assign(y);
                },
            ),
            (
                "GaussianInteger.div_exact_assign(&GaussianInteger)",
                &mut |(mut x, y)| {
                    x.div_exact_assign(&y);
                },
            ),
        ],
    );
}
