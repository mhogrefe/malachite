// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mul_scalar);
    register_demo!(runner, demo_natural_vector_mul_scalar_ref);
    register_demo!(runner, demo_natural_vector_scalar_mul_ref);
    register_demo!(runner, demo_natural_vector_mul_scalar_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_mul_scalar_evaluation_strategy
    );
}

fn demo_natural_vector_mul_scalar(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} * {c} = {}", v * &c);
    }
}

fn demo_natural_vector_mul_scalar_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} * &{c} = {}", &v * &c);
    }
}

fn demo_natural_vector_scalar_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("&{c} * &{v} = {}", &c * &v);
    }
}

fn demo_natural_vector_mul_scalar_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c) in natural_vector_natural_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v *= &c;
        println!("v := {v_old}; v *= {c}; v = {v}");
    }
}

fn benchmark_natural_vector_mul_scalar_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector * Natural",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector * Natural", &mut |(v, c)| {
                no_out!(v * c);
            }),
            ("NaturalVector * &Natural", &mut |(v, c)| {
                no_out!(v * &c);
            }),
            ("&NaturalVector * Natural", &mut |(v, c)| {
                no_out!(&v * c);
            }),
            ("&NaturalVector * &Natural", &mut |(v, c)| {
                no_out!(&v * &c);
            }),
            ("NaturalVector *= Natural", &mut |(mut v, c)| {
                v *= c;
            }),
            ("NaturalVector *= &Natural", &mut |(mut v, c)| {
                v *= &c;
            }),
        ],
    );
}
