// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::Display;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Div, Parity};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_unsigned_triple_gen_var_1;
use malachite_nz::test_util::natural_vector::arithmetic::mod_power_of_2_div::*;

fn option_to_string<T: Display>(x: Option<T>) -> String {
    x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
}

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_div);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_div_ref);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_div_odd_divisor);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_div_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_div_evaluation_strategy
    );
}

fn demo_natural_vector_mod_power_of_2_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, pow) in natural_vector_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let (v_old, c_old) = (v.clone(), c.clone());
        println!(
            "{v_old}.mod_power_of_2_div({c_old}, {pow}) = {}",
            option_to_string(v.mod_power_of_2_div(c, pow))
        );
    }
}

fn demo_natural_vector_mod_power_of_2_div_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, pow) in natural_vector_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_div({c}, {pow}) = {}",
            option_to_string((&v).mod_power_of_2_div(&c, pow))
        );
    }
}

// Only odd divisors, the case that FLINT's _fmpz_mod_vec_scalar_div_fmpz_mod handles.
fn demo_natural_vector_mod_power_of_2_div_odd_divisor(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, c, pow) in natural_vector_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .filter(|(_, c, _)| c.odd())
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_div({c}, {pow}) = {}",
            option_to_string((&v).mod_power_of_2_div(&c, pow))
        );
    }
}

fn benchmark_natural_vector_mod_power_of_2_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2_div(Natural, u64)",
        BenchmarkType::Algorithms,
        natural_vector_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, c, pow)| {
                no_out!((&v).mod_power_of_2_div(&c, pow));
            }),
            ("naive", &mut |(v, c, pow)| {
                no_out!(natural_vector_mod_power_of_2_div_naive(&v, &c, pow));
            }),
        ],
    );
}

fn benchmark_natural_vector_mod_power_of_2_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2_div(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.mod_power_of_2_div(Natural, u64)",
                &mut |(v, c, pow)| {
                    no_out!(v.mod_power_of_2_div(c, pow));
                },
            ),
            (
                "(&NaturalVector).mod_power_of_2_div(Natural, u64)",
                &mut |(v, c, pow)| {
                    no_out!((&v).mod_power_of_2_div(&c, pow));
                },
            ),
        ],
    );
}
