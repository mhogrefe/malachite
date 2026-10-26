// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Inverse, Pow, PowAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_matrix_bit_bucketer;
use malachite_q::test_util::generators::{
    rational_matrix_signed_pair_gen_var_2, rational_matrix_unsigned_pair_gen_var_4,
};
use malachite_q::test_util::rational_matrix::arithmetic::pow::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_matrix_pow);
    register_demo!(runner, demo_rational_matrix_pow_ref);
    register_demo!(runner, demo_rational_matrix_pow_assign);
    register_bench!(runner, benchmark_rational_matrix_pow_evaluation_strategy);
    register_bench!(runner, benchmark_rational_matrix_pow_algorithms);
    register_demo!(runner, demo_rational_matrix_pow_signed);
    register_demo!(runner, demo_rational_matrix_pow_signed_ref);
    register_demo!(runner, demo_rational_matrix_pow_signed_assign);
    register_bench!(
        runner,
        benchmark_rational_matrix_pow_signed_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_matrix_pow_signed_algorithms);
}

fn demo_rational_matrix_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp) in rational_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.pow({exp}) = {}", a.pow(exp));
    }
}

fn demo_rational_matrix_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp) in rational_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).pow({exp}) = {}", (&a).pow(exp));
    }
}

fn demo_rational_matrix_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, exp) in rational_matrix_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.pow_assign(exp);
        println!("a := {a_old}; a.pow_assign({exp}); a = {a}");
    }
}

fn benchmark_rational_matrix_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.pow(u64)", &mut |(a, exp)| {
                no_out!(a.pow(exp));
            }),
            ("(&RationalMatrix).pow(u64)", &mut |(a, exp)| {
                no_out!((&a).pow(exp));
            }),
            ("RationalMatrix.pow_assign(u64)", &mut |(mut a, exp)| {
                a.pow_assign(exp);
            }),
        ],
    );
}

fn benchmark_rational_matrix_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalMatrix).pow(u64)",
        BenchmarkType::Algorithms,
        rational_matrix_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, exp)| {
                no_out!((&a).pow(exp));
            }),
            ("matwise denominators", &mut |(a, exp)| {
                no_out!(rational_matrix_pow_matwise(&a, exp));
            }),
            ("naive", &mut |(a, exp)| {
                no_out!(rational_matrix_pow_naive(&a, exp));
            }),
        ],
    );
}

fn demo_rational_matrix_pow_signed(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp) in rational_matrix_signed_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.pow({exp}) = {}", a.pow(exp));
    }
}

fn demo_rational_matrix_pow_signed_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, exp) in rational_matrix_signed_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).pow({exp}) = {}", (&a).pow(exp));
    }
}

fn demo_rational_matrix_pow_signed_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, exp) in rational_matrix_signed_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.pow_assign(exp);
        println!("a := {a_old}; a.pow_assign({exp}); a = {a}");
    }
}

fn benchmark_rational_matrix_pow_signed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalMatrix.pow(i64)",
        BenchmarkType::EvaluationStrategy,
        rational_matrix_signed_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("RationalMatrix.pow(i64)", &mut |(a, exp)| {
                no_out!(a.pow(exp));
            }),
            ("(&RationalMatrix).pow(i64)", &mut |(a, exp)| {
                no_out!((&a).pow(exp));
            }),
            ("RationalMatrix.pow_assign(i64)", &mut |(mut a, exp)| {
                a.pow_assign(exp);
            }),
        ],
    );
}

fn benchmark_rational_matrix_pow_signed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&RationalMatrix).pow(i64)",
        BenchmarkType::Algorithms,
        rational_matrix_signed_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_matrix_bit_bucketer("a"),
        &mut [
            ("inverse, then power", &mut |(a, exp)| {
                no_out!((&a).pow(exp));
            }),
            ("power, then inverse", &mut |(a, exp)| {
                let p = (&a).pow(exp.unsigned_abs());
                no_out!(if exp >= 0 { p } else { p.inverse().unwrap() });
            }),
        ],
    );
}
