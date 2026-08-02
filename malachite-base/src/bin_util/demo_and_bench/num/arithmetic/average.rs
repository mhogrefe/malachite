// Copyright © 2026 Mikhail Hogrefe
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
    pair_max_bit_bucketer, pair_max_primitive_float_bucketer, triple_1_2_max_bit_bucketer,
};
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    primitive_float_pair_gen, signed_pair_gen, signed_signed_rounding_mode_triple_gen_var_5,
    unsigned_pair_gen_var_27, unsigned_unsigned_rounding_mode_triple_gen_var_9,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_average_unsigned);
    register_signed_demos!(runner, demo_average_signed);
    register_primitive_float_demos!(runner, demo_average_primitive_float);
    register_primitive_float_demos!(runner, demo_average_assign_primitive_float);
    register_unsigned_demos!(runner, demo_average_assign_unsigned);
    register_signed_demos!(runner, demo_average_assign_signed);
    register_unsigned_demos!(runner, demo_average_round_unsigned);
    register_signed_demos!(runner, demo_average_round_signed);
    register_unsigned_demos!(runner, demo_average_round_assign_unsigned);
    register_signed_demos!(runner, demo_average_round_assign_signed);

    register_unsigned_benches!(runner, benchmark_average_unsigned);
    register_signed_benches!(runner, benchmark_average_signed);
    register_primitive_float_benches!(runner, benchmark_average_primitive_float);
    register_unsigned_benches!(runner, benchmark_average_round_unsigned);
    register_signed_benches!(runner, benchmark_average_round_signed);
}

fn demo_average_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        println!("{}.average({}) = {}", x, y, x.average(y));
    }
}

fn demo_average_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, config).take(limit) {
        println!("({}).average({}) = {}", x, y, x.average(y));
    }
}

fn demo_average_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in unsigned_pair_gen_var_27::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.average_assign(y);
        println!("x := {old_x}; x.average_assign({y}); x = {x}");
    }
}

fn demo_average_assign_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in signed_pair_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.average_assign(y);
        println!("x := {old_x}; x.average_assign({y}); x = {x}");
    }
}

fn demo_average_round_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_9::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.average_round({}, {}) = {:?}",
            x,
            y,
            rm,
            x.average_round(y, rm)
        );
    }
}

fn demo_average_round_signed<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y, rm) in signed_signed_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).average_round({}, {}) = {:?}",
            x,
            y,
            rm,
            x.average_round(y, rm)
        );
    }
}

fn demo_average_round_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_9::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let o = x.average_round_assign(y, rm);
        println!("x := {old_x}; x.average_round_assign({y}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_average_round_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, rm) in signed_signed_rounding_mode_triple_gen_var_5::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let o = x.average_round_assign(y, rm);
        println!("x := {old_x}; x.average_round_assign({y}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_average_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.average({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_27::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.average(y)))],
    );
}

fn benchmark_average_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.average({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.average(y)))],
    );
}

fn benchmark_average_round_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.average_round({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_9::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, rm)| {
            no_out!(x.average_round(y, rm));
        })],
    );
}

fn benchmark_average_round_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.average_round({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_signed_rounding_mode_triple_gen_var_5::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, rm)| {
            no_out!(x.average_round(y, rm));
        })],
    );
}

fn demo_average_primitive_float<T: PrimitiveFloat>(gm: GenMode, config: &GenConfig, limit: usize) {
    for (x, y) in primitive_float_pair_gen::<T>().get(gm, config).take(limit) {
        println!(
            "({}).average({}) = {}",
            NiceFloat(x),
            NiceFloat(y),
            NiceFloat(x.average(y))
        );
    }
}

fn demo_average_assign_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y) in primitive_float_pair_gen::<T>().get(gm, config).take(limit) {
        let old_x = x;
        x.average_assign(y);
        println!(
            "x := {}; x.average_assign({}); x = {}",
            NiceFloat(old_x),
            NiceFloat(y),
            NiceFloat(x)
        );
    }
}

fn benchmark_average_primitive_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.average({})", T::NAME, T::NAME),
        BenchmarkType::Single,
        primitive_float_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_max_primitive_float_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y)| no_out!(x.average(y)))],
    );
}
