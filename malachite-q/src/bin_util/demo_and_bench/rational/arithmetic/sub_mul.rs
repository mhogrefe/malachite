// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::triple_rational_max_bit_bucketer;
use malachite_q::test_util::generators::rational_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sub_mul);
    register_demo!(runner, demo_rational_sub_mul_val_val_ref);
    register_demo!(runner, demo_rational_sub_mul_val_ref_val);
    register_demo!(runner, demo_rational_sub_mul_val_ref_ref);
    register_demo!(runner, demo_rational_sub_mul_ref_ref_ref);
    register_demo!(runner, demo_rational_sub_mul_assign);
    register_demo!(runner, demo_rational_sub_mul_assign_val_ref);
    register_demo!(runner, demo_rational_sub_mul_assign_ref_val);
    register_demo!(runner, demo_rational_sub_mul_assign_ref_ref);

    register_bench!(runner, benchmark_rational_sub_mul_evaluation_strategy);
    register_bench!(runner, benchmark_rational_sub_mul_algorithms);
    register_bench!(
        runner,
        benchmark_rational_sub_mul_assign_evaluation_strategy
    );
    register_bench!(runner, benchmark_rational_sub_mul_assign_algorithms);
}

fn demo_rational_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!("({x_old}).sub_mul({y_old}, {z_old}) = {}", x.sub_mul(y, z));
    }
}

fn demo_rational_sub_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("({x_old}).sub_mul({y_old}, &{z}) = {}", x.sub_mul(y, &z));
    }
}

fn demo_rational_sub_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        println!("({x_old}).sub_mul(&{y}, {z_old}) = {}", x.sub_mul(&y, z));
    }
}

fn demo_rational_sub_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!("({x_old}).sub_mul(&{y}, &{z}) = {}", x.sub_mul(&y, &z));
    }
}

fn demo_rational_sub_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        println!("(&{x}).sub_mul(&{y}, &{z}) = {}", (&x).sub_mul(&y, &z));
    }
}

fn demo_rational_sub_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        x.sub_mul_assign(y, z);
        println!("x := {x_old}; x.sub_mul_assign({y_old}, {z_old}); x = {x}");
    }
}

fn demo_rational_sub_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.sub_mul_assign(y, &z);
        println!("x := {x_old}; x.sub_mul_assign({y_old}, &{z}); x = {x}");
    }
}

fn demo_rational_sub_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        x.sub_mul_assign(&y, z);
        println!("x := {x_old}; x.sub_mul_assign(&{y}, {z_old}); x = {x}");
    }
}

fn demo_rational_sub_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in rational_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.sub_mul_assign(&y, &z);
        println!("x := {x_old}; x.sub_mul_assign(&{y}, &{z}); x = {x}");
    }
}

fn benchmark_rational_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sub_mul(Rational, Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Rational.sub_mul(Rational, Rational)", &mut |(x, y, z)| {
                no_out!(x.sub_mul(y, z));
            }),
            ("Rational.sub_mul(Rational, &Rational)", &mut |(x, y, z)| {
                no_out!(x.sub_mul(y, &z));
            }),
            ("Rational.sub_mul(&Rational, Rational)", &mut |(x, y, z)| {
                no_out!(x.sub_mul(&y, z));
            }),
            (
                "Rational.sub_mul(&Rational, &Rational)",
                &mut |(x, y, z)| no_out!(x.sub_mul(&y, &z)),
            ),
            (
                "(&Rational).sub_mul(&Rational, &Rational)",
                &mut |(x, y, z)| no_out!((&x).sub_mul(&y, &z)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_rational_sub_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sub_mul(Rational, Rational)",
        BenchmarkType::Algorithms,
        rational_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Rational.sub_mul(Rational, Rational)", &mut |(x, y, z)| {
                no_out!(x.sub_mul(y, z));
            }),
            ("Rational - Rational * Rational", &mut |(x, y, z)| {
                no_out!(x - y * z);
            }),
        ],
    );
}

fn benchmark_rational_sub_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sub_mul_assign(Rational, Rational)",
        BenchmarkType::EvaluationStrategy,
        rational_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Rational.sub_mul_assign(Rational, Rational)",
                &mut |(mut x, y, z)| x.sub_mul_assign(y, z),
            ),
            (
                "Rational.sub_mul_assign(Rational, &Rational)",
                &mut |(mut x, y, z)| x.sub_mul_assign(y, &z),
            ),
            (
                "Rational.sub_mul_assign(&Rational, Rational)",
                &mut |(mut x, y, z)| x.sub_mul_assign(&y, z),
            ),
            (
                "Rational.sub_mul_assign(&Rational, &Rational)",
                &mut |(mut x, y, z)| x.sub_mul_assign(&y, &z),
            ),
        ],
    );
}

fn benchmark_rational_sub_mul_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Rational.sub_mul_assign(Rational, Rational)",
        BenchmarkType::Algorithms,
        rational_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_rational_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Rational.sub_mul_assign(Rational, Rational)",
                &mut |(mut x, y, z)| x.sub_mul_assign(y, z),
            ),
            ("Rational -= Rational * Rational", &mut |(mut x, y, z)| {
                x -= y * z;
            }),
        ],
    );
}
