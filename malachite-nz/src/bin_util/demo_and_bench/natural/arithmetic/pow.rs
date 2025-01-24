// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::{
    pair_1_bits_times_pair_2_bucketer, pair_1_vec_len_times_pair_2_bucketer,
    triple_3_pair_1_bits_times_pair_2_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_31;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::pow::limbs_pow;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_4, natural_unsigned_pair_gen_var_4_nrm,
};
use malachite_nz::test_util::natural::arithmetic::pow::{
    natural_pow_naive, natural_pow_simple_binary,
};
use num::traits::Pow as NumPow;
use rug::ops::Pow as RugPow;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_pow);
    register_demo!(runner, demo_natural_pow_assign);
    register_demo!(runner, demo_natural_pow);
    register_demo!(runner, demo_natural_pow_ref);

    register_bench!(runner, benchmark_limbs_pow);
    register_bench!(runner, benchmark_natural_pow_assign);
    register_bench!(runner, benchmark_natural_pow_algorithms);
    register_bench!(runner, benchmark_natural_pow_library_comparison);
    register_bench!(runner, benchmark_natural_pow_evaluation_strategy);
}

fn demo_limbs_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, exp) in unsigned_vec_unsigned_pair_gen_var_31()
        .get(gm, config)
        .take(limit)
    {
        println!("limbs_pow({:?}, {}) = {:?}", xs, exp, limbs_pow(&xs, exp));
    }
}

fn demo_natural_pow_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, pow) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        n.pow_assign(pow);
        println!("x := {n_old}; x.pow_assign({pow}); x = {n}");
    }
}

fn demo_natural_pow(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n.clone();
        println!("{}.pow({}) = {}", n_old, pow, n.pow(pow));
    }
}

fn demo_natural_pow_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in natural_unsigned_pair_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{}).pow({}) = {}", n, pow, (&n).pow(pow));
    }
}

fn benchmark_limbs_pow(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_pow(&[Limb], u64)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_31().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_times_pair_2_bucketer("xs", "exp"),
        &mut [("Malachite", &mut |(ref xs, exp)| {
            no_out!(limbs_pow(xs, exp))
        })],
    );
}

fn benchmark_natural_pow_assign(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.pow_assign(u64)",
        BenchmarkType::Single,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("x", "exp"),
        &mut [("Malachite", &mut |(mut x, exp)| x.pow_assign(exp))],
    );
}

fn benchmark_natural_pow_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.pow(u64)",
        BenchmarkType::Algorithms,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("x", "exp"),
        &mut [
            ("default", &mut |(x, exp)| no_out!((&x).pow(exp))),
            ("naive", &mut |(x, exp)| no_out!(natural_pow_naive(&x, exp))),
            ("simple binary", &mut |(x, exp)| {
                no_out!(natural_pow_simple_binary(&x, exp))
            }),
            ("alt", &mut |(x, exp)| no_out!(x.pow_ref_alt(exp))),
        ],
    );
}

fn benchmark_natural_pow_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.pow(u64)",
        BenchmarkType::LibraryComparison,
        natural_unsigned_pair_gen_var_4_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_bits_times_pair_2_bucketer("x", "exp"),
        &mut [
            ("Malachite", &mut |(_, _, (x, exp))| no_out!(x.pow(exp))),
            ("num", &mut |((x, exp), _, _)| no_out!(x.pow(exp))),
            ("rug", &mut |(_, (x, exp), _)| {
                no_out!(x.pow(u32::exact_from(exp)))
            }),
        ],
    );
}

fn benchmark_natural_pow_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.pow(u64)",
        BenchmarkType::EvaluationStrategy,
        natural_unsigned_pair_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bits_times_pair_2_bucketer("x", "exp"),
        &mut [
            ("Natural.pow(u64)", &mut |(x, exp)| no_out!(x.pow(exp))),
            (
                "(&Natural).pow(u64)",
                &mut |(x, exp)| no_out!((&x).pow(exp)),
            ),
        ],
    );
}
