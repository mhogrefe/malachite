// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivAssignRem, DivRem};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_gaussian_integer_max_bit_bucketer;
use malachite_nz::test_util::gaussian_integer::arithmetic::div_rem::*;
use malachite_nz::test_util::generators::gaussian_integer_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_gaussian_integer_div_rem);
    register_demo!(runner, demo_gaussian_integer_div_rem_val_ref);
    register_demo!(runner, demo_gaussian_integer_div_rem_ref_val);
    register_demo!(runner, demo_gaussian_integer_div_rem_ref_ref);
    register_demo!(runner, demo_gaussian_integer_div_assign_rem);
    register_demo!(runner, demo_gaussian_integer_div_assign_rem_ref);

    register_bench!(runner, benchmark_gaussian_integer_div_rem_algorithms);
    register_bench!(
        runner,
        benchmark_gaussian_integer_div_rem_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_gaussian_integer_div_assign_rem_evaluation_strategy
    );
}

fn demo_gaussian_integer_div_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let (q, r) = x.div_rem(y);
        println!("({x_old}).div_rem({y_old}) = ({q}, {r})");
    }
}

fn demo_gaussian_integer_div_rem_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let (q, r) = x.div_rem(&y);
        println!("({x_old}).div_rem(&{y}) = ({q}, {r})");
    }
}

fn demo_gaussian_integer_div_rem_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let y_old = y.clone();
        let (q, r) = (&x).div_rem(y);
        println!("(&{x}).div_rem({y_old}) = ({q}, {r})");
    }
}

fn demo_gaussian_integer_div_rem_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in gaussian_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let (q, r) = (&x).div_rem(&y);
        println!("(&{x}).div_rem(&{y}) = ({q}, {r})");
    }
}

fn demo_gaussian_integer_div_assign_rem(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let r = x.div_assign_rem(y);
        println!("x := {x_old}; x.div_assign_rem({y_old}) = {r}; x = {x}");
    }
}

fn demo_gaussian_integer_div_assign_rem_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in gaussian_integer_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let r = x.div_assign_rem(&y);
        println!("x := {x_old}; x.div_assign_rem(&{y}) = {r}; x = {x}");
    }
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_div_rem_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.div_rem(GaussianInteger)",
        BenchmarkType::Algorithms,
        gaussian_integer_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| {
                no_out!((&x).div_rem(&y));
            }),
            ("naive", &mut |(x, y)| {
                no_out!(gaussian_integer_div_rem_naive(&x, &y));
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_div_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.div_rem(GaussianInteger)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("GaussianInteger.div_rem(GaussianInteger)", &mut |(x, y)| {
                no_out!(x.div_rem(y));
            }),
            (
                "GaussianInteger.div_rem(&GaussianInteger)",
                &mut |(x, y)| {
                    no_out!(x.div_rem(&y));
                },
            ),
            (
                "(&GaussianInteger).div_rem(GaussianInteger)",
                &mut |(x, y)| {
                    no_out!((&x).div_rem(y));
                },
            ),
            (
                "(&GaussianInteger).div_rem(&GaussianInteger)",
                &mut |(x, y)| {
                    no_out!((&x).div_rem(&y));
                },
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_gaussian_integer_div_assign_rem_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "GaussianInteger.div_assign_rem(GaussianInteger)",
        BenchmarkType::EvaluationStrategy,
        gaussian_integer_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_gaussian_integer_max_bit_bucketer("x", "y"),
        &mut [
            (
                "GaussianInteger.div_assign_rem(GaussianInteger)",
                &mut |(mut x, y)| {
                    no_out!(x.div_assign_rem(y));
                },
            ),
            (
                "GaussianInteger.div_assign_rem(&GaussianInteger)",
                &mut |(mut x, y)| {
                    no_out!(x.div_assign_rem(&y));
                },
            ),
        ],
    );
}
