// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    integer_polynomial_bit_bucketer, pair_1_integer_polynomial_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_unsigned_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_degree);
    register_demo!(runner, demo_integer_polynomial_len);
    register_demo!(runner, demo_integer_polynomial_coefficients_asc);
    register_demo!(runner, demo_integer_polynomial_into_coefficients_asc);
    register_demo!(runner, demo_integer_polynomial_coefficient);
    register_demo!(runner, demo_integer_polynomial_leading_coefficient);
    register_demo!(runner, demo_integer_polynomial_is_monic);

    register_bench!(runner, benchmark_integer_polynomial_degree);
    register_bench!(runner, benchmark_integer_polynomial_len);
    register_bench!(runner, benchmark_integer_polynomial_coefficients_asc);
    register_bench!(runner, benchmark_integer_polynomial_into_coefficients_asc);
    register_bench!(runner, benchmark_integer_polynomial_coefficient);
    register_bench!(runner, benchmark_integer_polynomial_leading_coefficient);
    register_bench!(runner, benchmark_integer_polynomial_is_monic);
}

fn demo_integer_polynomial_degree(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).degree() = {:?}", p.degree());
    }
}

fn demo_integer_polynomial_len(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).len() = {}", p.len());
    }
}

fn demo_integer_polynomial_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).coefficients_asc() = {}",
            p.coefficients_asc().to_debug_string()
        );
    }
}

fn demo_integer_polynomial_into_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).into_coefficients_asc() = {}",
            p.clone().into_coefficients_asc().to_debug_string()
        );
    }
}

fn demo_integer_polynomial_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, i) in integer_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({p}).coefficient({i}) = {}", p.coefficient(i));
    }
}

fn demo_integer_polynomial_leading_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).leading_coefficient() = {}", p.leading_coefficient());
    }
}

fn benchmark_integer_polynomial_degree(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.degree()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.degree()))],
    );
}

fn benchmark_integer_polynomial_len(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.len()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.len()))],
    );
}

fn benchmark_integer_polynomial_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.coefficients_asc()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.coefficients_asc()))],
    );
}

fn benchmark_integer_polynomial_into_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.into_coefficients_asc()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.into_coefficients_asc()))],
    );
}

fn benchmark_integer_polynomial_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.coefficient(u64)",
        BenchmarkType::Single,
        integer_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, i)| no_out!(p.coefficient(i)))],
    );
}

fn benchmark_integer_polynomial_leading_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.leading_coefficient()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.leading_coefficient()))],
    );
}

fn demo_integer_polynomial_is_monic(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).is_monic() = {}", p.is_monic());
    }
}

fn benchmark_integer_polynomial_is_monic(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.is_monic()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.is_monic()))],
    );
}
