// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::pair_1_rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_rational_pair_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_div_scalar);
    register_demo!(runner, demo_rational_vector_div_scalar_ref);
    register_demo!(runner, demo_rational_vector_div_scalar_assign);

    register_bench!(
        runner,
        benchmark_rational_vector_div_scalar_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_vector_div_scalar_algorithms);
}

fn demo_rational_vector_div_scalar(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in rational_vector_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!("{v_old} / {c} = {}", v / &c);
    }
}

fn demo_rational_vector_div_scalar_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c) in rational_vector_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("&{v} / &{c} = {}", &v / &c);
    }
}

fn demo_rational_vector_div_scalar_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c) in rational_vector_rational_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v /= &c;
        println!("v := {v_old}; v /= {c}; v = {v}");
    }
}

fn benchmark_rational_vector_div_scalar_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector / Rational",
        BenchmarkType::EvaluationStrategy,
        rational_vector_rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_vector_bit_bucketer("v"),
        &mut [
            ("RationalVector / Rational", &mut |(v, c)| {
                no_out!(v / c);
            }),
            ("RationalVector / &Rational", &mut |(v, c)| {
                no_out!(v / &c);
            }),
            ("&RationalVector / Rational", &mut |(v, c)| {
                no_out!(&v / c);
            }),
            ("&RationalVector / &Rational", &mut |(v, c)| {
                no_out!(&v / &c);
            }),
            ("RationalVector /= Rational", &mut |(mut v, c)| {
                v /= c;
            }),
            ("RationalVector /= &Rational", &mut |(mut v, c)| {
                v /= &c;
            }),
        ],
    );
}

fn benchmark_rational_vector_div_scalar_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector / Rational",
        BenchmarkType::Algorithms,
        rational_vector_rational_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, c)| {
                no_out!(v / c);
            }),
            ("multiplying by the reciprocal", &mut |(v, c)| {
                no_out!(v * c.reciprocal());
            }),
        ],
    );
}
