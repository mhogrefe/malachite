// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::modular::traits::ModEvaluate;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_1_integer_polynomial_bit_bucketer;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_nz::test_util::integer_polynomial::modular::mod_evaluate::mod_evaluate_u64_naive;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_polynomial_mod_evaluate_u64);
    register_bench!(
        runner,
        benchmark_integer_polynomial_mod_evaluate_u64_algorithms
    );
}

fn demo_integer_polynomial_mod_evaluate_u64(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, x, m) in integer_polynomial_unsigned_unsigned_triple_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "(&({p})).mod_evaluate({x}, {m}) = {}",
            (&p).mod_evaluate(x, m)
        );
    }
}

fn benchmark_integer_polynomial_mod_evaluate_u64_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "(&IntegerPolynomial).mod_evaluate(u64, u64)",
        BenchmarkType::Algorithms,
        integer_polynomial_unsigned_unsigned_triple_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_polynomial_bit_bucketer("p"),
        &mut [
            ("default", &mut |(p, x, m)| no_out!((&p).mod_evaluate(x, m))),
            ("naive", &mut |(p, x, m)| {
                no_out!(mod_evaluate_u64_naive(&p, x, m));
            }),
        ],
    );
}
