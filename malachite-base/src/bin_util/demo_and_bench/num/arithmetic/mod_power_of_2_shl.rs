// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2Shl, ModPowerOf2ShlAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_3_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    unsigned_signed_unsigned_triple_gen_var_1, unsigned_triple_gen_var_17,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_mod_power_of_2_shl_unsigned_unsigned);
    register_unsigned_signed_demos!(runner, demo_mod_power_of_2_shl_unsigned_signed);
    register_unsigned_unsigned_demos!(runner, demo_mod_power_of_2_shl_assign_unsigned_unsigned);
    register_unsigned_signed_demos!(runner, demo_mod_power_of_2_shl_assign_unsigned_signed);

    register_unsigned_unsigned_benches!(runner, benchmark_mod_power_of_2_shl_unsigned_unsigned);
    register_unsigned_signed_benches!(runner, benchmark_mod_power_of_2_shl_unsigned_signed);
    register_unsigned_unsigned_benches!(
        runner,
        benchmark_mod_power_of_2_shl_assign_unsigned_unsigned
    );
    register_unsigned_signed_benches!(runner, benchmark_mod_power_of_2_shl_assign_unsigned_signed);
}

fn demo_mod_power_of_2_shl_unsigned_unsigned<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, pow) in unsigned_triple_gen_var_17::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.mod_power_of_2_shl({}, {}) = {}",
            n,
            u,
            pow,
            n.mod_power_of_2_shl(u, pow)
        );
    }
}

fn demo_mod_power_of_2_shl_unsigned_signed<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, u, pow) in unsigned_signed_unsigned_triple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.mod_power_of_2_shl({}, {}) = {}",
            n,
            u,
            pow,
            n.mod_power_of_2_shl(u, pow)
        );
    }
}

fn demo_mod_power_of_2_shl_assign_unsigned_unsigned<
    T: ModPowerOf2ShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, pow) in unsigned_triple_gen_var_17::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.mod_power_of_2_shl_assign(u, pow);
        println!("x := {old_n}; x.mod_power_of_2_shl_assign({u}, {pow}); x = {n}");
    }
}

fn demo_mod_power_of_2_shl_assign_unsigned_signed<
    T: ModPowerOf2ShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, u, pow) in unsigned_signed_unsigned_triple_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.mod_power_of_2_shl_assign(u, pow);
        println!("x := {old_n}; x.mod_power_of_2_shl_assign({u}, {pow}); x = {n}");
    }
}

fn benchmark_mod_power_of_2_shl_unsigned_unsigned<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_shl({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_17::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(x, y, pow)| {
            no_out!(x.mod_power_of_2_shl(y, pow))
        })],
    );
}

fn benchmark_mod_power_of_2_shl_unsigned_signed<
    T: ModPowerOf2Shl<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_shl({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_1::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(x, y, pow)| {
            no_out!(x.mod_power_of_2_shl(y, pow))
        })],
    );
}

fn benchmark_mod_power_of_2_shl_assign_unsigned_unsigned<
    T: ModPowerOf2ShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_shl_assign({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_17::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, pow)| {
            x.mod_power_of_2_shl_assign(y, pow)
        })],
    );
}

fn benchmark_mod_power_of_2_shl_assign_unsigned_signed<
    T: ModPowerOf2ShlAssign<U> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_shl_assign({}, u64)", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_unsigned_triple_gen_var_1::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, pow)| {
            x.mod_power_of_2_shl_assign(y, pow)
        })],
    );
}
