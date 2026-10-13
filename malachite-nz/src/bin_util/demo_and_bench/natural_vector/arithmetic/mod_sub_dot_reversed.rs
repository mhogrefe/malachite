// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModDotReversed, ModSub, ModSubDotReversed, ModSubDotReversedAssign,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_sub_dot_reversed);
    register_demo!(runner, demo_natural_vector_mod_sub_dot_reversed_ref);
    register_demo!(runner, demo_natural_vector_mod_sub_dot_reversed_assign);
    register_bench!(
        runner,
        benchmark_natural_vector_mod_sub_dot_reversed_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_vector_mod_sub_dot_reversed_evaluation_strategy
    );
}

fn demo_natural_vector_mod_sub_dot_reversed(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x, m) in natural_vector_natural_vector_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let v_old = v.clone();
        let w_old = w.clone();
        let m_old = m.clone();
        println!(
            "{x_old}.mod_sub_dot_reversed({v_old}, {w_old}, {m_old}) = {}",
            x.mod_sub_dot_reversed(v, w, m)
        );
    }
}

fn demo_natural_vector_mod_sub_dot_reversed_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, x, m) in natural_vector_natural_vector_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).mod_sub_dot_reversed(&{v}, &{w}, &{m}) = {}",
            (&x).mod_sub_dot_reversed(&v, &w, &m)
        );
    }
}

fn demo_natural_vector_mod_sub_dot_reversed_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, mut x, m) in natural_vector_natural_vector_natural_natural_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_sub_dot_reversed_assign(&v, &w, &m);
        println!("x := {x_old}; x.mod_sub_dot_reversed_assign(&{v}, &{w}, &{m}); x = {x}");
    }
}

fn benchmark_natural_vector_mod_sub_dot_reversed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_sub_dot_reversed(NaturalVector, NaturalVector)",
        BenchmarkType::Algorithms,
        natural_vector_natural_vector_natural_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, w, x, m)| {
                no_out!(x.mod_sub_dot_reversed(&v, &w, &m));
            }),
            ("mod_dot_reversed, then mod_sub", &mut |(v, w, x, m)| {
                no_out!(x.mod_sub((&v).mod_dot_reversed(&w, &m), &m));
            }),
        ],
    );
}

fn benchmark_natural_vector_mod_sub_dot_reversed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_sub_dot_reversed(NaturalVector, NaturalVector)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_natural_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "Natural.mod_sub_dot_reversed(NaturalVector, NaturalVector)",
                &mut |(v, w, x, m)| {
                    no_out!(x.mod_sub_dot_reversed(v, w, m));
                },
            ),
            (
                "Natural.mod_sub_dot_reversed(&NaturalVector, &NaturalVector)",
                &mut |(v, w, x, m)| {
                    no_out!(x.mod_sub_dot_reversed(&v, &w, &m));
                },
            ),
            (
                "Natural.mod_sub_dot_reversed_assign(&NaturalVector, &NaturalVector)",
                &mut |(v, w, mut x, m)| {
                    no_out!(x.mod_sub_dot_reversed_assign(&v, &w, &m));
                },
            ),
        ],
    );
}
