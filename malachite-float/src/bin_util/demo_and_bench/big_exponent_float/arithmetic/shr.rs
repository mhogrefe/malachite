// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shr, ShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::{BenchmarkType, run_benchmark};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::bench::bucketers::pair_1_big_exponent_float_complexity_bucketer;
use malachite_float::test_util::generators::{
    big_exponent_float_signed_pair_gen, big_exponent_float_unsigned_pair_gen,
};

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_big_exponent_float_shr_unsigned);
    register_signed_demos!(runner, demo_big_exponent_float_shr_signed);

    register_unsigned_benches!(
        runner,
        benchmark_big_exponent_float_shr_unsigned_evaluation_strategy
    );
    register_signed_benches!(
        runner,
        benchmark_big_exponent_float_shr_signed_evaluation_strategy
    );
}

fn demo_big_exponent_float_shr_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a BigExponentFloat: Shr<T, Output = BigExponentFloat>,
{
    for (x, k) in big_exponent_float_unsigned_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{x:#x} >> {k} = {:#x}", &x >> k);
    }
}

fn benchmark_big_exponent_float_shr_unsigned_evaluation_strategy<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BigExponentFloat: Shr<T, Output = BigExponentFloat> + ShrAssign<T>,
    for<'a> &'a BigExponentFloat: Shr<T, Output = BigExponentFloat>,
{
    run_benchmark(
        &format!("BigExponentFloat >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_unsigned_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [
            (
                &format!("BigExponentFloat >> {}", T::NAME),
                &mut |(x, k)| {
                    no_out!(x >> k);
                },
            ),
            (
                &format!("&BigExponentFloat >> {}", T::NAME),
                &mut |(x, k)| {
                    no_out!(&x >> k);
                },
            ),
            (
                &format!("BigExponentFloat >>= {}", T::NAME),
                &mut |(mut x, k)| {
                    x >>= k;
                },
            ),
        ],
    );
}

fn demo_big_exponent_float_shr_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) where
    for<'a> &'a BigExponentFloat: Shr<T, Output = BigExponentFloat>,
{
    for (x, k) in big_exponent_float_signed_pair_gen::<T>()
        .get(gm, config)
        .take(limit)
    {
        println!("{x:#x} >> {k} = {:#x}", &x >> k);
    }
}

fn benchmark_big_exponent_float_shr_signed_evaluation_strategy<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) where
    BigExponentFloat: Shr<T, Output = BigExponentFloat> + ShrAssign<T>,
    for<'a> &'a BigExponentFloat: Shr<T, Output = BigExponentFloat>,
{
    run_benchmark(
        &format!("BigExponentFloat >> {}", T::NAME),
        BenchmarkType::EvaluationStrategy,
        big_exponent_float_signed_pair_gen::<T>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_1_big_exponent_float_complexity_bucketer("x"),
        &mut [
            (
                &format!("BigExponentFloat >> {}", T::NAME),
                &mut |(x, k)| {
                    no_out!(x >> k);
                },
            ),
            (
                &format!("&BigExponentFloat >> {}", T::NAME),
                &mut |(x, k)| {
                    no_out!(&x >> k);
                },
            ),
            (
                &format!("BigExponentFloat >>= {}", T::NAME),
                &mut |(mut x, k)| {
                    x >>= k;
                },
            ),
        ],
    );
}
