// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SubMulShl, SubMulShlAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::quadruple_1_2_3_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_integer_integer_unsigned_quadruple_gen_var_1;
use malachite_nz::test_util::integer::arithmetic::sub_mul_shl::sub_mul_shl_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_sub_mul_shl);
    register_demo!(runner, demo_integer_sub_mul_shl_ref);
    register_demo!(runner, demo_integer_sub_mul_shl_assign);

    register_bench!(runner, benchmark_integer_sub_mul_shl_evaluation_strategy);
    register_bench!(runner, benchmark_integer_sub_mul_shl_algorithms);
}

fn demo_integer_sub_mul_shl(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, bits) in integer_integer_integer_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "{x_old}.sub_mul_shl({y_old}, {z_old}, {bits}) = {}",
            x.sub_mul_shl(y, z, bits)
        );
    }
}

fn demo_integer_sub_mul_shl_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, bits) in integer_integer_integer_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).sub_mul_shl(&{y}, &{z}, {bits}) = {}",
            (&x).sub_mul_shl(&y, &z, bits)
        );
    }
}

fn demo_integer_sub_mul_shl_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, bits) in integer_integer_integer_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.sub_mul_shl_assign(&y, &z, bits);
        println!("x := {x_old}; x.sub_mul_shl_assign(&{y}, &{z}, {bits}); x = {x}");
    }
}

fn benchmark_integer_sub_mul_shl_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul_shl(Integer, Integer, u64)",
        BenchmarkType::EvaluationStrategy,
        integer_integer_integer_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.sub_mul_shl(Integer, Integer, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.sub_mul_shl(y, z, bits));
                },
            ),
            (
                "Integer.sub_mul_shl(Integer, &Integer, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.sub_mul_shl(y, &z, bits));
                },
            ),
            (
                "Integer.sub_mul_shl(&Integer, Integer, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.sub_mul_shl(&y, z, bits));
                },
            ),
            (
                "Integer.sub_mul_shl(&Integer, &Integer, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.sub_mul_shl(&y, &z, bits));
                },
            ),
            (
                "(&Integer).sub_mul_shl(&Integer, &Integer, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!((&x).sub_mul_shl(&y, &z, bits));
                },
            ),
            (
                "Integer.sub_mul_shl_assign(Integer, Integer, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.sub_mul_shl_assign(y, z, bits);
                },
            ),
            (
                "Integer.sub_mul_shl_assign(Integer, &Integer, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.sub_mul_shl_assign(y, &z, bits);
                },
            ),
            (
                "Integer.sub_mul_shl_assign(&Integer, Integer, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.sub_mul_shl_assign(&y, z, bits);
                },
            ),
            (
                "Integer.sub_mul_shl_assign(&Integer, &Integer, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.sub_mul_shl_assign(&y, &z, bits);
                },
            ),
        ],
    );
}

fn benchmark_integer_sub_mul_shl_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.sub_mul_shl(Integer, Integer, u64)",
        BenchmarkType::Algorithms,
        integer_integer_integer_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("default", &mut |(x, y, z, bits)| {
                no_out!(x.sub_mul_shl(y, z, bits));
            }),
            ("unfused", &mut |(x, y, z, bits)| {
                no_out!(x - ((y * z) << bits));
            }),
            ("naive", &mut |(x, y, z, bits)| {
                no_out!(sub_mul_shl_naive(&x, &y, &z, bits));
            }),
        ],
    );
}
