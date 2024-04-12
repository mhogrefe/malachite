// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_triple_gen, unsigned_triple_gen_var_19};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_eq_mod_unsigned);
    register_signed_demos!(runner, demo_eq_mod_signed);

    register_unsigned_benches!(runner, benchmark_eq_mod_unsigned);
    register_signed_benches!(runner, benchmark_eq_mod_signed);
}

fn demo_eq_mod_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen_var_19::<T>()
        .get(gm, config)
        .take(limit)
    {
        if x.eq_mod(y, z) {
            println!("{x} is equal to {y} mod {z}");
        } else {
            println!("{x} is not equal to {y} mod {z}");
        }
    }
}

fn demo_eq_mod_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, z) in signed_triple_gen::<T>().get(gm, config).take(limit) {
        if x.eq_mod(y, z) {
            println!("{x} is equal to {y} mod {z}");
        } else {
            println!("{x} is not equal to {y} mod {z}");
        }
    }
}

fn benchmark_eq_mod_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_mod({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_19::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.eq_mod(y, z)))],
    );
}

fn benchmark_eq_mod_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.eq_mod({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut |(x, y, z)| no_out!(x.eq_mod(y, z)))],
    );
}
