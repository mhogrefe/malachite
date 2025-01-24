// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedDiv, DivRem};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    pair_1_integer_bit_bucketer, pair_2_pair_1_integer_bit_bucketer,
    triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_pair_gen, integer_pair_gen_nm, integer_pair_gen_var_1, integer_pair_gen_var_1_nrm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_div);
    register_demo!(runner, demo_integer_div_val_ref);
    register_demo!(runner, demo_integer_div_ref_val);
    register_demo!(runner, demo_integer_div_ref_ref);
    register_demo!(runner, demo_integer_div_assign);
    register_demo!(runner, demo_integer_div_assign_ref);
    register_demo!(runner, demo_integer_checked_div);
    register_demo!(runner, demo_integer_checked_div_val_ref);
    register_demo!(runner, demo_integer_checked_div_ref_val);
    register_demo!(runner, demo_integer_checked_div_ref_ref);

    register_bench!(runner, benchmark_integer_div_library_comparison);
    register_bench!(runner, benchmark_integer_div_algorithms);
    register_bench!(runner, benchmark_integer_div_evaluation_strategy);
    register_bench!(runner, benchmark_integer_div_assign_evaluation_strategy);
    register_bench!(runner, benchmark_integer_checked_div_library_comparison);
    register_bench!(runner, benchmark_integer_checked_div_evaluation_strategy);
}

fn demo_integer_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} / {} = {}", x_old, y_old, x / y);
    }
}

fn demo_integer_div_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("{} / &{} = {}", x_old, y, x / &y);
    }
}

fn demo_integer_div_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!("&{} / {} = {}", x, y_old, &x / y);
    }
}

fn demo_integer_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        println!("&{} / &{} = {}", x, y, &x / &y);
    }
}

fn demo_integer_div_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x /= y.clone();
        println!("x := {x_old}; x /= {y}; x = {x}");
    }
}

fn demo_integer_div_assign_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen_var_1().get(gm, config).take(limit) {
        let x_old = x.clone();
        x /= &y;
        println!("x := {x_old}; x /= &{y}; x = {x}");
    }
}

fn demo_integer_checked_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).checked_div({}) = {:?}",
            x_old,
            y_old,
            x.checked_div(y)
        );
    }
}

fn demo_integer_checked_div_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({}).checked_div(&{}) = {:?}", x_old, y, x.checked_div(&y));
    }
}

fn demo_integer_checked_div_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).checked_div({}) = {:?}",
            x,
            y_old,
            (&x).checked_div(y)
        );
    }
}

fn demo_integer_checked_div_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, config).take(limit) {
        println!("(&{}).checked_div(&{}) = {:?}", x, y, (&x).checked_div(&y));
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_div_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer / Integer",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_1_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x / y)),
            ("num", &mut |((x, y), _, _)| no_out!(x / y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x / y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer / Integer",
        BenchmarkType::Algorithms,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("standard", &mut |(x, y)| no_out!(x / y)),
            ("using div_rem", &mut |(x, y)| no_out!(x.div_rem(y).0)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer / Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer / Integer", &mut |(x, y)| no_out!(x / y)),
            ("Integer / &Integer", &mut |(x, y)| no_out!(x / &y)),
            ("&Integer / Integer", &mut |(x, y)| no_out!(&x / y)),
            ("&Integer / &Integer", &mut |(x, y)| no_out!(&x / &y)),
        ],
    );
}

fn benchmark_integer_div_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer /= Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer /= Integer", &mut |(mut x, y)| no_out!(x /= y)),
            ("Integer /= &Integer", &mut |(mut x, y)| no_out!(x /= &y)),
        ],
    );
}

fn benchmark_integer_checked_div_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_div(Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_nm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x.checked_div(&y))),
            ("num", &mut |((x, y), _)| no_out!(x.checked_div(&y))),
        ],
    );
}

fn benchmark_integer_checked_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_div(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.checked_div(Integer)", &mut |(x, y)| {
                no_out!(x.checked_div(y))
            }),
            ("Integer.checked_div(&Integer)", &mut |(x, y)| {
                no_out!(x.checked_div(&y))
            }),
            ("(&Integer).checked_div(Integer)", &mut |(x, y)| {
                no_out!((&x).checked_div(y))
            }),
            ("(&Integer).checked_div(&Integer)", &mut |(x, y)| {
                no_out!((&x).checked_div(&y))
            }),
        ],
    );
}
