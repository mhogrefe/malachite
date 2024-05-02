// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_pair_gen_var_4, signed_pair_gen_var_6, unsigned_pair_gen_var_12,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_unsigned);
    register_signed_demos!(runner, demo_mod_signed);
    register_unsigned_demos!(runner, demo_mod_assign_unsigned);
    register_signed_demos!(runner, demo_mod_assign_signed);
    register_unsigned_demos!(runner, demo_neg_mod);
    register_unsigned_demos!(runner, demo_neg_mod_assign);
    register_signed_demos!(runner, demo_ceiling_mod);
    register_signed_demos!(runner, demo_ceiling_mod_assign);

    register_unsigned_benches!(runner, benchmark_mod_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_mod_signed_algorithms);
    register_unsigned_benches!(runner, benchmark_mod_assign_unsigned);
    register_signed_benches!(runner, benchmark_mod_assign_signed);
    register_unsigned_benches!(runner, benchmark_neg_mod_algorithms);
    register_unsigned_benches!(runner, benchmark_neg_mod_assign);
    register_signed_benches!(runner, benchmark_ceiling_mod_algorithms);
    register_signed_benches!(runner, benchmark_ceiling_mod_assign);
}

fn demo_mod_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.mod_op({}) = {}", x, y, x.mod_op(y));
    }
}

fn demo_mod_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_6::<T>().get(gm, config).take(limit) {
        println!("({}).mod_op({}) = {}", x, y, x.mod_op(y));
    }
}

fn demo_mod_assign_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.mod_assign(y);
        println!("x := {old_x}; x.mod_assign({y}); x = {x}");
    }
}

fn demo_mod_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_pair_gen_var_6::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.mod_assign(y);
        println!("x := {old_x}; x.mod_assign({y}); x = {x}");
    }
}

fn demo_neg_mod<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.neg_mod({}) = {}", x, y, x.neg_mod(y));
    }
}

fn demo_neg_mod_assign<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.neg_mod_assign(y);
        println!("x := {old_x}; x.neg_mod_assign({y}); x = {x}");
    }
}

fn demo_ceiling_mod<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_6::<T>().get(gm, config).take(limit) {
        println!("({}).ceiling_mod({}) = {}", x, y, x.ceiling_mod(y));
    }
}

fn demo_ceiling_mod_assign<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_pair_gen_var_6::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.ceiling_mod_assign(y);
        println!("x := {old_x}; x.ceiling_mod_assign({y}); x = {x}");
    }
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_mod_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using mod", &mut |(x, y)| no_out!(x.mod_op(y))),
            ("using div_mod", &mut |(x, y)| no_out!(x.div_mod(y).1)),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_mod_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using mod", &mut |(x, y)| no_out!(x.mod_op(y))),
            ("using div_mod", &mut |(x, y)| no_out!(x.div_mod(y).1)),
        ],
    );
}

fn benchmark_mod_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.mod_assign(y))],
    );
}

fn benchmark_mod_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.mod_assign(y))],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_neg_mod_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.neg_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using neg_mod", &mut |(x, y)| no_out!(x.neg_mod(y))),
            ("using ceiling_div_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_neg_mod(y).1)
            }),
        ],
    );
}

fn benchmark_neg_mod_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.neg_mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.neg_mod_assign(y))],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_ceiling_mod_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using ceiling_mod", &mut |(x, y)| no_out!(x.ceiling_mod(y))),
            ("using ceiling_div_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_mod(y).1)
            }),
        ],
    );
}

fn benchmark_ceiling_mod_assign<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_mod_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_6::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.ceiling_mod_assign(y))],
    );
}
