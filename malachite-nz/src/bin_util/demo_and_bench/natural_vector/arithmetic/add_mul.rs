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
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_vector_natural_triple_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_add_mul);
    register_demo!(runner, demo_natural_vector_add_mul_ref);
    register_demo!(runner, demo_natural_vector_add_mul_assign);

    register_bench!(runner, benchmark_natural_vector_add_mul_evaluation_strategy);
}

fn demo_natural_vector_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c) in natural_vector_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        let c_old = c.clone();
        println!("{u_old}.add_mul({v_old}, {c_old}) = {}", u.add_mul(v, c));
    }
}

fn demo_natural_vector_add_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c) in natural_vector_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!("(&{u}).add_mul(&{v}, &{c}) = {}", (&u).add_mul(&v, &c));
    }
}

fn demo_natural_vector_add_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut u, v, c) in natural_vector_natural_vector_natural_triple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        u.add_mul_assign(&v, &c);
        println!("u := {u_old}; u.add_mul_assign(&{v}, &{c}); u = {u}");
    }
}

fn benchmark_natural_vector_add_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.add_mul(NaturalVector, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_triple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("u"),
        &mut [
            (
                "NaturalVector.add_mul(NaturalVector, Natural)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(v, c));
                },
            ),
            (
                "NaturalVector.add_mul(NaturalVector, &Natural)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(v, &c));
                },
            ),
            (
                "NaturalVector.add_mul(&NaturalVector, Natural)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(&v, c));
                },
            ),
            (
                "NaturalVector.add_mul(&NaturalVector, &Natural)",
                &mut |(u, v, c)| {
                    no_out!(u.add_mul(&v, &c));
                },
            ),
            (
                "(&NaturalVector).add_mul(&NaturalVector, &Natural)",
                &mut |(u, v, c)| {
                    no_out!((&u).add_mul(&v, &c));
                },
            ),
            (
                "NaturalVector.add_mul_assign(NaturalVector, Natural)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(v, c);
                },
            ),
            (
                "NaturalVector.add_mul_assign(NaturalVector, &Natural)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(v, &c);
                },
            ),
            (
                "NaturalVector.add_mul_assign(&NaturalVector, Natural)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(&v, c);
                },
            ),
            (
                "NaturalVector.add_mul_assign(&NaturalVector, &Natural)",
                &mut |(mut u, v, c)| {
                    u.add_mul_assign(&v, &c);
                },
            ),
        ],
    );
}
