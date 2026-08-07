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
use malachite_nz::natural::arithmetic::gcd::extended_gcd_partial::extended_gcd_partial;
use malachite_nz::test_util::bench::bucketers::triple_1_natural_bit_bucketer;
use malachite_nz::test_util::generators::natural_triple_gen_var_10;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_extended_gcd_partial);
    register_bench!(runner, benchmark_natural_extended_gcd_partial);
}

fn demo_natural_extended_gcd_partial(gm: GenMode, config: &GenConfig, limit: usize) {
    for (r1, r2, l) in natural_triple_gen_var_10().get(gm, config).take(limit) {
        let r2_old = r2.clone();
        let r1_old = r1.clone();
        println!(
            "extended_gcd_partial({r2_old}, {r1_old}, {l}) = {:?}",
            extended_gcd_partial(r2, r1, &l)
        );
    }
}

fn benchmark_natural_extended_gcd_partial(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "extended_gcd_partial(Natural, Natural, &Natural)",
        BenchmarkType::Single,
        natural_triple_gen_var_10().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_natural_bit_bucketer("r1"),
        &mut [("Malachite", &mut |(r1, r2, l)| {
            no_out!(extended_gcd_partial(r2, r1, &l));
        })],
    );
}
