// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::BinomialCoefficient;
use malachite_base::test_util::bench::bucketers::usize_convertible_pair_max_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_28;
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::binomial_coefficient::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::{
    pair_2_pair_natural_max_bit_bucketer, pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_pair_gen_var_15, natural_pair_gen_var_15_rm, unsigned_pair_gen_var_45,
    unsigned_pair_gen_var_46, unsigned_pair_gen_var_47, unsigned_pair_gen_var_48,
    unsigned_pair_gen_var_49,
};
use malachite_nz::test_util::natural::arithmetic::binomial_coefficient::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_binomial_coefficient_limb_limb_bdiv);
    register_demo!(runner, demo_limbs_binomial_coefficient_limb_limb_small_k);
    register_demo!(runner, demo_limbs_binomial_coefficient_limb_limb_basecase);
    register_demo!(
        runner,
        demo_limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer
    );
    register_demo!(
        runner,
        demo_limbs_binomial_coefficient_limb_limb_goetgheluck
    );
    register_demo!(runner, demo_binomial_coefficient_limb_limb);
    register_demo!(runner, demo_natural_binomial_coefficient);
    register_demo!(runner, demo_natural_binomial_coefficient_ref);

    register_bench!(runner, benchmark_binomial_coefficient_limb_limb_algorithms);
    register_bench!(
        runner,
        benchmark_natural_binomial_coefficient_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_binomial_coefficient_algorithms);
    register_bench!(
        runner,
        benchmark_natural_binomial_coefficient_library_comparison
    );
}

fn demo_limbs_binomial_coefficient_limb_limb_bdiv(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, k) in unsigned_pair_gen_var_45().get(gm, config).take(limit) {
        println!(
            "limbs_binomial_coefficient_limb_limb_bdiv({}, {}) = {:?}",
            n,
            k,
            limbs_binomial_coefficient_limb_limb_bdiv(n, k),
        );
    }
}

fn demo_limbs_binomial_coefficient_limb_limb_small_k(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_46().get(gm, config).take(limit) {
        println!(
            "limbs_binomial_coefficient_limb_limb_small_k({}, {}) = {:?}",
            n,
            k,
            limbs_binomial_coefficient_limb_limb_small_k(n, k),
        );
    }
}

fn demo_limbs_binomial_coefficient_limb_limb_basecase(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_47().get(gm, config).take(limit) {
        println!(
            "limbs_binomial_coefficient_limb_limb_basecase({}, {}) = {}",
            n,
            k,
            limbs_binomial_coefficient_limb_limb_basecase(n, k),
        );
    }
}

fn demo_limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_48().get(gm, config).take(limit) {
        println!(
            "limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer({}, {}) = {:?}",
            n,
            k,
            limbs_binomial_coefficient_limb_limb_small_k_divide_and_conquer(n, k),
        );
    }
}

fn demo_limbs_binomial_coefficient_limb_limb_goetgheluck(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, k) in unsigned_pair_gen_var_49().get(gm, config).take(limit) {
        println!(
            "limbs_binomial_coefficient_limb_limb_goetgheluck({}, {}) = {:?}",
            n,
            k,
            limbs_binomial_coefficient_limb_limb_goetgheluck(n, k),
        );
    }
}

fn demo_binomial_coefficient_limb_limb(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, k) in unsigned_pair_gen_var_28().get(gm, config).take(limit) {
        println!(
            "binomial_coefficient_limb_limb({}, {}) = {}",
            n,
            k,
            binomial_coefficient_limb_limb(n, k),
        );
    }
}

fn demo_natural_binomial_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, k) in natural_pair_gen_var_15().get(gm, config).take(limit) {
        let n_orig = n.clone();
        let k_orig = k.clone();
        println!(
            "C({}, {}) = {}",
            n_orig,
            k_orig,
            Natural::binomial_coefficient(n, k)
        );
    }
}

fn demo_natural_binomial_coefficient_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, k) in natural_pair_gen_var_15().get(gm, config).take(limit) {
        println!(
            "C({}, {}) = {}",
            n,
            k,
            Natural::binomial_coefficient(&n, &k)
        );
    }
}

fn benchmark_binomial_coefficient_limb_limb_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "binomial_coefficient_limb_limb(Limb, Limb)",
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_28().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &usize_convertible_pair_max_bucketer("n", "k"),
        &mut [
            ("default", &mut |(n, k)| {
                no_out!(binomial_coefficient_limb_limb(n, k))
            }),
            ("naive", &mut |(n, k)| {
                no_out!(Natural::binomial_coefficient(
                    Natural::from(n),
                    Natural::from(k)
                ))
            }),
        ],
    );
}

fn benchmark_natural_binomial_coefficient_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.binomial_coefficient(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            (
                "Natural.binomial_coefficient(Natural, Natural)",
                &mut |(n, k)| no_out!(Natural::binomial_coefficient(n, k)),
            ),
            (
                "Natural.binomial_coefficient(&Natural, &Natural)",
                &mut |(n, k)| no_out!(Natural::binomial_coefficient(&n, &k)),
            ),
        ],
    );
}

fn benchmark_natural_binomial_coefficient_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.binomial_coefficient(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_15().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(n, k)| {
                no_out!(Natural::binomial_coefficient(n, k))
            }),
            ("naive 1", &mut |(n, k)| {
                no_out!(binomial_coefficient_naive_1(n, k))
            }),
            ("naive 2", &mut |(n, k)| {
                no_out!(binomial_coefficient_naive_2(n, k))
            }),
        ],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_binomial_coefficient_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.binomial_coefficient(Natural, Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_var_15_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (n, k))| {
                no_out!(Natural::binomial_coefficient(n, k))
            }),
            ("rug", &mut |((n, k), _)| no_out!(n.binomial(k))),
        ],
    );
}
