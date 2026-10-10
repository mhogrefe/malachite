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
use malachite_q::test_util::bench::bucketers::quadruple_1_2_3_rational_max_bit_bucketer;
use malachite_q::test_util::generators::rational_rational_rational_unsigned_quadruple_gen_var_1;
use malachite_q::test_util::rational::arithmetic::add_mul_shl::add_mul_shl_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_add_mul_shl);
    register_demo!(runner, demo_rational_add_mul_shl_ref);
    register_demo!(runner, demo_rational_add_mul_shl_assign);

    register_bench!(runner, benchmark_rational_add_mul_shl_evaluation_strategy);
    register_bench!(runner, benchmark_rational_add_mul_shl_algorithms);
}

fn demo_rational_add_mul_shl(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, bits) in rational_rational_rational_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "{x_old}.add_mul_shl({y_old}, {z_old}, {bits}) = {}",
            x.add_mul_shl(y, z, bits)
        );
    }
}

fn demo_rational_add_mul_shl_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, bits) in rational_rational_rational_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&{x}).add_mul_shl(&{y}, &{z}, {bits}) = {}",
            (&x).add_mul_shl(&y, &z, bits)
        );
    }
}

fn demo_rational_add_mul_shl_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, bits) in rational_rational_rational_unsigned_quadruple_gen_var_1::<u64>()
        .get(gm, config)
        .take(limit)
    {
        let x_old = x.clone();
        x.add_mul_shl_assign(&y, &z, bits);
        println!("x := {x_old}; x.add_mul_shl_assign(&{y}, &{z}, {bits}); x = {x}");
    }
}

fn benchmark_rational_add_mul_shl_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.add_mul_shl(Rational, Rational, u64)",
        BenchmarkType::EvaluationStrategy,
        rational_rational_rational_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Rational.add_mul_shl(Rational, Rational, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.add_mul_shl(y, z, bits));
                },
            ),
            (
                "Rational.add_mul_shl(Rational, &Rational, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.add_mul_shl(y, &z, bits));
                },
            ),
            (
                "Rational.add_mul_shl(&Rational, Rational, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.add_mul_shl(&y, z, bits));
                },
            ),
            (
                "Rational.add_mul_shl(&Rational, &Rational, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!(x.add_mul_shl(&y, &z, bits));
                },
            ),
            (
                "(&Rational).add_mul_shl(&Rational, &Rational, u64)",
                &mut |(x, y, z, bits)| {
                    no_out!((&x).add_mul_shl(&y, &z, bits));
                },
            ),
            (
                "Rational.add_mul_shl_assign(Rational, Rational, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.add_mul_shl_assign(y, z, bits);
                },
            ),
            (
                "Rational.add_mul_shl_assign(Rational, &Rational, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.add_mul_shl_assign(y, &z, bits);
                },
            ),
            (
                "Rational.add_mul_shl_assign(&Rational, Rational, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.add_mul_shl_assign(&y, z, bits);
                },
            ),
            (
                "Rational.add_mul_shl_assign(&Rational, &Rational, u64)",
                &mut |(mut x, y, z, bits)| {
                    x.add_mul_shl_assign(&y, &z, bits);
                },
            ),
        ],
    );
}

fn benchmark_rational_add_mul_shl_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.add_mul_shl(Rational, Rational, u64)",
        BenchmarkType::Algorithms,
        rational_rational_rational_unsigned_quadruple_gen_var_1::<u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_2_3_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("default", &mut |(x, y, z, bits)| {
                no_out!(x.add_mul_shl(y, z, bits));
            }),
            ("unfused", &mut |(x, y, z, bits)| {
                no_out!(x + ((y * z) << bits));
            }),
            ("naive", &mut |(x, y, z, bits)| {
                no_out!(add_mul_shl_naive(&x, &y, &z, bits));
            }),
        ],
    );
}
