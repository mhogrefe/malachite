// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedDiv;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_gaussian_rational_max_bit_bucketer;
use malachite_q::test_util::gaussian_rational::arithmetic::div::gaussian_rational_div_naive;
use malachite_q::test_util::generators::{
    gaussian_rational_pair_gen, gaussian_rational_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_rational_div);
    register_demo!(runner, demo_gaussian_rational_div_val_ref);
    register_demo!(runner, demo_gaussian_rational_div_ref_val);
    register_demo!(runner, demo_gaussian_rational_div_ref_ref);
    register_demo!(runner, demo_gaussian_rational_div_assign);
    register_demo!(runner, demo_gaussian_rational_div_assign_ref);
    register_demo!(runner, demo_gaussian_rational_checked_div);
    register_demo!(runner, demo_gaussian_rational_checked_div_val_ref);
    register_demo!(runner, demo_gaussian_rational_checked_div_ref_val);
    register_demo!(runner, demo_gaussian_rational_checked_div_ref_ref);

    register_bench!(runner, benchmark_gaussian_rational_div_algorithms);
    register_bench!(runner, benchmark_gaussian_rational_div_evaluation_strategy);
    register_bench!(
        runner,
        benchmark_gaussian_rational_div_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_gaussian_rational_checked_div_evaluation_strategy
    );
}

fn demo_gaussian_rational_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({x_old}) / ({y_old}) = {}", x / y);
    }
}

fn demo_gaussian_rational_div_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        println!("({x_old}) / (&{y}) = {}", x / &y);
    }
}

fn demo_gaussian_rational_div_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        println!("(&{x}) / ({y_old}) = {}", &x / y);
    }
}

fn demo_gaussian_rational_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{x}) / (&{y}) = {}", &x / &y);
    }
}

fn demo_gaussian_rational_div_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x /= y;
        println!("x := {x_old}; x /= {y_old}; x = {x}");
    }
}

fn demo_gaussian_rational_div_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x /= &y;
        println!("x := {x_old}; x /= &{y}; x = {x}");
    }
}

fn demo_gaussian_rational_checked_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({x_old}).checked_div({y_old}) = {:?}", x.checked_div(y));
    }
}

fn demo_gaussian_rational_checked_div_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({x_old}).checked_div(&{y}) = {:?}", x.checked_div(&y));
    }
}

fn demo_gaussian_rational_checked_div_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("(&{x}).checked_div({y_old}) = {:?}", (&x).checked_div(y));
    }
}

fn demo_gaussian_rational_checked_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_rational_pair_gen().get(gm, config).take(limit) {
        println!("(&{x}).checked_div(&{y}) = {:?}", (&x).checked_div(&y));
    }
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_gaussian_rational_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational / GaussianRational",
        BenchmarkType::Algorithms,
        gaussian_rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x / y)),
            ("naive", &mut |(x, y)| {
                no_out!(gaussian_rational_div_naive(&x, &y));
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_gaussian_rational_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational / GaussianRational",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("GaussianRational / GaussianRational", &mut |(x, y)| {
                no_out!(x / y);
            }),
            ("GaussianRational / &GaussianRational", &mut |(x, y)| {
                no_out!(x / &y);
            }),
            ("&GaussianRational / GaussianRational", &mut |(x, y)| {
                no_out!(&x / y);
            }),
            ("&GaussianRational / &GaussianRational", &mut |(x, y)| {
                no_out!(&x / &y);
            }),
        ],
    );
}

fn benchmark_gaussian_rational_div_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational /= GaussianRational",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_rational_max_bit_bucketer("x", "y"),
        &mut [
            ("GaussianRational /= GaussianRational", &mut |(mut x, y)| {
                x /= y;
            }),
            (
                "GaussianRational /= &GaussianRational",
                &mut |(mut x, y)| x /= &y,
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_gaussian_rational_checked_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianRational.checked_div(GaussianRational)",
        BenchmarkType::EvaluationStrategy,
        gaussian_rational_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_rational_max_bit_bucketer("x", "y"),
        &mut [
            (
                "GaussianRational.checked_div(GaussianRational)",
                &mut |(x, y)| {
                    no_out!(x.checked_div(y));
                },
            ),
            (
                "GaussianRational.checked_div(&GaussianRational)",
                &mut |(x, y)| {
                    no_out!(x.checked_div(&y));
                },
            ),
            (
                "(&GaussianRational).checked_div(GaussianRational)",
                &mut |(x, y)| {
                    no_out!((&x).checked_div(y));
                },
            ),
            (
                "(&GaussianRational).checked_div(&GaussianRational)",
                &mut |(x, y)| {
                    no_out!((&x).checked_div(&y));
                },
            ),
        ],
    );
}
