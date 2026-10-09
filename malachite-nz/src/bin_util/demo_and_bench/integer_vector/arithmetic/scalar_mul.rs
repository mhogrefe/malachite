// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer_vector::arithmetic::scalar_mul::{
    integers_mul_scalar, integers_mul_scalar_assign,
};
use malachite_nz::test_util::bench::bucketers::pair_1_integer_vector_bit_bucketer;
use malachite_nz::test_util::generators::{
    integer_vec_integer_pair_gen, integer_vector_integer_pair_gen,
};
use malachite_nz::test_util::integer_vector::arithmetic::scalar_mul::integers_mul_scalar_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integers_mul_scalar);
    register_demo!(runner, demo_integers_mul_scalar_assign);
    register_demo!(runner, demo_integer_vector_mul_scalar);
    register_demo!(runner, demo_integer_vector_mul_scalar_ref);
    register_demo!(runner, demo_integer_vector_scalar_mul_ref);
    register_demo!(runner, demo_integer_vector_mul_scalar_assign);

    register_bench!(runner, benchmark_integers_mul_scalar_algorithms);
    register_bench!(
        runner,
        benchmark_integer_vector_mul_scalar_evaluation_strategy
    );
}

fn demo_integers_mul_scalar(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, c) in integer_vec_integer_pair_gen().get(gm, config).take(limit) {
        println!(
            "integers_mul_scalar({xs:?}, {c}) = {:?}",
            integers_mul_scalar(&xs, &c)
        );
    }
}

fn demo_integers_mul_scalar_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut xs, c) in integer_vec_integer_pair_gen().get(gm, config).take(limit) {
        let xs_old = xs.clone();
        integers_mul_scalar_assign(&mut xs, &c);
        println!("xs := {xs_old:?}; integers_mul_scalar_assign(&mut xs, {c}); xs = {xs:?}");
    }
}

fn benchmark_integers_mul_scalar_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "integers_mul_scalar(&[Integer], &Integer)",
        BenchmarkType::Algorithms,
        integer_vec_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [
            ("default", &mut |(xs, c)| {
                no_out!(integers_mul_scalar(&xs, &c));
            }),
            ("in place", &mut |(mut xs, c)| {
                integers_mul_scalar_assign(&mut xs, &c);
            }),
            ("naive", &mut |(xs, c)| {
                no_out!(integers_mul_scalar_naive(&xs, &c));
            }),
        ],
    );
}

fn demo_integer_vector_mul_scalar(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in integer_vector_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} * {c} = {}", v * &c);
    }
}

fn demo_integer_vector_mul_scalar_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in integer_vector_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} * &{c} = {}", &v * &c);
    }
}

fn demo_integer_vector_scalar_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in integer_vector_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        println!("&{c} * &{v} = {}", &c * &v);
    }
}

fn demo_integer_vector_mul_scalar_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c) in integer_vector_integer_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v *= &c;
        println!("v := {v_old}; v *= {c}; v = {v}");
    }
}

fn benchmark_integer_vector_mul_scalar_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerVector * Integer",
        BenchmarkType::EvaluationStrategy,
        integer_vector_integer_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_vector_bit_bucketer("v"),
        &mut [
            ("IntegerVector * Integer", &mut |(v, c)| {
                no_out!(v * c);
            }),
            ("IntegerVector * &Integer", &mut |(v, c)| {
                no_out!(v * &c);
            }),
            ("&IntegerVector * Integer", &mut |(v, c)| {
                no_out!(&v * c);
            }),
            ("&IntegerVector * &Integer", &mut |(v, c)| {
                no_out!(&v * &c);
            }),
            ("IntegerVector *= Integer", &mut |(mut v, c)| {
                v *= c;
            }),
            ("IntegerVector *= &Integer", &mut |(mut v, c)| {
                v *= &c;
            }),
        ],
    );
}
