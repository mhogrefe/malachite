// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::triple_1_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_rounding_mode_triple_gen_var_1,
    unsigned_unsigned_rounding_mode_triple_gen_var_3,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_round_to_multiple_of_power_of_2_unsigned);
    register_unsigned_demos!(runner, demo_round_to_multiple_of_power_of_2_assign_unsigned);
    register_signed_demos!(runner, demo_round_to_multiple_of_power_of_2_signed);
    register_signed_demos!(runner, demo_round_to_multiple_of_power_of_2_assign_signed);

    register_unsigned_benches!(runner, benchmark_round_to_multiple_of_power_of_2_unsigned);
    register_unsigned_benches!(
        runner,
        benchmark_round_to_multiple_of_power_of_2_assign_unsigned
    );
    register_signed_benches!(runner, benchmark_round_to_multiple_of_power_of_2_signed);
    register_signed_benches!(
        runner,
        benchmark_round_to_multiple_of_power_of_2_assign_signed
    );
}

fn demo_round_to_multiple_of_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.round_to_multiple_of_power_of_2({}, {}) = {:?}",
            x,
            pow,
            rm,
            x.round_to_multiple_of_power_of_2(pow, rm)
        );
    }
}

fn demo_round_to_multiple_of_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_3::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let o = x.round_to_multiple_of_power_of_2_assign(pow, rm);
        println!(
            "x := {old_x}; x.round_to_multiple_of_power_of_2_assign({pow}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn demo_round_to_multiple_of_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, pow, rm) in signed_unsigned_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).round_to_multiple_of_power_of_2({}, {}) = {:?}",
            x,
            pow,
            rm,
            x.round_to_multiple_of_power_of_2(pow, rm)
        );
    }
}

fn demo_round_to_multiple_of_power_of_2_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, pow, rm) in signed_unsigned_rounding_mode_triple_gen_var_1::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let o = x.round_to_multiple_of_power_of_2_assign(pow, rm);
        println!(
            "x := {old_x}; x.round_to_multiple_of_power_of_2_assign({pow}, {rm}) = {o:?}; x = {x}"
        );
    }
}

fn benchmark_round_to_multiple_of_power_of_2_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.round_to_multiple_of_power_of_2({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y, rm)| {
            no_out!(x.round_to_multiple_of_power_of_2(y, rm))
        })],
    );
}

fn benchmark_round_to_multiple_of_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.round_to_multiple_of_power_of_2_assign({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_3::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.round_to_multiple_of_power_of_2_assign(y, rm))
        })],
    );
}

fn benchmark_round_to_multiple_of_power_of_2_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.round_to_multiple_of_power_of_2({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        signed_unsigned_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, y, rm)| {
            no_out!(x.round_to_multiple_of_power_of_2(y, rm))
        })],
    );
}

fn benchmark_round_to_multiple_of_power_of_2_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.round_to_multiple_of_power_of_2_assign({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        signed_unsigned_rounding_mode_triple_gen_var_1::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.round_to_multiple_of_power_of_2_assign(y, rm))
        })],
    );
}
