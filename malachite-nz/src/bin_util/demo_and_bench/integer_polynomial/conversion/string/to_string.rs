// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::strings::latex::ToLatex;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::GreekVars;
use malachite_base::vars::indexed::IndexedVars;
use malachite_nz::test_util::bench::bucketers::integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_to_string);
    register_demo!(runner, demo_integer_polynomial_to_string_with);
    register_bench!(runner, benchmark_integer_polynomial_to_string);
    register_bench!(
        runner,
        benchmark_integer_polynomial_to_string_by_language_algorithms
    );
}

fn demo_integer_polynomial_to_string(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("{p}");
    }
}

fn demo_integer_polynomial_to_string_with(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in integer_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "{} is {} and {}",
            p,
            p.to_string_with(GreekVars.var(0)),
            p.to_string_with(IndexedVars.var(7))
        );
    }
}

fn benchmark_integer_polynomial_to_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.to_string()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| no_out!(p.to_string()))],
    );
}

// The three languages share one writer, and differ only in how they spell a variable and attach an
// exponent. This shows what that difference costs.
fn benchmark_integer_polynomial_to_string_by_language_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.to_string()",
        BenchmarkType::Algorithms,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [
            ("plain", &mut |p| no_out!(p.to_string())),
            ("LaTeX", &mut |p| no_out!(p.to_latex_string())),
            ("Typst", &mut |p| no_out!(p.to_typst_string())),
        ],
    );
}
