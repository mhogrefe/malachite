// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use malachite_base::num::arithmetic::traits::{ModAdd, ModSum};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_sum);
    register_demo!(runner, demo_natural_mod_sum_ref);

    register_bench!(runner, benchmark_natural_mod_sum_evaluation_strategy);
    register_bench!(runner, benchmark_natural_mod_sum_algorithms);
}

fn demo_natural_mod_sum(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let m_old = m.clone();
        println!(
            "Natural::mod_sum({v_old}, {m_old}) = {}",
            Natural::mod_sum(v.elements.into_iter(), m)
        );
    }
}

fn demo_natural_mod_sum_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "Natural::mod_sum(&{v}, {m}) = {}",
            Natural::mod_sum(v.elements.iter(), &m)
        );
    }
}

fn benchmark_natural_mod_sum_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::mod_sum(Iterator<Item = Natural>, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("xs"),
        &mut [
            (
                "Natural::mod_sum(Iterator<Item = Natural>, Natural)",
                &mut |(v, m)| {
                    no_out!(Natural::mod_sum(v.elements.into_iter(), m));
                },
            ),
            (
                "Natural::mod_sum(Iterator<Item = &Natural>, Natural)",
                &mut |(v, m)| {
                    no_out!(Natural::mod_sum(v.elements.iter(), &m));
                },
            ),
        ],
    );
}

fn benchmark_natural_mod_sum_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::mod_sum(Iterator<Item = &Natural>, Natural)",
        BenchmarkType::Algorithms,
        natural_vector_natural_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("xs"),
        &mut [
            ("default", &mut |(v, m)| {
                no_out!(Natural::mod_sum(v.elements.iter(), &m));
            }),
            ("sum then reduce", &mut |(v, m)| {
                no_out!(Natural::sum(v.elements.iter()) % m);
            }),
            ("fold", &mut |(v, m)| {
                no_out!(
                    v.elements
                        .iter()
                        .fold(Natural::ZERO, |s, x| s.mod_add(x, &m))
                );
            }),
        ],
    );
}
