// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_bool_triple_gen_var_1, unsigned_unsigned_bool_triple_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_assign_bit_unsigned);
    register_signed_demos!(runner, demo_assign_bit_signed);
    register_unsigned_benches!(runner, benchmark_assign_bit_unsigned);
    register_signed_benches!(runner, benchmark_assign_bit_signed);
}

fn demo_assign_bit_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, index, bit) in unsigned_unsigned_bool_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!("x := {n_old}; x.assign_bit({index}, {bit}); x = {n}");
    }
}

fn demo_assign_bit_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut n, index, bit) in signed_unsigned_bool_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!("x := {n_old}; x.assign_bit({index}, {bit}); x = {n}");
    }
}

fn benchmark_assign_bit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.assign_bit(u64, bool)", T::NAME),
        BenchmarkType::Single,
        unsigned_unsigned_bool_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index, bit)| {
            n.assign_bit(index, bit)
        })],
    );
}

fn benchmark_assign_bit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.assign_bit(u64, bool)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_bool_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index, bit)| {
            n.assign_bit(index, bit)
        })],
    );
}
