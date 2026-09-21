// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_to_latex);
    register_bench!(runner, benchmark_integer_polynomial_to_latex_string);
}

fn demo_integer_polynomial_to_latex(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in integer_polynomial_gen().get(gm, config).take(limit) {
        println!("{}.to_latex() = {}", x, x.to_latex());
    }
}

fn benchmark_integer_polynomial_to_latex_string(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "IntegerPolynomial.to_latex_string()",
        BenchmarkType::Single,
        integer_polynomial_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &integer_polynomial_bit_bucketer("p"),
        &mut [("Malachite", &mut |x| no_out!(x.to_latex_string()))],
    );
}
