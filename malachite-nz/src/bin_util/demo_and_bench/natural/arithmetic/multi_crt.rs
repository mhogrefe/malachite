// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::bench::bucketers::pair_1_natural_vec_total_bit_bucketer;
use malachite_nz::test_util::generators::natural_vec_pair_gen_var_1;
use malachite_nz::test_util::natural::arithmetic::crt::multi_crt_simple;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_multi_crt);

    register_bench!(runner, benchmark_natural_multi_crt_algorithms);
}

fn demo_natural_multi_crt(gm: GenMode, config: &GenConfig, limit: usize) {
    for (ms, vs) in natural_vec_pair_gen_var_1().get(gm, config).take(limit) {
        println!(
            "multi_crt({ms:?}, {vs:?}) = {:?}",
            Natural::multi_crt(&ms, &vs)
        );
    }
}

fn benchmark_natural_multi_crt_algorithms(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::multi_crt(&[Natural], &[Natural])",
        BenchmarkType::Algorithms,
        natural_vec_pair_gen_var_1().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_vec_total_bit_bucketer("moduli"),
        &mut [
            ("subproduct tree", &mut |(ms, vs)| {
                no_out!(Natural::multi_crt(&ms, &vs));
            }),
            ("pairwise fold", &mut |(ms, vs)| {
                no_out!(multi_crt_simple(&ms, &vs));
            }),
        ],
    );
}
