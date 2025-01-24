// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShr, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_unsigned_abs_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_pair_gen_var_2, unsigned_signed_pair_gen_var_1,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_signed_demos!(runner, demo_arithmetic_checked_shr_unsigned_signed);
    register_signed_signed_demos!(runner, demo_arithmetic_checked_shr_signed_signed);
    register_unsigned_signed_benches!(runner, benchmark_arithmetic_checked_shr_unsigned_signed);
    register_signed_signed_benches!(runner, benchmark_arithmetic_checked_shr_signed_signed);
}

fn demo_arithmetic_checked_shr_unsigned_signed<
    T: ArithmeticCheckedShr<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in unsigned_signed_pair_gen_var_1::<T, U>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.arithmetic_checked_shr({}) = {:?}",
            n,
            i,
            n.arithmetic_checked_shr(i)
        );
    }
}

fn demo_arithmetic_checked_shr_signed_signed<
    T: ArithmeticCheckedShr<U, Output = T> + PrimitiveSigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, i) in signed_pair_gen_var_2::<T, U>().get(gm, config).take(limit) {
        println!(
            "({}).arithmetic_checked_shr({}) = {:?}",
            n,
            i,
            n.arithmetic_checked_shr(i)
        );
    }
}

fn benchmark_arithmetic_checked_shr_unsigned_signed<
    T: ArithmeticCheckedShr<U, Output = T> + PrimitiveUnsigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<U as UnsignedAbs>::Output>,
{
    run_benchmark(
        &format!("{}.arithmetic_checked_shr({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        unsigned_signed_pair_gen_var_1::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_unsigned_abs_bucketer("other"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.arithmetic_checked_shr(y))
        })],
    );
}

fn benchmark_arithmetic_checked_shr_signed_signed<
    T: ArithmeticCheckedShr<U, Output = T> + PrimitiveSigned,
    U: PrimitiveSigned,
>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    usize: TryFrom<<U as UnsignedAbs>::Output>,
{
    run_benchmark(
        &format!("{}.arithmetic_checked_shr({})", T::NAME, U::NAME),
        BenchmarkType::Single,
        signed_pair_gen_var_2::<T, U>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_unsigned_abs_bucketer("other"),
        &mut [("Malachite", &mut |(x, y)| {
            no_out!(x.arithmetic_checked_shr(y))
        })],
    );
}
