// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_integer_max_bit_bucketer;
use malachite_nz::test_util::generators::integer_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_add_mul);
    register_demo!(runner, demo_integer_add_mul_val_val_ref);
    register_demo!(runner, demo_integer_add_mul_val_ref_val);
    register_demo!(runner, demo_integer_add_mul_val_ref_ref);
    register_demo!(runner, demo_integer_add_mul_ref_ref_ref);
    register_demo!(runner, demo_integer_add_mul_assign);
    register_demo!(runner, demo_integer_add_mul_assign_val_ref);
    register_demo!(runner, demo_integer_add_mul_assign_ref_val);
    register_demo!(runner, demo_integer_add_mul_assign_ref_ref);

    register_bench!(runner, benchmark_integer_add_mul_evaluation_strategy);
    register_bench!(runner, benchmark_integer_add_mul_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_val_val_ref_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_val_ref_val_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_val_ref_ref_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_ref_ref_ref_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_assign_evaluation_strategy);
    register_bench!(runner, benchmark_integer_add_mul_assign_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_assign_val_ref_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_assign_ref_val_algorithms);
    register_bench!(runner, benchmark_integer_add_mul_assign_ref_ref_algorithms);
}

fn demo_integer_add_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "{}.add_mul({}, {}) = {}",
            x_old,
            y_old,
            z_old,
            x.add_mul(y, z)
        );
    }
}

fn demo_integer_add_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.add_mul({}, &({})) = {}",
            x_old,
            y_old,
            z,
            x.add_mul(y, &z)
        );
    }
}

fn demo_integer_add_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        println!(
            "{}.add_mul(&({}), {}) = {}",
            x_old,
            y,
            z_old,
            x.add_mul(&y, z)
        );
    }
}

fn demo_integer_add_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.add_mul(&({}), &({})) = {}",
            x_old,
            y,
            z,
            x.add_mul(&y, &z)
        );
    }
}

fn demo_integer_add_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).add_mul(&({}), &({})) = {}",
            x,
            y,
            z,
            (&x).add_mul(&y, &z)
        );
    }
}

fn demo_integer_add_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        x.add_mul_assign(y, z);
        println!("x := {x_old}; x.add_mul_assign({y_old}, {z_old}); x = {x}");
    }
}

fn demo_integer_add_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.add_mul_assign(y, &z);
        println!("x := {x_old}; x.add_mul_assign({y_old}, &({z})); x = {x}");
    }
}

fn demo_integer_add_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        x.add_mul_assign(&y, z);
        println!("x := {x_old}; x.add_mul_assign(&({y}), {z_old}); x = {x}");
    }
}

fn demo_integer_add_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in integer_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.add_mul_assign(&y, &z);
        println!("x := {x_old}; x.add_mul_assign(&({y}), &({z})); x = {x}");
    }
}

fn benchmark_integer_add_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.add_mul(Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, c))
            }),
            ("Integer.add_mul(Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, &c))
            }),
            ("Integer.add_mul(&Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, c))
            }),
            ("Integer.add_mul(&Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, &c))
            }),
            (
                "(&Integer).add_mul(&Integer, &Integer)",
                &mut |(a, b, c)| no_out!((&a).add_mul(&b, &c)),
            ),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_add_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul(Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.add_mul(Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, c))
            }),
            ("Integer + Integer * Integer", &mut |(a, b, c)| {
                no_out!(a + b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_add_mul_val_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul(Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.add_mul(Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(b, &c))
            }),
            ("Integer + Integer * &Integer", &mut |(a, b, c)| {
                no_out!(a + b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_add_mul_val_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul(&Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.add_mul(&Integer, Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, c))
            }),
            ("Integer + &Integer * Integer", &mut |(a, b, c)| {
                no_out!(a + &b * c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_add_mul_val_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Integer.add_mul(&Integer, &Integer)", &mut |(a, b, c)| {
                no_out!(a.add_mul(&b, &c))
            }),
            ("Integer + Integer * Integer", &mut |(a, b, c)| {
                no_out!(a + &b * &c)
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_add_mul_ref_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "(&Integer).add_mul(&Integer, &Integer)",
                &mut |(a, b, c)| no_out!((&a).add_mul(&b, &c)),
            ),
            ("&Integer + Integer * Integer", &mut |(a, b, c)| {
                no_out!(&a + &b * &c)
            }),
        ],
    );
}

fn benchmark_integer_add_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul_assign(Integer, Integer)",
        BenchmarkType::EvaluationStrategy,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.add_mul_assign(Integer, Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, c),
            ),
            (
                "Integer.add_mul_assign(Integer, &Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, &c),
            ),
            (
                "Integer.add_mul_assign(&Integer, Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, c),
            ),
            (
                "Integer.add_mul_assign(&Integer, &Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, &c),
            ),
        ],
    );
}

fn benchmark_integer_add_mul_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul_assign(Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.add_mul_assign(Integer, Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, c),
            ),
            ("Integer += Integer * Integer", &mut |(mut a, b, c)| {
                a += b * c
            }),
        ],
    );
}

fn benchmark_integer_add_mul_assign_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul_assign(Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.add_mul_assign(Integer, &Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(b, &c),
            ),
            ("Integer += Integer * &Integer", &mut |(mut a, b, c)| {
                a += b * &c
            }),
        ],
    );
}

fn benchmark_integer_add_mul_assign_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul_assign(&Integer, Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.add_mul_assign(&Integer, Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, c),
            ),
            ("Integer += &Integer * Integer", &mut |(mut a, b, c)| {
                a += &b * c
            }),
        ],
    );
}

fn benchmark_integer_add_mul_assign_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.add_mul_assign(&Integer, &Integer)",
        BenchmarkType::Algorithms,
        integer_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_integer_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Integer.add_mul_assign(&Integer, &Integer)",
                &mut |(mut a, b, c)| a.add_mul_assign(&b, &c),
            ),
            ("Integer += &Integer * &Integer", &mut |(mut a, b, c)| {
                a += &b * &c
            }),
        ],
    );
}
