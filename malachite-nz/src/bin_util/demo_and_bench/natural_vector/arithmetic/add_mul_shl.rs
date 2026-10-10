// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMulShl, AddMulShlAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_add_mul_shl);
    register_demo!(runner, demo_natural_vector_add_mul_shl_ref);
    register_demo!(runner, demo_natural_vector_add_mul_shl_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_add_mul_shl_evaluation_strategy
    );
}

fn demo_natural_vector_add_mul_shl(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c, bits) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        let c_old = c.clone();
        println!(
            "{u_old}.add_mul_shl({v_old}, {c_old}, {bits}) = {}",
            u.add_mul_shl(v, c, bits)
        );
    }
}

fn demo_natural_vector_add_mul_shl_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c, bits) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{u}).add_mul_shl(&{v}, &{c}, {bits}) = {}",
            (&u).add_mul_shl(&v, &c, bits)
        );
    }
}

fn demo_natural_vector_add_mul_shl_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut u, v, c, bits) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        u.add_mul_shl_assign(&v, &c, bits);
        println!("u := {u_old}; u.add_mul_shl_assign(&{v}, &{c}, {bits}); u = {u}");
    }
}

fn benchmark_natural_vector_add_mul_shl_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.add_mul_shl(NaturalVector, Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_vector_bit_bucketer("u"),
        &mut [
            (
                "NaturalVector.add_mul_shl(NaturalVector, Natural, u64)",
                &mut |(u, v, c, bits)| {
                    no_out!(u.add_mul_shl(v, c, bits));
                },
            ),
            (
                "NaturalVector.add_mul_shl(NaturalVector, &Natural, u64)",
                &mut |(u, v, c, bits)| {
                    no_out!(u.add_mul_shl(v, &c, bits));
                },
            ),
            (
                "NaturalVector.add_mul_shl(&NaturalVector, Natural, u64)",
                &mut |(u, v, c, bits)| {
                    no_out!(u.add_mul_shl(&v, c, bits));
                },
            ),
            (
                "NaturalVector.add_mul_shl(&NaturalVector, &Natural, u64)",
                &mut |(u, v, c, bits)| {
                    no_out!(u.add_mul_shl(&v, &c, bits));
                },
            ),
            (
                "(&NaturalVector).add_mul_shl(&NaturalVector, &Natural, u64)",
                &mut |(u, v, c, bits)| {
                    no_out!((&u).add_mul_shl(&v, &c, bits));
                },
            ),
            (
                "NaturalVector.add_mul_shl_assign(NaturalVector, Natural, u64)",
                &mut |(mut u, v, c, bits)| {
                    u.add_mul_shl_assign(v, c, bits);
                },
            ),
            (
                "NaturalVector.add_mul_shl_assign(NaturalVector, &Natural, u64)",
                &mut |(mut u, v, c, bits)| {
                    u.add_mul_shl_assign(v, &c, bits);
                },
            ),
            (
                "NaturalVector.add_mul_shl_assign(&NaturalVector, Natural, u64)",
                &mut |(mut u, v, c, bits)| {
                    u.add_mul_shl_assign(&v, c, bits);
                },
            ),
            (
                "NaturalVector.add_mul_shl_assign(&NaturalVector, &Natural, u64)",
                &mut |(mut u, v, c, bits)| {
                    u.add_mul_shl_assign(&v, &c, bits);
                },
            ),
        ],
    );
}
