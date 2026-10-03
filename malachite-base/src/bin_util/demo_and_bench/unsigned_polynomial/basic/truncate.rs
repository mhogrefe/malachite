// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::bench::bucketers::pair_1_unsigned_polynomial_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_pair_gen_var_1;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_truncate);
    register_demo!(runner, demo_unsigned_polynomial_truncate_assign);
    register_bench!(
        runner,
        benchmark_unsigned_polynomial_truncate_evaluation_strategy
    );
}

fn demo_unsigned_polynomial_truncate(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, len) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        println!("({p}).truncate({len}) = {}", p.truncate(len));
    }
}

fn demo_unsigned_polynomial_truncate_assign(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut p, len) in unsigned_polynomial_unsigned_pair_gen_var_1()
        .get(gm, config)
        .take(limit)
    {
        let p_old = p.clone();
        p.truncate_assign(len);
        println!("p := {p_old}; p.truncate_assign({len}); p = {p}");
    }
}

fn benchmark_unsigned_polynomial_truncate_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.truncate(u64)",
        BenchmarkType::EvaluationStrategy,
        unsigned_polynomial_unsigned_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_unsigned_polynomial_bit_bucketer("p"),
        &mut [
            ("p.truncate(u64)", &mut |(p, len)| no_out!(p.truncate(len))),
            ("p.truncate_assign(u64)", &mut |(mut p, len)| {
                p.truncate_assign(len);
            }),
        ],
    );
}
