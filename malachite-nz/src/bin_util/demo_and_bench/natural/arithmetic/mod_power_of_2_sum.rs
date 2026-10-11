// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Add, ModPowerOf2Sum};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_sum);
    register_demo!(runner, demo_natural_mod_power_of_2_sum_ref);

    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_sum_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_sum_algorithms);
}

fn demo_natural_mod_power_of_2_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "Natural::mod_power_of_2_sum({v_old}, {pow}) = {}",
            Natural::mod_power_of_2_sum(v.elements.into_iter(), pow)
        );
    }
}

fn demo_natural_mod_power_of_2_sum_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, pow) in natural_vector_unsigned_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::mod_power_of_2_sum(&{v}, {pow}) = {}",
            Natural::mod_power_of_2_sum(v.elements.iter(), pow)
        );
    }
}

fn benchmark_natural_mod_power_of_2_sum_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::mod_power_of_2_sum(Iterator<Item = Natural>, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("xs"),
        &mut [
            (
                "Natural::mod_power_of_2_sum(Iterator<Item = Natural>, u64)",
                &mut |(v, pow)| {
                    no_out!(Natural::mod_power_of_2_sum(v.elements.into_iter(), pow));
                },
            ),
            (
                "Natural::mod_power_of_2_sum(Iterator<Item = &Natural>, u64)",
                &mut |(v, pow)| {
                    no_out!(Natural::mod_power_of_2_sum(v.elements.iter(), pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_sum_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::mod_power_of_2_sum(Iterator<Item = &Natural>, u64)",
        BenchmarkType::Algorithms,
        natural_vector_unsigned_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("xs"),
        &mut [
            ("default", &mut |(v, pow)| {
                no_out!(Natural::mod_power_of_2_sum(v.elements.iter(), pow));
            }),
            ("sum then reduce", &mut |(v, pow)| {
                no_out!(Natural::sum(v.elements.iter()).mod_power_of_2(pow));
            }),
            ("fold", &mut |(v, pow)| {
                no_out!(
                    v.elements
                        .iter()
                        .fold(Natural::ZERO, |s, x| s.mod_power_of_2_add(x, pow))
                );
            }),
        ],
    );
}
