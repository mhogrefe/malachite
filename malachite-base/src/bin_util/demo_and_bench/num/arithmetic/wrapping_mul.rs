// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_wrapping_mul_unsigned);
    register_signed_demos!(runner, demo_wrapping_mul_signed);
    register_unsigned_demos!(runner, demo_wrapping_mul_assign_unsigned);
    register_signed_demos!(runner, demo_wrapping_mul_assign_signed);

    register_unsigned_benches!(runner, benchmark_wrapping_mul_unsigned);
    register_signed_benches!(runner, benchmark_wrapping_mul_signed);
    register_unsigned_benches!(runner, benchmark_wrapping_mul_assign_unsigned);
    register_signed_benches!(runner, benchmark_wrapping_mul_assign_signed);
}

fn demo_wrapping_mul_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        println!("{}.wrapping_mul({}) = {}", x, y, x.wrapping_mul(y));
    }
}

fn demo_wrapping_mul_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, config).take(limit) {
        println!("({}).wrapping_mul({}) = {}", x, y, x.wrapping_mul(y));
    }
}

fn demo_wrapping_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.wrapping_mul_assign(y);
        println!("x := {old_x}; x.wrapping_mul_assign({y}); x = {x}");
    }
}

fn demo_wrapping_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in signed_pair_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.wrapping_mul_assign(y);
        println!("x := {old_x}; x.wrapping_mul_assign({y}); x = {x}");
    }
}

fn benchmark_wrapping_mul_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_mul({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.wrapping_mul(y)))],
    );
}

fn benchmark_wrapping_mul_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_mul({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.wrapping_mul(y)))],
    );
}

fn benchmark_wrapping_mul_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_mul_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.wrapping_mul_assign(y))],
    );
}

fn benchmark_wrapping_mul_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_mul_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y)| x.wrapping_mul_assign(y))],
    );
}
