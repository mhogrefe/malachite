// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::Display;
use malachite_base::num::arithmetic::traits::{CoprimeWith, ModDiv};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_natural_triple_gen_var_1;
use malachite_nz::test_util::natural_vector::arithmetic::mod_div::*;

fn option_to_string<T: Display>(x: Option<T>) -> String {
    x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
}

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_div);
    register_demo!(runner, demo_natural_vector_mod_div_ref);
    register_demo!(runner, demo_natural_vector_mod_div_invertible_divisor);
    register_bench!(runner, benchmark_natural_vector_mod_div_algorithms);
    register_bench!(runner, benchmark_natural_vector_mod_div_evaluation_strategy);
}

fn demo_natural_vector_mod_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, m) in natural_vector_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (v_old, c_old, m_old) = (v.clone(), c.clone(), m.clone());
        println!(
            "{v_old}.mod_div({c_old}, {m_old}) = {}",
            option_to_string(v.mod_div(c, m))
        );
    }
}

fn demo_natural_vector_mod_div_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, m) in natural_vector_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_div({c}, {m}) = {}",
            option_to_string((&v).mod_div(&c, &m))
        );
    }
}

// Only divisors invertible modulo m, the case that FLINT's function handles.
fn demo_natural_vector_mod_div_invertible_divisor(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, m) in natural_vector_natural_natural_triple_gen_var_1()
        .get(gm, config)
        .filter(|(_, c, m)| c.coprime_with(m))
        .take(limit)
    {
        println!(
            "(&{v}).mod_div({c}, {m}) = {}",
            option_to_string((&v).mod_div(&c, &m))
        );
    }
}

fn benchmark_natural_vector_mod_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_div(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_vector_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, c, m)| {
                no_out!((&v).mod_div(&c, &m));
            }),
            ("naive", &mut |(v, c, m)| {
                no_out!(natural_vector_mod_div_naive(&v, &c, &m));
            }),
        ],
    );
}

fn benchmark_natural_vector_mod_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_div(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.mod_div(Natural, Natural)",
                &mut |(v, c, m)| {
                    no_out!(v.mod_div(c, m));
                },
            ),
            (
                "(&NaturalVector).mod_div(Natural, Natural)",
                &mut |(v, c, m)| {
                    no_out!((&v).mod_div(&c, &m));
                },
            ),
        ],
    );
}
