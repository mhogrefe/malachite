// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::mem::replace;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_polynomial_bit_bucketer, triple_1_rational_polynomial_bit_bucketer,
};
use malachite_q::test_util::generators::{
    rational_polynomial_integer_polynomial_natural_triple_gen_var_1,
    rational_polynomial_integer_polynomial_pair_gen, rational_polynomial_natural_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_mutate_numerator);
    register_demo!(runner, demo_rational_polynomial_mutate_denominator);
    register_demo!(
        runner,
        demo_rational_polynomial_mutate_numerator_and_denominator
    );

    register_bench!(runner, benchmark_rational_polynomial_mutate_numerator);
    register_bench!(runner, benchmark_rational_polynomial_mutate_denominator);
    register_bench!(
        runner,
        benchmark_rational_polynomial_mutate_numerator_and_denominator
    );
}

fn demo_rational_polynomial_mutate_numerator(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, new_numerator) in rational_polynomial_integer_polynomial_pair_gen()
        .get(gm, config)
        .take(limit)
    {
        let old_p = p.clone();
        let old_new_numerator = new_numerator.clone();
        let out = p.mutate_numerator(|n| replace(n, new_numerator));
        println!(
            "p := {old_p}; \
            p.mutate_numerator(|n| replace(n, {old_new_numerator})) = {out}; p = {p}",
        );
    }
}

fn demo_rational_polynomial_mutate_denominator(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, new_denominator) in rational_polynomial_natural_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let old_p = p.clone();
        let old_new_denominator = new_denominator.clone();
        let out = p.mutate_denominator(|d| replace(d, new_denominator));
        println!(
            "p := {old_p}; \
            p.mutate_denominator(|d| replace(d, {old_new_denominator})) = {out}; p = {p}",
        );
    }
}

fn demo_rational_polynomial_mutate_numerator_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut p, new_numerator, new_denominator) in
        rational_polynomial_integer_polynomial_natural_triple_gen_var_1()
            .get(gm, config)
            .take(limit)
    {
        let old_p = p.clone();
        let old_new_numerator = new_numerator.clone();
        let old_new_denominator = new_denominator.clone();
        let (old_numerator, old_denominator) = p.mutate_numerator_and_denominator(|n, d| {
            (replace(n, new_numerator), replace(d, new_denominator))
        });
        println!(
            "p := {old_p}; \
            p.mutate_numerator_and_denominator(|n, d| (replace(n, {old_new_numerator}), \
            replace(d, {old_new_denominator}))) = ({old_numerator}, {old_denominator}); p = {p}",
        );
    }
}

fn benchmark_rational_polynomial_mutate_numerator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.mutate_numerator(FnOnce(&mut IntegerPolynomial) -> T)",
        BenchmarkType::Single,
        rational_polynomial_integer_polynomial_pair_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(mut p, new_numerator)| {
            no_out!(p.mutate_numerator(|n| replace(n, new_numerator)));
        })],
    );
}

fn benchmark_rational_polynomial_mutate_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.mutate_denominator(FnOnce(&mut Natural) -> T)",
        BenchmarkType::Single,
        rational_polynomial_natural_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(mut p, new_denominator)| {
            no_out!(p.mutate_denominator(|d| replace(d, new_denominator)));
        })],
    );
}

fn benchmark_rational_polynomial_mutate_numerator_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.mutate_numerator_and_denominator(\
        FnOnce(&mut IntegerPolynomial, &mut Natural) -> T)",
        BenchmarkType::Single,
        rational_polynomial_integer_polynomial_natural_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(
            mut p,
            new_numerator,
            new_denominator,
        )| {
            no_out!(p.mutate_numerator_and_denominator(|n, d| {
                (replace(n, new_numerator), replace(d, new_denominator))
            }));
        })],
    );
}
