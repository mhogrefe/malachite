// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{MulSubMul, MulSubMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::quadruple_rational_max_bit_bucketer;
use malachite_q::test_util::generators::rational_quadruple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_mul_sub_mul);
    register_demo!(runner, demo_rational_mul_sub_mul_ref_ref_ref_ref);
    register_demo!(runner, demo_rational_mul_sub_mul_assign);

    register_bench!(runner, benchmark_rational_mul_sub_mul_evaluation_strategy);
    register_bench!(runner, benchmark_rational_mul_sub_mul_algorithms);
}

fn demo_rational_mul_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in rational_quadruple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        let w_old = w.clone();
        println!(
            "({x_old}).mul_sub_mul({y_old}, {z_old}, {w_old}) = {}",
            x.mul_sub_mul(y, z, w)
        );
    }
}

fn demo_rational_mul_sub_mul_ref_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z, w) in rational_quadruple_gen().get(gm, config).take(limit) {
        println!(
            "(&{x}).mul_sub_mul(&{y}, &{z}, &{w}) = {}",
            (&x).mul_sub_mul(&y, &z, &w)
        );
    }
}

fn demo_rational_mul_sub_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z, w) in rational_quadruple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        let w_old = w.clone();
        x.mul_sub_mul_assign(y, z, w);
        println!("x := {x_old}; x.mul_sub_mul_assign({y_old}, {z_old}, {w_old}); x = {x}");
    }
}

fn benchmark_rational_mul_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mul_sub_mul(Rational, Rational, Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_quadruple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_rational_max_bit_bucketer("x", "y", "z", "w"),
        &mut [
            (
                "Rational.mul_sub_mul(Rational, Rational, Rational)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(y, z, w)),
            ),
            (
                "Rational.mul_sub_mul(&Rational, &Rational, &Rational)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(&y, &z, &w)),
            ),
            (
                "(&Rational).mul_sub_mul(&Rational, &Rational, &Rational)",
                &mut |(x, y, z, w)| no_out!((&x).mul_sub_mul(&y, &z, &w)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_mul_sub_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.mul_sub_mul(Rational, Rational, Rational)",
        BenchmarkType::Algorithms,
        rational_quadruple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &quadruple_rational_max_bit_bucketer("x", "y", "z", "w"),
        &mut [
            (
                "Rational.mul_sub_mul(Rational, Rational, Rational)",
                &mut |(x, y, z, w)| no_out!(x.mul_sub_mul(y, z, w)),
            ),
            (
                "Rational * Rational - Rational * Rational",
                &mut |(x, y, z, w)| {
                    no_out!(x * y - z * w);
                },
            ),
        ],
    );
}
