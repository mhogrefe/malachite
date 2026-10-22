// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::bench::bucketers::pair_1_integer_matrix_bit_bucketer;
use malachite_nz::test_util::generators::integer_matrix_integer_pair_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_matrix_div_exact);
    register_demo!(runner, demo_integer_matrix_div_exact_ref);
    register_demo!(runner, demo_integer_matrix_div_exact_assign);
    register_bench!(
        runner,
        benchmark_integer_matrix_div_exact_evaluation_strategy
    );
    register_bench!(runner, benchmark_integer_matrix_div_exact_algorithms);
}

fn demo_integer_matrix_div_exact(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, c) in integer_matrix_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        println!("{a_old}.div_exact({c}) = {}", a.div_exact(&c));
    }
}

fn demo_integer_matrix_div_exact_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, c) in integer_matrix_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{a}).div_exact(&{c}) = {}", (&a).div_exact(&c));
    }
}

fn demo_integer_matrix_div_exact_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut a, c) in integer_matrix_integer_pair_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let a_old = a.clone();
        a.div_exact_assign(&c);
        println!("a := {a_old}; a.div_exact_assign({c}); a = {a}");
    }
}

fn benchmark_integer_matrix_div_exact_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerMatrix.div_exact(Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_matrix_integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("IntegerMatrix.div_exact(Integer)", &mut |(a, c)| {
                no_out!(a.div_exact(c));
            }),
            ("IntegerMatrix.div_exact(&Integer)", &mut |(a, c)| {
                no_out!(a.div_exact(&c));
            }),
            ("(&IntegerMatrix).div_exact(Integer)", &mut |(a, c)| {
                no_out!((&a).div_exact(c));
            }),
            ("(&IntegerMatrix).div_exact(&Integer)", &mut |(a, c)| {
                no_out!((&a).div_exact(&c));
            }),
            (
                "IntegerMatrix.div_exact_assign(Integer)",
                &mut |(mut a, c)| {
                    a.div_exact_assign(c);
                },
            ),
            (
                "IntegerMatrix.div_exact_assign(&Integer)",
                &mut |(mut a, c)| {
                    a.div_exact_assign(&c);
                },
            ),
        ],
    );
}

fn benchmark_integer_matrix_div_exact_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerMatrix).div_exact(&Integer)",
        BenchmarkType::Algorithms,
        integer_matrix_integer_pair_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_matrix_bit_bucketer("a"),
        &mut [
            ("default", &mut |(a, c)| {
                no_out!((&a).div_exact(&c));
            }),
            ("element-by-element division", &mut |(a, c)| {
                no_out!(IntegerMatrix::from_owned_elements(
                    a.height(),
                    a.width(),
                    a.elements.iter().map(|x| x / &c).collect()
                ));
            }),
        ],
    );
}
