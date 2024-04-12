// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_pair_gen_var_16;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_neg);
    register_unsigned_demos!(runner, demo_mod_neg_assign);
    register_unsigned_benches!(runner, benchmark_mod_neg);
    register_unsigned_benches!(runner, benchmark_mod_neg_assign);
}

fn demo_mod_neg<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, m) in unsigned_pair_gen_var_16::<T>().get(gm, config).take(limit) {
        println!("-{} ≡ {} mod {}", n, n.mod_neg(m), m);
    }
}

fn demo_mod_neg_assign<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, m) in unsigned_pair_gen_var_16::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.mod_neg_assign(m);
        println!("n := {old_n}; n.mod_neg_assign({m}); n = {n}");
    }
}

fn benchmark_mod_neg<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_neg({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_16::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [("Malachite", &mut |(n, m)| no_out!(n.mod_neg(m)))],
    );
}

fn benchmark_mod_neg_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_neg({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_16::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bit_bucketer("m"),
        &mut [("Malachite", &mut |(mut n, m)| n.mod_neg_assign(m))],
    );
}
