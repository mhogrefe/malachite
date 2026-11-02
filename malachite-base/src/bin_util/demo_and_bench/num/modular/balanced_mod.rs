// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_pair_gen_var_52, unsigned_pair_gen_var_52};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_balanced_mod_unsigned);
    register_signed_demos!(runner, demo_balanced_mod_signed);
    register_signed_demos!(runner, demo_balanced_mod_assign_signed);

    register_unsigned_benches!(runner, benchmark_balanced_mod_unsigned);
    register_signed_benches!(runner, benchmark_balanced_mod_signed);
}

fn demo_balanced_mod_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_52::<T>().get(gm, config).take(limit) {
        println!("{}.balanced_mod({}) = {}", x, y, x.balanced_mod(y));
    }
}

fn demo_balanced_mod_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_52::<T>().get(gm, config).take(limit) {
        println!("({}).balanced_mod({}) = {}", x, y, x.balanced_mod(y));
    }
}

fn demo_balanced_mod_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in signed_pair_gen_var_52::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.balanced_mod_assign(y);
        println!("x := {old_x}; x.balanced_mod_assign({y}); x = {x}");
    }
}

fn benchmark_balanced_mod_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.balanced_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_52::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.balanced_mod(y)))],
    );
}

fn benchmark_balanced_mod_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.balanced_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_52::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.balanced_mod(y)))],
    );
}
