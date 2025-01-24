// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::pair_1_bits_times_pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_3_pair_1_integer_bits_times_pair_2_bucketer;
use malachite_nz::test_util::generators::{
    integer_unsigned_pair_gen_var_2, integer_unsigned_pair_gen_var_2_nrm,
};
use rug::ops::Pow as RugPow;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_pow_assign);
    register_demo!(runner, demo_integer_pow);
    register_demo!(runner, demo_integer_pow_ref);

    register_bench!(runner, benchmark_integer_pow_assign);
    register_bench!(runner, benchmark_integer_pow_library_comparison);
    register_bench!(runner, benchmark_integer_pow_evaluation_strategy);
}

fn demo_integer_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, pow) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.pow_assign(pow);
        println!("x := {n_old}; x.pow_assign({pow}); x = {n}");
    }
}

fn demo_integer_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("({}).pow({}) = {}", n_old, pow, n.pow(pow));
    }
}

fn demo_integer_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in integer_unsigned_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).pow({}) = {}", n, pow, (&n).pow(pow));
    }
}

fn benchmark_integer_pow_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.pow_assign(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("n", "pow"),
        &mut [("Malachite", &mut |(mut x, exp)| x.pow_assign(exp))],
    );
}

fn benchmark_integer_pow_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.pow_assign(u64)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_var_2_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bits_times_pair_2_bucketer("n", "pow"),
        &mut [
            ("Malachite", &mut |(_, _, (x, exp))| no_out!(x.pow(exp))),
            ("num", &mut |((x, exp), _, _)| {
                no_out!(x.pow(u32::exact_from(exp)))
            }),
            ("rug", &mut |(_, (x, exp), _)| {
                no_out!(x.pow(u32::exact_from(exp)))
            }),
        ],
    );
}

fn benchmark_integer_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("n", "pow"),
        &mut [
            ("Integer.pow(u64)", &mut |(x, exp)| no_out!(x.pow(exp))),
            (
                "(&Integer).pow(u64)",
                &mut |(x, exp)| no_out!((&x).pow(exp)),
            ),
        ],
    );
}
