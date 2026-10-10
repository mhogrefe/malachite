// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen_var_3;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_div_exact);
    register_demo!(runner, demo_natural_vector_div_exact_ref);
    register_demo!(runner, demo_natural_vector_div_exact_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_div_exact_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_vector_div_exact_algorithms);
}

fn demo_natural_vector_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in natural_vector_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.div_exact({c}) = {}", v.div_exact(&c));
    }
}

fn demo_natural_vector_div_exact_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in natural_vector_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).div_exact(&{c}) = {}", (&v).div_exact(&c));
    }
}

fn demo_natural_vector_div_exact_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c) in natural_vector_natural_pair_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.div_exact_assign(&c);
        println!("v := {v_old}; v.div_exact_assign({c}); v = {v}");
    }
}

fn benchmark_natural_vector_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.div_exact(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("NaturalVector.div_exact(Natural)", &mut |(v, c)| {
                no_out!(v.div_exact(c));
            }),
            ("NaturalVector.div_exact(&Natural)", &mut |(v, c)| {
                no_out!(v.div_exact(&c));
            }),
            ("(&NaturalVector).div_exact(Natural)", &mut |(v, c)| {
                no_out!((&v).div_exact(c));
            }),
            ("(&NaturalVector).div_exact(&Natural)", &mut |(v, c)| {
                no_out!((&v).div_exact(&c));
            }),
            (
                "NaturalVector.div_exact_assign(Natural)",
                &mut |(mut v, c)| {
                    v.div_exact_assign(c);
                },
            ),
            (
                "NaturalVector.div_exact_assign(&Natural)",
                &mut |(mut v, c)| {
                    v.div_exact_assign(&c);
                },
            ),
        ],
    );
}

fn benchmark_natural_vector_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.div_exact(Natural)",
        BenchmarkType::Algorithms,
        natural_vector_natural_pair_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, c)| {
                no_out!(v.div_exact(&c));
            }),
            ("element-by-element division", &mut |(v, c)| {
                no_out!(NaturalVector {
                    elements: v.elements.iter().map(|x| x / &c).collect()
                });
            }),
        ],
    );
}
