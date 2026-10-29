// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::ModPowerOf2InverseTruncated;
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_unsigned_unsigned_triple_gen_var_4,
    natural_polynomial_unsigned_unsigned_triple_gen_var_6,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_inverse_truncated
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_inverse_truncated_ref
    );
    register_demo!(
        runner,
        demo_natural_polynomial_mod_power_of_2_inverse_truncated_long
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_inverse_truncated_evaluation_strategy
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_inverse_truncated_algorithms
    );
    register_bench!(
        runner,
        benchmark_natural_polynomial_mod_power_of_2_inverse_truncated_algorithms_long
    );
}

fn show(p: Option<NaturalPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

fn demo_natural_polynomial_mod_power_of_2_inverse_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        println!(
            "({p_old}).mod_power_of_2_inverse_truncated({len}, {pow}) = {}",
            show(p.mod_power_of_2_inverse_truncated(len, pow))
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_inverse_truncated_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_inverse_truncated({len}, {pow}) = {}",
            show((&p).mod_power_of_2_inverse_truncated(len, pow))
        );
    }
}

fn demo_natural_polynomial_mod_power_of_2_inverse_truncated_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (p, len, pow) in natural_polynomial_unsigned_unsigned_triple_gen_var_6()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_power_of_2_inverse_truncated({len}, {pow}) = {}",
            show((&p).mod_power_of_2_inverse_truncated(len, pow))
        );
    }
}

fn benchmark_natural_polynomial_mod_power_of_2_inverse_truncated_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_power_of_2_inverse_truncated(u64, u64)",
        BenchmarkType::EvaluationStrategy,
        natural_polynomial_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            (
                "p.mod_power_of_2_inverse_truncated(len, pow)",
                &mut |(p, len, pow)| {
                    no_out!(p.mod_power_of_2_inverse_truncated(len, pow));
                },
            ),
            (
                "(&p).mod_power_of_2_inverse_truncated(len, pow)",
                &mut |(p, len, pow)| {
                    no_out!((&p).mod_power_of_2_inverse_truncated(len, pow));
                },
            ),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_inverse_truncated_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_power_of_2_inverse_truncated(u64, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            ("default", &mut |(p, len, pow)| {
                no_out!((&p).mod_power_of_2_inverse_truncated(len, pow));
            }),
            ("naive", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_naive(&p, len, pow));
            }),
            ("basecase", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_basecase_wrapper(
                    &p, len, pow
                ));
            }),
            ("Newton", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_newton_wrapper(
                    &p, len, pow
                ));
            }),
        ],
    );
}

fn benchmark_natural_polynomial_mod_power_of_2_inverse_truncated_algorithms_long(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&NaturalPolynomial).mod_power_of_2_inverse_truncated(u64, u64)",
        BenchmarkType::Algorithms,
        natural_polynomial_unsigned_unsigned_triple_gen_var_6().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("len"),
        &mut [
            ("default", &mut |(p, len, pow)| {
                no_out!((&p).mod_power_of_2_inverse_truncated(len, pow));
            }),
            ("basecase", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_basecase_wrapper(
                    &p, len, pow
                ));
            }),
            ("Newton", &mut |(p, len, pow)| {
                no_out!(mod_power_of_2_inverse_truncated_newton_wrapper(
                    &p, len, pow
                ));
            }),
        ],
    );
}
