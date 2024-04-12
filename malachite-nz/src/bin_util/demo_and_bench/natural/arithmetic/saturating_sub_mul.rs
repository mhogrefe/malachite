// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    SaturatingSub, SaturatingSubAssign, SaturatingSubMul, SaturatingSubMulAssign,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_saturating_sub_mul);
    register_demo!(runner, demo_natural_saturating_sub_mul_val_val_ref);
    register_demo!(runner, demo_natural_saturating_sub_mul_val_ref_val);
    register_demo!(runner, demo_natural_saturating_sub_mul_val_ref_ref);
    register_demo!(runner, demo_natural_saturating_sub_mul_ref_ref_ref);
    register_demo!(runner, demo_natural_saturating_sub_mul_assign);
    register_demo!(runner, demo_natural_saturating_sub_mul_assign_val_ref);
    register_demo!(runner, demo_natural_saturating_sub_mul_assign_ref_val);
    register_demo!(runner, demo_natural_saturating_sub_mul_assign_ref_ref);

    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_saturating_sub_mul_algorithms);
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_val_val_ref_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_val_ref_val_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_val_ref_ref_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_ref_ref_ref_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_assign_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_assign_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_assign_val_ref_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_assign_ref_val_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_saturating_sub_mul_assign_ref_ref_algorithms
    );
}

fn demo_natural_saturating_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        println!(
            "{}.saturating_sub_mul({}, {}) = {}",
            x_old,
            y_old,
            z_old,
            x.saturating_sub_mul(y, z)
        );
    }
}

fn demo_natural_saturating_sub_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.saturating_sub_mul({}, &({})) = {}",
            x_old,
            y_old,
            z,
            x.saturating_sub_mul(y, &z)
        );
    }
}

fn demo_natural_saturating_sub_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        println!(
            "{}.saturating_sub_mul(&({}), {}) = {}",
            x_old,
            y,
            z_old,
            x.saturating_sub_mul(&y, z)
        );
    }
}

fn demo_natural_saturating_sub_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.saturating_sub_mul(&({}), &({})) = {}",
            x_old,
            y,
            z,
            x.saturating_sub_mul(&y, &z)
        );
    }
}

fn demo_natural_saturating_sub_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        println!(
            "(&{}).saturating_sub_mul(&({}), &({})) = {}",
            x,
            y,
            z,
            (&x).saturating_sub_mul(&y, &z)
        );
    }
}

fn demo_natural_saturating_sub_mul_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let z_old = z.clone();
        x.saturating_sub_mul_assign(y, z);
        println!("x := {x_old}; x.saturating_sub_mul_assign({y_old}, {z_old}); x = {x}");
    }
}

fn demo_natural_saturating_sub_mul_assign_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.saturating_sub_mul_assign(y, &z);
        println!("x := {x_old}; x.saturating_sub_mul_assign({y_old}, &({z})); x = {x}");
    }
}

fn demo_natural_saturating_sub_mul_assign_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        let z_old = z.clone();
        x.saturating_sub_mul_assign(&y, z);
        println!("x := {x_old}; x.saturating_sub_mul_assign(&({y}), {z_old}); x = {x}");
    }
}

fn demo_natural_saturating_sub_mul_assign_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y, z) in natural_triple_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.saturating_sub_mul_assign(&y, &z);
        println!("x := {x_old}; x.saturating_sub_mul_assign(&({y}), &({z})); x = {x}");
    }
}

fn benchmark_natural_saturating_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul(Natural, Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(b, c)),
            ),
            (
                "Natural.saturating_sub_mul(Natural, &Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(b, &c)),
            ),
            (
                "Natural.saturating_sub_mul(&Natural, Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(&b, c)),
            ),
            (
                "Natural.saturating_sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(&b, &c)),
            ),
            (
                "(&Natural).saturating_sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).saturating_sub_mul(&b, &c)),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul(Natural, Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(b, c)),
            ),
            (
                "Natural.saturating_sub(Natural * Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub(b * c)),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_val_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul(Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul(Natural, &Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(b, &c)),
            ),
            (
                "Natural.saturating_sub(Natural * &Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub(b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_val_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul(&Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul(&Natural, Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(&b, c)),
            ),
            (
                "Natural.saturating_sub(&Natural * Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub(&b * c)),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_val_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub_mul(&b, &c)),
            ),
            (
                "Natural.saturating_sub(Natural * Natural)",
                &mut |(a, b, c)| no_out!(a.saturating_sub(&b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_ref_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "(&Natural).saturating_sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).saturating_sub_mul(&b, &c)),
            ),
            (
                "(&Natural).saturating_sub(Natural * Natural)",
                &mut |(a, b, c)| no_out!((&a).saturating_sub(&b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul_assign(Natural, Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(b, c),
            ),
            (
                "Natural.saturating_sub_mul_assign(Natural, &Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(b, &c),
            ),
            (
                "Natural.saturating_sub_mul_assign(&Natural, Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(&b, c),
            ),
            (
                "Natural.saturating_sub_mul_assign(&Natural, &Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(&b, &c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul_assign(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul_assign(Natural, Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(b, c),
            ),
            (
                "Natural.saturating_sub_assign(Natural * Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_assign(b * c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul_assign(Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul_assign(Natural, &Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(b, &c),
            ),
            (
                "Natural.saturating_sub_assign(Natural * &Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_assign(b * &c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul_assign(&Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul_assign(&Natural, Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(&b, c),
            ),
            (
                "Natural.saturating_sub_assign(&Natural * Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_assign(&b * c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.saturating_sub_mul_assign(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.saturating_sub_mul_assign(&Natural, &Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_mul_assign(&b, &c),
            ),
            (
                "Natural.saturating_sub_assign(&Natural * &Natural)",
                &mut |(mut a, b, c)| a.saturating_sub_assign(&b * &c),
            ),
        ],
    );
}
