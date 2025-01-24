// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::test_util::bench::bucketers::signed_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::signed_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_saturating_neg_assign);
    register_signed_benches!(runner, benchmark_saturating_neg_assign);
}

fn demo_saturating_neg_assign<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut i in signed_gen::<T>().get(gm, config).take(limit) {
        let old_i = i;
        i.saturating_neg_assign();
        println!("i := {old_i}; i.saturating_neg_assign(); i = {i}");
    }
}

fn benchmark_saturating_neg_assign<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.saturating_neg_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut i| i.saturating_neg_assign())],
    );
}
