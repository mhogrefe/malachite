// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_triple_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_checked_sub_mul);
    register_demo!(runner, demo_natural_checked_sub_mul_val_val_ref);
    register_demo!(runner, demo_natural_checked_sub_mul_val_ref_val);
    register_demo!(runner, demo_natural_checked_sub_mul_val_ref_ref);
    register_demo!(runner, demo_natural_checked_sub_mul_ref_ref_ref);

    register_bench!(
        runner,
        benchmark_natural_checked_sub_mul_evaluation_strategy
    );
    register_bench!(runner, benchmark_natural_checked_sub_mul_algorithms);
    register_bench!(
        runner,
        benchmark_natural_checked_sub_mul_val_val_ref_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_checked_sub_mul_val_ref_val_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_checked_sub_mul_val_ref_ref_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_checked_sub_mul_ref_ref_ref_algorithms
    );
}

fn demo_natural_checked_sub_mul(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.checked_sub_mul({}, {}) = {:?}",
            a_old,
            b_old,
            c_old,
            a.checked_sub_mul(b, c)
        );
    }
}

fn demo_natural_checked_sub_mul_val_val_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.checked_sub_mul({}, &{}) = {:?}",
            a_old,
            b_old,
            c,
            a.checked_sub_mul(b, &c)
        );
    }
}

fn demo_natural_checked_sub_mul_val_ref_val(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.checked_sub_mul(&{}, {}) = {:?}",
            a_old,
            b,
            c_old,
            a.checked_sub_mul(&b, c)
        );
    }
}

fn demo_natural_checked_sub_mul_val_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!(
            "{}.checked_sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            a.checked_sub_mul(&b, &c)
        );
    }
}

fn demo_natural_checked_sub_mul_ref_ref_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for (a, b, c) in natural_triple_gen().get(gm, config).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).checked_sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            (&a).checked_sub_mul(&b, &c)
        );
    }
}

fn benchmark_natural_checked_sub_mul_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sub_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "Natural.checked_sub_mul(Natural, Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub_mul(b, c)),
            ),
            (
                "Natural.checked_sub_mul(Natural, &Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub_mul(b, &c)),
            ),
            (
                "Natural.checked_sub_mul(&Natural, Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub_mul(&b, c)),
            ),
            (
                "Natural.checked_sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub_mul(&b, &c)),
            ),
            (
                "(&Natural).checked_sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).checked_sub_mul(&b, &c)),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sub_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Natural.sub_mul(Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.checked_sub_mul(b, c))
            }),
            (
                "Natural.checked_sub(Natural * Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub(b * c)),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_val_val_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sub_mul(Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Natural.sub_mul(Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.checked_sub_mul(b, &c))
            }),
            (
                "Natural.checked_sub(Natural * &Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub(b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_val_ref_val_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sub_mul(&Natural, Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Natural.sub_mul(&Natural, Natural)", &mut |(a, b, c)| {
                no_out!(a.checked_sub_mul(&b, c))
            }),
            (
                "Natural.checked_sub(&Natural * Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub(&b * c)),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_val_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.checked_sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            ("Natural.sub_mul(&Natural, &Natural)", &mut |(a, b, c)| {
                no_out!(a.checked_sub_mul(&b, &c))
            }),
            (
                "Natural.checked_sub(&Natural * &Natural)",
                &mut |(a, b, c)| no_out!(a.checked_sub(&b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_ref_ref_ref_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&Natural).checked_sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        natural_triple_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_natural_max_bit_bucketer("x", "y", "z"),
        &mut [
            (
                "(&Natural).checked_sub_mul(&Natural, &Natural)",
                &mut |(a, b, c)| no_out!((&a).checked_sub_mul(&b, &c)),
            ),
            (
                "(&Natural).checked_sub(&Natural * &Natural)",
                &mut |(a, b, c)| no_out!((&a).checked_sub(&b * &c)),
            ),
        ],
    );
}
