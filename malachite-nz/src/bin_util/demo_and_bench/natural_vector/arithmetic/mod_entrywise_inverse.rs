// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::Display;
use malachite_base::num::arithmetic::traits::ModEntrywiseInverse;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen_var_2;
use malachite_nz::test_util::natural_vector::arithmetic::mod_entrywise_inverse::*;

fn option_to_string<T: Display>(x: Option<T>) -> String {
    x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
}

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_entrywise_inverse);
    register_demo!(runner, demo_natural_vector_mod_entrywise_inverse_ref);
    register_demo!(runner, demo_natural_vector_mod_entrywise_inverse_units);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_entrywise_inverse_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_vector_mod_entrywise_inverse_evaluation_strategy
    );
}

fn demo_natural_vector_mod_entrywise_inverse(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let (v_old, m_old) = (v.clone(), m.clone());
        println!(
            "{v_old}.mod_entrywise_inverse({m_old}) = {}",
            option_to_string(v.mod_entrywise_inverse(m))
        );
    }
}

fn demo_natural_vector_mod_entrywise_inverse_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_entrywise_inverse(&{m}) = {}",
            option_to_string((&v).mod_entrywise_inverse(&m))
        );
    }
}

// Only invertible elements, so that every line has inverses.
fn demo_natural_vector_mod_entrywise_inverse_units(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, m) in natural_vector_natural_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let u = natural_vector_retain_units(&v, &m);
        println!(
            "(&{u}).mod_entrywise_inverse(&{m}) = {}",
            option_to_string((&u).mod_entrywise_inverse(&m))
        );
    }
}

fn benchmark_natural_vector_mod_entrywise_inverse_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    // Only invertible elements, so that both algorithms do all their work.
    run_benchmark(
        "NaturalVector.mod_entrywise_inverse(&Natural)",
        BenchmarkType::Algorithms,
        natural_vector_natural_pair_gen_var_2()
            .get(gm, config)
            .map(|(v, m)| (natural_vector_retain_units(&v, &m), m)),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, m)| {
                no_out!((&v).mod_entrywise_inverse(&m));
            }),
            ("naive", &mut |(v, m)| {
                no_out!(natural_vector_mod_entrywise_inverse_naive(&v, &m));
            }),
        ],
    );
}

fn benchmark_natural_vector_mod_entrywise_inverse_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_entrywise_inverse(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_pair_gen_var_2()
            .get(gm, config)
            .map(|(v, m)| (natural_vector_retain_units(&v, &m), m)),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.mod_entrywise_inverse(Natural)",
                &mut |(v, m)| {
                    no_out!(v.mod_entrywise_inverse(m));
                },
            ),
            (
                "NaturalVector.mod_entrywise_inverse(&Natural)",
                &mut |(v, m)| {
                    no_out!(v.mod_entrywise_inverse(&m));
                },
            ),
            (
                "(&NaturalVector).mod_entrywise_inverse(Natural)",
                &mut |(v, m)| {
                    no_out!((&v).mod_entrywise_inverse(m));
                },
            ),
            (
                "(&NaturalVector).mod_entrywise_inverse(&Natural)",
                &mut |(v, m)| {
                    no_out!((&v).mod_entrywise_inverse(&m));
                },
            ),
        ],
    );
}
