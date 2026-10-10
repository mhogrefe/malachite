// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMul, ModPowerOf2AddMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_mul);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_mul_ref);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_mul_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_add_mul_evaluation_strategy
    );
}

fn demo_natural_vector_mod_power_of_2_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c, pow) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        let c_old = c.clone();
        println!(
            "{u_old}.mod_power_of_2_add_mul({v_old}, {c_old}, {pow}) = {}",
            u.mod_power_of_2_add_mul(v, c, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c, pow) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{u}).mod_power_of_2_add_mul(&{v}, &{c}, {pow}) = {}",
            (&u).mod_power_of_2_add_mul(&v, &c, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_mul_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut u, v, c, pow) in natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let u_old = u.clone();
        u.mod_power_of_2_add_mul_assign(&v, &c, pow);
        println!("u := {u_old}; u.mod_power_of_2_add_mul_assign(&{v}, &{c}, {pow}); u = {u}");
    }
}

fn benchmark_natural_vector_mod_power_of_2_add_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2_add_mul(NaturalVector, Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_natural_vector_bit_bucketer("u"),
        &mut [
            (
                "NaturalVector.mod_power_of_2_add_mul(NaturalVector, Natural, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!(u.mod_power_of_2_add_mul(v, c, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul(NaturalVector, &Natural, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!(u.mod_power_of_2_add_mul(v, &c, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul(&NaturalVector, Natural, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!(u.mod_power_of_2_add_mul(&v, c, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul(&NaturalVector, &Natural, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!(u.mod_power_of_2_add_mul(&v, &c, pow));
                },
            ),
            (
                "(&NaturalVector).mod_power_of_2_add_mul(&NaturalVector, &Natural, u64)",
                &mut |(u, v, c, pow)| {
                    no_out!((&u).mod_power_of_2_add_mul(&v, &c, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_assign(NaturalVector, Natural, u64)",
                &mut |(mut u, v, c, pow)| {
                    u.mod_power_of_2_add_mul_assign(v, c, pow);
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_assign(NaturalVector, &Natural, u64)",
                &mut |(mut u, v, c, pow)| {
                    u.mod_power_of_2_add_mul_assign(v, &c, pow);
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_assign(&NaturalVector, Natural, u64)",
                &mut |(mut u, v, c, pow)| {
                    u.mod_power_of_2_add_mul_assign(&v, c, pow);
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_assign(&NaturalVector, &Natural, u64)",
                &mut |(mut u, v, c, pow)| {
                    u.mod_power_of_2_add_mul_assign(&v, &c, pow);
                },
            ),
        ],
    );
}
