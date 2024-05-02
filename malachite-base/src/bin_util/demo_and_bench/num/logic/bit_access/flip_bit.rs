// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_2, unsigned_pair_gen_var_3,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_flip_bit_unsigned);
    register_signed_demos!(runner, demo_flip_bit_signed);
    register_unsigned_benches!(runner, benchmark_flip_bit_unsigned);
    register_signed_benches!(runner, benchmark_flip_bit_signed);
}

fn demo_flip_bit_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, index) in unsigned_pair_gen_var_3::<T>().get(gm, config).take(limit) {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {n_old}; x.flip_bit({index}); x = {n}");
    }
}

fn demo_flip_bit_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, index) in signed_unsigned_pair_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {n_old}; x.flip_bit({index}); x = {n}");
    }
}

fn benchmark_flip_bit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.flip_bit(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index)| n.flip_bit(index))],
    );
}

fn benchmark_flip_bit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.flip_bit(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index)| n.flip_bit(index))],
    );
}
