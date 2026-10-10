// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_1_rational_vector_bit_bucketer;
use malachite_q::test_util::generators::rational_vector_rational_vector_rational_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_vector_add_mul);
    register_demo!(runner, demo_rational_vector_add_mul_ref);
    register_demo!(runner, demo_rational_vector_add_mul_assign);

    register_bench!(
        runner,
        benchmark_rational_vector_add_mul_evaluation_strategy
    );
}

fn demo_rational_vector_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        let c_old = c.clone();
        println!("{u_old}.add_mul({v_old}, {c_old}) = {}", u.add_mul(v, c));
    }
}

fn demo_rational_vector_add_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{u}).add_mul(&{v}, &{c}) = {}", (&u).add_mul(&v, &c));
    }
}

fn demo_rational_vector_add_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut u, v, c) in rational_vector_rational_vector_rational_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        u.add_mul_assign(&v, &c);
        println!("u := {u_old}; u.add_mul_assign(&{v}, &{c}); u = {u}");
    }
}

fn benchmark_rational_vector_add_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalVector.add_mul(RationalVector, Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_vector_rational_vector_rational_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_vector_bit_bucketer("u"),
        &mut [
            (
                "RationalVector.add_mul(RationalVector, Rational)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(v, c));
                },
            ),
            (
                "RationalVector.add_mul(RationalVector, &Rational)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(v, &c));
                },
            ),
            (
                "RationalVector.add_mul(&RationalVector, Rational)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(&v, c));
                },
            ),
            (
                "RationalVector.add_mul(&RationalVector, &Rational)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(&v, &c));
                },
            ),
            (
                "(&RationalVector).add_mul(&RationalVector, &Rational)",
                &mut |(u, v, c)| {
                    no_out!((&u).add_mul(&v, &c));
                },
            ),
            (
                "RationalVector.add_mul_assign(RationalVector, Rational)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(v, c);
                },
            ),
            (
                "RationalVector.add_mul_assign(RationalVector, &Rational)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(v, &c);
                },
            ),
            (
                "RationalVector.add_mul_assign(&RationalVector, Rational)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(&v, c);
                },
            ),
            (
                "RationalVector.add_mul_assign(&RationalVector, &Rational)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(&v, &c);
                },
            ),
        ],
    );
}
