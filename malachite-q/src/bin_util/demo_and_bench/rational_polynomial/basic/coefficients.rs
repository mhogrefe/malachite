// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_q::test_util::bench::bucketers::{
    pair_1_rational_polynomial_bit_bucketer, rational_polynomial_bit_bucketer,
};
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_unsigned_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_degree);
    register_demo!(runner, demo_rational_polynomial_coefficients_asc);
    register_demo!(runner, demo_rational_polynomial_into_coefficients_asc);
    register_demo!(runner, demo_rational_polynomial_coefficient);
    register_demo!(runner, demo_rational_polynomial_leading_coefficient);

    register_bench!(runner, benchmark_rational_polynomial_degree);
    register_bench!(runner, benchmark_rational_polynomial_coefficients_asc);
    register_bench!(runner, benchmark_rational_polynomial_into_coefficients_asc);
    register_bench!(runner, benchmark_rational_polynomial_coefficient);
    register_bench!(runner, benchmark_rational_polynomial_leading_coefficient);
}

fn demo_rational_polynomial_degree(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).degree() = {:?}", p.degree());
    }
}

fn demo_rational_polynomial_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).to_coefficients_asc() = {}",
            p.to_coefficients_asc().to_debug_string()
        );
    }
}

fn demo_rational_polynomial_into_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).into_coefficients_asc() = {}",
            p.clone().into_coefficients_asc().to_debug_string()
        );
    }
}

fn demo_rational_polynomial_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, i) in rational_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({p}).coefficient({i}) = {}", p.coefficient(i));
    }
}

fn demo_rational_polynomial_leading_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).leading_coefficient() = {}", p.leading_coefficient());
    }
}

fn benchmark_rational_polynomial_degree(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.degree()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.degree()))],
    );
}

fn benchmark_rational_polynomial_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.to_coefficients_asc()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.to_coefficients_asc()))],
    );
}

fn benchmark_rational_polynomial_into_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.into_coefficients_asc()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.into_coefficients_asc()))],
    );
}

fn benchmark_rational_polynomial_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.coefficient(u64)",
        BenchmarkType::Single,
        rational_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, i)| no_out!(p.coefficient(i)))],
    );
}

fn benchmark_rational_polynomial_leading_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.leading_coefficient()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.leading_coefficient()))],
    );
}
