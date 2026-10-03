// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::bench::bucketers::unsigned_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::test_util::runner::Runner;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::GreekVars;
use malachite_base::vars::indexed::IndexedVars;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_to_latex);
    register_demo!(runner, demo_unsigned_polynomial_to_latex_string_with);
    register_bench!(runner, benchmark_unsigned_polynomial_to_latex_string);
    register_bench!(runner, benchmark_unsigned_polynomial_to_latex_string_with);
}

fn demo_unsigned_polynomial_to_latex(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!("{}.to_latex() = {}", x, x.to_latex());
    }
}

fn benchmark_unsigned_polynomial_to_latex_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.to_latex_string()",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |x| no_out!(x.to_latex_string()))],
    );
}

fn demo_unsigned_polynomial_to_latex_string_with(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in unsigned_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "{} is {} and {}",
            p,
            p.to_latex_string_with(GreekVars.var(0)),
            p.to_latex_string_with(IndexedVars.var(7))
        );
    }
}

fn benchmark_unsigned_polynomial_to_latex_string_with(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.to_latex_string_with(Var)",
        BenchmarkType::Single,
        unsigned_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            no_out!(p.to_latex_string_with(IndexedVars.var(7)));
        })],
    );
}
