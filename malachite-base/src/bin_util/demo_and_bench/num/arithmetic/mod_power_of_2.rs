// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_10,
    signed_unsigned_pair_gen_var_11, signed_unsigned_pair_gen_var_4, unsigned_pair_gen_var_2,
    unsigned_pair_gen_var_20,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_unsigned);
    register_signed_demos!(runner, demo_mod_power_of_2_signed);
    register_unsigned_demos!(runner, demo_mod_power_of_2_assign_unsigned);
    register_signed_demos!(runner, demo_mod_power_of_2_assign_signed);
    register_unsigned_demos!(runner, demo_rem_power_of_2_unsigned);
    register_signed_demos!(runner, demo_rem_power_of_2_signed);
    register_unsigned_demos!(runner, demo_rem_power_of_2_assign_unsigned);
    register_signed_demos!(runner, demo_rem_power_of_2_assign_signed);
    register_unsigned_demos!(runner, demo_neg_mod_power_of_2);
    register_unsigned_demos!(runner, demo_neg_mod_power_of_2_assign);
    register_unsigned_signed_match_demos!(runner, demo_ceiling_mod_power_of_2);
    register_unsigned_signed_match_demos!(runner, demo_ceiling_mod_power_of_2_assign);

    register_unsigned_benches!(runner, benchmark_mod_power_of_2_unsigned);
    register_signed_benches!(runner, benchmark_mod_power_of_2_signed);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_assign_unsigned);
    register_signed_benches!(runner, benchmark_mod_power_of_2_assign_signed);
    register_unsigned_benches!(runner, benchmark_rem_power_of_2_unsigned);
    register_signed_benches!(runner, benchmark_rem_power_of_2_signed);
    register_unsigned_benches!(runner, benchmark_rem_power_of_2_assign_unsigned);
    register_signed_benches!(runner, benchmark_rem_power_of_2_assign_signed);
    register_unsigned_benches!(runner, benchmark_neg_mod_power_of_2);
    register_unsigned_benches!(runner, benchmark_neg_mod_power_of_2_assign);
    register_unsigned_signed_match_benches!(runner, benchmark_ceiling_mod_power_of_2);
    register_unsigned_signed_match_benches!(runner, benchmark_ceiling_mod_power_of_2_assign);
}

fn demo_mod_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, pow) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("{} ≡ {} mod 2^{}", n, n.mod_power_of_2(pow), pow);
    }
}

fn demo_mod_power_of_2_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize)
where
    <T as ModPowerOf2>::Output: PrimitiveUnsigned,
{
    for (n, pow) in signed_unsigned_pair_gen_var_10::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{} ≡ {} mod 2^{}", n, n.mod_power_of_2(pow), pow);
    }
}

fn demo_mod_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.mod_power_of_2_assign(pow);
        println!("n := {old_n}; n.mod_power_of_2_assign({pow}); n = {n}");
    }
}

fn demo_mod_power_of_2_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    <T as ModPowerOf2>::Output: PrimitiveUnsigned,
{
    for (mut n, pow) in signed_unsigned_pair_gen_var_4::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.mod_power_of_2_assign(pow);
        println!("n := {old_n}; n.mod_power_of_2_assign({pow}); n = {n}");
    }
}

fn demo_rem_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, pow) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.rem_power_of_2({}) = {}", n, pow, n.rem_power_of_2(pow));
    }
}

fn demo_rem_power_of_2_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.rem_power_of_2({}) = {}", n, pow, n.rem_power_of_2(pow));
    }
}

fn demo_rem_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.rem_power_of_2_assign(pow);
        println!("n := {old_n}; n.rem_power_of_2_assign({pow}); n = {n}");
    }
}

fn demo_rem_power_of_2_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.rem_power_of_2_assign(pow);
        println!("n := {old_n}; n.rem_power_of_2_assign({pow}); n = {n}");
    }
}

fn demo_neg_mod_power_of_2<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (n, pow) in unsigned_pair_gen_var_20::<T>().get(gm, config).take(limit) {
        println!(
            "{}.neg_mod_power_of_2({}) = {}",
            n,
            pow,
            n.neg_mod_power_of_2(pow)
        );
    }
}

fn demo_neg_mod_power_of_2_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow) in unsigned_pair_gen_var_20::<T>().get(gm, config).take(limit) {
        let old_n = n;
        n.neg_mod_power_of_2_assign(pow);
        println!("n := {old_n}; n.neg_mod_power_of_2_assign({pow}); n = {n}");
    }
}

fn demo_ceiling_mod_power_of_2<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, pow) in signed_unsigned_pair_gen_var_11::<U, S>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).ceiling_mod_power_of_2({}) = {}",
            n,
            pow,
            n.ceiling_mod_power_of_2(pow)
        );
    }
}

fn demo_ceiling_mod_power_of_2_assign<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut n, pow) in signed_unsigned_pair_gen_var_11::<U, S>()
        .get(gm, config)
        .take(limit)
    {
        let old_n = n;
        n.ceiling_mod_power_of_2_assign(pow);
        println!("n := {old_n}; n.ceiling_mod_power_of_2_assign({pow}); n = {n}");
    }
}

fn benchmark_mod_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| no_out!(n.mod_power_of_2(pow)))],
    );
}

fn benchmark_mod_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    <T as ModPowerOf2>::Output: PrimitiveUnsigned,
{
    run_benchmark(
        &format!("{}.mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_10::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| no_out!(n.mod_power_of_2(pow)))],
    );
}

fn benchmark_mod_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.mod_power_of_2_assign(pow)
        })],
    );
}

fn benchmark_mod_power_of_2_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.mod_power_of_2_assign(pow)
        })],
    );
}

fn benchmark_rem_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rem_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| no_out!(n.rem_power_of_2(pow)))],
    );
}

fn benchmark_rem_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    <T as ModPowerOf2>::Output: PrimitiveUnsigned,
{
    run_benchmark(
        &format!("{}.rem_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| no_out!(n.rem_power_of_2(pow)))],
    );
}

fn benchmark_rem_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rem_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.rem_power_of_2_assign(pow)
        })],
    );
}

fn benchmark_rem_power_of_2_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.rem_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.rem_power_of_2_assign(pow)
        })],
    );
}

fn benchmark_neg_mod_power_of_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.neg_mod_power_of_2(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_20::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.neg_mod_power_of_2(pow))
        })],
    );
}

fn benchmark_neg_mod_power_of_2_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.neg_mod_power_of_2_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_20::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.neg_mod_power_of_2_assign(pow)
        })],
    );
}

fn benchmark_ceiling_mod_power_of_2<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_mod_power_of_2(u64)", S::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_11::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, pow)| {
            no_out!(n.ceiling_mod_power_of_2(pow))
        })],
    );
}

fn benchmark_ceiling_mod_power_of_2_assign<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_mod_power_of_2_assign(u64)", S::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_11::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [("Malachite", &mut |(mut n, pow)| {
            n.ceiling_mod_power_of_2_assign(pow)
        })],
    );
}
