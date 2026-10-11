// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShl, ModPowerOf2AddMulShlAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quintuple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_mul_shl);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_add_mul_shl_ref);
    register_demo!(
        runner,
        demo_natural_vector_mod_power_of_2_add_mul_shl_assign
    );

    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_add_mul_shl_evaluation_strategy
    );
}

fn demo_natural_vector_mod_power_of_2_add_mul_shl(gm: GenMode, config: &GenConfig, limit: usize) {
    for (u, v, c, bits, pow) in
        natural_vector_natural_vector_natural_unsigned_unsigned_quintuple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let u_old = u.clone();
        let v_old = v.clone();
        let c_old = c.clone();
        println!(
            "{u_old}.mod_power_of_2_add_mul_shl({v_old}, {c_old}, {bits}, {pow}) = {}",
            u.mod_power_of_2_add_mul_shl(v, c, bits, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_mul_shl_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (u, v, c, bits, pow) in
        natural_vector_natural_vector_natural_unsigned_unsigned_quintuple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        println!(
            "(&{u}).mod_power_of_2_add_mul_shl(&{v}, &{c}, {bits}, {pow}) = {}",
            (&u).mod_power_of_2_add_mul_shl(&v, &c, bits, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_add_mul_shl_assign(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut u, v, c, bits, pow) in
        natural_vector_natural_vector_natural_unsigned_unsigned_quintuple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let u_old = u.clone();
        u.mod_power_of_2_add_mul_shl_assign(&v, &c, bits, pow);
        println!(
            "u := {u_old}; u.mod_power_of_2_add_mul_shl_assign(&{v}, &{c}, {bits}, {pow}); u = {u}"
        );
    }
}

fn benchmark_natural_vector_mod_power_of_2_add_mul_shl_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2_add_mul_shl(NaturalVector, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_vector_natural_vector_natural_unsigned_unsigned_quintuple_gen_var_1()
            .get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quintuple_1_natural_vector_bit_bucketer("u"),
        &mut [
            (
                "NaturalVector.mod_power_of_2_add_mul_shl(NaturalVector, Natural)",
                &mut |(u, v, c, bits, pow)| {
                    no_out!(u.mod_power_of_2_add_mul_shl(v, c, bits, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_shl(NaturalVector, &Natural)",
                &mut |(u, v, c, bits, pow)| {
                    no_out!(u.mod_power_of_2_add_mul_shl(v, &c, bits, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_shl(&NaturalVector, Natural)",
                &mut |(u, v, c, bits, pow)| {
                    no_out!(u.mod_power_of_2_add_mul_shl(&v, c, bits, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_shl(&NaturalVector, &Natural)",
                &mut |(u, v, c, bits, pow)| {
                    no_out!(u.mod_power_of_2_add_mul_shl(&v, &c, bits, pow));
                },
            ),
            (
                "(&NaturalVector).mod_power_of_2_add_mul_shl(&NaturalVector, &Natural)",
                &mut |(u, v, c, bits, pow)| {
                    no_out!((&u).mod_power_of_2_add_mul_shl(&v, &c, bits, pow));
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_shl_assign(NaturalVector, Natural)",
                &mut |(mut u, v, c, bits, pow)| {
                    u.mod_power_of_2_add_mul_shl_assign(v, c, bits, pow);
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_shl_assign(NaturalVector, &Natural)",
                &mut |(mut u, v, c, bits, pow)| {
                    u.mod_power_of_2_add_mul_shl_assign(v, &c, bits, pow);
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_shl_assign(&NaturalVector, Natural)",
                &mut |(mut u, v, c, bits, pow)| {
                    u.mod_power_of_2_add_mul_shl_assign(&v, c, bits, pow);
                },
            ),
            (
                "NaturalVector.mod_power_of_2_add_mul_shl_assign(&NaturalVector, &Natural)",
                &mut |(mut u, v, c, bits, pow)| {
                    u.mod_power_of_2_add_mul_shl_assign(&v, &c, bits, pow);
                },
            ),
        ],
    );
}
