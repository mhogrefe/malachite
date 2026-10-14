// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::fmt::Display;
use malachite_base::num::arithmetic::traits::{CoprimeWith, ModDiv};
use malachite_base::test_util::bench::bucketers::triple_1_unsigned_vector_dimension_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vector_unsigned_unsigned_triple_gen_var_2;
use malachite_base::test_util::runner::Runner;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_div::*;

fn option_to_string<T: Display>(x: Option<T>) -> String {
    x.map_or_else(|| "None".to_string(), |x| format!("Some({x})"))
}

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_vector_mod_div);
    register_demo!(runner, demo_unsigned_vector_mod_div_ref);
    register_demo!(runner, demo_unsigned_vector_mod_div_invertible_divisor);
    register_bench!(runner, benchmark_unsigned_vector_mod_div_algorithms);
    register_bench!(
        runner,
        benchmark_unsigned_vector_mod_div_evaluation_strategy
    );
}

fn demo_unsigned_vector_mod_div(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.mod_div({c}, {m}) = {}",
            option_to_string(v.mod_div(c, m))
        );
    }
}

fn demo_unsigned_vector_mod_div_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_div({c}, {m}) = {}",
            option_to_string((&v).mod_div(c, m))
        );
    }
}

// Only divisors invertible modulo m, the case that FLINT's function handles.
fn demo_unsigned_vector_mod_div_invertible_divisor(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, m) in unsigned_vector_unsigned_unsigned_triple_gen_var_2::<u64>()
        .get(gm, config)
        .filter(|(_, c, m)| c.coprime_with(*m))
        .take(limit)
    {
        println!(
            "(&{v}).mod_div({c}, {m}) = {}",
            option_to_string((&v).mod_div(c, m))
        );
    }
}

fn benchmark_unsigned_vector_mod_div_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_div(u64, u64)",
        BenchmarkType::Algorithms,
        unsigned_vector_unsigned_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("default", &mut |(v, c, m)| {
                no_out!((&v).mod_div(c, m));
            }),
            ("naive", &mut |(v, c, m)| {
                no_out!(unsigned_vector_mod_div_naive(&v, c, m));
            }),
        ],
    );
}

fn benchmark_unsigned_vector_mod_div_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedVector<u64>.mod_div(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vector_unsigned_unsigned_triple_gen_var_2::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_unsigned_vector_dimension_bucketer("v"),
        &mut [
            ("UnsignedVector<u64>.mod_div(u64, u64)", &mut |(v, c, m)| {
                no_out!(v.mod_div(c, m));
            }),
            (
                "(&UnsignedVector<u64>).mod_div(u64, u64)",
                &mut |(v, c, m)| {
                    no_out!((&v).mod_div(c, m));
                },
            ),
        ],
    );
}
