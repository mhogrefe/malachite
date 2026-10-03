// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_base::vars::VarScheme;
use malachite_base::vars::greek::GreekVars;
use malachite_base::vars::indexed::IndexedVars;
use malachite_q::test_util::bench::bucketers::rational_polynomial_bit_bucketer;
use malachite_q::test_util::generators::rational_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_polynomial_to_typst);
    register_demo!(runner, demo_rational_polynomial_to_typst_string_with);
    register_bench!(runner, benchmark_rational_polynomial_to_typst_string);
    register_bench!(runner, benchmark_rational_polynomial_to_typst_string_with);
}

fn demo_rational_polynomial_to_typst(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in rational_polynomial_gen().get(gm, config).take(limit) {
        println!("{}.to_typst() = {}", x, x.to_typst());
    }
}

fn benchmark_rational_polynomial_to_typst_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.to_typst_string()",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |x| no_out!(x.to_typst_string()))],
    );
}

fn demo_rational_polynomial_to_typst_string_with(gm: GenMode, config: &GenConfig, limit: usize) {
    for p in rational_polynomial_gen().get(gm, config).take(limit) {
        println!(
            "{} is {} and {}",
            p,
            p.to_typst_string_with(GreekVars.var(0)),
            p.to_typst_string_with(IndexedVars.var(7))
        );
    }
}

fn benchmark_rational_polynomial_to_typst_string_with(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalPolynomial.to_typst_string_with(Var)",
        BenchmarkType::Single,
        rational_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &rational_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |p| {
            no_out!(p.to_typst_string_with(IndexedVars.var(7)));
        })],
    );
}
