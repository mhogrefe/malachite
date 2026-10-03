// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::polynomial::EqTruncated;
use malachite_base::test_util::bench::bucketers::triple_1_2_unsigned_polynomial_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::*;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_unsigned_polynomial_eq_truncated);
    register_bench!(runner, benchmark_unsigned_polynomial_eq_truncated);
}

fn demo_unsigned_polynomial_eq_truncated(gm: GenMode, config: &GenConfig, limit: usize) {
    for (p, q, len) in unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_3()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({p}).eq_truncated(&({q}), {len}) = {}",
            p.eq_truncated(&q, len)
        );
    }
}

fn benchmark_unsigned_polynomial_eq_truncated(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "UnsignedPolynomial<u64>.eq_truncated(&UnsignedPolynomial<u64>, u64)",
        BenchmarkType::Single,
        unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_unsigned_polynomial_max_len_bucketer("p", "q"),
        &mut [("Malachite", &mut |(p, q, len)| {
            no_out!(p.eq_truncated(&q, len));
        })],
    );
}
