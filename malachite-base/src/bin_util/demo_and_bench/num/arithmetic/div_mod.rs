// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::bench::bucketers::pair_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{signed_pair_gen_var_4, unsigned_pair_gen_var_12};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_div_mod_unsigned);
    register_signed_demos!(runner, demo_div_mod_signed);
    register_unsigned_demos!(runner, demo_div_assign_mod_unsigned);
    register_signed_demos!(runner, demo_div_assign_mod_signed);
    register_unsigned_demos!(runner, demo_div_rem_unsigned);
    register_signed_demos!(runner, demo_div_rem_signed);
    register_unsigned_demos!(runner, demo_div_assign_rem_unsigned);
    register_signed_demos!(runner, demo_div_assign_rem_signed);
    register_unsigned_demos!(runner, demo_ceiling_div_neg_mod);
    register_unsigned_demos!(runner, demo_ceiling_div_assign_neg_mod);
    register_signed_demos!(runner, demo_ceiling_div_mod);
    register_signed_demos!(runner, demo_ceiling_div_assign_mod);

    register_unsigned_benches!(runner, benchmark_div_mod_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_div_mod_signed_algorithms);
    register_unsigned_benches!(runner, benchmark_div_assign_mod_unsigned);
    register_signed_benches!(runner, benchmark_div_assign_mod_signed);
    register_unsigned_benches!(runner, benchmark_div_rem_unsigned_algorithms);
    register_signed_benches!(runner, benchmark_div_rem_signed_algorithms);
    register_unsigned_benches!(runner, benchmark_div_assign_rem_unsigned);
    register_signed_benches!(runner, benchmark_div_assign_rem_signed);
    register_unsigned_benches!(runner, benchmark_ceiling_div_neg_mod_algorithms);
    register_unsigned_benches!(runner, benchmark_ceiling_div_assign_neg_mod);
    register_signed_benches!(runner, benchmark_ceiling_div_mod_algorithms);
    register_signed_benches!(runner, benchmark_ceiling_div_assign_mod);
}

fn demo_div_mod_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.div_mod({}) = {:?}", x, y, x.div_mod(y));
    }
}

fn demo_div_mod_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_4::<T>().get(gm, config).take(limit) {
        println!("({}).div_mod({}) = {:?}", x, y, x.div_mod(y));
    }
}

fn demo_div_assign_mod_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let r = x.div_assign_mod(y);
        println!("x := {old_x}; x.div_assign_mod({y}) = {r}; x = {x}");
    }
}

fn demo_div_assign_mod_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_pair_gen_var_4::<T>().get(gm, config).take(limit) {
        let old_x = x;
        let r = x.div_assign_mod(y);
        println!("x := {old_x}; x.div_assign_mod({y}) = {r}; x = {x}");
    }
}

fn demo_div_rem_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{}.div_rem({}) = {:?}", x, y, x.div_rem(y));
    }
}

fn demo_div_rem_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_4::<T>().get(gm, config).take(limit) {
        println!("({}).div_rem({}) = {:?}", x, y, x.div_rem(y));
    }
}

fn demo_div_assign_rem_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let r = x.div_assign_rem(y);
        println!("x := {old_x}; x.div_assign_rem({y}) = {r}; x = {x}");
    }
}

fn demo_div_assign_rem_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_pair_gen_var_4::<T>().get(gm, config).take(limit) {
        let old_x = x;
        let r = x.div_assign_rem(y);
        println!("x := {old_x}; x.div_assign_rem({y}) = {r}; x = {x}");
    }
}

fn demo_ceiling_div_neg_mod<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.ceiling_div_neg_mod({}) = {:?}",
            x,
            y,
            x.ceiling_div_neg_mod(y)
        );
    }
}

fn demo_ceiling_div_assign_neg_mod<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in unsigned_pair_gen_var_12::<T, T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let r = x.ceiling_div_assign_neg_mod(y);
        println!("x := {old_x}; x.ceiling_div_assign_neg_mod({y}) = {r}; x = {x}");
    }
}

fn demo_ceiling_div_mod<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen_var_4::<T>().get(gm, config).take(limit) {
        println!(
            "({}).ceiling_div_mod({}) = {:?}",
            x,
            y,
            x.ceiling_div_mod(y)
        );
    }
}

fn demo_ceiling_div_assign_mod<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_pair_gen_var_4::<T>().get(gm, config).take(limit) {
        let old_x = x;
        let r = x.ceiling_div_assign_mod(y);
        println!("x := {old_x}; x.ceiling_div_assign_mod({y}) = {r}; x = {x}");
    }
}

#[allow(clippy::no_effect)]
fn benchmark_div_mod_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using div_mod", &mut |(x, y)| no_out!(x.div_mod(y))),
            ("using / and %", &mut |(x, y)| no_out!((x / y, x % y))),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_div_mod_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using div_mod", &mut |(x, y)| no_out!(x.div_mod(y))),
            ("using div_round and mod_op", &mut |(x, y)| {
                no_out!((x.div_round(y, Floor), x.mod_op(y)))
            }),
        ],
    );
}

fn benchmark_div_assign_mod_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_assign_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| no_out!(x.div_assign_mod(y)))],
    );
}

fn benchmark_div_assign_mod_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_assign_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| no_out!(x.div_assign_mod(y)))],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_div_rem_unsigned_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_rem({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using div_rem", &mut |(x, y)| no_out!(x.div_rem(y))),
            ("using / and %", &mut |(x, y)| no_out!((x / y, x % y))),
        ],
    );
}

#[allow(clippy::no_effect)]
fn benchmark_div_rem_signed_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_rem({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using div_rem", &mut |(x, y)| no_out!(x.div_rem(y))),
            ("using / and %", &mut |(x, y)| no_out!((x / y, x % y))),
        ],
    );
}

fn benchmark_div_assign_rem_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_assign_rem({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| no_out!(x.div_assign_rem(y)))],
    );
}

fn benchmark_div_assign_rem_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.div_assign_rem({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| no_out!(x.div_assign_rem(y)))],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_ceiling_div_neg_mod_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_div_neg_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using ceiling_div_neg_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_neg_mod(y))
            }),
            ("using div_round and neg_mod", &mut |(x, y)| {
                no_out!((x.div_round(y, Ceiling), x.neg_mod(y)))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_ceiling_div_mod_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_div_mod({})", T::NAME, T::NAME),
        BenchmarkType::Algorithms,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [
            ("using ceiling_div_mod", &mut |(x, y)| {
                no_out!(x.ceiling_div_mod(y))
            }),
            ("using div_round and ceiling_mod", &mut |(x, y)| {
                no_out!((x.div_round(y, Ceiling), x.ceiling_mod(y)))
            }),
        ],
    );
}

fn benchmark_ceiling_div_assign_neg_mod<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_div_assign_neg_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_12::<T, T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| {
            no_out!(x.ceiling_div_assign_neg_mod(y))
        })],
    );
}

fn benchmark_ceiling_div_assign_mod<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_div_assign_mod({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_4::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| {
            no_out!(x.ceiling_div_assign_mod(y))
        })],
    );
}
