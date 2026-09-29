// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::bench::bucketers::triple_1_2_vec_max_len_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::mul_high::classical::mul_high_to_out_classical;
use malachite_nz::test_util::generators::integer_vec_integer_vec_unsigned_triple_gen_var_4;
use malachite_nz::test_util::integer_polynomial::arithmetic::mul::*;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_mul_high_to_out_classical);

    register_bench!(runner, benchmark_mul_high_to_out_algorithms);
}

fn demo_mul_high_to_out_classical(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys, start) in integer_vec_integer_vec_unsigned_triple_gen_var_4()
        .get(gm, config)
        .take(limit)
    {
        let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
        mul_high_to_out_classical(&mut out, &xs, &ys, usize::exact_from(start));
        println!("mul_high_to_out_classical(_, {xs:?}, {ys:?}, {start}) = {out:?}");
    }
}

fn benchmark_mul_high_to_out_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "mul_high_to_out(&mut [Integer], &[Integer], &[Integer], usize)",
        BenchmarkType::Algorithms,
        integer_vec_integer_vec_unsigned_triple_gen_var_4().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_vec_max_len_bucketer("xs", "ys"),
        &mut [
            ("classical", &mut |(xs, ys, start)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                mul_high_to_out_classical(&mut out, &xs, &ys, usize::exact_from(start));
            }),
            ("naive", &mut |(xs, ys, start)| {
                let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
                {
                    let start = usize::exact_from(start);
                    out[start..].clone_from_slice(&integers_mul_naive(&xs, &ys)[start..]);
                };
            }),
        ],
    );
}
