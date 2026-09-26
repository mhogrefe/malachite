// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::bench::bucketers::{
    pair_1_unsigned_polynomial_bit_bucketer, unsigned_polynomial_bit_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_polynomial_gen, unsigned_polynomial_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_degree);
    register_demo!(runner, demo_unsigned_polynomial_len);
    register_demo!(runner, demo_unsigned_polynomial_coefficients_asc);
    register_demo!(runner, demo_unsigned_polynomial_into_coefficients_asc);
    register_demo!(runner, demo_unsigned_polynomial_coefficient);
    register_demo!(runner, demo_unsigned_polynomial_leading_coefficient);
    register_demo!(runner, demo_unsigned_polynomial_is_monic);

    register_bench!(runner, benchmark_unsigned_polynomial_degree);
    register_bench!(runner, benchmark_unsigned_polynomial_len);
    register_bench!(runner, benchmark_unsigned_polynomial_coefficients_asc);
    register_bench!(runner, benchmark_unsigned_polynomial_into_coefficients_asc);
    register_bench!(runner, benchmark_unsigned_polynomial_coefficient);
    register_bench!(runner, benchmark_unsigned_polynomial_leading_coefficient);
    register_bench!(runner, benchmark_unsigned_polynomial_is_monic);
}

fn demo_unsigned_polynomial_degree(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).degree() = {:?}", p.degree());
    }
}

fn demo_unsigned_polynomial_len(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).len() = {}", p.len());
    }
}

fn demo_unsigned_polynomial_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).coefficients_asc() = {}",
            p.coefficients_asc().to_debug_string()
        );
    }
}

fn demo_unsigned_polynomial_into_coefficients_asc(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "({p}).into_coefficients_asc() = {}",
            p.clone().into_coefficients_asc().to_debug_string()
        );
    }
}

fn demo_unsigned_polynomial_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, i) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({p}).coefficient({i}) = {}", p.coefficient(i));
    }
}

fn demo_unsigned_polynomial_leading_coefficient(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).leading_coefficient() = {}", p.leading_coefficient());
    }
}

fn benchmark_unsigned_polynomial_degree(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.degree()",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.degree()))],
    );
}

fn benchmark_unsigned_polynomial_len(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.len()",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.len()))],
    );
}

fn benchmark_unsigned_polynomial_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.coefficients_asc()",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.coefficients_asc()))],
    );
}

fn benchmark_unsigned_polynomial_into_coefficients_asc(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.into_coefficients_asc()",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.into_coefficients_asc()))],
    );
}

fn benchmark_unsigned_polynomial_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.coefficient(u64)",
        BenchmarkType::Single,
        unsigned_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |(p, i)| no_out!(p.coefficient(i)))],
    );
}

fn benchmark_unsigned_polynomial_leading_coefficient(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.leading_coefficient()",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.leading_coefficient()))],
    );
}

fn demo_unsigned_polynomial_is_monic(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!("({p}).is_monic() = {}", p.is_monic());
    }
}

fn benchmark_unsigned_polynomial_is_monic(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.is_monic()",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.is_monic()))],
    );
}
