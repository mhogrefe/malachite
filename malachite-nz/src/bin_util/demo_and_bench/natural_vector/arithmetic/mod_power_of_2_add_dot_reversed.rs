// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddDotReversed, ModPowerOf2AddDotReversedAssign,
    ModPowerOf2DotReversed,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_dot_reversed);
    register_demo!(
        runner,
        demo_natural_vector_mod_power_of_2_add_dot_reversed_ref
    );
    register_demo!(
        runner,
        demo_natural_vector_mod_power_of_2_add_dot_reversed_assign
    );
    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_add_dot_reversed_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_add_dot_reversed_evaluation_strategy
    );
}

fn demo_natural_vector_mod_power_of_2_add_dot_reversed(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, x, pow) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        let w_old = w.clone();
        println!(
            "{x_old}.mod_power_of_2_add_dot_reversed({v_old}, {w_old}, {pow}) = {}",
            x.mod_power_of_2_add_dot_reversed(v, w, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_dot_reversed_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, x, pow) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mod_power_of_2_add_dot_reversed(&{v}, &{w}, {pow}) = {}",
            (&x).mod_power_of_2_add_dot_reversed(&v, &w, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_dot_reversed_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (v, w, mut x, pow) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_power_of_2_add_dot_reversed_assign(&v, &w, pow);
        println!(
            "x := {x_old}; x.mod_power_of_2_add_dot_reversed_assign(&{v}, &{w}, {pow}); x = {x}"
        );
    }
}

fn benchmark_natural_vector_mod_power_of_2_add_dot_reversed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_add_dot_reversed(NaturalVector, NaturalVector)",
        BenchmarkType::Algorithms,
        natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, w, x, pow)| {
                no_out!(x.mod_power_of_2_add_dot_reversed(&v, &w, pow));
            }),
            (
                "mod_power_of_2_dot_reversed, then mod_power_of_2_add",
                &mut |(v, w, x, pow)| {
                    no_out!(x.mod_power_of_2_add((&v).mod_power_of_2_dot_reversed(&w, pow), pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_vector_mod_power_of_2_add_dot_reversed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_add_dot_reversed(NaturalVector, NaturalVector)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "Natural.mod_power_of_2_add_dot_reversed(NaturalVector, NaturalVector)",
                &mut |(v, w, x, pow)| {
                    no_out!(x.mod_power_of_2_add_dot_reversed(v, w, pow));
                },
            ),
            (
                "Natural.mod_power_of_2_add_dot_reversed(&NaturalVector, &NaturalVector)",
                &mut |(v, w, x, pow)| {
                    no_out!(x.mod_power_of_2_add_dot_reversed(&v, &w, pow));
                },
            ),
            (
                "Natural.mod_power_of_2_add_dot_reversed_assign(&NaturalVector, &NaturalVector)",
                &mut |(v, w, mut x, pow)| {
                    no_out!(x.mod_power_of_2_add_dot_reversed_assign(&v, &w, pow));
                },
            ),
        ],
    );
}
