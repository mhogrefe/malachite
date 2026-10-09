// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Mul, ModPowerOf2MulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_vector_bit_bucketer;
use malachite_nz::test_util::generators::natural_vector_natural_unsigned_triple_gen_var_1;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_vector_mod_power_of_2_mul);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_mul_ref);
    register_demo!(runner, demo_natural_vector_mod_power_of_2_mul_assign);

    register_bench!(
        runner,
        benchmark_natural_vector_mod_power_of_2_mul_algorithms
    );
}

fn demo_natural_vector_mod_power_of_2_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, pow) in natural_vector_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        println!(
            "{v_old}.mod_power_of_2_mul({c}, {pow}) = {}",
            v.mod_power_of_2_mul(&c, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (v, c, pow) in natural_vector_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{v}).mod_power_of_2_mul(&{c}, {pow}) = {}",
            (&v).mod_power_of_2_mul(&c, pow)
        );
    }
}

fn demo_natural_vector_mod_power_of_2_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut v, c, pow) in natural_vector_natural_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let v_old = v.clone();
        v.mod_power_of_2_mul_assign(&c, pow);
        println!("v := {v_old}; v.mod_power_of_2_mul_assign({c}, {pow}); v = {v}");
    }
}

fn benchmark_natural_vector_mod_power_of_2_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalVector.mod_power_of_2_mul(Natural, u64)",
        BenchmarkType::Algorithms,
        natural_vector_natural_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_vector_bit_bucketer("v"),
        &mut [
            ("default", &mut |(v, c, pow)| {
                no_out!(v.mod_power_of_2_mul(c, pow));
            }),
            ("multiply, then reduce", &mut |(v, c, pow)| {
                no_out!((v * c).mod_power_of_2(pow));
            }),
        ],
    );
}
