// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_numerator_ref);
    register_demo!(runner, demo_rational_polynomial_denominator_ref);
    register_demo!(
        runner,
        demo_rational_polynomial_into_numerator_and_denominator
    );
    register_bench!(runner, benchmark_rational_polynomial_numerator_ref);
    register_bench!(runner, benchmark_rational_polynomial_denominator_ref);
    register_bench!(
        runner,
        benchmark_rational_polynomial_into_numerator_and_denominator
    );
}

fn demo_rational_polynomial_numerator_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).numerator_ref() = {}", p.numerator_ref());
    }
}

fn demo_rational_polynomial_denominator_ref(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).denominator_ref() = {}", p.denominator_ref());
    }
}

fn demo_rational_polynomial_into_numerator_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        let p_old = p.clone();
        let (numerator, denominator) = p.into_numerator_and_denominator();
        println!("({p_old}).into_numerator_and_denominator() = ({numerator}, {denominator})");
    }
}

fn benchmark_rational_polynomial_numerator_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.numerator_ref()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.numerator_ref()))],
    );
}

fn benchmark_rational_polynomial_denominator_ref(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.denominator_ref()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.denominator_ref()))],
    );
}

fn benchmark_rational_polynomial_into_numerator_and_denominator(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.into_numerator_and_denominator()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            no_out!(p.into_numerator_and_denominator());
        })],
    );
}
