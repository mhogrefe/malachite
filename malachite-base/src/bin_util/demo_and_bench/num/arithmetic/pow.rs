// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::{
    pair_1_bit_bucketer, pair_1_primitive_float_bucketer,
};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_pair_gen, primitive_float_signed_pair_gen, signed_unsigned_pair_gen_var_15,
    unsigned_pair_gen_var_29,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_pow_assign_unsigned);
    register_signed_demos!(runner, demo_pow_assign_signed);
    register_primitive_float_demos!(runner, demo_pow_assign_i64_primitive_float);
    register_primitive_float_demos!(runner, demo_pow_assign_primitive_float_primitive_float);

    register_unsigned_benches!(runner, benchmark_pow_assign_unsigned);
    register_signed_benches!(runner, benchmark_pow_assign_signed);
    register_primitive_float_benches!(runner, benchmark_pow_assign_i64_primitive_float);
    register_primitive_float_benches!(runner, benchmark_pow_assign_primitive_float_primitive_float);
}

fn demo_pow_assign_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in unsigned_pair_gen_var_29::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.pow_assign(y);
        println!("x := {old_x}; x.pow_assign({y}); x = {x}");
    }
}

fn demo_pow_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_unsigned_pair_gen_var_15::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.pow_assign(y);
        println!("x := {old_x}; x.pow_assign({y}); x = {x}");
    }
}

fn demo_pow_assign_i64_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in primitive_float_signed_pair_gen::<T, i64>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        x.pow_assign(y);
        println!(
            "x := {}; x.pow_assign({}); x = {}",
            NiceFloat(old_x),
            y,
            NiceFloat(x)
        );
    }
}

fn demo_pow_assign_primitive_float_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in primitive_float_pair_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.pow_assign(y);
        println!(
            "x := {}; x.pow_assign({}); x = {}",
            NiceFloat(old_x),
            NiceFloat(y),
            NiceFloat(x)
        );
    }
}

fn benchmark_pow_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.pow_assign(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_29::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, y)| x.pow_assign(y))],
    );
}

fn benchmark_pow_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.pow_assign(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_15::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, y)| x.pow_assign(y))],
    );
}

fn benchmark_pow_assign_i64_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.pow_assign(i64)", T::NAME),
        BenchmarkType::Single,
        primitive_float_signed_pair_gen::<T, i64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, y)| x.pow_assign(y))],
    );
}

fn benchmark_pow_assign_primitive_float_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.pow_assign({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, y)| x.pow_assign(y))],
    );
}
