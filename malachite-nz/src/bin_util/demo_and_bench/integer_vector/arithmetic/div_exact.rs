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
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_integer_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_div_exact);
    register_demo!(runner, demo_integer_vector_div_exact_ref);
    register_demo!(runner, demo_integer_vector_div_exact_assign);

    register_bench!(
        runner,
        benchmark_integer_vector_div_exact_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_vector_div_exact_algorithms);
}

fn demo_integer_vector_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in integer_vector_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old}.div_exact({c}) = {}", v.div_exact(&c));
    }
}

fn demo_integer_vector_div_exact_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in integer_vector_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{v}).div_exact(&{c}) = {}", (&v).div_exact(&c));
    }
}

fn demo_integer_vector_div_exact_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c) in integer_vector_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.div_exact_assign(&c);
        println!("v := {v_old}; v.div_exact_assign({c}); v = {v}");
    }
}

fn benchmark_integer_vector_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_vector_integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("IntegerVector.div_exact(Integer)", &mut |(v, c)| {
                no_out!(v.div_exact(c));
            }),
            ("IntegerVector.div_exact(&Integer)", &mut |(v, c)| {
                no_out!(v.div_exact(&c));
            }),
            ("(&IntegerVector).div_exact(Integer)", &mut |(v, c)| {
                no_out!((&v).div_exact(c));
            }),
            ("(&IntegerVector).div_exact(&Integer)", &mut |(v, c)| {
                no_out!((&v).div_exact(&c));
            }),
            (
                "IntegerVector.div_exact_assign(Integer)",
                &mut |(mut v, c)| {
                    v.div_exact_assign(c);
                },
            ),
            (
                "IntegerVector.div_exact_assign(&Integer)",
                &mut |(mut v, c)| {
                    v.div_exact_assign(&c);
                },
            ),
        ],
    );
}

fn benchmark_integer_vector_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.div_exact(Integer)",
        BenchmarkType::Algorithms,
        integer_vector_integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, c)| {
                no_out!(v.div_exact(&c));
            }),
            ("element-by-element division", &mut |(v, c)| {
                no_out!(IntegerVector {
                    elements: v.elements.iter().map(|x| x / &c).collect()
                });
            }),
        ],
    );
}
