// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddMul, ModPowerOf2AddMulAssign, ModPowerOf2Mul,
};
use malachite_base::test_util::bench::bucketers::quadruple_4_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::generators::natural_natural_natural_unsigned_quadruple_gen_var_2;
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_add_mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_mod_power_of_2_add_mul);
    register_demo!(runner, demo_natural_mod_power_of_2_add_mul_ref);
    register_demo!(runner, demo_natural_mod_power_of_2_add_mul_assign);

    register_bench!(
        runner,
        benchmark_natural_mod_power_of_2_add_mul_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_mod_power_of_2_add_mul_algorithms);
}

fn demo_natural_mod_power_of_2_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, pow) in natural_natural_natural_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "{} + {} * {} ≡ {} mod 2^{}",
            x_old,
            y_old,
            z_old,
            x.mod_power_of_2_add_mul(y, z, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_2_add_mul_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, pow) in natural_natural_natural_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{}).mod_power_of_2_add_mul(&{}, &{}, {}) = {}",
            x,
            y,
            z,
            pow,
            (&x).mod_power_of_2_add_mul(&y, &z, pow)
        );
    }
}

fn demo_natural_mod_power_of_2_add_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, pow) in natural_natural_natural_unsigned_quadruple_gen_var_2()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.mod_power_of_2_add_mul_assign(&y, &z, pow);
        println!("x := {x_old}; x.mod_power_of_2_add_mul_assign(&{y}, &{z}, {pow}); x = {x}");
    }
}

fn benchmark_natural_mod_power_of_2_add_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_add_mul(Natural, Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_natural_natural_unsigned_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_bucketer("pow"),
        &mut [
            (
                "Natural.mod_power_of_2_add_mul(Natural, Natural, u64)",
                &mut |(x, y, z, pow)| {
                    no_out!(x.mod_power_of_2_add_mul(y, z, pow));
                },
            ),
            (
                "Natural.mod_power_of_2_add_mul(Natural, &Natural, u64)",
                &mut |(x, y, z, pow)| {
                    no_out!(x.mod_power_of_2_add_mul(y, &z, pow));
                },
            ),
            (
                "Natural.mod_power_of_2_add_mul(&Natural, Natural, u64)",
                &mut |(x, y, z, pow)| {
                    no_out!(x.mod_power_of_2_add_mul(&y, z, pow));
                },
            ),
            (
                "Natural.mod_power_of_2_add_mul(&Natural, &Natural, u64)",
                &mut |(x, y, z, pow)| {
                    no_out!(x.mod_power_of_2_add_mul(&y, &z, pow));
                },
            ),
            (
                "(&Natural).mod_power_of_2_add_mul(&Natural, &Natural, u64)",
                &mut |(x, y, z, pow)| {
                    no_out!((&x).mod_power_of_2_add_mul(&y, &z, pow));
                },
            ),
            (
                "Natural.mod_power_of_2_add_mul_assign(Natural, Natural, u64)",
                &mut |(mut x, y, z, pow)| {
                    x.mod_power_of_2_add_mul_assign(y, z, pow);
                },
            ),
            (
                "Natural.mod_power_of_2_add_mul_assign(Natural, &Natural, u64)",
                &mut |(mut x, y, z, pow)| {
                    x.mod_power_of_2_add_mul_assign(y, &z, pow);
                },
            ),
            (
                "Natural.mod_power_of_2_add_mul_assign(&Natural, Natural, u64)",
                &mut |(mut x, y, z, pow)| {
                    x.mod_power_of_2_add_mul_assign(&y, z, pow);
                },
            ),
            (
                "Natural.mod_power_of_2_add_mul_assign(&Natural, &Natural, u64)",
                &mut |(mut x, y, z, pow)| {
                    x.mod_power_of_2_add_mul_assign(&y, &z, pow);
                },
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_add_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_power_of_2_add_mul(Natural, Natural, u64)",
        BenchmarkType::Algorithms,
        natural_natural_natural_unsigned_quadruple_gen_var_2().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_4_bucketer("pow"),
        &mut [
            ("default", &mut |(x, y, z, pow)| {
                no_out!(x.mod_power_of_2_add_mul(y, z, pow));
            }),
            ("unfused", &mut |(x, y, z, pow)| {
                no_out!(x.mod_power_of_2_add(y.mod_power_of_2_mul(z, pow), pow));
            }),
            ("naive", &mut |(x, y, z, pow)| {
                no_out!(mod_power_of_2_add_mul_naive(&x, &y, &z, pow));
            }),
        ],
    );
}
