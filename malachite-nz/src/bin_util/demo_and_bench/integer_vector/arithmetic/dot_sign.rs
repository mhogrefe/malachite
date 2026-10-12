// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Dot, DotSign, Sign};
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::bench::bucketers::pair_integer_vector_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_vector_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_vector_dot_sign);
    register_demo!(runner, demo_integer_vector_dot_sign_ref_ref);
    register_bench!(runner, benchmark_integer_vector_dot_sign_algorithms);
    register_bench!(
        runner,
        benchmark_integer_vector_dot_sign_cancelling_algorithms
    );
    register_bench!(
        runner,
        benchmark_integer_vector_dot_sign_evaluation_strategy
    );
}

fn demo_integer_vector_dot_sign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        let v_old = v.clone();
        let w_old = w.clone();
        println!("{v_old}.dot_sign({w_old}) = {:?}", v.dot_sign(w));
    }
}

fn demo_integer_vector_dot_sign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, w) in integer_vector_pair_gen_var_1().get(gm, config).take(limit) {
        println!("(&{v}).dot_sign(&{w}) = {:?}", (&v).dot_sign(&w));
    }
}

fn benchmark_integer_vector_dot_sign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.dot_sign(IntegerVector)",
        BenchmarkType::Algorithms,
        integer_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("default", &mut |(v, w)| {
                no_out!((&v).dot_sign(&w));
            }),
            ("sign of the dot product", &mut |(v, w)| {
                no_out!((&v).dot(&w).sign());
            }),
        ],
    );
}

fn benchmark_integer_vector_dot_sign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.dot_sign(IntegerVector)",
        BenchmarkType::EvaluationStrategy,
        integer_vector_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("IntegerVector.dot_sign(IntegerVector)", &mut |(v, w)| {
                no_out!(v.dot_sign(w));
            }),
            ("IntegerVector.dot_sign(&IntegerVector)", &mut |(v, w)| {
                no_out!(v.dot_sign(&w));
            }),
            ("(&IntegerVector).dot_sign(IntegerVector)", &mut |(v, w)| {
                no_out!((&v).dot_sign(w));
            }),
            (
                "(&IntegerVector).dot_sign(&IntegerVector)",
                &mut |(v, w)| {
                    no_out!((&v).dot_sign(&w));
                },
            ),
        ],
    );
}

// Pairs extended by one coordinate, 1 in the first vector and the negative of the dot product in
// the second, so that the dot product is 0 and every screen fails.
fn benchmark_integer_vector_dot_sign_cancelling_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector.dot_sign(IntegerVector), cancelling",
        BenchmarkType::Algorithms,
        integer_vector_pair_gen_var_1()
            .get(gm, config)
            .map(|(v, w)| {
                let dot = (&v).dot(&w);
                let mut xs = v.into_elements();
                let mut ys = w.into_elements();
                xs.push(Integer::ONE);
                ys.push(-dot);
                (
                    IntegerVector::from_owned_elements(xs),
                    IntegerVector::from_owned_elements(ys),
                )
            }),
        gm.name(),
        limit,
        file_name,
        &pair_integer_vector_max_bit_bucketer("v", "w"),
        &mut [
            ("default", &mut |(v, w)| {
                no_out!((&v).dot_sign(&w));
            }),
            ("sign of the dot product", &mut |(v, w)| {
                no_out!((&v).dot(&w).sign());
            }),
        ],
    );
}
