// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Dot, DotReversed};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_vector_natural_triple_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_dot_reversed);
    register_demo!(runner, demo_natural_vector_dot_reversed_val_ref);
    register_demo!(runner, demo_natural_vector_dot_reversed_ref_val);
    register_demo!(runner, demo_natural_vector_dot_reversed_ref_ref);
    register_bench!(runner, benchmark_natural_vector_dot_reversed_algorithms);
    register_bench!(
        runner,
        benchmark_natural_vector_dot_reversed_evaluation_strategy
    );
}

fn demo_natural_vector_dot_reversed(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, _) in natural_vector_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.dot_reversed({w_old}) = {}", v.dot_reversed(w));
    }
}

fn demo_natural_vector_dot_reversed_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, _) in natural_vector_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.dot_reversed(&{w}) = {}", v.dot_reversed(&w));
    }
}

fn demo_natural_vector_dot_reversed_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, _) in natural_vector_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let w_old = w.clone();
        println!("(&{v}).dot_reversed({w_old}) = {}", (&v).dot_reversed(w));
    }
}

fn demo_natural_vector_dot_reversed_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w, _) in natural_vector_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).dot_reversed(&{w}) = {}", (&v).dot_reversed(&w));
    }
}

fn benchmark_natural_vector_dot_reversed_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.dot_reversed(NaturalVector)",
        BenchmarkType::Algorithms,
        natural_vector_natural_vector_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, w, _)| {
                no_out!(v.dot_reversed(w));
            }),
            ("reverse, then dot", &mut |(v, mut w, _)| {
                w.elements.reverse();
                no_out!(v.dot(w));
            }),
            ("collect reversed, then dot", &mut |(v, w, _)| {
                no_out!(v.dot(NaturalVector::from_owned_elements(
                    w.elements.into_iter().rev().collect()
                )));
            }),
        ],
    );
}

fn benchmark_natural_vector_dot_reversed_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.dot_reversed(NaturalVector)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            (
                "NaturalVector.dot_reversed(NaturalVector)",
                &mut |(v, w, _)| {
                    no_out!(v.dot_reversed(w));
                },
            ),
            (
                "NaturalVector.dot_reversed(&NaturalVector)",
                &mut |(v, w, _)| {
                    no_out!(v.dot_reversed(&w));
                },
            ),
            (
                "(&NaturalVector).dot_reversed(NaturalVector)",
                &mut |(v, w, _)| {
                    no_out!((&v).dot_reversed(w));
                },
            ),
            (
                "(&NaturalVector).dot_reversed(&NaturalVector)",
                &mut |(v, w, _)| {
                    no_out!((&v).dot_reversed(&w));
                },
            ),
        ],
    );
}
