// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BinomialCoefficient;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_integer_max_bit_bucketer, pair_integer_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{integer_pair_gen_var_7, integer_pair_gen_var_7_rm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_binomial_coefficient);
    register_demo!(runner, demo_integer_binomial_coefficient_ref);

    register_bench!(
        runner,
        benchmark_integer_binomial_coefficient_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_integer_binomial_coefficient_library_comparison
    );
}

fn demo_integer_binomial_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, k) in integer_pair_gen_var_7().get(gm, config).take(limit) {
        let n_orig = n.clone();
        let k_orig = k.clone();
        println!(
            "C({}, {}) = {}",
            n_orig,
            k_orig,
            Integer::binomial_coefficient(n, k)
        );
    }
}

fn demo_integer_binomial_coefficient_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, k) in integer_pair_gen_var_7().get(gm, config).take(limit) {
        println!(
            "C({}, {}) = {}",
            n,
            k,
            Integer::binomial_coefficient(&n, &k)
        );
    }
}

fn benchmark_integer_binomial_coefficient_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.binomial_coefficient(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen_var_7().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            (
                "Integer.binomial_coefficient(Integer, Integer)",
                &mut |(n, k)| no_out!(Integer::binomial_coefficient(n, k)),
            ),
            (
                "Integer.binomial_coefficient(&Integer, &Integer)",
                &mut |(n, k)| no_out!(Integer::binomial_coefficient(&n, &k)),
            ),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_binomial_coefficient_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.binomial_coefficient(Integer, Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_var_7_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (n, k))| {
                no_out!(Integer::binomial_coefficient(n, k))
            }),
            ("rug", &mut |((n, k), _)| no_out!(n.binomial(k))),
        ],
    );
}
