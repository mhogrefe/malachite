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
use malachite_nz::test_util::bench::bucketers::{
    natural_polynomial_bit_bucketer, pair_1_natural_polynomial_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_unsigned_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_polynomial_degree);
    register_demo!(runner, demo_natural_polynomial_coefficients_asc);
    register_demo!(runner, demo_natural_polynomial_into_coefficients_asc);
    register_demo!(runner, demo_natural_polynomial_coefficient);
    register_demo!(runner, demo_natural_polynomial_leading_coefficient);

    register_bench!(runner, benchmark_natural_polynomial_degree);
    register_bench!(runner, benchmark_natural_polynomial_coefficients_asc);
    register_bench!(runner, benchmark_natural_polynomial_into_coefficients_asc);
    register_bench!(runner, benchmark_natural_polynomial_coefficient);
    register_bench!(runner, benchmark_natural_polynomial_leading_coefficient);
}

fn demo_natural_polynomial_degree(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).degree() = {:?}", p.degree());
    }
}

fn demo_natural_polynomial_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).coefficients_asc() = {}",
            p.coefficients_asc().to_debug_string()
        );
    }
}

fn demo_natural_polynomial_into_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).into_coefficients_asc() = {}",
            p.clone().into_coefficients_asc().to_debug_string()
        );
    }
}

fn demo_natural_polynomial_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, i) in natural_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({p}).coefficient({i}) = {}", p.coefficient(i));
    }
}

fn demo_natural_polynomial_leading_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in natural_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).leading_coefficient() = {}", p.leading_coefficient());
    }
}

fn benchmark_natural_polynomial_degree(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.degree()",
        BenchmarkType::Single,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.degree()))],
    );
}

fn benchmark_natural_polynomial_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.coefficients_asc()",
        BenchmarkType::Single,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.coefficients_asc()))],
    );
}

fn benchmark_natural_polynomial_into_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.into_coefficients_asc()",
        BenchmarkType::Single,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.into_coefficients_asc()))],
    );
}

fn benchmark_natural_polynomial_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.coefficient(u64)",
        BenchmarkType::Single,
        natural_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, i)| no_out!(p.coefficient(i)))],
    );
}

fn benchmark_natural_polynomial_leading_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "NaturalPolynomial.leading_coefficient()",
        BenchmarkType::Single,
        natural_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &natural_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.leading_coefficient()))],
    );
}
