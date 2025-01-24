// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::test_util::bench::bucketers::triple_1_2_max_bit_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_signed_rounding_mode_triple_gen_var_2, unsigned_unsigned_rounding_mode_triple_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_round_to_multiple_unsigned);
    register_unsigned_demos!(runner, demo_round_to_multiple_assign_unsigned);
    register_signed_unsigned_match_demos!(runner, demo_round_to_multiple_signed);
    register_signed_unsigned_match_demos!(runner, demo_round_to_multiple_assign_signed);

    register_unsigned_benches!(runner, benchmark_round_to_multiple_unsigned);
    register_unsigned_benches!(runner, benchmark_round_to_multiple_assign_unsigned);
    register_signed_unsigned_match_benches!(runner, benchmark_round_to_multiple_signed);
    register_signed_unsigned_match_benches!(runner, benchmark_round_to_multiple_assign_signed);
}

fn demo_round_to_multiple_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.round_to_multiple({}, {}) = {:?}",
            x,
            y,
            rm,
            x.round_to_multiple(y, rm)
        );
    }
}

fn demo_round_to_multiple_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, rm) in unsigned_unsigned_rounding_mode_triple_gen_var_2::<T>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let o = x.round_to_multiple_assign(y, rm);
        println!("x := {old_x}; x.round_to_multiple_assign({y}, {rm}) = {o:?}; x = {x}");
    }
}

fn demo_round_to_multiple_signed<
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (x, y, rm) in signed_signed_rounding_mode_triple_gen_var_2::<U, S>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "({}).round_to_multiple({}, {}) = {:?}",
            x,
            y,
            rm,
            x.round_to_multiple(y, rm)
        );
    }
}

fn demo_round_to_multiple_assign_signed<
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (mut x, y, rm) in signed_signed_rounding_mode_triple_gen_var_2::<U, S>()
        .get(gm, config)
        .take(limit)
    {
        let old_x = x;
        let o = x.round_to_multiple_assign(y, rm);
        println!("x := {old_x}; x.round_to_multiple_assign({y}, {rm}) = {o:?}; x = {x}");
    }
}

fn benchmark_round_to_multiple_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.round_to_multiple({}, RoundingMode)", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, rm)| {
            no_out!(x.round_to_multiple(y, rm))
        })],
    );
}

fn benchmark_round_to_multiple_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.round_to_multiple_assign({}, RoundingMode)",
            T::NAME,
            T::NAME
        ),
        BenchmarkType::Single,
        unsigned_unsigned_rounding_mode_triple_gen_var_2::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.round_to_multiple_assign(y, rm))
        })],
    );
}

fn benchmark_round_to_multiple_signed<
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.round_to_multiple({}, RoundingMode)", S::NAME, S::NAME),
        BenchmarkType::Single,
        signed_signed_rounding_mode_triple_gen_var_2::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(x, y, rm)| {
            no_out!(x.round_to_multiple(y, rm))
        })],
    );
}

fn benchmark_round_to_multiple_assign_signed<
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
    U: PrimitiveUnsigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!(
            "{}.round_to_multiple_assign({}, RoundingMode)",
            S::NAME,
            S::NAME
        ),
        BenchmarkType::Single,
        signed_signed_rounding_mode_triple_gen_var_2::<U, S>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_1_2_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(mut x, y, rm)| {
            no_out!(x.round_to_multiple_assign(y, rm))
        })],
    );
}
